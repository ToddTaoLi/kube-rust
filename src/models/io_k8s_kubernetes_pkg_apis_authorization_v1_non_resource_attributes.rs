/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AuthorizationV1NonResourceAttributes : NonResourceAttributes includes the authorization attributes available for non-resource requests to the Authorizer interface

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthorizationV1NonResourceAttributes {
  /// Path is the URL path of the request
  #[serde(rename = "path")]
  path: Option<String>,
  /// Verb is the standard HTTP verb
  #[serde(rename = "verb")]
  verb: Option<String>
}

impl AuthorizationV1NonResourceAttributes {
  /// NonResourceAttributes includes the authorization attributes available for non-resource requests to the Authorizer interface
  pub fn new() -> AuthorizationV1NonResourceAttributes {
    AuthorizationV1NonResourceAttributes {
      path: None,
      verb: None
    }
  }

  pub fn set_path(&mut self, path: String) {
    self.path = Some(path);
  }

  pub fn with_path(mut self, path: String) -> AuthorizationV1NonResourceAttributes {
    self.path = Some(path);
    self
  }

  pub fn path(&self) -> Option<&String> {
    self.path.as_ref()
  }

  pub fn reset_path(&mut self) {
    self.path = None;
  }

  pub fn set_verb(&mut self, verb: String) {
    self.verb = Some(verb);
  }

  pub fn with_verb(mut self, verb: String) -> AuthorizationV1NonResourceAttributes {
    self.verb = Some(verb);
    self
  }

  pub fn verb(&self) -> Option<&String> {
    self.verb.as_ref()
  }

  pub fn reset_verb(&mut self) {
    self.verb = None;
  }

}



