use anyhow::{anyhow, Result};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    pub error: String,
    pub data: Option<Value>,
    pub request_id: Option<u64>,
}

impl Response {
    pub fn into_result(self) -> Result<Value> {
        if self.error == "success" {
            Ok(self.data.unwrap_or_default())
        } else {
            Err(anyhow!(self.error))
        }
    }

    pub fn into_result_of<T: DeserializeOwned>(self) -> Result<T> {
        Ok(serde_json::from_value(self.into_result()?)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_result_of<T: DeserializeOwned>(data: Value) -> T {
        let response = Response {
            error: "success".into(),
            data: Some(data),
            request_id: None,
        };

        response.into_result_of::<T>().unwrap()
    }

    #[test]
    fn into_result_of() {
        assert_eq!(test_result_of::<()>(Value::Null), ());
        assert_eq!(test_result_of::<i32>(Value::from(5)), 5);
        assert_eq!(test_result_of::<f32>(Value::from(5)), 5.0);
        assert_eq!(test_result_of::<f32>(Value::from(1.25)), 1.25);
        assert_eq!(test_result_of::<bool>(Value::from(true)), true);
    }

    #[test]
    fn into_result_of_none() {
        let response = Response {
            error: "success".into(),
            data: None,
            request_id: None,
        };

        assert_eq!(response.into_result_of::<()>().unwrap(), ());
    }

    #[test]
    fn into_result_of_bad() {
        let response = Response {
            error: "success".into(),
            data: Some(Value::from("foo")),
            request_id: None,
        };

        assert!(response.into_result_of::<i32>().is_err());
    }
}
