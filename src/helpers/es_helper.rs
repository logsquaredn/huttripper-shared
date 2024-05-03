use core::fmt;
use std::collections::HashMap;

use elasticsearch::{auth::Credentials, cert::CertificateValidation, http::{transport::{SingleNodeConnectionPool, TransportBuilder}, Url}, indices::{IndicesCreateParts, IndicesDeleteParts, IndicesExistsParts}, BulkOperation, BulkParts, Elasticsearch, SearchParts};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone)]
pub struct ESHelper {
    pub client: Elasticsearch
}

pub fn create_es_helper(elasticsearch_url: &str, elasticsearch_user: &str, elasticsearch_password: &str) -> Result<ESHelper, ElasticsearchCreateClientError> {
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
                    .unwrap_or("failed to get text from elasticsearch response body".to_string());
                return Err(ElasticsearchDeleteIndexError {
                    message: format!("non success status code received when trying to delete index: {}: {}", code, reason)
                })
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
                .unwrap_or("failed to get text from elasticsearch response body".to_string());
            return Err(ElasticsearchCreateIndexError {
                message: format!("non success status code received when trying to create index: {}: {}", code.as_str(), reason)
            })
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
            let reason = bulk_res
                .text()
                .await
                .unwrap_or("failed to get text from elasticsearch response body".to_string());
            return Err(ElasticsearchBulkIndexError {
                message: format!("non success status code received when trying to bulk index: {}: {}", code, reason)
            })
        }

        Ok(())
    }

    pub async fn search(&self, index: &str, body: Value) -> Result<Vec<ElasticsearchHit>, ElasticsearchSearchError> {
        let search_res = self.client
            .search(SearchParts::Index(&[index]))
            .body(body)
            .send()
            .await
            .map_err(|err| ElasticsearchSearchError{message: err.to_string()})?;
        let code = search_res.status_code();
        if !code.is_success() {
            let reason = search_res
                .text()
                .await
                .unwrap_or("failed to get text from elasticserach response body".to_string());
            return Err(ElasticsearchSearchError {
                message: format!("non success status code received when trying to search: {}: {}", code, reason)
            })
        }

        let json: Value = search_res
            .json()
            .await
            .map_err(|err| ElasticsearchSearchError{message: format!("failed to get json from elasticsearch response body: {}", err.to_string())})?;

        let hits: Result<Vec<ElasticsearchHit>, ElasticsearchSearchError> = json["hits"]["hits"]
            .as_array()
            .unwrap_or(&vec![])
            .to_owned()
            .iter()
            .map(|hit| 
                serde_json::from_value(hit.to_owned())
                    .map_err(|err| ElasticsearchSearchError{message: format!("failed to get elasticsearch hits: {}", err)})
            )
            .collect();

        Ok(hits?)
    }

    pub fn parse_scores_from_hits(hits: Vec<ElasticsearchHit>, key: &str) -> HashMap<String, f32> {
        hits
            .iter()
            .map(|hit| (hit._source[key].as_str(), hit._score))
            .filter(|entry| entry.0.is_some() && !entry.0.unwrap().is_empty())
            .map(|entry| (entry.0.unwrap().to_string(), entry.1))
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct ElasticsearchHit {
    pub _id: String,
    pub _index: String,
    pub _score: f32,
    pub _source: Value
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

#[derive(Debug)]
pub struct ElasticsearchSearchError {
    pub message: String
}

impl fmt::Display for ElasticsearchSearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ElasticsearchSearchError {
    fn description(&self) -> &str {
        &self.message
    }
}
