use core::fmt;

use serde::{Serialize,Deserialize};


#[derive(Serialize,Deserialize,Debug, Clone)]
#[serde(untagged)]
pub enum Message {
    Requset(Requset),
    Response(Response),
    Notification(Notification),
}

impl From<Requset> for Message {
    fn from(req: Requset) -> Self {
        Message::Requset(req)
    }
}
impl From<Response> for Message {
    fn from(res: Response) -> Self {
        Message::Response(res)
    }
}
impl From<Notification> for Message {
    fn from(noti:Notification) -> Self {
        Message::Notification(noti)
    }
}
#[derive(Debug,Serialize,Deserialize,Clone,PartialEq, Eq, PartialOrd, Ord,Hash)]
#[serde(transparent)]
pub struct RequestId(IdRepr);

#[derive(Debug,Serialize,Deserialize,Clone,PartialEq, Eq, PartialOrd, Ord,Hash)]
#[serde(untagged)]
enum IdRepr{
    I32(i32),
    String(String)
}
impl From<i32> for RequestId {
    fn from(id: i32) -> Self {
        RequestId(IdRepr::I32(id))
    }
}
impl From<String> for RequestId {
    fn from(id: String) -> Self {
        RequestId(IdRepr::String(id))
    }
}
impl fmt::Display for RequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            IdRepr::I32(it) => fmt::Display::fmt(it, f),
            IdRepr::String(it) =>fmt::Debug::fmt(it, f),
        }
    }
}
#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Requset {
    pub id:RequestId,
    pub method: String,
    #[serde(default = "serde_json::Value::default")]
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub params: serde_json::Value,
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Response{
    pub id:RequestId,
    pub result: Option<serde_json::Value>,
    pub error:Option<ResponseError>,
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct ResponseError{
    pub code:i32,
    pub message:String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data:Option<serde_json::Value>,
}
#[allow(unused)]
#[derive(Debug,Clone,Copy)]
pub enum ErrorCode {
    // Defined by JSON RPC:
    ParseError = -32700,
    InvalidRequest = -32600,
    MethodNotFound = -32601,
    InvalidParams = -32602,
    InternalError = -32603,
    ServerErrorStart = -32099,
    ServerErrorEnd = -32000,

    /// Error code indicating that a server received a notification or
    /// request before the server has received the `initialize` request.
    ServerNotInitialized = -32002,
    UnknownErrorCode = -32001,

    // Defined by the protocol:
    /// The client has canceled a request and a server has detected
    /// the cancel.
    RequestCanceled = -32800,

    /// The server detected that the content of a document got
    /// modified outside normal conditions. A server should
    /// NOT send this error code if it detects a content change
    /// in it unprocessed messages. The result even computed
    /// on an older state might still be useful for the client.
    ///
    /// If a client decides that a result is not of any use anymore
    /// the client should cancel the request.
    ContentModified = -32801,

    /// The server cancelled the request. This error code should
    /// only be used for requests that explicitly support being
    /// server cancellable.
    ///
    /// @since 3.17.0
    ServerCancelled = -32802,
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Notification{
    pub method:String,
    #[serde(default = "serde_json::Value::default")]
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub params:serde_json::Value,
}
