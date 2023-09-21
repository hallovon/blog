use serde::Serialize;

/// 服务端统一响应
#[derive(Serialize)]
pub struct Response<T: Serialize> {
    /// 响应状态码，如果没有错误，则为0
    pub code: i32,
    /// 提示信息，如果没有错误，则为OK
    pub msg: String,
    /// 响应的数据，如果发生错误，则为None
    pub data: Option<T>,
}

impl<T> Response<T>
where
    T: Serialize,
{
    pub fn new(code: i32, msg: String, data: Option<T>) -> Self {
        Self { code, msg, data }
    }

    pub fn ok(data: T) -> Self {
        Self::new(0, "OK".to_string(), Some(data))
    }

    pub fn err(code: i32, msg: String) -> Self {
        Self::new(code, msg, None)
    }
}
