/*
 * IoT Edge Module Workload API
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 2018-06-28
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptRequest {
    /// The data to be encrypted.
    #[serde(rename = "plaintext")]
    plaintext: String,
    /// An initialization vector used to encrypt the data.
    #[serde(rename = "initializationVector")]
    initialization_vector: String,
}

impl EncryptRequest {
    pub fn new(plaintext: String, initialization_vector: String) -> Self {
        EncryptRequest {
            plaintext,
            initialization_vector,
        }
    }

    pub fn set_plaintext(&mut self, plaintext: String) {
        self.plaintext = plaintext;
    }

    pub fn with_plaintext(mut self, plaintext: String) -> Self {
        self.plaintext = plaintext;
        self
    }

    pub fn plaintext(&self) -> &String {
        &self.plaintext
    }

    pub fn set_initialization_vector(&mut self, initialization_vector: String) {
        self.initialization_vector = initialization_vector;
    }

    pub fn with_initialization_vector(mut self, initialization_vector: String) -> Self {
        self.initialization_vector = initialization_vector;
        self
    }

    pub fn initialization_vector(&self) -> &String {
        &self.initialization_vector
    }
}
