use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::ApiEnvelope;
use crate::Client;
use crate::core::{ApiResponse, Error, RequestOptions};
use crate::generated::ops;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadFileQuery {
    pub file_token_type: Option<String>,
}

impl UploadFileQuery {
    pub fn to_query_pairs(&self) -> Vec<(String, String)> {
        let mut query = Vec::new();
        if let Some(v) = &self.file_token_type {
            query.push(("file_token_type".to_string(), v.clone()));
        }
        query
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DownloadFileQuery {
    pub extra: Option<String>,
}

impl DownloadFileQuery {
    pub fn to_query_pairs(&self) -> Vec<(String, String)> {
        let mut query = Vec::new();
        if let Some(v) = &self.extra {
            query.push(("extra".to_string(), v.clone()));
        }
        query
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListFileQuery {
    pub page_token: Option<String>,
    pub page_size: Option<u32>,
    pub folder_token: Option<String>,
    pub direction: Option<String>,
    pub order_by: Option<String>,
}

impl ListFileQuery {
    pub fn to_query_pairs(&self) -> Vec<(String, String)> {
        let mut query = Vec::new();
        if let Some(v) = &self.page_token {
            query.push(("page_token".to_string(), v.clone()));
        }
        if let Some(v) = self.page_size {
            query.push(("page_size".to_string(), v.to_string()));
        }
        if let Some(v) = &self.folder_token {
            query.push(("folder_token".to_string(), v.clone()));
        }
        if let Some(v) = &self.direction {
            query.push(("direction".to_string(), v.clone()));
        }
        if let Some(v) = &self.order_by {
            query.push(("order_by".to_string(), v.clone()));
        }
        query
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateFolderBody {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MoveFileToTrashBody {
    pub file_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadFileResponseData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListFileResponseData {
    #[serde(default)]
    pub files: Vec<FileInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

pub struct DriveV1FileApi<'a> {
    client: &'a Client,
}

impl<'a> DriveV1FileApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn upload(
        &self,
        query: Vec<(String, String)>,
        body: Value,
        options: RequestOptions,
    ) -> Result<ApiResponse, Error> {
        self.client
            .call_with_body(
                ops::drive::v1::file::UPLOAD_ALL,
                HashMap::new(),
                query,
                &body,
                options,
            )
            .await
    }

    pub async fn upload_typed(
        &self,
        query: &UploadFileQuery,
        body: Value,
        options: RequestOptions,
    ) -> Result<ApiEnvelope<UploadFileResponseData>, Error> {
        let raw = self.upload(query.to_query_pairs(), body, options).await?;
        raw.json()
    }

    pub async fn download(
        &self,
        file_token: impl Into<String>,
        query: Vec<(String, String)>,
        options: RequestOptions,
    ) -> Result<ApiResponse, Error> {
        let mut path_params = HashMap::new();
        path_params.insert("file_token".to_string(), file_token.into());
        self.client
            .call(
                ops::drive::v1::file::DOWNLOAD,
                path_params,
                query,
                None,
                options,
            )
            .await
    }

    pub async fn list(
        &self,
        query: Vec<(String, String)>,
        options: RequestOptions,
    ) -> Result<ApiResponse, Error> {
        self.client
            .call(
                ops::drive::v1::file::LIST,
                HashMap::new(),
                query,
                None,
                options,
            )
            .await
    }

    pub async fn list_typed(
        &self,
        query: &ListFileQuery,
        options: RequestOptions,
    ) -> Result<ApiEnvelope<ListFileResponseData>, Error> {
        let raw = self.list(query.to_query_pairs(), options).await?;
        raw.json()
    }

    pub async fn create_folder(
        &self,
        body: &CreateFolderBody,
        options: RequestOptions,
    ) -> Result<ApiResponse, Error> {
        self.client
            .call_with_body(
                ops::drive::v1::file::CREATE_FOLDER,
                HashMap::new(),
                Vec::new(),
                body,
                options,
            )
            .await
    }

    pub async fn move_file(
        &self,
        body: &MoveFileToTrashBody,
        options: RequestOptions,
    ) -> Result<ApiResponse, Error> {
        self.client
            .call_with_body(
                ops::drive::v1::file::MOVE,
                HashMap::new(),
                Vec::new(),
                body,
                options,
            )
            .await
    }

    pub async fn delete(
        &self,
        file_token: impl Into<String>,
        options: RequestOptions,
    ) -> Result<ApiResponse, Error> {
        let mut path_params = HashMap::new();
        path_params.insert("file_token".to_string(), file_token.into());
        self.client
            .call(
                ops::drive::v1::file::DELETE,
                path_params,
                Vec::new(),
                None,
                options,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_query_serializes() {
        let query = ListFileQuery {
            page_size: Some(50),
            folder_token: Some("fld_xxx".to_string()),
            ..Default::default()
        };
        let pairs = query.to_query_pairs();
        assert!(pairs.contains(&("page_size".to_string(), "50".to_string())));
        assert!(pairs.contains(&("folder_token".to_string(), "fld_xxx".to_string())));
    }
}
