//! EPP commands relating to nominet specific features

use super::{proto, EPPClientServerFeatures, Request, Response, Sender};
use super::router::HandleReqReturn;

#[derive(Debug)]
pub struct TagListRequest {
    pub return_path: Sender<TagListResponse>,
}

/// Response to a tag list query
#[derive(Debug)]
pub struct TagListResponse {
    /// Tags returned
    pub tags: Vec<Tag>
}

#[derive(Debug)]
pub struct Tag {
    /// Tag ID
    pub tag: String,
    /// Legal name of the tag
    pub name: String,
    /// Trading name of the tag
    pub trading_name: Option<String>,
    /// Does this tag require handshaking
    pub handshake: bool
}

pub fn handle_tag_list(
    client: &EPPClientServerFeatures,
    _req: &TagListRequest,
) -> HandleReqReturn<TagListResponse> {
    if !client.nominet_tag_list {
        return Err(Response::Unsupported);
    }
    let command = proto::EPPInfo::TagList {};
    Ok((proto::EPPCommandType::Info(command), None))
}

pub fn handle_tag_list_response(response: proto::EPPResponse) -> Response<TagListResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPNominetTagInfoResult(tag_list) => {
                Response::Ok(TagListResponse {
                    tags: match tag_list.tags.into_iter().map(|t| Ok(Tag {
                        tag: t.tag,
                        name: t.name,
                        trading_name: t.trading_name,
                        handshake: match t.handshake.as_str() {
                            "Y" => true,
                            "N" => false,
                            _ => return Err(Response::InternalServerError)
                        }
                    })).collect() {
                        Ok(t) => t,
                        Err(e) => return e
                    }
                })
            }
            _ => Response::InternalServerError,
        },
        None => Response::InternalServerError,
    }
}

/// Fetches a list of registered tags
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn tag_list(
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<TagListResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::NominetTagList(Box::new(TagListRequest {
            return_path: sender,
        })),
        receiver,
    )
    .await
}