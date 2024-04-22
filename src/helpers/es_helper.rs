use core::fmt;

use elasticsearch::{auth::Credentials, cert::CertificateValidation, http::{transport::{SingleNodeConnectionPool, TransportBuilder}, Url}, indices::{IndicesCreateParts, IndicesDeleteParts, IndicesExistsParts}, BulkOperation, BulkParts, Elasticsearch};
use serde::Serialize;
use serde_json::Value;

#[derive(Clone)]
pub struct ESHelper {
    pub client: Elasticsearch
}

pub async fn create_es_helper(elasticsearch_url: &str, elasticsearch_user: &str, elasticsearch_password: &str) -> Result<ESHelper, ElasticsearchCreateClientError> {
    let conn_pool = SingleNodeConnectionPool::new(Url::parse(elasticsearch_url)
        .map_err(|err| ElasticsearchCreateClientError{message: err.to_string()})?
    );
    let credentials = Credentials::Basic(elasticsearch_user.to_string(), elasticsearch_password.to_string());
    let mut builder = TransportBuilder::new(conn_pool);
    builder = builder.auth(credentials);
    builder = builder.cert_validation(CertificateValidation::None);
    let transport = builder.build()
        .map_err(|err| ElasticsearchCreateClientError{message: err.to_string()})?;
    let es_client = Elasticsearch::new(transport);

    Ok(ESHelper { 
        client: es_client 
    })
}

impl ESHelper {

    pub async fn delete_index(&self, index: &str) -> Result<(), ElasticsearchDeleteIndexError> {
        let exists_res = self.client
            .indices()
            .exists(IndicesExistsParts::Index(&[index]))
            .send()
            .await
            .map_err(|err| ElasticsearchDeleteIndexError{message: err.to_string()})?;  
        if exists_res.status_code().is_success() {
            let delete_res = self.client
                .indices()
                .delete(IndicesDeleteParts::Index(&[index]))
                .send()
                .await
                .map_err(|err| ElasticsearchDeleteIndexError{message: err.to_string()})?; 
            let code =  delete_res.status_code();
            if !code.is_success() {
                let reason = delete_res
                    .text()
                    .await
                    .unwrap_or("faield to get text from elasticsearch response body".to_string());
                Err(ElasticsearchDeleteIndexError {
                    message: format!("non success status code received when trying to delete index: {}: {}", code, reason)
                })?
            }
        }

        Ok(())
    }

    pub async fn create_index(&self, index: &str, body: Value) -> Result<(), ElasticsearchCreateIndexError> {
        let create_res = self.client
            .indices()
            .create(IndicesCreateParts::Index(index))
            .body(body)
            .send()
            .await
            .map_err(|err| ElasticsearchCreateIndexError{message: err.to_string()})?;
        let code = create_res.status_code();
        if !code.is_success() {
            let reason = create_res
                .text()
                .await
                .unwrap_or("faield to get text from elasticsearch response body".to_string());
            Err(ElasticsearchCreateIndexError {
                message: format!("non success status code received when trying to create index: {}: {}", code.as_str(), reason)
            })?
        }

        Ok(())
    }

    pub async fn bulk_index<T>(&self, index: &str, bulk_ops: Vec<T>) -> Result<(), ElasticsearchBulkIndexError> where T: Serialize {
        let body: Vec<BulkOperation<_>> = bulk_ops
            .iter()
            .map(|b| BulkOperation::index(b).into())
            .collect(); 

        let bulk_res = self.client
            .bulk(BulkParts::Index(index))
            .body(body)
            .send()
            .await
            .map_err(|err| ElasticsearchBulkIndexError{message: err.to_string()})?;
        let code = bulk_res.status_code();
        if !code.is_success() {
            let json: Value = bulk_res
                .json()
                .await
                .map_err(|err| ElasticsearchBulkIndexError{message: err.to_string()})?;
            if json["errors"].as_bool().unwrap_or(false) {
                let failed: Vec<&Value> = json["items"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|v| !v["error"].is_null())
                    .collect();

                Err(ElasticsearchBulkIndexError {
                    message: format!("failed to bulk index: {}: {:?}", code, failed)
                })?
            } else {
                Err(ElasticsearchBulkIndexError {
                    message: format!("non success status code received when trying to bulk index: {}: {}", code, json)
                })?
            }        
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ElasticsearchCreateClientError {
    pub message: String
}

impl fmt::Display for ElasticsearchCreateClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ElasticsearchCreateClientError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Debug)]
pub struct ElasticsearchDeleteIndexError {
    pub message: String
}

impl fmt::Display for ElasticsearchDeleteIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ElasticsearchDeleteIndexError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Debug)]
pub struct ElasticsearchCreateIndexError {
    pub message: String
}

impl fmt::Display for ElasticsearchCreateIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ElasticsearchCreateIndexError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Debug)]
pub struct ElasticsearchBulkIndexError {
    pub message: String
}

impl fmt::Display for ElasticsearchBulkIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ElasticsearchBulkIndexError {
    fn description(&self) -> &str {
        &self.message
    }
}
