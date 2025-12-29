use std::{
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use log::{debug, error, trace};
use reqwest::header::HeaderMap;
use serde_json::Value;
use sha1::{Digest, Sha1};

use crate::{
    endpoint::Endpoint,
    json_extractor::{
        Continuation, extract_playlist_info, from_json, get_continuation, get_playlist, get_video,
        get_video_from_album,
    },
    types::{Result, YoutubeMusicError, YoutubeMusicPlaylistRef, YoutubeMusicVideoRef},
    utils::StringUtils,
};

const YT_DOMAIN: &str = "https://music.youtube.com";

#[derive(Debug)]
pub struct YoutubeMusicInstance {
    sapisid: String,
    innertube_api_key: String,
    client_version: String,
    cookies: String,
    account_id: Option<String>,
}

impl YoutubeMusicInstance {
    pub async fn new(headers: HeaderMap, account_id: Option<String>) -> Result<Self> {
        trace!("Creating new YoutubeMusicInstance");

        let rest_client = reqwest::ClientBuilder::default()
            .default_headers(headers.clone())
            .build()
            .map_err(YoutubeMusicError::RequestError)?;

        trace!("Fetching YoutubeMusic homepage");

        let response: String = rest_client
            .get(YT_DOMAIN)
            .headers(headers.clone())
            .send()
            .await
            .map_err(YoutubeMusicError::RequestError)?
            .text()
            .await
            .map_err(YoutubeMusicError::RequestError)?;

        if response.contains("<base href=\"https://accounts.google.com/v3/signin/\">")
            || response.contains("<base href=\"https://consent.youtube.com/\"")
        {
            error!("Need to login");
            return Err(YoutubeMusicError::NeedToLogin);
        }

        trace!("Parsing cookies");
        let cookies = headers
            .get("Cookie")
            .ok_or(YoutubeMusicError::NoCookieAttribute)?;

        let cookie_bytes = cookies.as_bytes();
        let cookies = String::from_utf8(cookie_bytes.to_vec())
            .map_err(YoutubeMusicError::InvalidCookie)?
            .to_string();

        let sapisid = cookies
            .between("SAPISID=", ";")
            .ok_or_else(|| YoutubeMusicError::NoSapsidInCookie)?;

        let innertube_api_key = response
            .between("INNERTUBE_API_KEY\":\"", "\"")
            .ok_or_else(|| YoutubeMusicError::CantFindInnerTubeApiKey(response.to_string()))?;

        trace!("Innertube API key: {}", innertube_api_key);

        let client_version = response
            .between("INNERTUBE_CLIENT_VERSION\":\"", "\"")
            .ok_or_else(|| {
                YoutubeMusicError::CantFindInnerTubeClientVersion(response.to_string())
            })?;

        trace!("Innertube client version: {}", client_version);
        trace!("account id {:?}", account_id);

        Ok(Self {
            sapisid: sapisid.to_string(),
            innertube_api_key: innertube_api_key.to_string(),
            client_version: client_version.to_string(),
            cookies,
            account_id,
        })
    }

    pub async fn from_header_file(path: &Path) -> Result<Self> {
        let mut headers = HeaderMap::new();
        for header in tokio::fs::read_to_string(path)
            .await
            .map_err(YoutubeMusicError::IoError)?
            .lines()
        {
            if let Some((key, value)) = header.split_once(": ") {
                headers.insert(
                    match key.to_lowercase().as_str() {
                        "cookie" => reqwest::header::COOKIE,
                        "user-agent" => reqwest::header::USER_AGENT,
                        _ => {
                            #[cfg(test)]
                            println!("Unknown header key: {key}");
                            continue;
                        }
                    },
                    value.parse().unwrap(),
                );
            }
        }

        if !headers.contains_key(reqwest::header::COOKIE) {
            return Err(YoutubeMusicError::InvalidHeaders);
        }

        if !headers.contains_key(reqwest::header::USER_AGENT) {
            headers.insert(
                reqwest::header::USER_AGENT,
                "Mozilla/5.0 (X11; Linux x86_64; rv:108.0) Gecko/20100101 Firefox/108.0"
                    .parse()
                    .unwrap(),
            );
        }

        let account_path = path
            .parent()
            .unwrap_or(Path::new("../"))
            .join("account_id.txt");

        let account_id = match tokio::fs::read_to_string(account_path).await {
            Ok(mut id) => {
                if id.ends_with("\n") {
                    id.pop();
                    if id.ends_with("\r") {
                        id.pop();
                    }
                }
                Some(id)
            }
            Err(_) => None,
        };

        Self::new(headers, account_id).await
    }

    pub async fn browse_raw(
        &self,
        endpoint_route: &str,
        endpoint_key: &str,
        endpoint_param: &str,
    ) -> Result<String> {
        trace!("Browse {endpoint_route}");
        let url = format!(
            "https://music.youtube.com/youtubei/v1/{endpoint_route}?key={}&prettyPrint=false",
            self.innertube_api_key
        );
        let body = match &self.account_id {
            Some(id) => format!(
                r#"{{"context":{{"client":{{"clientName":"WEB_REMIX","clientVersion":"{}"}},"user":{{"onBehalfOfUser":"{id}"}}}},"{endpoint_key}":"{endpoint_param}"}}"#,
                self.client_version,
            ),
            None => format!(
                r#"{{"context":{{"client":{{"clientName":"WEB_REMIX","clientVersion":"{}"}}}},"{endpoint_key}":"{endpoint_param}"}}"#,
                self.client_version
            ),
        };

        reqwest::Client::new()
            .post(&url)
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("SAPISIDHASH {}", self.compute_sapi_hash()),
            )
            .header("X-Origin", "https://music.youtube.com")
            .header("Cookie", &self.cookies)
            .body(body)
            .send()
            .await
            .map_err(YoutubeMusicError::RequestError)?
            .text()
            .await
            .map_err(YoutubeMusicError::RequestError)
    }

    pub async fn browse(
        &self,
        endpoint: &Endpoint,
        continuations: bool,
    ) -> Result<(serde_json::Value, Vec<Continuation>)> {
        let playlist_json: Value = serde_json::from_str(
            &self
                .browse_raw(
                    &endpoint.get_route(),
                    &endpoint.get_key(),
                    &endpoint.get_param(),
                )
                .await?,
        )
        .map_err(YoutubeMusicError::SerdeJson)?;
        debug!("Browse response: {playlist_json}");

        if playlist_json.get("error").is_some() {
            error!("Error in browse ({endpoint:?})");
            error!("{:?}", playlist_json);
            return Err(YoutubeMusicError::YoutubeMusicError(playlist_json));
        }

        let continuation = if continuations {
            from_json(&playlist_json, get_continuation)?
        } else {
            Vec::new()
        };

        Ok((playlist_json, continuation))
    }

    pub async fn browse_continuation_raw(
        &self,
        Continuation {
            continuation,
            click_tracking_params,
        }: &Continuation,
    ) -> Result<String> {
        trace!("Browse continuation {continuation}");

        let url = format!(
            "https://music.youtube.com/youtubei/v1/browse?ctoken={continuation}&continuation={continuation}&type=next&itct={click_tracking_params}&key={}&prettyPrint=false",
            self.innertube_api_key
        );

        let body = match &self.account_id {
            Some(id) => format!(
                r#"{{"context":{{"client":{{"clientName":"WEB_REMIX","clientVersion":"{}"}},"user":{{"onBehalfOfUser":"{id}"}}}}}}"#,
                self.client_version
            ),
            None => format!(
                r#"{{"context":{{"client":{{"clientName":"WEB_REMIX","clientVersion":"{}"}}}}}}"#,
                self.client_version
            ),
        };

        reqwest::Client::new()
            .post(&url)
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("SAPISIDHASH {}", self.compute_sapi_hash()),
            )
            .header("X-Origin", "https://music.youtube.com")
            .header("Cookie", &self.cookies)
            .body(body)
            .send()
            .await
            .map_err(YoutubeMusicError::RequestError)?
            .text()
            .await
            .map_err(YoutubeMusicError::RequestError)
    }

    pub async fn browse_continuation(
        &self,
        continuation: &Continuation,
        continuations: bool,
    ) -> Result<(Value, Vec<Continuation>)> {
        let playlist_json: Value =
            serde_json::from_str(&self.browse_continuation_raw(continuation).await?)
                .map_err(YoutubeMusicError::SerdeJson)?;

        debug!("Browse continuation response: {playlist_json}");
        if playlist_json.get("error").is_some() {
            error!("Error in browse_continuation");
            error!("{:?}", playlist_json);
            return Err(YoutubeMusicError::YoutubeMusicError(playlist_json));
        }
        let continuation = if continuations {
            from_json(&playlist_json, get_continuation)?
        } else {
            Vec::new()
        };
        Ok((playlist_json, continuation))
    }

    pub async fn get_library(
        &self,
        endpoint: &Endpoint,
        mut n_continuations: usize,
    ) -> Result<Vec<YoutubeMusicPlaylistRef>> {
        let (library_json, mut continuations) = self.browse(endpoint, n_continuations > 0).await?;

        trace!("Fetched library");
        debug!("Library response: {library_json}");
        debug!("Continuations: {continuations:?}");

        let mut library = from_json(&library_json, get_playlist)?;
        debug!("Library: {library:?}");

        while let Some(continuation) = continuations.pop() {
            n_continuations -= 1;
            trace!("Fetching continuation {continuation:?} ({endpoint:?})");

            let (library_json, new_continuations) = self
                .browse_continuation(&continuation, (n_continuations - 1) > 0)
                .await?;

            debug!("Library response: {library_json}");
            continuations.extend(new_continuations);

            let new_library = from_json(&library_json, get_playlist)?;
            trace!("Fetched {} playlists", new_library.len());
            debug!("Library response: {library_json}");

            library.extend(new_library);

            if n_continuations == 0 {
                break;
            }
        }

        Ok(library)
    }

    fn compute_sapi_hash(&self) -> String {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp = since_the_epoch.as_secs();

        let mut hasher = Sha1::new();
        hasher.update(format!("{timestamp} {} {YT_DOMAIN}", self.sapisid));
        let result = hasher.finalize();
        let mut hex = String::with_capacity(40);

        for byte in result {
            hex.push_str(&format!("{byte:02x}"));
        }

        trace!("Computed SAPI Hash{timestamp}_{hex}");
        format!("{timestamp}_{hex}")
    }
}

fn parse_playlist(playlist_json: &Value) -> Result<Vec<YoutubeMusicVideoRef>> {
    let mut videos = from_json(playlist_json, get_video)?;
    let info = extract_playlist_info(playlist_json);

    for mut video in from_json(playlist_json, get_video_from_album)? {
        if videos.iter().any(|x| x.video_id == video.video_id) {
            continue;
        }
        if let Some((title, artists)) = info.as_ref() {
            if video.album.is_empty() {
                video.album = title.to_string();
            }
            if video.author.is_empty() {
                video.author = artists.to_string();
            }
        }
        videos.push(video);
    }
    Ok(videos)
}
