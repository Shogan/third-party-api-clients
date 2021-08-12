//! The data types sent to and returned from the API client.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Simple User
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SimpleUser {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organizations_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub received_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub site_admin: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscriptions_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// All of the following types:
///
/// - `SimpleUser`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum UserAllOf {
    /**
     * Simple User
     */
    SimpleUser(SimpleUser),
}

/// The set of permissions for the GitHub app
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Permissions {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub checks: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployments: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub metadata: String,
}

/// GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitHubApp {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_secret: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub installations_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub owner: UserAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pem: String,
    /**
     * The set of permissions for the GitHub app
     */
    #[serde()]
    pub permissions: Permissions,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub webhook_secret: String,
}

/// Basic Error
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct BasicError {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Validation Error Simple
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ValidationErrorSimple {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation_url: String,
    /**
     * Validation Error Simple
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

/// One of the following types:
///
/// - `String`
/// - `f64`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum WebhookConfigInsecureSslOneOf {
    String(String),
    F64(f64),
}

/// Configuration object of the webhook
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct WebhookConfig {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    /**
     * Configuration object of the webhook
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSslOneOf>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Delivery made by a webhook, without request and response information.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct HookDeliveryItem {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub action: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub delivered_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub duration: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub guid: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub installation_id: i64,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub redelivery: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub repository_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub status_code: i64,
}

/// Scim Error
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimError {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub detail: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
     * Scim Error
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub scim_type: String,
    /**
     * Scim Error
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub status: i64,
}

/// One of the following types:
///
/// - `String`
/// - `i64`
/// - `Vec<String>`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ValueOneOf {
    String(String),
    /**
     * The list of events for the GitHub app
     */
    StringVector(Vec<String>),
    I64(i64),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Errors {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub index: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub resource: String,
    /**
     * One of the following types:
     *  
     *  - `String`
     *  - `i64`
     *  - `Vec<String>`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<ValueOneOf>,
}

/// Validation Error
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ValidationError {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation_url: String,
    /**
     * Validation Error
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<Errors>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Data {}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Request {
    /**
     * The request headers sent with the webhook delivery.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Data>,
    /**
     * The request headers sent with the webhook delivery.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<Data>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Response {
    /**
     * The request headers sent with the webhook delivery.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Data>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub payload: String,
}

/// Delivery made by a webhook.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct HookDelivery {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub action: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub delivered_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub duration: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub guid: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub installation_id: i64,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub redelivery: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub repository_id: i64,
    #[serde()]
    pub request: Request,
    #[serde()]
    pub response: Response,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub status_code: i64,
}

/// An enterprise account
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Enterprise {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub website_url: String,
}

/**
 * The level of permission to grant the access token to retrieve Pages statuses, configuration, and builds, as well as create new builds. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Pages {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Pages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Pages::Read => "read",
            Pages::Write => "write",
            Pages::Noop => "",
            Pages::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Pages {
    fn default() -> Pages {
        Pages::Noop
    }
}
impl Pages {
    pub fn is_noop(&self) -> bool {
        matches!(self, Pages::Noop)
    }
}

/**
 * The level of permission to grant the access token to manage repository projects, columns, and cards. Can be one of: `read`, `write`, or `admin`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum RepositoryProjects {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for RepositoryProjects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            RepositoryProjects::Admin => "admin",
            RepositoryProjects::Read => "read",
            RepositoryProjects::Write => "write",
            RepositoryProjects::Noop => "",
            RepositoryProjects::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for RepositoryProjects {
    fn default() -> RepositoryProjects {
        RepositoryProjects::Noop
    }
}
impl RepositoryProjects {
    pub fn is_noop(&self) -> bool {
        matches!(self, RepositoryProjects::Noop)
    }
}

/**
 * The level of permission to grant the access token for viewing an organization's plan. Can be one of: `read`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum OrganizationPlan {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for OrganizationPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            OrganizationPlan::Read => "read",
            OrganizationPlan::Noop => "",
            OrganizationPlan::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for OrganizationPlan {
    fn default() -> OrganizationPlan {
        OrganizationPlan::Noop
    }
}
impl OrganizationPlan {
    pub fn is_noop(&self) -> bool {
        matches!(self, OrganizationPlan::Noop)
    }
}

/**
 * The level of permission to grant the access token to update GitHub Actions workflow files. Can be one of: `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Workflows {
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Workflows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Workflows::Write => "write",
            Workflows::Noop => "",
            Workflows::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Workflows {
    fn default() -> Workflows {
        Workflows::Noop
    }
}
impl Workflows {
    pub fn is_noop(&self) -> bool {
        matches!(self, Workflows::Noop)
    }
}

/// The permissions granted to the user-to-server access token.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AppPermissions {
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administration: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checks: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_references: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployments: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environments: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_administration: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_hooks: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_packages: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_plan: Option<OrganizationPlan>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_projects: Option<RepositoryProjects>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_secrets: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_self_hosted_runners: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_user_blocking: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packages: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_requests: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_hooks: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_projects: Option<RepositoryProjects>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_scanning_alerts: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_events: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub single_file: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_discussions: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vulnerability_alerts: Option<OrganizationPlan>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Workflows>,
}

/// Any of the following types:
///
/// - `SimpleUser`
/// - `Enterprise`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum AccountAnyOf {
    /**
     * An enterprise account
     */
    Enterprise(Enterprise),
    /**
     * Simple User
     */
    SimpleUser(SimpleUser),
}

/**
 * Describe whether all repositories have been selected or there's a selection involved
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum RepositorySelection {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "selected")]
    Selected,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for RepositorySelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            RepositorySelection::All => "all",
            RepositorySelection::Selected => "selected",
            RepositorySelection::Noop => "",
            RepositorySelection::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for RepositorySelection {
    fn default() -> RepositorySelection {
        RepositorySelection::Noop
    }
}
impl RepositorySelection {
    pub fn is_noop(&self) -> bool {
        matches!(self, RepositorySelection::Noop)
    }
}

/// Installation
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Installation {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_tokens_url: String,
    /**
     * Any of the following types:
     *  
     *  - `SimpleUser`
     *  - `Enterprise`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub account: AccountAnyOf,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub app_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub app_slug: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contact_email: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    /**
     * Installation
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_multiple_single_files: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde()]
    pub permissions: AppPermissions,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repositories_url: String,
    /**
     * Describe whether all repositories have been selected or there's a selection involved
     */
    #[serde(default, skip_serializing_if = "RepositorySelection::is_noop")]
    pub repository_selection: RepositorySelection,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub single_file_name: String,
    /**
     * Installation
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub single_file_paths: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub suspended_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub suspended_by: UserAllOf,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub target_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target_type: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// License Simple
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct LicenseSimple {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub spdx_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// All of the following types:
///
/// - `LicenseSimple`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum LicenseAllOf {
    /**
     * License Simple
     */
    LicenseSimple(LicenseSimple),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RepositoryPermissions {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub admin: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub maintain: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub pull: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub push: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub triage: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Owner {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organizations_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub received_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub site_admin: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscriptions_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RepositoryTemplatePermissions {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub admin: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub pull: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub push: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TemplateRepository {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_auto_merge: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_merge_commit: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_rebase_merge: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_squash_merge: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub archived: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignees_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blobs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branches_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub clone_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub collaborators_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub compare_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contributors_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_branch: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub delete_branch_on_merge: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub disabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub downloads_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub fork: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forks_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_refs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_downloads: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_issues: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_pages: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_projects: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_wiki: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_template: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub keys_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub languages_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merges_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub milestones_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mirror_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub network_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notifications_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub open_issues_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<RepositoryTemplatePermissions>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub private: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pulls_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pushed_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub releases_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ssh_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub stargazers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stargazers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub subscribers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscribers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svn_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub temp_clone_token: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trees_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub watchers_count: i64,
}

/// A git repository
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Repository {
    /**
     * A git repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_auto_merge: bool,
    /**
     * A git repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_merge_commit: bool,
    /**
     * A git repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_rebase_merge: bool,
    /**
     * A git repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_squash_merge: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub archived: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignees_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blobs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branches_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub clone_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub collaborators_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub compare_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contributors_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_branch: String,
    /**
     * A git repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub delete_branch_on_merge: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub disabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub downloads_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub fork: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forks: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forks_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_refs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_downloads: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_issues: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_pages: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_projects: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_wiki: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * A git repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_template: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub keys_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub languages_url: String,
    /**
     * All of the following types:
     *  
     *  - `LicenseSimple`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub license: LicenseAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub master_branch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merges_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub milestones_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mirror_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * A git repository
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub network_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notifications_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub open_issues: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub open_issues_count: i64,
    /**
     * A git repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<UserAllOf>,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<SimpleUser>,
    /**
     * A git repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<RepositoryPermissions>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub private: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pulls_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub pushed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub releases_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ssh_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub stargazers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stargazers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    /**
     * A git repository
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub subscribers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscribers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svn_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub temp_clone_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_repository: Option<TemplateRepository>,
    /**
     * A git repository
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trees_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub watchers: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub watchers_count: i64,
}

/// Authentication token for a GitHub App installed on a user or org.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct InstallationToken {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub expires_at: String,
    /**
     * Authentication token for a GitHub App installed on a user or org.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_multiple_single_files: bool,
    /**
     * Authentication token for a GitHub App installed on a user or org.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<AppPermissions>,
    /**
     * Authentication token for a GitHub App installed on a user or org.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<Repository>,
    /**
     * Authentication token for a GitHub App installed on a user or org.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_selection: Option<RepositorySelection>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub single_file: String,
    /**
     * Authentication token for a GitHub App installed on a user or org.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub single_file_paths: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct App {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// The authorization associated with an OAuth Access.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ApplicationGrant {
    #[serde()]
    pub app: App,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * The authorization associated with an OAuth Access.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserAllOf>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScopedInstallation {
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<SimpleUser>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_multiple_single_files: bool,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde()]
    pub permissions: AppPermissions,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repositories_url: String,
    /**
     * Describe whether all repositories have been selected or there's a selection involved
     */
    #[serde(default, skip_serializing_if = "RepositorySelection::is_noop")]
    pub repository_selection: RepositorySelection,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub single_file_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub single_file_paths: Vec<String>,
}

/// All of the following types:
///
/// - `ScopedInstallation`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum InstallationAllOf {
    ScopedInstallation(ScopedInstallation),
}

/// The authorization for an OAuth app, GitHub App, or a Personal Access Token.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Authorization {
    #[serde()]
    pub app: App,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fingerprint: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hashed_token: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The authorization for an OAuth app, GitHub App, or a Personal Access Token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationAllOf>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note_url: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token_last_eight: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * The authorization for an OAuth app, GitHub App, or a Personal Access Token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserAllOf>,
}

/// Code Of Conduct
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeOfConduct {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The policy that controls the repositories in the organization that are allowed to run GitHub Actions. Can be one of: `all`, `none`, or `selected`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum EnabledRepositories {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "selected")]
    Selected,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for EnabledRepositories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EnabledRepositories::All => "all",
            EnabledRepositories::None => "none",
            EnabledRepositories::Selected => "selected",
            EnabledRepositories::Noop => "",
            EnabledRepositories::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for EnabledRepositories {
    fn default() -> EnabledRepositories {
        EnabledRepositories::Noop
    }
}
impl EnabledRepositories {
    pub fn is_noop(&self) -> bool {
        matches!(self, EnabledRepositories::Noop)
    }
}

/**
 * The permissions policy that controls the actions that are allowed to run. Can be one of: `all`, `local_only`, or `selected`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum AllowedActions {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "local_only")]
    LocalOnly,
    #[serde(rename = "selected")]
    Selected,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for AllowedActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            AllowedActions::All => "all",
            AllowedActions::LocalOnly => "local_only",
            AllowedActions::Selected => "selected",
            AllowedActions::Noop => "",
            AllowedActions::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for AllowedActions {
    fn default() -> AllowedActions {
        AllowedActions::Noop
    }
}
impl AllowedActions {
    pub fn is_noop(&self) -> bool {
        matches!(self, AllowedActions::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsEnterprisePermissions {
    /**
     * The permissions policy that controls the actions that are allowed to run. Can be one of: `all`, `local_only`, or `selected`.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,
    /**
     * The policy that controls the repositories in the organization that are allowed to run GitHub Actions. Can be one of: `all`, `none`, or `selected`.
     */
    #[serde()]
    pub enabled_organizations: EnabledRepositories,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub selected_actions_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub selected_organizations_url: String,
}

/// Organization Simple
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrganizationSimple {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub members_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_members_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SelectedActions {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub github_owned_allowed: bool,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patterns_allowed: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub verified_allowed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RunnerGroupsEnterprise {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allows_public_repositories: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub default: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub id: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub runners_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub selected_organizations_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
}

/**
 * The type of label. Read-only labels are applied automatically when the runner is configured.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Type {
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "read-only")]
    ReadOnly,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Type::Custom => "custom",
            Type::ReadOnly => "read-only",
            Type::Noop => "",
            Type::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Type {
    fn default() -> Type {
        Type::Noop
    }
}
impl Type {
    pub fn is_noop(&self) -> bool {
        matches!(self, Type::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Labels {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The type of label. Read-only labels are applied automatically when the runner is configured.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<Type>,
}

/// A self hosted runner
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Runner {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub busy: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<Labels>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub os: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
}

/// Runner Application
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RunnerApplication {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub architecture: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub download_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub filename: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub os: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha_256_checksum: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub temp_download_token: String,
}

/// Authentication Token
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AuthenticationToken {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The request headers sent with the webhook delivery.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Data>,
    /**
     * Authentication Token
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<Repository>,
    /**
     * Authentication Token
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_selection: Option<RepositorySelection>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub single_file: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActorLocation {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AuditLogEvent {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "@timestamp"
    )]
    pub timestamp: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "_document_id"
    )]
    pub document_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub action: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub active: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub active_was: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub actor: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub actor_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_location: Option<ActorLocation>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blocked_user: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub business: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub config: Vec<String>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub config_was: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub created_at: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deploy_key_fingerprint: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub emoji: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events_were: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub explanation: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fingerprint: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub hook_id: i64,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub limited_availability: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub old_user: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub openssh_public_key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub org: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub org_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub previous_visibility: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub read_only: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repo: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub repository_public: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target_login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub team: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub transport_protocol: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub transport_protocol_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MinutesUsedBreakdown {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub macos: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub ubuntu: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub windows: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsBillingUsage {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub included_minutes: i64,
    #[serde()]
    pub minutes_used_breakdown: MinutesUsedBreakdown,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_minutes_used: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_paid_minutes_used: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PackagesBillingUsage {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub included_gigabytes_bandwidth: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_gigabytes_bandwidth_used: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_paid_gigabytes_bandwidth_used: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CombinedBillingUsage {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub days_left_in_billing_cycle: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub estimated_paid_storage_for_month: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub estimated_storage_for_month: i64,
}

/// Actor
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Actor {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Color-coded labels help you categorize and filter your issues (just like labels in Gmail).
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Label {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub color: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub default: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The state of the milestone.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum State {
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "open")]
    Open,
    FallthroughString(String),
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            State::Closed => "closed",
            State::Open => "open",
            State::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for State {
    fn default() -> State {
        State::Open
    }
}

/// A collection of related issues and pull requests.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Milestone {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub closed_issues: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub creator: UserAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub due_on: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub number: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub open_issues: i64,
    /**
     * The state of the milestone.
     */
    #[serde(default)]
    pub state: State,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * How the author is associated with the repository.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum AuthorAssociation {
    #[serde(rename = "COLLABORATOR")]
    Collaborator,
    #[serde(rename = "CONTRIBUTOR")]
    Contributor,
    #[serde(rename = "FIRST_TIMER")]
    FirstTimer,
    #[serde(rename = "FIRST_TIME_CONTRIBUTOR")]
    FirstTimeContributor,
    #[serde(rename = "MANNEQUIN")]
    Mannequin,
    #[serde(rename = "MEMBER")]
    Member,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "OWNER")]
    Owner,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for AuthorAssociation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            AuthorAssociation::Collaborator => "COLLABORATOR",
            AuthorAssociation::Contributor => "CONTRIBUTOR",
            AuthorAssociation::FirstTimer => "FIRST_TIMER",
            AuthorAssociation::FirstTimeContributor => "FIRST_TIME_CONTRIBUTOR",
            AuthorAssociation::Mannequin => "MANNEQUIN",
            AuthorAssociation::Member => "MEMBER",
            AuthorAssociation::None => "NONE",
            AuthorAssociation::Owner => "OWNER",
            AuthorAssociation::Noop => "",
            AuthorAssociation::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for AuthorAssociation {
    fn default() -> AuthorAssociation {
        AuthorAssociation::Noop
    }
}
impl AuthorAssociation {
    pub fn is_noop(&self) -> bool {
        matches!(self, AuthorAssociation::Noop)
    }
}

/// All of the following types:
///
/// - `Milestone`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum MilestoneAllOf {
    /**
     * A collection of related issues and pull requests.
     */
    Milestone(Milestone),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub diff_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub merged_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub patch_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// All of the following types:
///
/// - `GitHubApp`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum AppAllOf {
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    GitHubApp(GitHubApp),
}

/// Issue Simple
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssueSimple {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub active_lock_reason: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub assignee: UserAllOf,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<SimpleUser>,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub comments: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<Label>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub locked: bool,
    /**
     * All of the following types:
     *  
     *  - `Milestone`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub milestone: MilestoneAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub number: i64,
    /**
     * Issue Simple
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<AppAllOf>,
    /**
     * Issue Simple
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
    /**
     * Issue Simple
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timeline_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub user: UserAllOf,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReactionRollup {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "+1"
    )]
    pub plus_one: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "-1"
    )]
    pub minus_one: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub confused: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub eyes: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub heart: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub hooray: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub laugh: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub rocket: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Comments provide a way for people to collaborate on an issue.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssueComment {
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Comments provide a way for people to collaborate on an issue.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<AppAllOf>,
    /**
     * Comments provide a way for people to collaborate on an issue.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<ReactionRollup>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub user: UserAllOf,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Repo {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EventPayloadPages {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub action: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub page_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub summary: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Payload {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub action: String,
    /**
     * Comments provide a way for people to collaborate on an issue.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<IssueComment>,
    /**
     * Issue Simple
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<IssueSimple>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pages: Vec<EventPayloadPages>,
}

/// Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Event {
    /**
     * Actor
     */
    #[serde()]
    pub actor: Actor,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org: Option<Actor>,
    #[serde()]
    pub payload: Payload,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub public: bool,
    #[serde()]
    pub repo: Repo,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

/// Hypermedia Link with Type
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct LinkWithType {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub href: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Links {
    /**
     * Hypermedia Link with Type
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_user: Option<LinkWithType>,
    /**
     * Hypermedia Link with Type
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_user_actor: Option<LinkWithType>,
    /**
     * Hypermedia Link with Type
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_user_organization: Option<LinkWithType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub current_user_organizations: Vec<LinkWithType>,
    /**
     * Hypermedia Link with Type
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_user_public: Option<LinkWithType>,
    /**
     * Hypermedia Link with Type
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_advisories: Option<LinkWithType>,
    /**
     * Hypermedia Link with Type
     */
    #[serde()]
    pub timeline: LinkWithType,
    /**
     * Hypermedia Link with Type
     */
    #[serde()]
    pub user: LinkWithType,
}

/// Feed
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Feed {
    #[serde(rename = "_links")]
    pub links: Links,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub current_user_actor_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub current_user_organization_url: String,
    /**
     * Feed
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub current_user_organization_urls: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub current_user_public_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub current_user_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub security_advisories_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timeline_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_url: String,
}

/// Base Gist
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct BaseGist {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub comments: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde()]
    pub files: Data,
    /**
     * Base Gist
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forks: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_pull_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_push_url: String,
    /**
     * Base Gist
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Base Gist
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<UserAllOf>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub public: bool,
    /**
     * Base Gist
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub truncated: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub user: UserAllOf,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Plan {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub collaborators: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub private_repos: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub space: i64,
}

/// Public User
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PublicUser {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub bio: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blog: String,
    /**
     * Public User
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub collaborators: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Public User
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub disk_usage: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub followers: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub following: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub hireable: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organizations_url: String,
    /**
     * Public User
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub owned_private_repos: i64,
    /**
     * Public User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    /**
     * Public User
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub private_gists: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub public_gists: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub public_repos: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub received_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub site_admin: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscriptions_url: String,
    /**
     * Public User
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub suspended_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Public User
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_private_repos: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub twitter_username: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Stats {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub additions: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub deletions: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total: i64,
}

/// Gist History
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GistHistory {
    /**
     * Gist History
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_status: Option<Stats>,
    /**
     * Gist History
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub committed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Forks {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Public User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<PublicUser>,
}

/// Gist
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Gist {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub comments: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde()]
    pub files: Data,
    /**
     * Gist
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forks: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_pull_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_push_url: String,
    /**
     * Gist
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Gist
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<UserAllOf>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub public: bool,
    /**
     * Gist
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub truncated: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub user: UserAllOf,
}

/// Gist Simple
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GistSimple {
    /**
     * Gist Simple
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub comments: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Gist Simple
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Data>,
    /**
     * Gist
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fork_of: Option<Gist>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forks: Vec<Forks>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_pull_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_push_url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<GistHistory>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<SimpleUser>,
    /**
     * Gist Simple
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub public: bool,
    /**
     * Gist Simple
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub truncated: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user: String,
}

/// A comment made to a gist.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GistComment {
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub user: UserAllOf,
}

/// Gist Commit
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GistCommit {
    #[serde()]
    pub change_status: Stats,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub committed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub user: UserAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

/// Gitignore Template
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitignoreTemplate {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct LabelsData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub color: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub default: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// One of the following types:
///
/// - `String`
/// - `LabelsData`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum LabelsOneOf {
    LabelsData(LabelsData),
    String(String),
}

/// Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Issue {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub active_lock_reason: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub assignee: UserAllOf,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<SimpleUser>,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed_by: Option<UserAllOf>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub comments: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Labels to associate with this issue; pass one or more label names to replace the set of labels on this issue; send an empty array to clear all labels from the issue; note that the labels are silently dropped for users without push access to the repository
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<LabelsOneOf>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub locked: bool,
    /**
     * All of the following types:
     *  
     *  - `Milestone`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub milestone: MilestoneAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub number: i64,
    /**
     * Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<AppAllOf>,
    /**
     * Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
    /**
     * Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<ReactionRollup>,
    /**
     * Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timeline_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub user: UserAllOf,
}

/// License
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct License {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub featured: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub implementation: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limitations: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub spdx_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Marketplace Listing Plan
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MarketplaceListingPlan {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub accounts_url: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bullets: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_free_trial: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub monthly_price_in_cents: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub number: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub price_model: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub unit_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub yearly_price_in_cents: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MarketplacePendingChange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<chrono::NaiveDate>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_installed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<MarketplaceListingPlan>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unit_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MarketplacePurchase {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub billing_cycle: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub free_trial_ends_on: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_installed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_billing_date: Option<chrono::NaiveDate>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub on_free_trial: bool,
    /**
     * Marketplace Listing Plan
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<MarketplaceListingPlan>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unit_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
}

/// Marketplace Purchase
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MarketplacePurchaseData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marketplace_pending_change: Option<MarketplacePendingChange>,
    #[serde()]
    pub marketplace_purchase: MarketplacePurchase,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organization_billing_email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SshKeyFingerprints {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha256_dsa: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha256_rsa: String,
}

/// Api Overview
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ApiOverview {
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub api: Vec<String>,
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependabot: Vec<String>,
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub git: Vec<String>,
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hooks: Vec<String>,
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub importer: Vec<String>,
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<String>,
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pages: Vec<String>,
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssh_key_fingerprints: Option<SshKeyFingerprints>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub verifiable_password_authentication: bool,
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub web: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MinimalRepositoryPermissions {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub admin: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub maintain: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub pull: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub push: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub triage: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MinimalRepositoryLicense {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub spdx_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Minimal Repository
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MinimalRepository {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub archived: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignees_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blobs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branches_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub clone_url: String,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code_of_conduct: Option<CodeOfConduct>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub collaborators_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub compare_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contributors_url: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_branch: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub delete_branch_on_merge: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub disabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub downloads_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub fork: bool,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forks: i64,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forks_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_refs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_downloads: bool,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_issues: bool,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_pages: bool,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_projects: bool,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_wiki: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_template: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub keys_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub languages_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<MinimalRepositoryLicense>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merges_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub milestones_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mirror_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub network_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notifications_url: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub open_issues: i64,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub open_issues_count: i64,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<SimpleUser>,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<MinimalRepositoryPermissions>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub private: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pulls_url: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub pushed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub releases_url: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ssh_url: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub stargazers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stargazers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub subscribers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscribers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svn_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub temp_clone_token: String,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_repository: Option<Data>,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trees_url: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub watchers: i64,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub watchers_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Subject {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub latest_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Thread
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Thread {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_read_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    /**
     * Minimal Repository
     */
    #[serde()]
    pub repository: MinimalRepository,
    #[serde()]
    pub subject: Subject,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub unread: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Thread Subscription
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ThreadSubscription {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub ignored: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub subscribed: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub thread_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrganizationFullPlan {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub filled_seats: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub private_repos: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub seats: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub space: i64,
}

/// Organization Full
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrganizationFull {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub billing_email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blog: String,
    /**
     * Organization Full
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub collaborators: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_repository_permission: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Organization Full
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub disk_usage: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub followers: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub following: i64,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_organization_projects: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_repository_projects: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Organization Full
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_verified: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub members_allowed_repository_creation_type: String,
    /**
     * Organization Full
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub members_can_create_internal_repositories: bool,
    /**
     * Organization Full
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub members_can_create_pages: bool,
    /**
     * Organization Full
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub members_can_create_private_pages: bool,
    /**
     * Organization Full
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub members_can_create_private_repositories: bool,
    /**
     * Organization Full
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub members_can_create_public_pages: bool,
    /**
     * Organization Full
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub members_can_create_public_repositories: bool,
    /**
     * Organization Full
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub members_can_create_repositories: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub members_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Organization Full
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub owned_private_repos: i64,
    /**
     * Organization Full
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<OrganizationFullPlan>,
    /**
     * Organization Full
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub private_gists: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub public_gists: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_members_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub public_repos: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    /**
     * Organization Full
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_private_repos: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub twitter_username: String,
    /**
     * Organization Full
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub two_factor_requirement_enabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsOrganizationPermissions {
    /**
     * The permissions policy that controls the actions that are allowed to run. Can be one of: `all`, `local_only`, or `selected`.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,
    /**
     * The policy that controls the repositories in the organization that are allowed to run GitHub Actions. Can be one of: `all`, `none`, or `selected`.
     */
    #[serde()]
    pub enabled_repositories: EnabledRepositories,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub selected_actions_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub selected_repositories_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RunnerGroupsOrg {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allows_public_repositories: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub default: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub id: f64,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub inherited: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub inherited_allows_public_repositories: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub runners_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub selected_repositories_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
}

/**
 * Visibility of a secret
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Visibility {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "selected")]
    Selected,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Visibility::All => "all",
            Visibility::Private => "private",
            Visibility::Selected => "selected",
            Visibility::Noop => "",
            Visibility::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Visibility {
    fn default() -> Visibility {
        Visibility::Noop
    }
}
impl Visibility {
    pub fn is_noop(&self) -> bool {
        matches!(self, Visibility::Noop)
    }
}

/// Secrets for GitHub Actions for an organization.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrganizationActionsSecret {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub selected_repositories_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Visibility of a secret
     */
    #[serde(default, skip_serializing_if = "Visibility::is_noop")]
    pub visibility: Visibility,
}

/// The public key used for setting Actions Secrets.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsPublicKey {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    /**
     * The public key used for setting Actions Secrets.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Credential Authorization
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CredentialAuthorization {
    /**
     * Credential Authorization
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub authorized_credential_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub authorized_credential_note: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub authorized_credential_title: String,
    /**
     * Credential Authorization
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub credential_accessed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub credential_authorized_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub credential_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub credential_type: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fingerprint: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    /**
     * Credential Authorization
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token_last_eight: String,
}

/// Organization Invitation
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrganizationInvitation {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub failed_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub failed_reason: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub invitation_teams_url: String,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inviter: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub role: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub team_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Config {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub insecure_ssl: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Org Hook
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgHook {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub active: bool,
    #[serde()]
    pub config: Config,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deliveries_url: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ping_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The type of GitHub user that can comment, open issues, or create pull requests while the interaction limit is in effect. Can be one of: `existing_users`, `contributors_only`, `collaborators_only`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum InteractionGroup {
    #[serde(rename = "collaborators_only")]
    CollaboratorsOnly,
    #[serde(rename = "contributors_only")]
    ContributorsOnly,
    #[serde(rename = "existing_users")]
    ExistingUsers,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for InteractionGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            InteractionGroup::CollaboratorsOnly => "collaborators_only",
            InteractionGroup::ContributorsOnly => "contributors_only",
            InteractionGroup::ExistingUsers => "existing_users",
            InteractionGroup::Noop => "",
            InteractionGroup::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for InteractionGroup {
    fn default() -> InteractionGroup {
        InteractionGroup::Noop
    }
}
impl InteractionGroup {
    pub fn is_noop(&self) -> bool {
        matches!(self, InteractionGroup::Noop)
    }
}

/// Interaction limit settings.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct InteractionLimits {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The type of GitHub user that can comment, open issues, or create pull requests while the interaction limit is in effect. Can be one of: `existing_users`, `contributors_only`, `collaborators_only`.
     */
    #[serde()]
    pub limit: InteractionGroup,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub origin: String,
}

/**
 * The duration of the interaction restriction. Can be one of: `one_day`, `three_days`, `one_week`, `one_month`, `six_months`. Default: `one_day`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum InteractionExpiry {
    #[serde(rename = "one_day")]
    OneDay,
    #[serde(rename = "one_month")]
    OneMonth,
    #[serde(rename = "one_week")]
    OneWeek,
    #[serde(rename = "six_months")]
    SixMonths,
    #[serde(rename = "three_days")]
    ThreeDays,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for InteractionExpiry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            InteractionExpiry::OneDay => "one_day",
            InteractionExpiry::OneMonth => "one_month",
            InteractionExpiry::OneWeek => "one_week",
            InteractionExpiry::SixMonths => "six_months",
            InteractionExpiry::ThreeDays => "three_days",
            InteractionExpiry::Noop => "",
            InteractionExpiry::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for InteractionExpiry {
    fn default() -> InteractionExpiry {
        InteractionExpiry::Noop
    }
}
impl InteractionExpiry {
    pub fn is_noop(&self) -> bool {
        matches!(self, InteractionExpiry::Noop)
    }
}

/// Limit interactions to a specific type of user for a specified duration
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct InteractionLimit {
    /**
     * Limit interactions to a specific type of user for a specified duration
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry: Option<InteractionExpiry>,
    /**
     * The type of GitHub user that can comment, open issues, or create pull requests while the interaction limit is in effect. Can be one of: `existing_users`, `contributors_only`, `collaborators_only`.
     */
    #[serde()]
    pub limit: InteractionGroup,
}

/// Groups of organization members that gives permissions on specified repositories.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamSimple {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ldap_dn: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub members_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub permission: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub privacy: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repositories_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamPermissions {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub admin: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub maintain: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub pull: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub push: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub triage: bool,
}

/// All of the following types:
///
/// - `TeamSimple`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ParentAllOf {
    /**
     * Groups of organization members that gives permissions on specified repositories.
     */
    TeamSimple(TeamSimple),
}

/// Groups of organization members that gives permissions on specified repositories.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Team {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub members_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * All of the following types:
     *  
     *  - `TeamSimple`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub parent: ParentAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub permission: String,
    /**
     * Groups of organization members that gives permissions on specified repositories.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<TeamPermissions>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub privacy: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repositories_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The state of the member in the organization. The `pending` state indicates the user has not yet accepted an invitation.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum OrgMembershipState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for OrgMembershipState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            OrgMembershipState::Active => "active",
            OrgMembershipState::Pending => "pending",
            OrgMembershipState::Noop => "",
            OrgMembershipState::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for OrgMembershipState {
    fn default() -> OrgMembershipState {
        OrgMembershipState::Noop
    }
}
impl OrgMembershipState {
    pub fn is_noop(&self) -> bool {
        matches!(self, OrgMembershipState::Noop)
    }
}

/**
 * The user's membership type in the organization.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Role {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "billing_manager")]
    BillingManager,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Role::Admin => "admin",
            Role::BillingManager => "billing_manager",
            Role::Member => "member",
            Role::Noop => "",
            Role::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Role {
    fn default() -> Role {
        Role::Noop
    }
}
impl Role {
    pub fn is_noop(&self) -> bool {
        matches!(self, Role::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgMembershipPermissions {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub can_create_repository: bool,
}

/// Org Membership
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgMembership {
    /**
     * Organization Simple
     */
    #[serde()]
    pub organization: OrganizationSimple,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organization_url: String,
    /**
     * Org Membership
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<OrgMembershipPermissions>,
    /**
     * The user's membership type in the organization.
     */
    #[serde(default, skip_serializing_if = "Role::is_noop")]
    pub role: Role,
    /**
     * The state of the member in the organization. The `pending` state indicates the user has not yet accepted an invitation.
     */
    #[serde(default, skip_serializing_if = "OrgMembershipState::is_noop")]
    pub state: OrgMembershipState,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub user: UserAllOf,
}

/// A migration.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Migration {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * A migration.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub exclude_attachments: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub guid: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub lock_repositories: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub owner: UserAllOf,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<Repository>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum PackageType {
    #[serde(rename = "container")]
    Container,
    #[serde(rename = "docker")]
    Docker,
    #[serde(rename = "maven")]
    Maven,
    #[serde(rename = "npm")]
    Npm,
    #[serde(rename = "nuget")]
    Nuget,
    #[serde(rename = "rubygems")]
    Rubygems,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for PackageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PackageType::Container => "container",
            PackageType::Docker => "docker",
            PackageType::Maven => "maven",
            PackageType::Npm => "npm",
            PackageType::Nuget => "nuget",
            PackageType::Rubygems => "rubygems",
            PackageType::Noop => "",
            PackageType::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for PackageType {
    fn default() -> PackageType {
        PackageType::Noop
    }
}
impl PackageType {
    pub fn is_noop(&self) -> bool {
        matches!(self, PackageType::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum PackageVisibility {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for PackageVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PackageVisibility::Private => "private",
            PackageVisibility::Public => "public",
            PackageVisibility::Noop => "",
            PackageVisibility::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for PackageVisibility {
    fn default() -> PackageVisibility {
        PackageVisibility::Noop
    }
}
impl PackageVisibility {
    pub fn is_noop(&self) -> bool {
        matches!(self, PackageVisibility::Noop)
    }
}

/// All of the following types:
///
/// - `MinimalRepository`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum RepositoryAllOf {
    /**
     * Minimal Repository
     */
    MinimalRepository(MinimalRepository),
}

/// A software package
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Package {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * A software package
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<UserAllOf>,
    #[serde(default, skip_serializing_if = "PackageType::is_noop")]
    pub package_type: PackageType,
    /**
     * A software package
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<RepositoryAllOf>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub version_count: i64,
    #[serde(default, skip_serializing_if = "PackageVisibility::is_noop")]
    pub visibility: PackageVisibility,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Container {
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Docker {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tag: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Metadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<Container>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docker: Option<Docker>,
    #[serde(default, skip_serializing_if = "PackageType::is_noop")]
    pub package_type: PackageType,
}

/// A version of a software package
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PackageVersion {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * A version of a software package
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub license: String,
    /**
     * A version of a software package
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub package_html_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The baseline permission that all organization members have on this project. Only present if owner is an organization.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum OrganizationPermission {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for OrganizationPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            OrganizationPermission::Admin => "admin",
            OrganizationPermission::None => "none",
            OrganizationPermission::Read => "read",
            OrganizationPermission::Write => "write",
            OrganizationPermission::Noop => "",
            OrganizationPermission::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for OrganizationPermission {
    fn default() -> OrganizationPermission {
        OrganizationPermission::Noop
    }
}
impl OrganizationPermission {
    pub fn is_noop(&self) -> bool {
        matches!(self, OrganizationPermission::Noop)
    }
}

/// Projects are a way to organize columns and cards of work.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Project {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub columns_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub creator: UserAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub number: i64,
    /**
     * Projects are a way to organize columns and cards of work.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_permission: Option<OrganizationPermission>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub owner_url: String,
    /**
     * Projects are a way to organize columns and cards of work.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub private: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Groups {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub synced_at: String,
}

/// External Groups to be mapped to a team for membership
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GroupMapping {
    /**
     * External Groups to be mapped to a team for membership
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<Groups>,
}

/**
 * The level of privacy this team should have
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Privacy {
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "secret")]
    Secret,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Privacy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Privacy::Closed => "closed",
            Privacy::Secret => "secret",
            Privacy::Noop => "",
            Privacy::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Privacy {
    fn default() -> Privacy {
        Privacy::Noop
    }
}
impl Privacy {
    pub fn is_noop(&self) -> bool {
        matches!(self, Privacy::Noop)
    }
}

/// Groups of organization members that gives permissions on specified repositories.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FullTeam {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ldap_dn: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub members_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub members_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Organization Full
     */
    #[serde()]
    pub organization: OrganizationFull,
    /**
     * Groups of organization members that gives permissions on specified repositories.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<ParentAllOf>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub permission: String,
    /**
     * Groups of organization members that gives permissions on specified repositories.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<Privacy>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub repos_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repositories_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// A team discussion is a persistent record of a free-form conversation within a team.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamDiscussion {
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub author: UserAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_version: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub comments_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_edited_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub number: i64,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub pinned: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub private: bool,
    /**
     * A team discussion is a persistent record of a free-form conversation within a team.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<ReactionRollup>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub team_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// A reply to a discussion within a team.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamDiscussionComment {
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub author: UserAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_version: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub discussion_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_edited_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub number: i64,
    /**
     * A reply to a discussion within a team.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<ReactionRollup>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The reaction to use
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Content {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "-1")]
    MinusOne,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "eyes")]
    Eyes,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Content::PlusOne => "+1",
            Content::MinusOne => "-1",
            Content::Confused => "confused",
            Content::Eyes => "eyes",
            Content::Heart => "heart",
            Content::Hooray => "hooray",
            Content::Laugh => "laugh",
            Content::Rocket => "rocket",
            Content::Noop => "",
            Content::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Content {
    fn default() -> Content {
        Content::Noop
    }
}
impl Content {
    pub fn is_noop(&self) -> bool {
        matches!(self, Content::Noop)
    }
}

/// Reactions to conversations provide a way to help people express their feelings more simply and effectively.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Reaction {
    /**
     * The reaction to use
     */
    #[serde(default, skip_serializing_if = "Content::is_noop")]
    pub content: Content,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub user: UserAllOf,
}

/**
 * The role of the user in the team.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum TeamMembershipRole {
    #[serde(rename = "maintainer")]
    Maintainer,
    #[serde(rename = "member")]
    Member,
    FallthroughString(String),
}

impl std::fmt::Display for TeamMembershipRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            TeamMembershipRole::Maintainer => "maintainer",
            TeamMembershipRole::Member => "member",
            TeamMembershipRole::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for TeamMembershipRole {
    fn default() -> TeamMembershipRole {
        TeamMembershipRole::Member
    }
}

/// Team Membership
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamMembership {
    /**
     * The role of the user in the team.
     */
    #[serde(default)]
    pub role: TeamMembershipRole,
    /**
     * The state of the member in the organization. The `pending` state indicates the user has not yet accepted an invitation.
     */
    #[serde(default, skip_serializing_if = "OrgMembershipState::is_noop")]
    pub state: OrgMembershipState,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamProjectPermissions {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub admin: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub read: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub write: bool,
}

/// A team's access to a project.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamProject {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub columns_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub number: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organization_permission: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub owner_url: String,
    #[serde()]
    pub permissions: TeamProjectPermissions,
    /**
     * A team's access to a project.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub private: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// A team's access to a repository.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamRepository {
    /**
     * A team's access to a repository.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_auto_merge: bool,
    /**
     * A team's access to a repository.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_merge_commit: bool,
    /**
     * A team's access to a repository.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_rebase_merge: bool,
    /**
     * A team's access to a repository.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_squash_merge: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub archived: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignees_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blobs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branches_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub clone_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub collaborators_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub compare_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contributors_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_branch: String,
    /**
     * A team's access to a repository.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub delete_branch_on_merge: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub disabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub downloads_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub fork: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forks: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forks_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_refs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_downloads: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_issues: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_pages: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_projects: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_wiki: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * A team's access to a repository.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_template: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub keys_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub languages_url: String,
    /**
     * All of the following types:
     *  
     *  - `LicenseSimple`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub license: LicenseAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub master_branch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merges_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub milestones_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mirror_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * A team's access to a repository.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub network_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notifications_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub open_issues: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub open_issues_count: i64,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub owner: UserAllOf,
    /**
     * A team's access to a repository.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<RepositoryPermissions>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub private: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pulls_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub pushed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub releases_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ssh_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub stargazers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stargazers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    /**
     * A team's access to a repository.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub subscribers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscribers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svn_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub temp_clone_token: String,
    /**
     * A team's access to a repository.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_repository: Option<Data>,
    /**
     * A team's access to a repository.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trees_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub watchers: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub watchers_count: i64,
}

/// Project cards represent a scope of work.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectCard {
    /**
     * Project cards represent a scope of work.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub archived: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub column_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub column_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub creator: UserAllOf,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub project_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub project_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Project columns contain cards of work.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectColumn {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub cards_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub project_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Repository Collaborator Permission
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RepositoryCollaboratorPermission {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub permission: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub user: UserAllOf,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RateLimit {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub limit: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub remaining: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub reset: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub used: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Resources {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code_scanning_upload: Option<RateLimit>,
    #[serde()]
    pub core: RateLimit,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graphql: Option<RateLimit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_manifest: Option<RateLimit>,
    #[serde()]
    pub search: RateLimit,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_import: Option<RateLimit>,
}

/// Rate Limit Overview
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RateLimitOverview {
    #[serde()]
    pub rate: RateLimit,
    #[serde()]
    pub resources: Resources,
}

/// Code of Conduct Simple
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeOfConductSimple {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FullRepositoryPermissions {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub admin: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub pull: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub push: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Status {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Status::Disabled => "disabled",
            Status::Enabled => "enabled",
            Status::Noop => "",
            Status::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Status {
    fn default() -> Status {
        Status::Noop
    }
}
impl Status {
    pub fn is_noop(&self) -> bool {
        matches!(self, Status::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SecretScanning {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SecurityAnalysis {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub advanced_security: Option<SecretScanning>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_scanning: Option<SecretScanning>,
}

/// Full Repository
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FullRepository {
    /**
     * Full Repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_auto_merge: bool,
    /**
     * Full Repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_merge_commit: bool,
    /**
     * Full Repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_rebase_merge: bool,
    /**
     * Full Repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_squash_merge: bool,
    /**
     * Full Repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub anonymous_access_enabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub archived: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignees_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blobs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branches_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub clone_url: String,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code_of_conduct: Option<CodeOfConductSimple>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub collaborators_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub compare_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contributors_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_branch: String,
    /**
     * Full Repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub delete_branch_on_merge: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub disabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub downloads_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub fork: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forks: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forks_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_refs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_downloads: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_issues: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_pages: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_projects: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_wiki: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Full Repository
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_template: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub keys_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub languages_url: String,
    /**
     * All of the following types:
     *  
     *  - `LicenseSimple`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub license: LicenseAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub master_branch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merges_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub milestones_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mirror_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub network_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notifications_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub open_issues: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub open_issues_count: i64,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<UserAllOf>,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<SimpleUser>,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<Repository>,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<FullRepositoryPermissions>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub private: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pulls_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub pushed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub releases_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_and_analysis: Option<SecurityAnalysis>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Repository>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ssh_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub stargazers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stargazers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub subscribers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscribers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svn_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub temp_clone_token: String,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_repository: Option<Data>,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trees_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub watchers: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub watchers_count: i64,
}

/// An artifact
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Artifact {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_download_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub expired: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size_in_bytes: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The phase of the lifecycle that the job is currently in.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum JobStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            JobStatus::Completed => "completed",
            JobStatus::InProgress => "in_progress",
            JobStatus::Queued => "queued",
            JobStatus::Noop => "",
            JobStatus::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for JobStatus {
    fn default() -> JobStatus {
        JobStatus::Noop
    }
}
impl JobStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, JobStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Steps {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub conclusion: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub number: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The phase of the lifecycle that the job is currently in.
     */
    #[serde(default, skip_serializing_if = "JobStatus::is_noop")]
    pub status: JobStatus,
}

/// Information of a job execution in a workflow run
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Job {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub check_run_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub conclusion: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head_sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub run_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub run_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The phase of the lifecycle that the job is currently in.
     */
    #[serde(default, skip_serializing_if = "JobStatus::is_noop")]
    pub status: JobStatus,
    /**
     * Information of a job execution in a workflow run
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<Steps>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsRepositoryPermissions {
    /**
     * The permissions policy that controls the actions that are allowed to run. Can be one of: `all`, `local_only`, or `selected`.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub selected_actions_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Head {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    #[serde()]
    pub repo: Repo,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestMinimal {
    #[serde()]
    pub base: Head,
    #[serde()]
    pub head: Head,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub number: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Author {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// Simple Commit
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SimpleCommit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<Author>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committer: Option<Author>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tree_id: String,
}

/// All of the following types:
///
/// - `SimpleCommit`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum HeadCommitAllOf {
    /**
     * Simple Commit
     */
    SimpleCommit(SimpleCommit),
}

/// An invocation of a workflow
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct WorkflowRun {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub artifacts_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub cancel_url: String,
    /**
     * An invocation of a workflow
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub check_suite_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub check_suite_node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub check_suite_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub conclusion: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head_branch: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleCommit`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub head_commit: HeadCommitAllOf,
    /**
     * Minimal Repository
     */
    #[serde()]
    pub head_repository: MinimalRepository,
    /**
     * An invocation of a workflow
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub head_repository_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head_sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub jobs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub logs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pull_requests: Vec<PullRequestMinimal>,
    /**
     * Minimal Repository
     */
    #[serde()]
    pub repository: MinimalRepository,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub rerun_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub run_number: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub workflow_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub workflow_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Environments {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * Whether deployment to the environment(s) was approved or rejected
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum EnvironmentApprovalState {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for EnvironmentApprovalState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EnvironmentApprovalState::Approved => "approved",
            EnvironmentApprovalState::Rejected => "rejected",
            EnvironmentApprovalState::Noop => "",
            EnvironmentApprovalState::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for EnvironmentApprovalState {
    fn default() -> EnvironmentApprovalState {
        EnvironmentApprovalState::Noop
    }
}
impl EnvironmentApprovalState {
    pub fn is_noop(&self) -> bool {
        matches!(self, EnvironmentApprovalState::Noop)
    }
}

/// An entry in the reviews log for environment deployments
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnvironmentApproval {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comment: String,
    /**
     * The list of environments that were approved or rejected
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub environments: Vec<Environments>,
    /**
     * Whether deployment to the environment(s) was approved or rejected
     */
    #[serde(default, skip_serializing_if = "EnvironmentApprovalState::is_noop")]
    pub state: EnvironmentApprovalState,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<SimpleUser>,
}

/**
 * The type of reviewer. Must be one of: `User` or `Team`
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum DeploymentReviewerType {
    #[serde(rename = "Team")]
    Team,
    #[serde(rename = "User")]
    User,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for DeploymentReviewerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            DeploymentReviewerType::Team => "Team",
            DeploymentReviewerType::User => "User",
            DeploymentReviewerType::Noop => "",
            DeploymentReviewerType::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for DeploymentReviewerType {
    fn default() -> DeploymentReviewerType {
        DeploymentReviewerType::Noop
    }
}
impl DeploymentReviewerType {
    pub fn is_noop(&self) -> bool {
        matches!(self, DeploymentReviewerType::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Environment {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Any of the following types:
///
/// - `SimpleUser`
/// - `Team`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ReviewerAnyOf {
    /**
     * Simple User
     */
    SimpleUser(SimpleUser),
    /**
     * Groups of organization members that gives permissions on specified repositories.
     */
    Team(Team),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Reviewers {
    /**
     * Any of the following types:
     *  
     *  - `SimpleUser`
     *  - `Team`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer: Option<ReviewerAnyOf>,
    /**
     * The type of reviewer. Must be one of: `User` or `Team`
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<DeploymentReviewerType>,
}

/// Details of a deployment that is waiting for protection rules to pass
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PendingDeployment {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub current_user_can_approve: bool,
    #[serde()]
    pub environment: Environment,
    /**
     * The people or teams that may approve jobs that reference the environment. You can list up to six users or teams as reviewers. The reviewers must have at least read access to the repository. Only one of the required reviewers needs to approve the job for it to proceed.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewers: Vec<Reviewers>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub wait_timer: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub wait_timer_started_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// One of the following types:
///
/// - `Data`
/// - `String`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum PayloadOneOf {
    Data(Data),
    String(String),
}

/// A request for a specific ref(branch,sha,tag) to be deployed
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Deployment {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub creator: UserAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub environment: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub original_environment: String,
    /**
     * One of the following types:
     *  
     *  - `Data`
     *  - `String`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub payload: PayloadOneOf,
    /**
     * A request for a specific ref(branch,sha,tag) to be deployed
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<AppAllOf>,
    /**
     * A request for a specific ref(branch,sha,tag) to be deployed
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub production_environment: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub task: String,
    /**
     * A request for a specific ref(branch,sha,tag) to be deployed
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub transient_environment: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Macos {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub jobs: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_ms: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Billable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub macos: Option<Macos>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubuntu: Option<Macos>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows: Option<Macos>,
}

/// Workflow Run Usage
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct WorkflowRunUsage {
    #[serde()]
    pub billable: Billable,
    /**
     * Workflow Run Usage
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub run_duration_ms: i64,
}

/// Set secrets for GitHub Actions.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsSecret {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum WorkflowState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "disabled_fork")]
    DisabledFork,
    #[serde(rename = "disabled_inactivity")]
    DisabledInactivity,
    #[serde(rename = "disabled_manually")]
    DisabledManually,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for WorkflowState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            WorkflowState::Active => "active",
            WorkflowState::Deleted => "deleted",
            WorkflowState::DisabledFork => "disabled_fork",
            WorkflowState::DisabledInactivity => "disabled_inactivity",
            WorkflowState::DisabledManually => "disabled_manually",
            WorkflowState::Noop => "",
            WorkflowState::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for WorkflowState {
    fn default() -> WorkflowState {
        WorkflowState::Noop
    }
}
impl WorkflowState {
    pub fn is_noop(&self) -> bool {
        matches!(self, WorkflowState::Noop)
    }
}

/// A GitHub Actions workflow
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Workflow {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub badge_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * A GitHub Actions workflow
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(default, skip_serializing_if = "WorkflowState::is_noop")]
    pub state: WorkflowState,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Windows {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_ms: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct WorkflowUsageBillable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub macos: Option<Windows>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubuntu: Option<Windows>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows: Option<Windows>,
}

/// Workflow Usage
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct WorkflowUsage {
    #[serde()]
    pub billable: WorkflowUsageBillable,
}

/// An autolink reference.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Autolink {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key_prefix: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url_template: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnforceAdmins {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DismissalRestrictions {
    /**
     * The list of teams with review dismissal access.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub teams: Vec<Team>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub users_url: String,
}

/// Protected Branch Pull Request Review
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProtectedBranchPullRequestReview {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub dismiss_stale_reviews: bool,
    /**
     * Protected Branch Pull Request Review
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissal_restrictions: Option<DismissalRestrictions>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub require_code_owner_reviews: bool,
    /**
     * Protected Branch Pull Request Review
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub required_approving_review_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Teams {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub members_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub parent: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub permission: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub privacy: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repositories_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct BranchRestrictionPolicyAppsOwner {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub members_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organizations_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_members_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub received_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub site_admin: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscriptions_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct BranchRestrictionPolicyAppsPermissions {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub metadata: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub single_file: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Apps {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<BranchRestrictionPolicyAppsOwner>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<BranchRestrictionPolicyAppsPermissions>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
}

/// Branch Restriction Policy
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct BranchRestrictionPolicy {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub apps: Vec<Apps>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub apps_url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub teams: Vec<Teams>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<Owner>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub users_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RequiredStatusChecks {
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contexts: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contexts_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub enforcement_level: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub strict: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AllowDeletions {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
}

/// Branch Protection
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct BranchProtection {
    /**
     * Branch Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_deletions: Option<AllowDeletions>,
    /**
     * Branch Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_force_pushes: Option<AllowDeletions>,
    /**
     * Branch Protection
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    /**
     * Branch Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforce_admins: Option<EnforceAdmins>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub protection_url: String,
    /**
     * Branch Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_conversation_resolution: Option<AllowDeletions>,
    /**
     * Branch Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_linear_history: Option<AllowDeletions>,
    /**
     * Branch Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_pull_request_reviews: Option<ProtectedBranchPullRequestReview>,
    /**
     * Branch Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_signatures: Option<EnforceAdmins>,
    /**
     * Branch Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_status_checks: Option<RequiredStatusChecks>,
    /**
     * Branch Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<BranchRestrictionPolicy>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Tree {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Short Branch
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ShortBranch {
    #[serde()]
    pub commit: Tree,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub protected: bool,
    /**
     * Short Branch
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protection: Option<BranchProtection>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub protection_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Tagger {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Verification {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub payload: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub signature: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub verified: bool,
}

/// All of the following types:
///
/// - `GitUser`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum AuthorAllOf {
    GitUser(GitUser),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommitData {
    /**
     * All of the following types:
     *  
     *  - `GitUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub author: AuthorAllOf,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub comment_count: i64,
    /**
     * All of the following types:
     *  
     *  - `GitUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub committer: AuthorAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde()]
    pub tree: Tree,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<Verification>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Parents {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Files {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub additions: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blob_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub changes: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub deletions: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub filename: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub patch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub previous_filename: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub raw_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
}

/// Commit
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommitDataType {
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub author: UserAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde()]
    pub commit: CommitData,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub committer: UserAllOf,
    /**
     * Commit
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<Files>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parents: Vec<Parents>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    /**
     * Commit
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<Stats>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct BranchWithProtectionLinks {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "self"
    )]
    pub self_: String,
}

/// Branch With Protection
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct BranchWithProtection {
    #[serde(rename = "_links")]
    pub links: BranchWithProtectionLinks,
    /**
     * Commit
     */
    #[serde()]
    pub commit: CommitDataType,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pattern: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub protected: bool,
    /**
     * Branch Protection
     */
    #[serde()]
    pub protection: BranchProtection,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub protection_url: String,
    /**
     * Branch With Protection
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub required_approving_review_count: i64,
}

/// Status Check Policy
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct StatusCheckPolicy {
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contexts: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contexts_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub strict: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProtectedBranchRequiredPullRequestReviewsDismissalRestrictions {
    /**
     * The list of teams with review dismissal access.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub teams: Vec<Team>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub users_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RequiredPullRequestReviews {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub dismiss_stale_reviews: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissal_restrictions:
        Option<ProtectedBranchRequiredPullRequestReviewsDismissalRestrictions>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub require_code_owner_reviews: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub required_approving_review_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProtectedBranchRequiredLinearHistory {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
}

/// Branch protections protect branches
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProtectedBranch {
    /**
     * Branch protections protect branches
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_deletions: Option<ProtectedBranchRequiredLinearHistory>,
    /**
     * Branch protections protect branches
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_force_pushes: Option<ProtectedBranchRequiredLinearHistory>,
    /**
     * Branch protections protect branches
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforce_admins: Option<EnforceAdmins>,
    /**
     * Branch protections protect branches
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_conversation_resolution: Option<AllowDeletions>,
    /**
     * Branch protections protect branches
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_linear_history: Option<ProtectedBranchRequiredLinearHistory>,
    /**
     * Branch protections protect branches
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_pull_request_reviews: Option<RequiredPullRequestReviews>,
    /**
     * Branch protections protect branches
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_signatures: Option<EnforceAdmins>,
    /**
     * Branch protections protect branches
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_status_checks: Option<StatusCheckPolicy>,
    /**
     * Branch protections protect branches
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<BranchRestrictionPolicy>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// A deployment created as the result of an Actions check run from a workflow that references an environment
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DeploymentSimple {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub environment: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub original_environment: String,
    /**
     * A deployment created as the result of an Actions check run from a workflow that references an environment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<AppAllOf>,
    /**
     * A deployment created as the result of an Actions check run from a workflow that references an environment
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub production_environment: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub task: String,
    /**
     * A deployment created as the result of an Actions check run from a workflow that references an environment
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub transient_environment: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Conclusion {
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "skipped")]
    Skipped,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "timed_out")]
    TimedOut,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Conclusion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Conclusion::ActionRequired => "action_required",
            Conclusion::Cancelled => "cancelled",
            Conclusion::Failure => "failure",
            Conclusion::Neutral => "neutral",
            Conclusion::Skipped => "skipped",
            Conclusion::Success => "success",
            Conclusion::TimedOut => "timed_out",
            Conclusion::Noop => "",
            Conclusion::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Conclusion {
    fn default() -> Conclusion {
        Conclusion::Noop
    }
}
impl Conclusion {
    pub fn is_noop(&self) -> bool {
        matches!(self, Conclusion::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Output {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub annotations_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub annotations_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub summary: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub text: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CheckSuite {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
}

/// A check performed on the code of a given code change
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CheckRun {
    /**
     * All of the following types:
     *  
     *  - `GitHubApp`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub app: AppAllOf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_suite: Option<CheckSuite>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<Conclusion>,
    /**
     * A check performed on the code of a given code change
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment: Option<DeploymentSimple>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub details_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head_sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde()]
    pub output: Output,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pull_requests: Vec<PullRequestMinimal>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The phase of the lifecycle that the job is currently in.
     */
    #[serde(default, skip_serializing_if = "JobStatus::is_noop")]
    pub status: JobStatus,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Check Annotation
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CheckAnnotation {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub annotation_level: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blob_href: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub end_column: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub end_line: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub raw_details: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub start_column: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub start_line: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

/// A suite of checks performed on the code of a given code change
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CheckSuiteData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub after: String,
    /**
     * All of the following types:
     *  
     *  - `GitHubApp`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub app: AppAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub before: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub check_runs_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<Conclusion>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head_branch: String,
    /**
     * Simple Commit
     */
    #[serde()]
    pub head_commit: SimpleCommit,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head_sha: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub latest_check_runs_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pull_requests: Vec<PullRequestMinimal>,
    /**
     * Minimal Repository
     */
    #[serde()]
    pub repository: MinimalRepository,
    /**
     * The phase of the lifecycle that the job is currently in.
     */
    #[serde(default, skip_serializing_if = "JobStatus::is_noop")]
    pub status: JobStatus,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AutoTriggerChecks {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub app_id: i64,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub setting: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Preferences {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub auto_trigger_checks: Vec<AutoTriggerChecks>,
}

/// Check suite configuration preferences for a repository.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CheckSuitePreference {
    #[serde()]
    pub preferences: Preferences,
    /**
     * Minimal Repository
     */
    #[serde()]
    pub repository: MinimalRepository,
}

/**
 * State of a code scanning alert.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum CodeScanningAlertState {
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "dismissed")]
    Dismissed,
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for CodeScanningAlertState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CodeScanningAlertState::Closed => "closed",
            CodeScanningAlertState::Dismissed => "dismissed",
            CodeScanningAlertState::Fixed => "fixed",
            CodeScanningAlertState::Open => "open",
            CodeScanningAlertState::Noop => "",
            CodeScanningAlertState::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for CodeScanningAlertState {
    fn default() -> CodeScanningAlertState {
        CodeScanningAlertState::Noop
    }
}
impl CodeScanningAlertState {
    pub fn is_noop(&self) -> bool {
        matches!(self, CodeScanningAlertState::Noop)
    }
}

/**
 * **Required when the state is dismissed.** The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum CodeScanningAlertDismissedReason {
    #[serde(rename = "false positive")]
    FalsePositive,
    #[serde(rename = "used in tests")]
    UsedInTests,
    #[serde(rename = "won't fix")]
    WonTFix,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for CodeScanningAlertDismissedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CodeScanningAlertDismissedReason::FalsePositive => "false positive",
            CodeScanningAlertDismissedReason::UsedInTests => "used in tests",
            CodeScanningAlertDismissedReason::WonTFix => "won't fix",
            CodeScanningAlertDismissedReason::Noop => "",
            CodeScanningAlertDismissedReason::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for CodeScanningAlertDismissedReason {
    fn default() -> CodeScanningAlertDismissedReason {
        CodeScanningAlertDismissedReason::Noop
    }
}
impl CodeScanningAlertDismissedReason {
    pub fn is_noop(&self) -> bool {
        matches!(self, CodeScanningAlertDismissedReason::Noop)
    }
}

/**
 * The severity of the alert.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Severity {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Severity::Error => "error",
            Severity::None => "none",
            Severity::Note => "note",
            Severity::Warning => "warning",
            Severity::Noop => "",
            Severity::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Severity {
    fn default() -> Severity {
        Severity::Noop
    }
}
impl Severity {
    pub fn is_noop(&self) -> bool {
        matches!(self, Severity::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningAlertRuleSummary {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The severity of the alert.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<Severity>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningAnalysisTool {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub guid: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

/// Describe a region within a file for the alert.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningAlertLocation {
    /**
     * Describe a region within a file for the alert.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub end_column: i64,
    /**
     * Describe a region within a file for the alert.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub end_line: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    /**
     * Describe a region within a file for the alert.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub start_column: i64,
    /**
     * Describe a region within a file for the alert.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub start_line: i64,
}

/**
 * A classification of the file. For example to identify it as generated.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum CodeScanningAlertClassification {
    #[serde(rename = "generated")]
    Generated,
    #[serde(rename = "library")]
    Library,
    #[serde(rename = "source")]
    Source,
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for CodeScanningAlertClassification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CodeScanningAlertClassification::Generated => "generated",
            CodeScanningAlertClassification::Library => "library",
            CodeScanningAlertClassification::Source => "source",
            CodeScanningAlertClassification::Test => "test",
            CodeScanningAlertClassification::Noop => "",
            CodeScanningAlertClassification::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for CodeScanningAlertClassification {
    fn default() -> CodeScanningAlertClassification {
        CodeScanningAlertClassification::Noop
    }
}
impl CodeScanningAlertClassification {
    pub fn is_noop(&self) -> bool {
        matches!(self, CodeScanningAlertClassification::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Message {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningAlertInstance {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub analysis_key: String,
    /**
     * Classifications that have been applied to the file that triggered the alert.
     *  For example identifying it as documentation, or a generated file.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifications: Vec<CodeScanningAlertClassification>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub environment: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Describe a region within a file for the alert.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<CodeScanningAlertLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    /**
     * State of a code scanning alert.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<CodeScanningAlertState>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningAlertItems {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub dismissed_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissed_by: Option<SimpleUser>,
    /**
     * \*\*Required when the state is dismissed.\*\* The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissed_reason: Option<CodeScanningAlertDismissedReason>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub instances_url: String,
    #[serde()]
    pub most_recent_instance: CodeScanningAlertInstance,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub number: i64,
    #[serde()]
    pub rule: CodeScanningAlertRuleSummary,
    /**
     * State of a code scanning alert.
     */
    #[serde()]
    pub state: CodeScanningAlertState,
    #[serde()]
    pub tool: CodeScanningAnalysisTool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The security severity of the alert.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum SecuritySeverityLevel {
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for SecuritySeverityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SecuritySeverityLevel::Critical => "critical",
            SecuritySeverityLevel::High => "high",
            SecuritySeverityLevel::Low => "low",
            SecuritySeverityLevel::Medium => "medium",
            SecuritySeverityLevel::Noop => "",
            SecuritySeverityLevel::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for SecuritySeverityLevel {
    fn default() -> SecuritySeverityLevel {
        SecuritySeverityLevel::Noop
    }
}
impl SecuritySeverityLevel {
    pub fn is_noop(&self) -> bool {
        matches!(self, SecuritySeverityLevel::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningAlertRule {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub help: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The security severity of the alert.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_severity_level: Option<SecuritySeverityLevel>,
    /**
     * The severity of the alert.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<Severity>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningAlert {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub dismissed_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissed_by: Option<SimpleUser>,
    /**
     * \*\*Required when the state is dismissed.\*\* The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissed_reason: Option<CodeScanningAlertDismissedReason>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instances: Option<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub instances_url: String,
    #[serde()]
    pub most_recent_instance: CodeScanningAlertInstance,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub number: i64,
    #[serde()]
    pub rule: CodeScanningAlertRule,
    /**
     * State of a code scanning alert.
     */
    #[serde()]
    pub state: CodeScanningAlertState,
    #[serde()]
    pub tool: CodeScanningAnalysisTool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * Sets the state of the code scanning alert. Can be one of `open` or `dismissed`. You must provide `dismissed_reason` when you set the state to `dismissed`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum CodeScanningAlertSetState {
    #[serde(rename = "dismissed")]
    Dismissed,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for CodeScanningAlertSetState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CodeScanningAlertSetState::Dismissed => "dismissed",
            CodeScanningAlertSetState::Open => "open",
            CodeScanningAlertSetState::Noop => "",
            CodeScanningAlertSetState::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for CodeScanningAlertSetState {
    fn default() -> CodeScanningAlertSetState {
        CodeScanningAlertSetState::Noop
    }
}
impl CodeScanningAlertSetState {
    pub fn is_noop(&self) -> bool {
        matches!(self, CodeScanningAlertSetState::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningAnalysis {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub analysis_key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub category: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_sha: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub deletable: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub environment: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub results_count: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub rules_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sarif_id: String,
    #[serde()]
    pub tool: CodeScanningAnalysisTool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tool_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub warning: String,
}

/// Successful deletion of a code scanning analysis
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AnalysisDeletion {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub confirm_delete_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_analysis_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningSarifsReceipt {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * `pending` files have not yet been processed, while `complete` means all results in the SARIF have been stored.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ProcessingStatus {
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for ProcessingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ProcessingStatus::Complete => "complete",
            ProcessingStatus::Pending => "pending",
            ProcessingStatus::Noop => "",
            ProcessingStatus::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for ProcessingStatus {
    fn default() -> ProcessingStatus {
        ProcessingStatus::Noop
    }
}
impl ProcessingStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ProcessingStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningSarifsStatus {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub analyses_url: String,
    /**
     * `pending` files have not yet been processed, while `complete` means all results in the SARIF have been stored.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processing_status: Option<ProcessingStatus>,
}

/// Collaborator
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Collaborator {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organizations_url: String,
    /**
     * Collaborator
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<FullRepositoryPermissions>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub received_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub site_admin: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscriptions_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The permission associated with the invitation.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum RepositoryInvitationPermissions {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "maintain")]
    Maintain,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "triage")]
    Triage,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for RepositoryInvitationPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            RepositoryInvitationPermissions::Admin => "admin",
            RepositoryInvitationPermissions::Maintain => "maintain",
            RepositoryInvitationPermissions::Read => "read",
            RepositoryInvitationPermissions::Triage => "triage",
            RepositoryInvitationPermissions::Write => "write",
            RepositoryInvitationPermissions::Noop => "",
            RepositoryInvitationPermissions::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for RepositoryInvitationPermissions {
    fn default() -> RepositoryInvitationPermissions {
        RepositoryInvitationPermissions::Noop
    }
}
impl RepositoryInvitationPermissions {
    pub fn is_noop(&self) -> bool {
        matches!(self, RepositoryInvitationPermissions::Noop)
    }
}

/// Repository invitations let you manage who you collaborate with.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RepositoryInvitation {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Repository invitations let you manage who you collaborate with.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub expired: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub invitee: UserAllOf,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub inviter: UserAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * The permission associated with the invitation.
     */
    #[serde(
        default,
        skip_serializing_if = "RepositoryInvitationPermissions::is_noop"
    )]
    pub permissions: RepositoryInvitationPermissions,
    /**
     * Minimal Repository
     */
    #[serde()]
    pub repository: MinimalRepository,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Commit Comment
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommitComment {
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub line: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub position: i64,
    /**
     * Commit Comment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<ReactionRollup>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub user: UserAllOf,
}

/// Branch Short
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct BranchShort {
    #[serde()]
    pub commit: Tree,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub protected: bool,
}

/// Hypermedia Link
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Link {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub href: String,
}

/**
 * The merge method to use.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum MergeMethod {
    #[serde(rename = "merge")]
    Merge,
    #[serde(rename = "rebase")]
    Rebase,
    #[serde(rename = "squash")]
    Squash,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for MergeMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            MergeMethod::Merge => "merge",
            MergeMethod::Rebase => "rebase",
            MergeMethod::Squash => "squash",
            MergeMethod::Noop => "",
            MergeMethod::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for MergeMethod {
    fn default() -> MergeMethod {
        MergeMethod::Noop
    }
}
impl MergeMethod {
    pub fn is_noop(&self) -> bool {
        matches!(self, MergeMethod::Noop)
    }
}

/// The status of auto merging a pull request.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AutoMerge {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_title: String,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled_by: Option<SimpleUser>,
    /**
     * The merge method to use.
     */
    #[serde(default, skip_serializing_if = "MergeMethod::is_noop")]
    pub merge_method: MergeMethod,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Base {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    /**
     * A git repository
     */
    #[serde()]
    pub repo: Repository,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub user: UserAllOf,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestSimpleLinks {
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub comments: Link,
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub commits: Link,
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub html: Link,
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub issue: Link,
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub review_comment: Link,
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub review_comments: Link,
    /**
     * Hypermedia Link
     */
    #[serde(rename = "self")]
    pub self_: Link,
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub statuses: Link,
}

/// Pull Request Simple
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestSimple {
    #[serde(rename = "_links")]
    pub links: PullRequestSimpleLinks,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub active_lock_reason: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub assignee: UserAllOf,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<SimpleUser>,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    /**
     * The status of auto merging a pull request.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_merge: Option<AutoMerge>,
    #[serde()]
    pub base: Base,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub diff_url: String,
    /**
     * Pull Request Simple
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub draft: bool,
    #[serde()]
    pub head: Base,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<LabelsData>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub locked: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merge_commit_sha: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub merged_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * All of the following types:
     *  
     *  - `Milestone`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub milestone: MilestoneAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub number: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub patch_url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requested_reviewers: Vec<SimpleUser>,
    /**
     * Pull Request Simple
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requested_teams: Vec<Team>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub review_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub review_comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub user: UserAllOf,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SimpleCommitStatus {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub context: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub required: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Combined Commit Status
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CombinedCommitStatus {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    /**
     * Minimal Repository
     */
    #[serde()]
    pub repository: MinimalRepository,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<SimpleCommitStatus>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// The status of a commit.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct StatusData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub context: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommunityHealthFile {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// All of the following types:
///
/// - `CodeOfConductSimple`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum CodeOfConductAll {
    /**
     * Code of Conduct Simple
     */
    CodeOfConductSimple(CodeOfConductSimple),
}

/// All of the following types:
///
/// - `CommunityHealthFile`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ReadmeAllOf {
    CommunityHealthFile(CommunityHealthFile),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommunityProfileFiles {
    /**
     * All of the following types:
     *  
     *  - `CodeOfConductSimple`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub code_of_conduct: CodeOfConductAll,
    /**
     * All of the following types:
     *  
     *  - `CommunityHealthFile`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub code_of_conduct_file: ReadmeAllOf,
    /**
     * All of the following types:
     *  
     *  - `CommunityHealthFile`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub contributing: ReadmeAllOf,
    /**
     * All of the following types:
     *  
     *  - `CommunityHealthFile`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub issue_template: ReadmeAllOf,
    /**
     * All of the following types:
     *  
     *  - `LicenseSimple`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub license: LicenseAllOf,
    /**
     * All of the following types:
     *  
     *  - `CommunityHealthFile`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub pull_request_template: ReadmeAllOf,
    /**
     * All of the following types:
     *  
     *  - `CommunityHealthFile`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub readme: ReadmeAllOf,
}

/// Community Profile
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommunityProfile {
    /**
     * Community Profile
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub content_reports_enabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation: String,
    #[serde()]
    pub files: CommunityProfileFiles,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub health_percentage: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Diff Entry
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DiffEntry {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub additions: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blob_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub changes: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub deletions: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub filename: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub patch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub previous_filename: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub raw_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum CommitComparisonStatus {
    #[serde(rename = "ahead")]
    Ahead,
    #[serde(rename = "behind")]
    Behind,
    #[serde(rename = "diverged")]
    Diverged,
    #[serde(rename = "identical")]
    Identical,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for CommitComparisonStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CommitComparisonStatus::Ahead => "ahead",
            CommitComparisonStatus::Behind => "behind",
            CommitComparisonStatus::Diverged => "diverged",
            CommitComparisonStatus::Identical => "identical",
            CommitComparisonStatus::Noop => "",
            CommitComparisonStatus::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for CommitComparisonStatus {
    fn default() -> CommitComparisonStatus {
        CommitComparisonStatus::Noop
    }
}
impl CommitComparisonStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, CommitComparisonStatus::Noop)
    }
}

/// Commit Comparison
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommitComparison {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub ahead_by: i64,
    /**
     * Commit
     */
    #[serde()]
    pub base_commit: CommitDataType,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub behind_by: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commits: Vec<CommitDataType>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub diff_url: String,
    /**
     * Commit Comparison
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<DiffEntry>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Commit
     */
    #[serde()]
    pub merge_base_commit: CommitDataType,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub patch_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub permalink_url: String,
    #[serde(default, skip_serializing_if = "CommitComparisonStatus::is_noop")]
    pub status: CommitComparisonStatus,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_commits: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Content Reference attachments allow you to provide context around URLs posted in comments
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ContentReferenceAttachment {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ContentTreeEntriesLinks {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "self"
    )]
    pub self_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Entries {
    #[serde(rename = "_links")]
    pub links: ContentTreeEntriesLinks,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub download_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Content Tree
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ContentTree {
    #[serde(rename = "_links")]
    pub links: ContentTreeEntriesLinks,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub download_url: String,
    /**
     * Content Tree
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<Entries>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Content File
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ContentFile {
    #[serde(rename = "_links")]
    pub links: ContentTreeEntriesLinks,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub download_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub encoding: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub submodule_git_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// An object describing a symlink
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SymlinkContent {
    #[serde(rename = "_links")]
    pub links: ContentTreeEntriesLinks,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub download_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// An object describing a symlink
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ContentSubmodule {
    #[serde(rename = "_links")]
    pub links: ContentTreeEntriesLinks,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub download_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub submodule_git_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FileCommitContent {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<ContentTreeEntriesLinks>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub download_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FileCommitVerification {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub payload: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub signature: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub verified: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FileCommit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<Tagger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committer: Option<Tagger>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parents: Vec<Parents>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree: Option<Tree>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<FileCommitVerification>,
}

/// File Commit
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FileCommitData {
    #[serde()]
    pub commit: FileCommit,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<FileCommitContent>,
}

/// Contributor
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Contributor {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub contributions: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Contributor
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organizations_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub received_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    /**
     * Contributor
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub site_admin: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscriptions_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The state of the status.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum DeploymentStatusState {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for DeploymentStatusState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            DeploymentStatusState::Error => "error",
            DeploymentStatusState::Failure => "failure",
            DeploymentStatusState::InProgress => "in_progress",
            DeploymentStatusState::Inactive => "inactive",
            DeploymentStatusState::Pending => "pending",
            DeploymentStatusState::Queued => "queued",
            DeploymentStatusState::Success => "success",
            DeploymentStatusState::Noop => "",
            DeploymentStatusState::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for DeploymentStatusState {
    fn default() -> DeploymentStatusState {
        DeploymentStatusState::Noop
    }
}
impl DeploymentStatusState {
    pub fn is_noop(&self) -> bool {
        matches!(self, DeploymentStatusState::Noop)
    }
}

/// The status of a deployment.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DeploymentStatus {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub creator: UserAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub environment: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub environment_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub log_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * The status of a deployment.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<AppAllOf>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    /**
     * The state of the status.
     */
    #[serde(default, skip_serializing_if = "DeploymentStatusState::is_noop")]
    pub state: DeploymentStatusState,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// The type of deployment branch policy for this environment. To allow all branches to deploy, set to `null`.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DeploymentBranchPolicy {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub custom_branch_policies: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub protected_branches: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProtectionRules {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub wait_timer: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProtectionRulesData {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * The people or teams that may approve jobs that reference the environment. You can list up to six users or teams as reviewers. The reviewers must have at least read access to the repository. Only one of the required reviewers needs to approve the job for it to proceed.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewers: Vec<Reviewers>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProtectionRulesDataType {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

/// Any of the following types:
///
/// - `ProtectionRules`
/// - `ProtectionRulesData`
/// - `ProtectionRulesDataType`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ProtectionRulesAnyOf {
    ProtectionRules(ProtectionRules),
    ProtectionRulesData(ProtectionRulesData),
    ProtectionRulesDataType(ProtectionRulesDataType),
}

/// Details of a deployment environment
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnvironmentData {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The type of deployment branch policy for this environment. To allow all branches to deploy, set to `null`.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_branch_policy: Option<DeploymentBranchPolicy>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Details of a deployment environment
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protection_rules: Vec<ProtectionRulesAnyOf>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Blob
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Blob {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub encoding: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub highlighted_content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Identifying information for the git-user
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Committer {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// Low-level Git commit operations within a repository
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCommit {
    /**
     * Identifying information for the git-user
     */
    #[serde()]
    pub author: Committer,
    /**
     * Identifying information for the git-user
     */
    #[serde()]
    pub committer: Committer,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parents: Vec<Parents>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde()]
    pub tree: Tree,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde()]
    pub verification: Verification,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Object {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Git references within a repository
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitRef {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde()]
    pub object: Object,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Metadata for a Git tag
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitTag {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde()]
    pub object: Object,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tag: String,
    #[serde()]
    pub tagger: Tagger,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Metadata for a Git tag
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<Verification>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitTree {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mode: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// The hierarchy between files in a Git repository.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitTreeData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    /**
     * Objects specifying a tree structure
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tree: Vec<GitTree>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub truncated: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct HookResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub code: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct HookConfig {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub digest: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * One of the following types:
     *  
     *  - `String`
     *  - `f64`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSslOneOf>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub password: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub room: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subdomain: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Webhooks for repositories.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Hook {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub active: bool,
    #[serde()]
    pub config: HookConfig,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deliveries_url: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde()]
    pub last_response: HookResponse,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ping_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub test_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ImportStatus {
    #[serde(rename = "auth")]
    Auth,
    #[serde(rename = "auth_failed")]
    AuthFailed,
    #[serde(rename = "choose")]
    Choose,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "detecting")]
    Detecting,
    #[serde(rename = "detection_found_multiple")]
    DetectionFoundMultiple,
    #[serde(rename = "detection_found_nothing")]
    DetectionFoundNothing,
    #[serde(rename = "detection_needs_auth")]
    DetectionNeedsAuth,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "importing")]
    Importing,
    #[serde(rename = "mapping")]
    Mapping,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "pushing")]
    Pushing,
    #[serde(rename = "setup")]
    Setup,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "waiting_to_push")]
    WaitingToPush,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for ImportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ImportStatus::Auth => "auth",
            ImportStatus::AuthFailed => "auth_failed",
            ImportStatus::Choose => "choose",
            ImportStatus::Complete => "complete",
            ImportStatus::Detecting => "detecting",
            ImportStatus::DetectionFoundMultiple => "detection_found_multiple",
            ImportStatus::DetectionFoundNothing => "detection_found_nothing",
            ImportStatus::DetectionNeedsAuth => "detection_needs_auth",
            ImportStatus::Error => "error",
            ImportStatus::Importing => "importing",
            ImportStatus::Mapping => "mapping",
            ImportStatus::None => "none",
            ImportStatus::Pushing => "pushing",
            ImportStatus::Setup => "setup",
            ImportStatus::Unknown => "unknown",
            ImportStatus::WaitingToPush => "waiting_to_push",
            ImportStatus::Noop => "",
            ImportStatus::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for ImportStatus {
    fn default() -> ImportStatus {
        ImportStatus::Noop
    }
}
impl ImportStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ImportStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectChoices {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub human_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tfvc_project: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vcs: String,
}

/// A repository import from an external source.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Import {
    /**
     * A repository import from an external source.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub authors_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub authors_url: String,
    /**
     * A repository import from an external source.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub commit_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error_message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub failed_step: String,
    /**
     * A repository import from an external source.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_large_files: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * A repository import from an external source.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub import_percent: i64,
    /**
     * A repository import from an external source.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub large_files_count: i64,
    /**
     * A repository import from an external source.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub large_files_size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
     * A repository import from an external source.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub project_choices: Vec<ProjectChoices>,
    /**
     * A repository import from an external source.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub push_percent: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    #[serde(default, skip_serializing_if = "ImportStatus::is_noop")]
    pub status: ImportStatus,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status_text: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svc_root: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svn_root: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tfvc_project: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * A repository import from an external source.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub use_lfs: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vcs: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vcs_url: String,
}

/// Porter Author
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PorterAuthor {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub import_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub remote_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub remote_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Porter Large File
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PorterLargeFile {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub oid: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ref_name: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
}

/// Issue Event Label
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssueEventLabel {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub color: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DismissedReview {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub dismissal_commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub dismissal_message: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub review_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
}

/// Issue Event Milestone
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssueEventMilestone {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

/// Issue Event Project Card
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssueEventProjectCard {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub column_name: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub previous_column_name: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub project_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub project_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Rename {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub to: String,
}

/// Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssueEvent {
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub actor: UserAllOf,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<UserAllOf>,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigner: Option<UserAllOf>,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_association: Option<AuthorAssociation>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissed_review: Option<DismissedReview>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<IssueSimple>,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<IssueEventLabel>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub lock_reason: String,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestone: Option<IssueEventMilestone>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<AppAllOf>,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_card: Option<IssueEventProjectCard>,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rename: Option<Rename>,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_reviewer: Option<UserAllOf>,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_team: Option<Team>,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_requester: Option<UserAllOf>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Labeled Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct LabeledIssueEvent {
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Issue Event Label
     */
    #[serde()]
    pub label: IssueEventLabel,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<GitHubApp>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Assigned Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AssignedIssueEvent {
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<SimpleUser>,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<SimpleUser>,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigner: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<GitHubApp>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Milestoned Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MilestonedIssueEvent {
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Issue Event Milestone
     */
    #[serde()]
    pub milestone: IssueEventMilestone,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<GitHubApp>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Renamed Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RenamedIssueEvent {
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<GitHubApp>,
    #[serde()]
    pub rename: Rename,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Review Requested Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReviewRequestedIssueEvent {
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<GitHubApp>,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_reviewer: Option<SimpleUser>,
    /**
     * Review Requested Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_team: Option<Team>,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_requester: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Review Request Removed Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReviewRequestRemovedIssueEvent {
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<GitHubApp>,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_reviewer: Option<SimpleUser>,
    /**
     * Review Request Removed Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_team: Option<Team>,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_requester: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Review Dismissed Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReviewDismissedIssueEvent {
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde()]
    pub dismissed_review: DismissedReview,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<GitHubApp>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Locked Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct LockedIssueEvent {
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub lock_reason: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<GitHubApp>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Added to Project Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AddedProjectIssueEvent {
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<GitHubApp>,
    /**
     * Added to Project Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_card: Option<IssueEventProjectCard>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Moved Column in Project Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MovedColumnInProjectIssueEvent {
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<GitHubApp>,
    /**
     * Moved Column in Project Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_card: Option<IssueEventProjectCard>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Removed from Project Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RemovedFromProjectIssueEvent {
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<GitHubApp>,
    /**
     * Removed from Project Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_card: Option<IssueEventProjectCard>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Converted Note to Issue Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ConvertedNoteIssueEvent {
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<GitHubApp>,
    /**
     * Converted Note to Issue Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_card: Option<IssueEventProjectCard>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Any of the following types:
///
/// - `LabeledIssueEvent`
/// - `LabeledIssueEvent`
/// - `AssignedIssueEvent`
/// - `AssignedIssueEvent`
/// - `MilestonedIssueEvent`
/// - `MilestonedIssueEvent`
/// - `RenamedIssueEvent`
/// - `ReviewRequestedIssueEvent`
/// - `ReviewRequestRemovedIssueEvent`
/// - `ReviewDismissedIssueEvent`
/// - `LockedIssueEvent`
/// - `AddedProjectIssueEvent`
/// - `MovedColumnInProjectIssueEvent`
/// - `RemovedFromProjectIssueEvent`
/// - `ConvertedNoteIssueEvent`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum IssueEventAnyOf {
    /**
     * Added to Project Issue Event
     */
    AddedProjectIssueEvent(AddedProjectIssueEvent),
    /**
     * Assigned Issue Event
     */
    AssignedIssueEvent(AssignedIssueEvent),
    /**
     * Converted Note to Issue Issue Event
     */
    ConvertedNoteIssueEvent(ConvertedNoteIssueEvent),
    /**
     * Labeled Issue Event
     */
    LabeledIssueEvent(LabeledIssueEvent),
    /**
     * Locked Issue Event
     */
    LockedIssueEvent(LockedIssueEvent),
    /**
     * Milestoned Issue Event
     */
    MilestonedIssueEvent(MilestonedIssueEvent),
    /**
     * Moved Column in Project Issue Event
     */
    MovedColumnInProjectIssueEvent(MovedColumnInProjectIssueEvent),
    /**
     * Removed from Project Issue Event
     */
    RemovedFromProjectIssueEvent(RemovedFromProjectIssueEvent),
    /**
     * Renamed Issue Event
     */
    RenamedIssueEvent(RenamedIssueEvent),
    /**
     * Review Dismissed Issue Event
     */
    ReviewDismissedIssueEvent(ReviewDismissedIssueEvent),
    /**
     * Review Request Removed Issue Event
     */
    ReviewRequestRemovedIssueEvent(ReviewRequestRemovedIssueEvent),
    /**
     * Review Requested Issue Event
     */
    ReviewRequestedIssueEvent(ReviewRequestedIssueEvent),
}

/// Timeline Comment Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimelineCommentEvent {
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<SimpleUser>,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<GitHubApp>,
    /**
     * Timeline Comment Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<ReactionRollup>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<SimpleUser>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Source {
    /**
     * Issue Simple
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<IssueSimple>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

/// Timeline Cross Referenced Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimelineCrossReferencedEvent {
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde()]
    pub source: Source,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Timeline Committed Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimelineCommittedEvent {
    /**
     * Identifying information for the git-user
     */
    #[serde()]
    pub author: Committer,
    /**
     * Identifying information for the git-user
     */
    #[serde()]
    pub committer: Committer,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parents: Vec<Parents>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde()]
    pub tree: Tree,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde()]
    pub verification: Verification,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimelineReviewedEventLinks {
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub html: Link,
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub pull_request: Link,
}

/// Timeline Reviewed Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimelineReviewedEvent {
    #[serde(rename = "_links")]
    pub links: TimelineReviewedEventLinks,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pull_request_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * Timeline Reviewed Event
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub submitted_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<SimpleUser>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestReviewCommentLinks {
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub html: Link,
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub pull_request: Link,
    /**
     * Hypermedia Link
     */
    #[serde(rename = "self")]
    pub self_: Link,
}

/**
 * The side of the diff to which the comment applies. The side of the last line of the range for a multi-line comment
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Side {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
    FallthroughString(String),
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Side::Left => "LEFT",
            Side::Right => "RIGHT",
            Side::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Side {
    fn default() -> Side {
        Side::Right
    }
}

/// Pull Request Review Comments are comments on a portion of the Pull Request's diff.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestReviewComment {
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentLinks,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub diff_hunk: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Pull Request Review Comments are comments on a portion of the Pull Request's diff.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub in_reply_to_id: i64,
    /**
     * Pull Request Review Comments are comments on a portion of the Pull Request's diff.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub line: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub original_commit_id: String,
    /**
     * Pull Request Review Comments are comments on a portion of the Pull Request's diff.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub original_line: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub original_position: i64,
    /**
     * Pull Request Review Comments are comments on a portion of the Pull Request's diff.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub original_start_line: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub position: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub pull_request_review_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pull_request_url: String,
    /**
     * Pull Request Review Comments are comments on a portion of the Pull Request's diff.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<ReactionRollup>,
    /**
     * Pull Request Review Comments are comments on a portion of the Pull Request's diff.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    /**
     * Pull Request Review Comments are comments on a portion of the Pull Request's diff.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub start_line: i64,
    /**
     * The side of the first line of the range for a multi-line comment.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_side: Option<Side>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<SimpleUser>,
}

/// Timeline Line Commented Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimelineLineCommentedEvent {
    /**
     * Timeline Line Commented Event
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<PullRequestReviewComment>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
}

/// Timeline Commit Commented Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimelineCommitCommentedEvent {
    /**
     * Timeline Commit Commented Event
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<CommitComment>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
}

/// Timeline Assigned Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimelineAssignedIssueEvent {
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<SimpleUser>,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<GitHubApp>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// An SSH key granting access to a single repository.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DeployKey {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub read_only: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub verified: bool,
}

/// License Content
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct LicenseContent {
    #[serde(rename = "_links")]
    pub links: ContentTreeEntriesLinks,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub download_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub encoding: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * All of the following types:
     *  
     *  - `LicenseSimple`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub license: LicenseAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Default, Deserialize, Debug, Clone, JsonSchema)]
pub struct PagesSourceHash {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum PagesHttpsCertificateState {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "authorization_created")]
    AuthorizationCreated,
    #[serde(rename = "authorization_pending")]
    AuthorizationPending,
    #[serde(rename = "authorization_revoked")]
    AuthorizationRevoked,
    #[serde(rename = "authorized")]
    Authorized,
    #[serde(rename = "bad_authz")]
    BadAuthz,
    #[serde(rename = "destroy_pending")]
    DestroyPending,
    #[serde(rename = "dns_changed")]
    DnsChanged,
    #[serde(rename = "errored")]
    Errored,
    #[serde(rename = "issued")]
    Issued,
    #[serde(rename = "new")]
    New,
    #[serde(rename = "uploaded")]
    Uploaded,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for PagesHttpsCertificateState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PagesHttpsCertificateState::Approved => "approved",
            PagesHttpsCertificateState::AuthorizationCreated => "authorization_created",
            PagesHttpsCertificateState::AuthorizationPending => "authorization_pending",
            PagesHttpsCertificateState::AuthorizationRevoked => "authorization_revoked",
            PagesHttpsCertificateState::Authorized => "authorized",
            PagesHttpsCertificateState::BadAuthz => "bad_authz",
            PagesHttpsCertificateState::DestroyPending => "destroy_pending",
            PagesHttpsCertificateState::DnsChanged => "dns_changed",
            PagesHttpsCertificateState::Errored => "errored",
            PagesHttpsCertificateState::Issued => "issued",
            PagesHttpsCertificateState::New => "new",
            PagesHttpsCertificateState::Uploaded => "uploaded",
            PagesHttpsCertificateState::Noop => "",
            PagesHttpsCertificateState::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for PagesHttpsCertificateState {
    fn default() -> PagesHttpsCertificateState {
        PagesHttpsCertificateState::Noop
    }
}
impl PagesHttpsCertificateState {
    pub fn is_noop(&self) -> bool {
        matches!(self, PagesHttpsCertificateState::Noop)
    }
}

#[derive(Serialize, Default, Deserialize, Debug, Clone, JsonSchema)]
pub struct PagesHttpsCertificate {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub domains: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "PagesHttpsCertificateState::is_noop")]
    pub state: PagesHttpsCertificateState,
}

/**
 * The status of the most recent build of the Page.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum PageStatus {
    #[serde(rename = "building")]
    Building,
    #[serde(rename = "built")]
    Built,
    #[serde(rename = "errored")]
    Errored,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for PageStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PageStatus::Building => "building",
            PageStatus::Built => "built",
            PageStatus::Errored => "errored",
            PageStatus::Noop => "",
            PageStatus::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for PageStatus {
    fn default() -> PageStatus {
        PageStatus::Noop
    }
}
impl PageStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, PageStatus::Noop)
    }
}

/// The configuration for GitHub Pages for a repository.
#[derive(Serialize, Default, Deserialize, Debug, Clone, JsonSchema)]
pub struct Page {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub cname: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub custom_404: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde()]
    pub https_certificate: PagesHttpsCertificate,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub https_enforced: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub public: bool,
    #[serde()]
    pub source: PagesSourceHash,
    /**
     * The status of the most recent build of the Page.
     */
    #[serde(default, skip_serializing_if = "PageStatus::is_noop")]
    pub status: PageStatus,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Error {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

/// Page Build
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PageBuild {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub duration: i64,
    #[serde()]
    pub error: Error,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub pusher: UserAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Page Build Status
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PageBuildStatus {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Domain {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub caa_error: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub dns_resolves: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enforces_https: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_cname_record: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_mx_records_present: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub host: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub https_error: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_a_record: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_apex_domain: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_cloudflare_ip: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_cname_to_fastly: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_cname_to_github_user_domain: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_cname_to_pages_dot_github_dot_com: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_fastly_ip: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_https_eligible: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_non_github_pages_ip_present: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_old_ip_address: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_pages_domain: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_pointed_to_github_pages_ip: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_proxied: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_served_by_pages: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_valid: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_valid_domain: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub nameservers: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub responds_to_https: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub should_be_a_record: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AltDomain {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub caa_error: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub dns_resolves: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enforces_https: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_cname_record: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_mx_records_present: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub host: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub https_error: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_a_record: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_apex_domain: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_cloudflare_ip: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_cname_to_fastly: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_cname_to_github_user_domain: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_cname_to_pages_dot_github_dot_com: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_fastly_ip: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_https_eligible: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_non_github_pages_ip_present: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_old_ip_address: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_pages_domain: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_pointed_to_github_pages_ip: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_proxied: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_served_by_pages: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_valid: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_valid_domain: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub nameservers: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub responds_to_https: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub should_be_a_record: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uri: String,
}

/// Pages Health Check Status
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PagesHealthCheck {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt_domain: Option<AltDomain>,
    /**
     * Pages Health Check Status
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<Domain>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct User {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organizations_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub received_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub site_admin: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscriptions_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestHeadRepo {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_merge_commit: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_rebase_merge: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_squash_merge: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub archived: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignees_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blobs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branches_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub clone_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub collaborators_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub compare_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contributors_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_branch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub disabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub downloads_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub fork: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forks: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forks_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_refs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_downloads: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_issues: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_pages: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_projects: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_wiki: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub keys_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub languages_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<MinimalRepositoryLicense>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub master_branch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merges_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub milestones_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mirror_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notifications_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub open_issues: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub open_issues_count: i64,
    #[serde()]
    pub owner: User,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<FullRepositoryPermissions>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub private: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pulls_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub pushed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub releases_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ssh_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub stargazers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stargazers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscribers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svn_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub temp_clone_token: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trees_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub watchers: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub watchers_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestHead {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repo: Option<PullRequestHeadRepo>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde()]
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestBaseRepo {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_merge_commit: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_rebase_merge: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_squash_merge: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub archived: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignees_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blobs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branches_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub clone_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub collaborators_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub compare_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contributors_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_branch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub disabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub downloads_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub fork: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forks: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forks_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_refs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_downloads: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_issues: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_pages: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_projects: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_wiki: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub keys_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub languages_url: String,
    /**
     * All of the following types:
     *  
     *  - `LicenseSimple`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub license: LicenseAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub master_branch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merges_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub milestones_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mirror_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notifications_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub open_issues: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub open_issues_count: i64,
    #[serde()]
    pub owner: User,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<FullRepositoryPermissions>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub private: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pulls_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub pushed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub releases_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ssh_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub stargazers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stargazers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscribers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svn_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub temp_clone_token: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trees_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub watchers: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub watchers_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestBase {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    #[serde()]
    pub repo: PullRequestBaseRepo,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde()]
    pub user: User,
}

/// Pull requests let you tell others about changes you've pushed to a repository on GitHub. Once a pull request is sent, interested parties can review the set of changes, discuss potential modifications, and even push follow-up commits if necessary.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestData {
    #[serde(rename = "_links")]
    pub links: PullRequestSimpleLinks,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub active_lock_reason: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub additions: i64,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub assignee: UserAllOf,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<SimpleUser>,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    /**
     * The status of auto merging a pull request.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_merge: Option<AutoMerge>,
    #[serde()]
    pub base: PullRequestBase,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub changed_files: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub comments: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub commits: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub deletions: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub diff_url: String,
    /**
     * Pull requests let you tell others about changes you've pushed to a repository on GitHub. Once a pull request is sent, interested parties can review the set of changes, discuss potential modifications, and even push follow-up commits if necessary.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub draft: bool,
    #[serde()]
    pub head: PullRequestHead,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<LabelsData>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub locked: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub maintainer_can_modify: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merge_commit_sha: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub mergeable: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mergeable_state: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub merged: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub merged_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub merged_by: UserAllOf,
    /**
     * All of the following types:
     *  
     *  - `Milestone`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub milestone: MilestoneAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub number: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub patch_url: String,
    /**
     * Pull requests let you tell others about changes you've pushed to a repository on GitHub. Once a pull request is sent, interested parties can review the set of changes, discuss potential modifications, and even push follow-up commits if necessary.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub rebaseable: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requested_reviewers: Vec<SimpleUser>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requested_teams: Vec<TeamSimple>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub review_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub review_comments: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub review_comments_url: String,
    /**
     * The state of the milestone.
     */
    #[serde(default)]
    pub state: State,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub user: UserAllOf,
}

/// Pull Request Merge Result
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestMergeResult {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub merged: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
}

/// Pull Request Review Request
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestReview {
    /**
     * The list of teams with review dismissal access.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub teams: Vec<Team>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<SimpleUser>,
}

/// Pull Request Reviews are reviews on pull requests.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestReviewData {
    #[serde(rename = "_links")]
    pub links: TimelineReviewedEventLinks,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pull_request_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * Pull Request Reviews are reviews on pull requests.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub submitted_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub user: UserAllOf,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReviewCommentLinks {
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub html: Link,
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub pull_request: Link,
    /**
     * Hypermedia Link
     */
    #[serde(rename = "self")]
    pub self_: Link,
}

/// Legacy Review Comment
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReviewComment {
    #[serde(rename = "_links")]
    pub links: ReviewCommentLinks,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub diff_hunk: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Legacy Review Comment
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub in_reply_to_id: i64,
    /**
     * Legacy Review Comment
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub line: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub original_commit_id: String,
    /**
     * Legacy Review Comment
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub original_line: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub original_position: i64,
    /**
     * Legacy Review Comment
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub original_start_line: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub position: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub pull_request_review_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pull_request_url: String,
    /**
     * Legacy Review Comment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<ReactionRollup>,
    /**
     * Legacy Review Comment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    /**
     * Legacy Review Comment
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub start_line: i64,
    /**
     * Legacy Review Comment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_side: Option<Side>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub user: UserAllOf,
}

/**
 * State of the release asset.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ReleaseAssetState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "uploaded")]
    Uploaded,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for ReleaseAssetState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReleaseAssetState::Open => "open",
            ReleaseAssetState::Uploaded => "uploaded",
            ReleaseAssetState::Noop => "",
            ReleaseAssetState::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for ReleaseAssetState {
    fn default() -> ReleaseAssetState {
        ReleaseAssetState::Noop
    }
}
impl ReleaseAssetState {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReleaseAssetState::Noop)
    }
}

/// Data related to a release.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReleaseAsset {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub browser_download_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub download_count: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    /**
     * State of the release asset.
     */
    #[serde(default, skip_serializing_if = "ReleaseAssetState::is_noop")]
    pub state: ReleaseAssetState,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub uploader: UserAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// A release.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Release {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assets: Vec<ReleaseAsset>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assets_url: String,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub discussion_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub draft: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * A release.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub mentions_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub prerelease: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * A release.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<ReactionRollup>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tag_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tarball_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target_commitish: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub upload_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zipball_url: String,
}

/**
 * Sets the state of the secret scanning alert. Can be either `open` or `resolved`. You must provide `resolution` when you set the state to `resolved`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum SecretScanningAlertState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for SecretScanningAlertState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SecretScanningAlertState::Open => "open",
            SecretScanningAlertState::Resolved => "resolved",
            SecretScanningAlertState::Noop => "",
            SecretScanningAlertState::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for SecretScanningAlertState {
    fn default() -> SecretScanningAlertState {
        SecretScanningAlertState::Noop
    }
}
impl SecretScanningAlertState {
    pub fn is_noop(&self) -> bool {
        matches!(self, SecretScanningAlertState::Noop)
    }
}

/**
 * **Required when the `state` is `resolved`.** The reason for resolving the alert. Can be one of `false_positive`, `wont_fix`, `revoked`, or `used_in_tests`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum SecretScanningAlertResolution {
    #[serde(rename = "false_positive")]
    FalsePositive,
    #[serde(rename = "revoked")]
    Revoked,
    #[serde(rename = "used_in_tests")]
    UsedInTests,
    #[serde(rename = "wont_fix")]
    WontFix,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for SecretScanningAlertResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SecretScanningAlertResolution::FalsePositive => "false_positive",
            SecretScanningAlertResolution::Revoked => "revoked",
            SecretScanningAlertResolution::UsedInTests => "used_in_tests",
            SecretScanningAlertResolution::WontFix => "wont_fix",
            SecretScanningAlertResolution::Noop => "",
            SecretScanningAlertResolution::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for SecretScanningAlertResolution {
    fn default() -> SecretScanningAlertResolution {
        SecretScanningAlertResolution::Noop
    }
}
impl SecretScanningAlertResolution {
    pub fn is_noop(&self) -> bool {
        matches!(self, SecretScanningAlertResolution::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SecretScanningAlert {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub number: i64,
    /**
     * \*\*Required when the `state` is `resolved`.\*\* The reason for resolving the alert. Can be one of `false_positive`, `wont_fix`, `revoked`, or `used_in_tests`.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<SecretScanningAlertResolution>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Simple User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_by: Option<SimpleUser>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret_type: String,
    /**
     * Sets the state of the secret scanning alert. Can be either `open` or `resolved`. You must provide `resolution` when you set the state to `resolved`.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<SecretScanningAlertState>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Stargazer
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Stargazer {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub starred_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub user: UserAllOf,
}

/// Commit Activity
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommitActivity {
    /**
     * Code Frequency Stat
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub days: Vec<i64>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub week: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Weeks {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub a: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub c: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub d: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub w: i64,
}

/// Contributor Activity
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ContributorActivity {
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub author: UserAllOf,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weeks: Vec<Weeks>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ParticipationStats {
    /**
     * Code Frequency Stat
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub all: Vec<i64>,
    /**
     * Code Frequency Stat
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owner: Vec<i64>,
}

/// Repository invitations let you manage who you collaborate with.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RepositorySubscription {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub ignored: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub subscribed: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Tag
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Tag {
    #[serde()]
    pub commit: Tree,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tarball_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zipball_url: String,
}

/// A topic aggregates entities that are related to a subject.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Topic {
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Traffic {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub count: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub uniques: i64,
}

/// Clone Traffic
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CloneTraffic {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub clones: Vec<Traffic>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub count: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub uniques: i64,
}

/// Content Traffic
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ContentTraffic {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub uniques: i64,
}

/// Referrer Traffic
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReferrerTraffic {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub referrer: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub uniques: i64,
}

/// View Traffic
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ViewTraffic {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub count: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub uniques: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub views: Vec<Traffic>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Members {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "$ref"
    )]
    pub ref_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Meta {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_modified: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub resource_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimEnterpriseGroup {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<Members>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimGroupListEnterprise {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ScimEnterpriseGroup>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub items_per_page: f64,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub start_index: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_results: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Name {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub family_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub given_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Emails {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub primary: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimUserListEnterpriseResourcesGroups {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimEnterpriseUser {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub active: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<Emails>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<ScimUserListEnterpriseResourcesGroups>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimUserListEnterprise {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ScimEnterpriseUser>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub items_per_page: f64,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub start_index: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_results: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimUserName {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub family_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub formatted: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub given_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimUserEmails {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub primary: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimUserMeta {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub resource_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Op {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Op::Add => "add",
            Op::Remove => "remove",
            Op::Replace => "replace",
            Op::Noop => "",
            Op::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Op {
    fn default() -> Op {
        Op::Noop
    }
}
impl Op {
    pub fn is_noop(&self) -> bool {
        matches!(self, Op::Noop)
    }
}

/// One of the following types:
///
/// - `String`
/// - `Data`
/// - `Vec<String>`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ScimUserOperationsValueOneOf {
    Data(Data),
    String(String),
    /**
     * The list of events for the GitHub app
     */
    StringVector(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Operations {
    #[serde(default, skip_serializing_if = "Op::is_noop")]
    pub op: Op,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    /**
     * One of the following types:
     *  
     *  - `String`
     *  - `Data`
     *  - `Vec<String>`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<ScimUserOperationsValueOneOf>,
}

/// SCIM /Users provisioning endpoints
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimUser {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub active: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    /**
     * user emails
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<ScimUserEmails>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_id: String,
    /**
     * SCIM /Users provisioning endpoints
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde()]
    pub meta: ScimUserMeta,
    #[serde()]
    pub name: ScimUserName,
    /**
     * SCIM /Users provisioning endpoints
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<Operations>,
    /**
     * SCIM /Users provisioning endpoints
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub organization_id: i64,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_name: String,
}

/// SCIM User List
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimUserList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ScimUser>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub items_per_page: i64,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub start_index: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_results: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Matches {
    /**
     * Code Frequency Stat
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indices: Vec<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SearchResultTextMatches {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fragment: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub matches: Vec<Matches>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub object_type: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub object_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub property: String,
}

/// Code Search Result Item
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeSearchResultItem {
    /**
     * Code Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub file_size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    /**
     * Code Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Code Search Result Item
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub line_numbers: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    /**
     * Minimal Repository
     */
    #[serde()]
    pub repository: MinimalRepository,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub score: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    /**
     * Code Search Result Item
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub text_matches: Vec<SearchResultTextMatches>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// All of the following types:
///
/// - `Tagger`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum CommitterAllOf {
    Tagger(Tagger),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommitSearchResultItem {
    /**
     * Identifying information for the git-user
     */
    #[serde()]
    pub author: Committer,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub comment_count: i64,
    /**
     * All of the following types:
     *  
     *  - `Tagger`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub committer: CommitterAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde()]
    pub tree: Tree,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<Verification>,
}

/// Commit Search Result Item
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommitSearchResultItemData {
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub author: UserAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde()]
    pub commit: CommitSearchResultItem,
    /**
     * All of the following types:
     *  
     *  - `Tagger`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub committer: CommitterAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parents: Vec<Parents>,
    /**
     * Minimal Repository
     */
    #[serde()]
    pub repository: MinimalRepository,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub score: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    /**
     * Commit Search Result Item
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub text_matches: Vec<SearchResultTextMatches>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Issue Search Result Item
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssueSearchResultItem {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub active_lock_reason: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub assignee: UserAllOf,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<SimpleUser>,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub comments: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Issue Search Result Item
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub draft: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<LabelsData>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub locked: bool,
    /**
     * All of the following types:
     *  
     *  - `Milestone`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub milestone: MilestoneAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub number: i64,
    /**
     * Issue Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<AppAllOf>,
    /**
     * Issue Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
    /**
     * Issue Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub score: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * Issue Search Result Item
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub text_matches: Vec<SearchResultTextMatches>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timeline_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub user: UserAllOf,
}

/// Label Search Result Item
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct LabelSearchResultItem {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub color: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub default: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub score: f64,
    /**
     * Label Search Result Item
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub text_matches: Vec<SearchResultTextMatches>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Repo Search Result Item
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RepoSearchResultItem {
    /**
     * Repo Search Result Item
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_auto_merge: bool,
    /**
     * Repo Search Result Item
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_merge_commit: bool,
    /**
     * Repo Search Result Item
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_rebase_merge: bool,
    /**
     * Repo Search Result Item
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allow_squash_merge: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub archived: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignees_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blobs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branches_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub clone_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub collaborators_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub compare_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contributors_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_branch: String,
    /**
     * Repo Search Result Item
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub delete_branch_on_merge: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub disabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub downloads_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub fork: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forks: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forks_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_refs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_downloads: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_issues: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_pages: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_projects: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_wiki: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub keys_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub languages_url: String,
    /**
     * All of the following types:
     *  
     *  - `LicenseSimple`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub license: LicenseAllOf,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub master_branch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merges_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub milestones_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mirror_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notifications_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub open_issues: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub open_issues_count: i64,
    /**
     * All of the following types:
     *  
     *  - `SimpleUser`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub owner: UserAllOf,
    /**
     * Repo Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<FullRepositoryPermissions>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub private: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pulls_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub pushed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub releases_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub score: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ssh_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub stargazers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stargazers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscribers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svn_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub temp_clone_token: String,
    /**
     * Repo Search Result Item
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub text_matches: Vec<SearchResultTextMatches>,
    /**
     * Repo Search Result Item
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trees_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub watchers: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub watchers_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TopicRelation {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub relation_type: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub topic_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Related {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_relation: Option<TopicRelation>,
}

/// Topic Search Result Item
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TopicSearchResultItem {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<Related>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_by: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub curated: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub featured: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub logo_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<Related>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub released: String,
    /**
     * Topic Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub repository_count: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub score: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub short_description: String,
    /**
     * Topic Search Result Item
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub text_matches: Vec<SearchResultTextMatches>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// User Search Result Item
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct UserSearchResultItem {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub bio: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blog: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    /**
     * User Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    /**
     * User Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub followers: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    /**
     * User Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub following: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    /**
     * User Search Result Item
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub hireable: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organizations_url: String,
    /**
     * User Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub public_gists: i64,
    /**
     * User Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub public_repos: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub received_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub score: f64,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub site_admin: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscriptions_url: String,
    /**
     * User Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub suspended_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * User Search Result Item
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub text_matches: Vec<SearchResultTextMatches>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    /**
     * User Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Private User
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PrivateUser {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub bio: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blog: String,
    /**
     * Private User
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub business_plus: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub collaborators: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub disk_usage: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub followers: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub following: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub hireable: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ldap_dn: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organizations_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub owned_private_repos: i64,
    /**
     * Private User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub private_gists: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub public_gists: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub public_repos: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub received_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub site_admin: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscriptions_url: String,
    /**
     * Private User
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub suspended_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_private_repos: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub twitter_username: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub two_factor_authentication: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Email
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Email {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub primary: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub verified: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GpgKeyEmails {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub verified: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Subkeys {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub can_certify: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub can_encrypt_comms: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub can_encrypt_storage: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub can_sign: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub expires_at: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub primary_key_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub raw_key: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subkeys: Vec<String>,
}

/// A unique encryption key
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GpgKey {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub can_certify: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub can_encrypt_comms: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub can_encrypt_storage: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub can_sign: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<GpgKeyEmails>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub primary_key_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub raw_key: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subkeys: Vec<Subkeys>,
}

/// Key
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Key {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub read_only: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub verified: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MarketplaceAccount {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organization_billing_email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// User Marketplace Purchase
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct UserMarketplacePurchase {
    #[serde()]
    pub account: MarketplaceAccount,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub billing_cycle: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub free_trial_ends_on: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub next_billing_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub on_free_trial: bool,
    /**
     * Marketplace Listing Plan
     */
    #[serde()]
    pub plan: MarketplaceListingPlan,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unit_count: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Starred Repository
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct StarredRepository {
    /**
     * A git repository
     */
    #[serde()]
    pub repo: Repository,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub starred_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Contexts {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub octicon: String,
}

/// Hovercard
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Hovercard {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contexts: Vec<Contexts>,
}

/// Key Simple
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct KeySimple {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
}

/**
 * The event types to include:
 *   
 *   - `web` - returns web (non-Git) events
 *   - `git` - returns Git events
 *   - `all` - returns both web and Git events
 *   
 *   The default is `web`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Include {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "git")]
    Git,
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Include {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Include::All => "all",
            Include::Git => "git",
            Include::Web => "web",
            Include::Noop => "",
            Include::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Include {
    fn default() -> Include {
        Include::Noop
    }
}
impl Include {
    pub fn is_noop(&self) -> bool {
        matches!(self, Include::Noop)
    }
}

/**
 * The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
 *   
 *   The default is `desc`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Order {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Order::Asc => "asc",
            Order::Desc => "desc",
            Order::Noop => "",
            Order::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Order {
    fn default() -> Order {
        Order::Noop
    }
}
impl Order {
    pub fn is_noop(&self) -> bool {
        matches!(self, Order::Noop)
    }
}

/**
 * One of `created` (when the repository was starred) or `updated` (when it was last pushed to).
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Sort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
    FallthroughString(String),
}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Sort::Created => "created",
            Sort::Updated => "updated",
            Sort::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Sort {
    fn default() -> Sort {
        Sort::Created
    }
}

/**
 * Returns workflow runs with the check run `status` or `conclusion` that you specify. For example, a conclusion can be `success` or a status can be `in_progress`. Only GitHub can set a status of `waiting` or `requested`. For a list of the possible `status` and `conclusion` options, see "[Create a check run](https://docs.github.com/rest/reference/checks#create-a-check-run)."
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum WorkflowRunStatus {
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "skipped")]
    Skipped,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "timed_out")]
    TimedOut,
    #[serde(rename = "waiting")]
    Waiting,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for WorkflowRunStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            WorkflowRunStatus::ActionRequired => "action_required",
            WorkflowRunStatus::Cancelled => "cancelled",
            WorkflowRunStatus::Completed => "completed",
            WorkflowRunStatus::Failure => "failure",
            WorkflowRunStatus::InProgress => "in_progress",
            WorkflowRunStatus::Neutral => "neutral",
            WorkflowRunStatus::Queued => "queued",
            WorkflowRunStatus::Requested => "requested",
            WorkflowRunStatus::Skipped => "skipped",
            WorkflowRunStatus::Stale => "stale",
            WorkflowRunStatus::Success => "success",
            WorkflowRunStatus::TimedOut => "timed_out",
            WorkflowRunStatus::Waiting => "waiting",
            WorkflowRunStatus::Noop => "",
            WorkflowRunStatus::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for WorkflowRunStatus {
    fn default() -> WorkflowRunStatus {
        WorkflowRunStatus::Noop
    }
}
impl WorkflowRunStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, WorkflowRunStatus::Noop)
    }
}

/// One of the following types:
///
/// - `String`
/// - `i64`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum TitleOneOf {
    String(String),
    I64(i64),
}

/**
 * Must be one of: `day`, `week`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Per {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    FallthroughString(String),
}

impl std::fmt::Display for Per {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Per::Day => "day",
            Per::Week => "week",
            Per::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Per {
    fn default() -> Per {
        Per::Day
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsMergeResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ServiceUnavailableResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Block {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ForbiddenGistResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block: Option<Block>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MetaRootResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub authorizations_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code_search_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_search_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub current_user_authorizations_html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub current_user_repositories_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub current_user_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub emails_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub emojis_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub feeds_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hub_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_search_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub keys_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label_search_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notifications_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organization_repositories_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organization_teams_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organization_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub rate_limit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_search_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub topic_search_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_organizations_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_repositories_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_search_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AppsCreateFromManifestResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pem: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub webhook_secret: String,
}

/// All of the following types:
///
/// - `GitHubApp`
/// - `AppsCreateFromManifestResponse`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum AppsCreateFromManifestResponseAllOf {
    AppsCreateFromManifestResponse(AppsCreateFromManifestResponse),
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    GitHubApp(GitHubApp),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AppsUpdateWebhookConfigAppRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSslOneOf>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AppsCreateInstallationAccessTokenRequest {
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<AppPermissions>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<String>,
    /**
     * Code Frequency Stat
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repository_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AppsCheckTokenRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AppsScopeTokenRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_token: String,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<AppPermissions>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<String>,
    /**
     * Code Frequency Stat
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repository_ids: Vec<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub target_id: i64,
}

/// All of the following types:
///
/// - `Authorization`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum AppsCheckAuthorizationResponseAllOf {
    /**
     * The authorization for an OAuth app, GitHub App, or a Personal Access Token.
     */
    Authorization(Authorization),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OauthAuthorizationsCreateAuthorizationRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fingerprint: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note_url: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OauthAuthorizationsGetCreateAuthorizationAppRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fingerprint: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note_url: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OauthAuthorizationsGetCreateAuthorizationAppFingerprintRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note_url: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OauthAuthorizationsUpdateAuthorizationRequest {
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_scopes: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fingerprint: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note_url: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove_scopes: Vec<String>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminSetGithubActionsPermissionsRequest {
    /**
     * The permissions policy that controls the actions that are allowed to run. Can be one of: `all`, `local_only`, or `selected`.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,
    /**
     * The policy that controls the repositories in the organization that are allowed to run GitHub Actions. Can be one of: `all`, `none`, or `selected`.
     */
    #[serde()]
    pub enabled_organizations: EnabledRepositories,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminListOrgAccessSelfHostedRunnerGroupInResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub organizations: Vec<OrganizationSimple>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_count: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminSetOrgAccessSelfHostedRunnerGroupInRequest {
    /**
     * Code Frequency Stat
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selected_organization_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminListSelfHostedRunnerGroupsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runner_groups: Vec<RunnerGroupsEnterprise>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_count: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminCreateSelfHostedRunnerGroupRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Code Frequency Stat
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runners: Vec<i64>,
    /**
     * Code Frequency Stat
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selected_organization_ids: Vec<i64>,
    /**
     * Describe whether all repositories have been selected or there's a selection involved
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<RepositorySelection>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminUpdateSelfHostedRunnerGroupRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Describe whether all repositories have been selected or there's a selection involved
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<RepositorySelection>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsListSelfHostedRunnersInGroupOrgResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runners: Vec<Runner>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_count: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsSetSelfHostedRunnersInGroupOrgRequest {
    /**
     * Code Frequency Stat
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runners: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminListSelfHostedRunnersResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runners: Vec<Runner>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_count: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Public {
    #[serde(rename = "false")]
    False,
    #[serde(rename = "true")]
    True,
    FallthroughString(String),
}

impl std::fmt::Display for Public {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Public::False => "false",
            Public::True => "true",
            Public::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Public {
    fn default() -> Public {
        Public::False
    }
}

/// One of the following types:
///
/// - `bool`
/// - `Public`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum PublicOneOf {
    Public(Public),
    Bool(bool),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GistsCreateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde()]
    pub files: Data,
    /**
     * One of the following types:
     *  
     *  - `bool`
     *  - `Public`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<PublicOneOf>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GistsUpdateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Data>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsUpdateReviewRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AppsListInstallationReposResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<Repository>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_selection: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
}

/**
 * Indicates which sorts of issues to return. Can be one of:  
 *   \* `assigned`: Issues assigned to you  
 *   \* `created`: Issues created by you  
 *   \* `mentioned`: Issues mentioning you  
 *   \* `subscribed`: Issues you're subscribed to updates for  
 *   \* `all` or `repos`: All issues the authenticated user can see, regardless of participation or creation
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Filter {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "assigned")]
    Assigned,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "mentioned")]
    Mentioned,
    #[serde(rename = "repos")]
    Repos,
    #[serde(rename = "subscribed")]
    Subscribed,
    FallthroughString(String),
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Filter::All => "all",
            Filter::Assigned => "assigned",
            Filter::Created => "created",
            Filter::Mentioned => "mentioned",
            Filter::Repos => "repos",
            Filter::Subscribed => "subscribed",
            Filter::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Filter {
    fn default() -> Filter {
        Filter::Assigned
    }
}

/**
 * Indicates the state of the issues to return. Can be either `open`, `closed`, or `all`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum IssuesListState {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "open")]
    Open,
    FallthroughString(String),
}

impl std::fmt::Display for IssuesListState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            IssuesListState::All => "all",
            IssuesListState::Closed => "closed",
            IssuesListState::Open => "open",
            IssuesListState::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for IssuesListState {
    fn default() -> IssuesListState {
        IssuesListState::Open
    }
}

/**
 * What to sort results by. Can be either `created`, `updated`, `comments`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum IssuesListSort {
    #[serde(rename = "comments")]
    Comments,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
    FallthroughString(String),
}

impl std::fmt::Display for IssuesListSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            IssuesListSort::Comments => "comments",
            IssuesListSort::Created => "created",
            IssuesListSort::Updated => "updated",
            IssuesListSort::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for IssuesListSort {
    fn default() -> IssuesListSort {
        IssuesListSort::Created
    }
}

/**
 * The rendering mode.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Mode {
    #[serde(rename = "gfm")]
    Gfm,
    #[serde(rename = "markdown")]
    Markdown,
    FallthroughString(String),
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Mode::Gfm => "gfm",
            Mode::Markdown => "markdown",
            Mode::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::Markdown
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MarkdownRenderRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub context: String,
    /**
     * The rendering mode.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActivityMarkNotificationsAsReadRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_read_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActivitySetThreadSubscriptionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignored: Option<bool>,
}

/**
 * Specifies which types of repositories non-admin organization members can create. Can be one of:  
 *   \* `all` - all organization members can create public and private repositories.  
 *   \* `private` - members can create private repositories. This option is only available to repositories that are part of an organization on GitHub Enterprise Cloud.  
 *   \* `none` - only admin members can create repositories.  
 *   **Note:** This parameter is deprecated and will be removed in the future. Its return value ignores internal repositories. Using this parameter overrides values set in `members_can_create_repositories`. See the parameter deprecation notice in the operation description for details.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum MembersAllowedRepositoryCreationType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for MembersAllowedRepositoryCreationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            MembersAllowedRepositoryCreationType::All => "all",
            MembersAllowedRepositoryCreationType::None => "none",
            MembersAllowedRepositoryCreationType::Private => "private",
            MembersAllowedRepositoryCreationType::Noop => "",
            MembersAllowedRepositoryCreationType::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for MembersAllowedRepositoryCreationType {
    fn default() -> MembersAllowedRepositoryCreationType {
        MembersAllowedRepositoryCreationType::Noop
    }
}
impl MembersAllowedRepositoryCreationType {
    pub fn is_noop(&self) -> bool {
        matches!(self, MembersAllowedRepositoryCreationType::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgsUpdateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub billing_email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blog: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    /**
     * The baseline permission that all organization members have on this project. Only present if owner is an organization.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_repository_permission: Option<OrganizationPermission>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_organization_projects: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_repository_projects: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    /**
     * Specifies which types of repositories non-admin organization members can create. Can be one of:  
     *  \\* `all` - all organization members can create public and private repositories.  
     *  \\* `private` - members can create private repositories. This option is only available to repositories that are part of an organization on GitHub Enterprise Cloud.  
     *  \\* `none` - only admin members can create repositories.  
     *  \*\*Note:\*\* This parameter is deprecated and will be removed in the future. Its return value ignores internal repositories. Using this parameter overrides values set in `members_can_create_repositories`. See the parameter deprecation notice in the operation description for details.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_allowed_repository_creation_type: Option<MembersAllowedRepositoryCreationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_internal_repositories: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_private_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_private_repositories: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_public_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_public_repositories: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_repositories: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub twitter_username: String,
}

/// One of the following types:
///
/// - `ValidationError`
/// - `ValidationErrorSimple`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum OrgsUpdateResponseOneOf {
    /**
     * Validation Error
     */
    ValidationError(ValidationError),
    /**
     * Validation Error Simple
     */
    ValidationErrorSimple(ValidationErrorSimple),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsSetGithubPermissionsOrganizationRequest {
    /**
     * The permissions policy that controls the actions that are allowed to run. Can be one of: `all`, `local_only`, or `selected`.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,
    /**
     * The policy that controls the repositories in the organization that are allowed to run GitHub Actions. Can be one of: `all`, `none`, or `selected`.
     */
    #[serde()]
    pub enabled_repositories: EnabledRepositories,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsListSelectedRepositoriesEnabledGithubOrganizationResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<Repository>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_count: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsSetRepoAccessSelfHostedRunnerGroupInOrgRequest {
    /**
     * Code Frequency Stat
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selected_repository_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsListSelfHostedRunnerGroupsOrgResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runner_groups: Vec<RunnerGroupsOrg>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_count: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsCreateSelfHostedRunnerGroupOrgRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Code Frequency Stat
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runners: Vec<i64>,
    /**
     * Code Frequency Stat
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selected_repository_ids: Vec<i64>,
    /**
     * Visibility of a secret
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsUpdateSelfHostedRunnerGroupOrgRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Visibility of a secret
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsListRepoAccessSelfHostedRunnerGroupInOrgResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<MinimalRepository>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_count: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsListSelfHostedRunnersOrgResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runners: Vec<Runner>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsListOrgSecretsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<OrganizationActionsSecret>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsCreateUpdateOrgSecretRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub encrypted_value: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key_id: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selected_repository_ids: Vec<String>,
    /**
     * Visibility of a secret
     */
    #[serde(default, skip_serializing_if = "Visibility::is_noop")]
    pub visibility: Visibility,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsListSelectedReposOrgSecretResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<MinimalRepository>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsSetSelectedReposOrgSecretRequest {
    /**
     * Code Frequency Stat
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selected_repository_ids: Vec<i64>,
}

/// Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#create-hook-config-params).
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgsCreateWebhookRequestConfig {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#create-hook-config-params).
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSslOneOf>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub password: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgsCreateWebhookRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#create-hook-config-params).
     */
    #[serde()]
    pub config: OrgsCreateWebhookRequestConfig,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#update-hook-config-params).
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgsUpdateWebhookRequestConfig {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#update-hook-config-params).
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSslOneOf>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgsUpdateWebhookRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#update-hook-config-params).
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<OrgsUpdateWebhookRequestConfig>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AppsListInstallationsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub installations: Vec<Installation>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
}

/// Any of the following types:
///
/// - `InteractionLimits`
/// - `Data`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum InteractionsGetRestrictionsResponseAnyOf {
    Data(Data),
    /**
     * Interaction limit settings.
     */
    InteractionLimits(InteractionLimits),
}

/**
 * Specify role for new member. Can be one of:  
 *   \* `admin` - Organization owners with full administrative rights to the organization and complete access to all repositories and teams.  
 *   \* `direct_member` - Non-owner organization members with ability to see other members and join teams by invitation.  
 *   \* `billing_manager` - Non-owner organization members with ability to manage the billing settings of your organization.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum OrgsCreateInvitationRequestRole {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "billing_manager")]
    BillingManager,
    #[serde(rename = "direct_member")]
    DirectMember,
    FallthroughString(String),
}

impl std::fmt::Display for OrgsCreateInvitationRequestRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            OrgsCreateInvitationRequestRole::Admin => "admin",
            OrgsCreateInvitationRequestRole::BillingManager => "billing_manager",
            OrgsCreateInvitationRequestRole::DirectMember => "direct_member",
            OrgsCreateInvitationRequestRole::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for OrgsCreateInvitationRequestRole {
    fn default() -> OrgsCreateInvitationRequestRole {
        OrgsCreateInvitationRequestRole::DirectMember
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgsCreateInvitationRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub invitee_id: i64,
    /**
     * Specify role for new member. Can be one of:  
     *  \\* `admin` - Organization owners with full administrative rights to the organization and complete access to all repositories and teams.  
     *  \\* `direct_member` - Non-owner organization members with ability to see other members and join teams by invitation.  
     *  \\* `billing_manager` - Non-owner organization members with ability to manage the billing settings of your organization.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<OrgsCreateInvitationRequestRole>,
    /**
     * Code Frequency Stat
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub team_ids: Vec<i64>,
}

/**
 * Filter members returned in the list. Can be one of:  
 *   \* `2fa_disabled` - Members without [two-factor authentication](https://github.com/blog/1614-two-factor-authentication) enabled. Available for organization owners.  
 *   \* `all` - All members the authenticated user can see.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum OrgsListMembersFilter {
    #[serde(rename = "2fa_disabled")]
    TwoFaDisabled,
    #[serde(rename = "all")]
    All,
    FallthroughString(String),
}

impl std::fmt::Display for OrgsListMembersFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            OrgsListMembersFilter::TwoFaDisabled => "2fa_disabled",
            OrgsListMembersFilter::All => "all",
            OrgsListMembersFilter::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for OrgsListMembersFilter {
    fn default() -> OrgsListMembersFilter {
        OrgsListMembersFilter::All
    }
}

/**
 * Filter members returned by their role. Can be one of:  
 *   \* `all` - All members of the organization, regardless of role.  
 *   \* `admin` - Organization owners.  
 *   \* `member` - Non-owner organization members.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum OrgsListMembersRole {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "member")]
    Member,
    FallthroughString(String),
}

impl std::fmt::Display for OrgsListMembersRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            OrgsListMembersRole::Admin => "admin",
            OrgsListMembersRole::All => "all",
            OrgsListMembersRole::Member => "member",
            OrgsListMembersRole::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for OrgsListMembersRole {
    fn default() -> OrgsListMembersRole {
        OrgsListMembersRole::All
    }
}

/**
 * The role to give the user in the organization. Can be one of:  
 *   \* `admin` - The user will become an owner of the organization.  
 *   \* `member` - The user will become a non-owner member of the organization.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum OrgsSetMembershipUserRequestRole {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "member")]
    Member,
    FallthroughString(String),
}

impl std::fmt::Display for OrgsSetMembershipUserRequestRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            OrgsSetMembershipUserRequestRole::Admin => "admin",
            OrgsSetMembershipUserRequestRole::Member => "member",
            OrgsSetMembershipUserRequestRole::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for OrgsSetMembershipUserRequestRole {
    fn default() -> OrgsSetMembershipUserRequestRole {
        OrgsSetMembershipUserRequestRole::Member
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgsSetMembershipUserRequest {
    /**
     * The role to give the user in the organization. Can be one of:  
     *  \\* `admin` - The user will become an owner of the organization.  
     *  \\* `member` - The user will become a non-owner member of the organization.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<OrgsSetMembershipUserRequestRole>,
}

/**
 * Allowed values that can be passed to the exclude param.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Exclude {
    #[serde(rename = "repositories")]
    Repositories,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Exclude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Exclude::Repositories => "repositories",
            Exclude::Noop => "",
            Exclude::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Exclude {
    fn default() -> Exclude {
        Exclude::Noop
    }
}
impl Exclude {
    pub fn is_noop(&self) -> bool {
        matches!(self, Exclude::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MigrationsStartRequest {
    /**
     * Exclude attributes from the API response to improve performance
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<Exclude>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_attachments: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_repositories: Option<bool>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<String>,
}

/**
 * The state of the package, either active or deleted.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum PackagesGetAllPackageVersionsOwnedByOrgState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "deleted")]
    Deleted,
    FallthroughString(String),
}

impl std::fmt::Display for PackagesGetAllPackageVersionsOwnedByOrgState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PackagesGetAllPackageVersionsOwnedByOrgState::Active => "active",
            PackagesGetAllPackageVersionsOwnedByOrgState::Deleted => "deleted",
            PackagesGetAllPackageVersionsOwnedByOrgState::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for PackagesGetAllPackageVersionsOwnedByOrgState {
    fn default() -> PackagesGetAllPackageVersionsOwnedByOrgState {
        PackagesGetAllPackageVersionsOwnedByOrgState::Active
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsCreateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/**
 * Specifies the types of repositories you want returned. Can be one of `all`, `public`, `private`, `forks`, `sources`, `member`, `internal`. Note: For GitHub AE, can be one of `all`, `private`, `forks`, `sources`, `member`, `internal`. Default: `all`. If your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+, `type` can also be `internal`. However, the `internal` value is not yet supported when a GitHub App calls this API with an installation access token.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ReposListOrgType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "forks")]
    Forks,
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "sources")]
    Sources,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for ReposListOrgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReposListOrgType::All => "all",
            ReposListOrgType::Forks => "forks",
            ReposListOrgType::Internal => "internal",
            ReposListOrgType::Member => "member",
            ReposListOrgType::Private => "private",
            ReposListOrgType::Public => "public",
            ReposListOrgType::Sources => "sources",
            ReposListOrgType::Noop => "",
            ReposListOrgType::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for ReposListOrgType {
    fn default() -> ReposListOrgType {
        ReposListOrgType::Noop
    }
}
impl ReposListOrgType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReposListOrgType::Noop)
    }
}

/**
 * Can be one of `created`, `updated`, `pushed`, `full_name`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ReposListOrgSort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "full_name")]
    FullName,
    #[serde(rename = "pushed")]
    Pushed,
    #[serde(rename = "updated")]
    Updated,
    FallthroughString(String),
}

impl std::fmt::Display for ReposListOrgSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReposListOrgSort::Created => "created",
            ReposListOrgSort::FullName => "full_name",
            ReposListOrgSort::Pushed => "pushed",
            ReposListOrgSort::Updated => "updated",
            ReposListOrgSort::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for ReposListOrgSort {
    fn default() -> ReposListOrgSort {
        ReposListOrgSort::Created
    }
}

/**
 * Can be `public` or `private`. If your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+, `visibility` can also be `internal`. Note: For GitHub Enterprise Server and GitHub AE, this endpoint will only list repositories available to all users on the enterprise. For more information, see "[Creating an internal repository](https://help.github.com/en/github/creating-cloning-and-archiving-repositories/about-repository-visibility#about-internal-repositories)" in the GitHub Help documentation.  
 *   The `visibility` parameter overrides the `private` parameter when you use both parameters with the `nebula-preview` preview header.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ReposCreateInOrgRequestVisibility {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "visibility")]
    Visibility,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for ReposCreateInOrgRequestVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReposCreateInOrgRequestVisibility::Internal => "internal",
            ReposCreateInOrgRequestVisibility::Private => "private",
            ReposCreateInOrgRequestVisibility::Public => "public",
            ReposCreateInOrgRequestVisibility::Visibility => "visibility",
            ReposCreateInOrgRequestVisibility::Noop => "",
            ReposCreateInOrgRequestVisibility::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for ReposCreateInOrgRequestVisibility {
    fn default() -> ReposCreateInOrgRequestVisibility {
        ReposCreateInOrgRequestVisibility::Noop
    }
}
impl ReposCreateInOrgRequestVisibility {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReposCreateInOrgRequestVisibility::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateInOrgRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_auto_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_init: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gitignore_template: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_issues: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_projects: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_wiki: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub license_template: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub team_id: i64,
    /**
     * Can be `public` or `private`. If your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+, `visibility` can also be `internal`. Note: For GitHub Enterprise Server and GitHub AE, this endpoint will only list repositories available to all users on the enterprise. For more information, see "[Creating an internal repository](https://help.github.com/en/github/creating-cloning-and-archiving-repositories/about-repository-visibility#about-internal-repositories)" in the GitHub Help documentation.  
     *  The `visibility` parameter overrides the `private` parameter when you use both parameters with the `nebula-preview` preview header.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<ReposCreateInOrgRequestVisibility>,
}

/**
 * **Deprecated**. The permission that new repositories will be added to the team with when none is specified. Can be one of:  
 *   \* `pull` - team members can pull, but not push to or administer newly-added repositories.  
 *   \* `push` - team members can pull and push, but not administer newly-added repositories.  
 *   \* `admin` - team members can pull, push and administer newly-added repositories.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Permission {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "pull")]
    Pull,
    #[serde(rename = "push")]
    Push,
    FallthroughString(String),
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Permission::Admin => "admin",
            Permission::Pull => "pull",
            Permission::Push => "push",
            Permission::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Permission {
    fn default() -> Permission {
        Permission::Pull
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsCreateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub maintainers: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub parent_team_id: i64,
    /**
     * \*\*Deprecated\*\*. The permission that new repositories will be added to the team with when none is specified. Can be one of:  
     *  \\* `pull` - team members can pull, but not push to or administer newly-added repositories.  
     *  \\* `push` - team members can pull and push, but not administer newly-added repositories.  
     *  \\* `admin` - team members can pull, push and administer newly-added repositories.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<Permission>,
    /**
     * The level of privacy this team should have
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<Privacy>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repo_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsUpdateInOrgRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub parent_team_id: i64,
    /**
     * \*\*Deprecated\*\*. The permission that new repositories will be added to the team with when none is specified. Can be one of:  
     *  \\* `pull` - team members can pull, but not push to or administer newly-added repositories.  
     *  \\* `push` - team members can pull and push, but not administer newly-added repositories.  
     *  \\* `admin` - team members can pull, push and administer newly-added repositories.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<Permission>,
    /**
     * The level of privacy this team should have
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<Privacy>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsCreateDiscussionInOrgRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsUpdateDiscussionInOrgRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReactionsCreateIssueRequest {
    /**
     * The reaction to use
     */
    #[serde(default, skip_serializing_if = "Content::is_noop")]
    pub content: Content,
}

/**
 * Filters members returned by their role in the team. Can be one of:  
 *   \* `member` - normal members of the team.  
 *   \* `maintainer` - team maintainers.  
 *   \* `all` - all members of the team.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum TeamsListMembersInOrgRole {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "maintainer")]
    Maintainer,
    #[serde(rename = "member")]
    Member,
    FallthroughString(String),
}

impl std::fmt::Display for TeamsListMembersInOrgRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            TeamsListMembersInOrgRole::All => "all",
            TeamsListMembersInOrgRole::Maintainer => "maintainer",
            TeamsListMembersInOrgRole::Member => "member",
            TeamsListMembersInOrgRole::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for TeamsListMembersInOrgRole {
    fn default() -> TeamsListMembersInOrgRole {
        TeamsListMembersInOrgRole::All
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsAddUpdateMembershipUserInOrgRequest {
    /**
     * The role of the user in the team.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<TeamMembershipRole>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsAddCollaboratorRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<RepositoryProjects>,
}

/**
 * The permission to grant the team on this repository. Can be one of:  
 *   \* `pull` - team members can pull, but not push to or administer this repository.  
 *   \* `push` - team members can pull and push, but not administer this repository.  
 *   \* `admin` - team members can pull, push and administer this repository.  
 *   \* `maintain` - team members can manage the repository without access to sensitive or destructive actions. Recommended for project managers. Only applies to repositories owned by organizations.  
 *   \* `triage` - team members can proactively manage issues and pull requests without write access. Recommended for contributors who triage a repository. Only applies to repositories owned by organizations.  
 *     
 *   If no permission is specified, the team's `permission` attribute will be used to determine what permission to grant the team on this repository.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum TeamsAddUpdateRepoPermissionsInOrgRequestPermission {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "maintain")]
    Maintain,
    #[serde(rename = "pull")]
    Pull,
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "triage")]
    Triage,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for TeamsAddUpdateRepoPermissionsInOrgRequestPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            TeamsAddUpdateRepoPermissionsInOrgRequestPermission::Admin => "admin",
            TeamsAddUpdateRepoPermissionsInOrgRequestPermission::Maintain => "maintain",
            TeamsAddUpdateRepoPermissionsInOrgRequestPermission::Pull => "pull",
            TeamsAddUpdateRepoPermissionsInOrgRequestPermission::Push => "push",
            TeamsAddUpdateRepoPermissionsInOrgRequestPermission::Triage => "triage",
            TeamsAddUpdateRepoPermissionsInOrgRequestPermission::Noop => "",
            TeamsAddUpdateRepoPermissionsInOrgRequestPermission::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for TeamsAddUpdateRepoPermissionsInOrgRequestPermission {
    fn default() -> TeamsAddUpdateRepoPermissionsInOrgRequestPermission {
        TeamsAddUpdateRepoPermissionsInOrgRequestPermission::Noop
    }
}
impl TeamsAddUpdateRepoPermissionsInOrgRequestPermission {
    pub fn is_noop(&self) -> bool {
        matches!(
            self,
            TeamsAddUpdateRepoPermissionsInOrgRequestPermission::Noop
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsAddUpdateRepoPermissionsInOrgRequest {
    /**
     * The permission to grant the team on this repository. Can be one of:  
     *  \\* `pull` - team members can pull, but not push to or administer this repository.  
     *  \\* `push` - team members can pull and push, but not administer this repository.  
     *  \\* `admin` - team members can pull, push and administer this repository.  
     *  \\* `maintain` - team members can manage the repository without access to sensitive or destructive actions. Recommended for project managers. Only applies to repositories owned by organizations.  
     *  \\* `triage` - team members can proactively manage issues and pull requests without write access. Recommended for contributors who triage a repository. Only applies to repositories owned by organizations.  
     *    
     *  If no permission is specified, the team's `permission` attribute will be used to determine what permission to grant the team on this repository.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<TeamsAddUpdateRepoPermissionsInOrgRequestPermission>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsCreateUpdateIdpGroupConnectionsInOrgRequestGroups {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsCreateUpdateIdpGroupConnectionsInOrgRequest {
    /**
     * The IdP groups you want to connect to a GitHub team. When updating, the new `groups` object will replace the original one. You must include any existing groups that you don't want to remove.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<TeamsCreateUpdateIdpGroupConnectionsInOrgRequestGroups>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsDeleteResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation_url: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsUpdateCardRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsMoveCardRequest {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub column_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub position: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsMoveCardResponseErrors {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub resource: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsMoveCardResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation_url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ProjectsMoveCardResponseErrors>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsMoveCardResponseErrorsData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsCreateCardResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation_url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ProjectsMoveCardResponseErrorsData>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsUpdateColumnRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/**
 * Filters the project cards that are returned by the card's state. Can be one of `all`,`archived`, or `not_archived`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ArchivedState {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "archived")]
    Archived,
    #[serde(rename = "not_archived")]
    NotArchived,
    FallthroughString(String),
}

impl std::fmt::Display for ArchivedState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ArchivedState::All => "all",
            ArchivedState::Archived => "archived",
            ArchivedState::NotArchived => "not_archived",
            ArchivedState::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for ArchivedState {
    fn default() -> ArchivedState {
        ArchivedState::NotArchived
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsCreateCardRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsCreateCardRequestData {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub content_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
}

/// One of the following types:
///
/// - `ProjectsCreateCardRequest`
/// - `ProjectsCreateCardRequestData`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ProjectsCreateCardRequestOneOf {
    ProjectsCreateCardRequest(ProjectsCreateCardRequest),
    ProjectsCreateCardRequestData(ProjectsCreateCardRequestData),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsMoveColumnRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub position: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsUpdateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The baseline permission that all organization members have on this project. Only present if owner is an organization.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_permission: Option<OrganizationPermission>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
}

/**
 * Filters the collaborators by their affiliation. Can be one of:  
 *   \* `outside`: Outside collaborators of a project that are not a member of the project's organization.  
 *   \* `direct`: Collaborators with permissions to a project, regardless of organization membership status.  
 *   \* `all`: All collaborators the authenticated user can see.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Affiliation {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "outside")]
    Outside,
    FallthroughString(String),
}

impl std::fmt::Display for Affiliation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Affiliation::All => "all",
            Affiliation::Direct => "direct",
            Affiliation::Outside => "outside",
            Affiliation::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Affiliation {
    fn default() -> Affiliation {
        Affiliation::All
    }
}

/// Use the `status` property to enable or disable GitHub Advanced Security for this repository. For more information, see "[About GitHub Advanced Security](/github/getting-started-with-github/learning-about-github/about-github-advanced-security)."
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateRequestSecurityAnalysisAdvanced {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
}

/// Specify which security and analysis features to enable or disable. For example, to enable GitHub Advanced Security, use this data in the body of the PATCH request: `{"security_and_analysis": {"advanced_security": {"status": "enabled"}}}`. If you have admin permissions for a private repository covered by an Advanced Security license, you can check which security and analysis features are currently enabled by using a `GET /repos/{owner}/{repo}` request.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateRequestSecurityAnalysis {
    /**
     * Specify which security and analysis features to enable or disable. For example, to enable GitHub Advanced Security, use this data in the body of the PATCH request: `{"security_and_analysis": {"advanced_security": {"status": "enabled"}}}`. If you have admin permissions for a private repository covered by an Advanced Security license, you can check which security and analysis features are currently enabled by using a `GET /repos/{owner}/{repo}` request.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub advanced_security: Option<ReposUpdateRequestSecurityAnalysisAdvanced>,
    /**
     * Specify which security and analysis features to enable or disable. For example, to enable GitHub Advanced Security, use this data in the body of the PATCH request: `{"security_and_analysis": {"advanced_security": {"status": "enabled"}}}`. If you have admin permissions for a private repository covered by an Advanced Security license, you can check which security and analysis features are currently enabled by using a `GET /repos/{owner}/{repo}` request.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_scanning: Option<ReposUpdateRequestSecurityAnalysisAdvanced>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_auto_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_branch: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_issues: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_projects: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_wiki: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    /**
     * Specify which security and analysis features to enable or disable. For example, to enable GitHub Advanced Security, use this data in the body of the PATCH request: `{"security_and_analysis": {"advanced_security": {"status": "enabled"}}}`. If you have admin permissions for a private repository covered by an Advanced Security license, you can check which security and analysis features are currently enabled by using a `GET /repos/{owner}/{repo}` request.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_and_analysis: Option<ReposUpdateRequestSecurityAnalysis>,
    /**
     * Can be `public` or `private`. If your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+, `visibility` can also be `internal`. Note: For GitHub Enterprise Server and GitHub AE, this endpoint will only list repositories available to all users on the enterprise. For more information, see "[Creating an internal repository](https://help.github.com/en/github/creating-cloning-and-archiving-repositories/about-repository-visibility#about-internal-repositories)" in the GitHub Help documentation.  
     *  The `visibility` parameter overrides the `private` parameter when you use both parameters with the `nebula-preview` preview header.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<ReposCreateInOrgRequestVisibility>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsListArtifactsRepoResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<Artifact>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsSetGithubPermissionsRepositoryRequest {
    /**
     * The permissions policy that controls the actions that are allowed to run. Can be one of: `all`, `local_only`, or `selected`.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsListWorkflowRunsResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub workflow_runs: Vec<WorkflowRun>,
}

/**
 * Filters jobs by their `completed_at` timestamp. Can be one of:  
 *   \* `latest`: Returns jobs from the most recent execution of the workflow run.  
 *   \* `all`: Returns all jobs for a workflow run, including from old executions of the workflow run.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ActionsListJobsWorkflowRunFilter {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "latest")]
    Latest,
    FallthroughString(String),
}

impl std::fmt::Display for ActionsListJobsWorkflowRunFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ActionsListJobsWorkflowRunFilter::All => "all",
            ActionsListJobsWorkflowRunFilter::Latest => "latest",
            ActionsListJobsWorkflowRunFilter::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for ActionsListJobsWorkflowRunFilter {
    fn default() -> ActionsListJobsWorkflowRunFilter {
        ActionsListJobsWorkflowRunFilter::Latest
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsListJobsWorkflowRunResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jobs: Vec<Job>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsReviewPendingDeploymentsRunRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comment: String,
    /**
     * Code Frequency Stat
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub environment_ids: Vec<i64>,
    /**
     * Whether deployment to the environment(s) was approved or rejected
     */
    #[serde(default, skip_serializing_if = "EnvironmentApprovalState::is_noop")]
    pub state: EnvironmentApprovalState,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsListRepoSecretsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<ActionsSecret>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsCreateUpdateRepoSecretRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub encrypted_value: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsListRepoWorkflowsResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub workflows: Vec<Workflow>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsCreateWorkflowDispatchRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Data>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateAutolinkRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key_prefix: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url_template: String,
}

/// Require status checks to pass before merging. Set to `null` to disable.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateBranchProtectionRequestRequiredStatusChecks {
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contexts: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub strict: bool,
}

/// Specify which users and teams can dismiss pull request reviews. Pass an empty `dismissal_restrictions` object to disable. User and team `dismissal_restrictions` are only available for organization-owned repositories. Omit this parameter for personal repositories.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateBranchProtectionRequestRequiredPullReviewsDismissalRestrictions {
    /**
     * Specify which users and teams can dismiss pull request reviews. Pass an empty `dismissal_restrictions` object to disable. User and team `dismissal_restrictions` are only available for organization-owned repositories. Omit this parameter for personal repositories.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub teams: Vec<String>,
    /**
     * Specify which users and teams can dismiss pull request reviews. Pass an empty `dismissal_restrictions` object to disable. User and team `dismissal_restrictions` are only available for organization-owned repositories. Omit this parameter for personal repositories.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<String>,
}

/// Require at least one approving review on a pull request, before merging. Set to `null` to disable.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateBranchProtectionRequestRequiredPullReviews {
    /**
     * Require at least one approving review on a pull request, before merging. Set to `null` to disable.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub dismiss_stale_reviews: bool,
    /**
     * Require at least one approving review on a pull request, before merging. Set to `null` to disable.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissal_restrictions:
        Option<ReposUpdateBranchProtectionRequestRequiredPullReviewsDismissalRestrictions>,
    /**
     * Require at least one approving review on a pull request, before merging. Set to `null` to disable.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub require_code_owner_reviews: bool,
    /**
     * Require at least one approving review on a pull request, before merging. Set to `null` to disable.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub required_approving_review_count: i64,
}

/// Restrict who can push to the protected branch. User, app, and team `restrictions` are only available for organization-owned repositories. Set to `null` to disable.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Restrictions {
    /**
     * Restrict who can push to the protected branch. User, app, and team `restrictions` are only available for organization-owned repositories. Set to `null` to disable.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub apps: Vec<String>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub teams: Vec<String>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateBranchProtectionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_deletions: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_force_pushes: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforce_admins: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_conversation_resolution: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_linear_history: Option<bool>,
    /**
     * Require at least one approving review on a pull request, before merging. Set to `null` to disable.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_pull_request_reviews:
        Option<ReposUpdateBranchProtectionRequestRequiredPullReviews>,
    /**
     * Require status checks to pass before merging. Set to `null` to disable.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_status_checks: Option<ReposUpdateBranchProtectionRequestRequiredStatusChecks>,
    /**
     * Restrict who can push to the protected branch. User, app, and team `restrictions` are only available for organization-owned repositories. Set to `null` to disable.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restrictions>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdatePullRequestReviewProtection {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub dismiss_stale_reviews: bool,
    /**
     * Specify which users and teams can dismiss pull request reviews. Pass an empty `dismissal_restrictions` object to disable. User and team `dismissal_restrictions` are only available for organization-owned repositories. Omit this parameter for personal repositories.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissal_restrictions:
        Option<ReposUpdateBranchProtectionRequestRequiredPullReviewsDismissalRestrictions>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub require_code_owner_reviews: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub required_approving_review_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateStatusCheckProtectionRequest {
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contexts: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposAddStatusCheckContextsRequest {
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contexts: Vec<String>,
}

/// One of the following types:
///
/// - `ReposAddStatusCheckContextsRequest`
/// - `Vec<String>`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ReposAddStatusCheckContextsRequestOneOf {
    ReposAddStatusCheckContextsRequest(ReposAddStatusCheckContextsRequest),
    /**
     * The list of events for the GitHub app
     */
    StringVector(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposAddAppAccessRestrictionsRequest {
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub apps: Vec<String>,
}

/// One of the following types:
///
/// - `ReposAddAppAccessRestrictionsRequest`
/// - `Vec<String>`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ReposAddAppAccessRestrictionsRequestOneOf {
    ReposAddAppAccessRestrictionsRequest(ReposAddAppAccessRestrictionsRequest),
    /**
     * The list of events for the GitHub app
     */
    StringVector(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposAddTeamAccessRestrictionsRequest {
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub teams: Vec<String>,
}

/// One of the following types:
///
/// - `ReposAddTeamAccessRestrictionsRequest`
/// - `Vec<String>`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ReposAddTeamAccessRestrictionsRequestOneOf {
    ReposAddTeamAccessRestrictionsRequest(ReposAddTeamAccessRestrictionsRequest),
    /**
     * The list of events for the GitHub app
     */
    StringVector(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposAddUserAccessRestrictionsRequest {
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<String>,
}

/// One of the following types:
///
/// - `ReposAddUserAccessRestrictionsRequest`
/// - `Vec<String>`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ReposAddUserAccessRestrictionsRequestOneOf {
    ReposAddUserAccessRestrictionsRequest(ReposAddUserAccessRestrictionsRequest),
    /**
     * The list of events for the GitHub app
     */
    StringVector(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposRenameBranchRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub new_name: String,
}

/**
 * **Required if you provide `completed_at` or a `status` of `completed`**. The final conclusion of the check. Can be one of `action_required`, `cancelled`, `failure`, `neutral`, `success`, `skipped`, `stale`, or `timed_out`. When the conclusion is `action_required`, additional details should be provided on the site specified by `details_url`.  
 *   **Note:** Providing `conclusion` will automatically set the `status` parameter to `completed`. You cannot change a check run conclusion to `stale`, only GitHub can set this.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ChecksCreateRequestConclusion {
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "skipped")]
    Skipped,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "timed_out")]
    TimedOut,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for ChecksCreateRequestConclusion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ChecksCreateRequestConclusion::ActionRequired => "action_required",
            ChecksCreateRequestConclusion::Cancelled => "cancelled",
            ChecksCreateRequestConclusion::Failure => "failure",
            ChecksCreateRequestConclusion::Neutral => "neutral",
            ChecksCreateRequestConclusion::Skipped => "skipped",
            ChecksCreateRequestConclusion::Stale => "stale",
            ChecksCreateRequestConclusion::Success => "success",
            ChecksCreateRequestConclusion::TimedOut => "timed_out",
            ChecksCreateRequestConclusion::Noop => "",
            ChecksCreateRequestConclusion::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for ChecksCreateRequestConclusion {
    fn default() -> ChecksCreateRequestConclusion {
        ChecksCreateRequestConclusion::Noop
    }
}
impl ChecksCreateRequestConclusion {
    pub fn is_noop(&self) -> bool {
        matches!(self, ChecksCreateRequestConclusion::Noop)
    }
}

/**
 * The level of the annotation. Can be one of `notice`, `warning`, or `failure`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum AnnotationLevel {
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for AnnotationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            AnnotationLevel::Failure => "failure",
            AnnotationLevel::Notice => "notice",
            AnnotationLevel::Warning => "warning",
            AnnotationLevel::Noop => "",
            AnnotationLevel::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for AnnotationLevel {
    fn default() -> AnnotationLevel {
        AnnotationLevel::Noop
    }
}
impl AnnotationLevel {
    pub fn is_noop(&self) -> bool {
        matches!(self, AnnotationLevel::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Annotations {
    /**
     * The level of the annotation. Can be one of `notice`, `warning`, or `failure`.
     */
    #[serde(default, skip_serializing_if = "AnnotationLevel::is_noop")]
    pub annotation_level: AnnotationLevel,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub end_column: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub end_line: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub raw_details: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub start_column: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub start_line: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Images {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub alt: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub caption: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_url: String,
}

/// Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object) description.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ChecksCreateRequestOutput {
    /**
     * Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object) description.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<Annotations>,
    /**
     * Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object) description.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<Images>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub summary: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub text: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ChecksCreateRequestActions {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub identifier: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ChecksCreateRequest {
    /**
     * Displays a button on GitHub that can be clicked to alert your app to do additional tasks. For example, a code linting app can display a button that automatically fixes detected errors. The button created in this object is displayed after the check run completes. When a user clicks the button, GitHub sends the [`check_run.requested_action` webhook](https://docs.github.com/webhooks/event-payloads/#check_run) to your app. Each action includes a `label`, `identifier` and `description`. A maximum of three actions are accepted. See the [`actions` object](https://docs.github.com/rest/reference/checks#actions-object) description. To learn more about check runs and requested actions, see "[Check runs and requested actions](https://docs.github.com/rest/reference/checks#check-runs-and-requested-actions)." To learn more about check runs and requested actions, see "[Check runs and requested actions](https://docs.github.com/rest/reference/checks#check-runs-and-requested-actions)."
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<ChecksCreateRequestActions>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * \*\*Required if you provide `completed_at` or a `status` of `completed`\*\*. The final conclusion of the check. Can be one of `action_required`, `cancelled`, `failure`, `neutral`, `success`, `skipped`, `stale`, or `timed_out`. When the conclusion is `action_required`, additional details should be provided on the site specified by `details_url`.  
     *  \*\*Note:\*\* Providing `conclusion` will automatically set the `status` parameter to `completed`. You cannot change a check run conclusion to `stale`, only GitHub can set this.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<ChecksCreateRequestConclusion>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub details_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head_sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object) description.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<ChecksCreateRequestOutput>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The phase of the lifecycle that the job is currently in.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,
}

/// Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object-1) description.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ChecksUpdateRequestOutput {
    /**
     * Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object-1) description.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<Annotations>,
    /**
     * Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object-1) description.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<Images>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub summary: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub text: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ChecksUpdateRequest {
    /**
     * Displays a button on GitHub that can be clicked to alert your app to do additional tasks. For example, a code linting app can display a button that automatically fixes detected errors. The button created in this object is displayed after the check run completes. When a user clicks the button, GitHub sends the [`check_run.requested_action` webhook](https://docs.github.com/webhooks/event-payloads/#check_run) to your app. Each action includes a `label`, `identifier` and `description`. A maximum of three actions are accepted. See the [`actions` object](https://docs.github.com/rest/reference/checks#actions-object) description. To learn more about check runs and requested actions, see "[Check runs and requested actions](https://docs.github.com/rest/reference/checks#check-runs-and-requested-actions)." To learn more about check runs and requested actions, see "[Check runs and requested actions](https://docs.github.com/rest/reference/checks#check-runs-and-requested-actions)."
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<ChecksCreateRequestActions>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * \*\*Required if you provide `completed_at` or a `status` of `completed`\*\*. The final conclusion of the check. Can be one of `action_required`, `cancelled`, `failure`, `neutral`, `success`, `skipped`, `stale`, or `timed_out`. When the conclusion is `action_required`, additional details should be provided on the site specified by `details_url`.  
     *  \*\*Note:\*\* Providing `conclusion` will automatically set the `status` parameter to `completed`. You cannot change a check run conclusion to `stale`, only GitHub can set this.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<ChecksCreateRequestConclusion>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub details_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object-1) description.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<ChecksUpdateRequestOutput>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The phase of the lifecycle that the job is currently in.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ChecksCreateSuiteRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head_sha: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ChecksListRefResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub check_runs: Vec<CheckRun>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningUpdateAlertRequest {
    /**
     * \*\*Required when the state is dismissed.\*\* The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissed_reason: Option<CodeScanningAlertDismissedReason>,
    /**
     * Sets the state of the code scanning alert. Can be one of `open` or `dismissed`. You must provide `dismissed_reason` when you set the state to `dismissed`.
     */
    #[serde()]
    pub state: CodeScanningAlertSetState,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningUploadSarifRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub checkout_uri: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sarif: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tool_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposAddCollaboratorRequest {
    /**
     * The permission to grant the team on this repository. Can be one of:  
     *  \\* `pull` - team members can pull, but not push to or administer this repository.  
     *  \\* `push` - team members can pull and push, but not administer this repository.  
     *  \\* `admin` - team members can pull, push and administer this repository.  
     *  \\* `maintain` - team members can manage the repository without access to sensitive or destructive actions. Recommended for project managers. Only applies to repositories owned by organizations.  
     *  \\* `triage` - team members can proactively manage issues and pull requests without write access. Recommended for contributors who triage a repository. Only applies to repositories owned by organizations.  
     *    
     *  If no permission is specified, the team's `permission` attribute will be used to determine what permission to grant the team on this repository.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<TeamsAddUpdateRepoPermissionsInOrgRequestPermission>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub permissions: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateCommitCommentRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub line: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub position: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ChecksListSuitesRefResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub check_suites: Vec<CheckSuiteData>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
}

/// One of the following types:
///
/// - `Vec<Entries>`
/// - `ContentFile`
/// - `SymlinkContent`
/// - `ContentSubmodule`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ReposGetContentResponseOneOf {
    /**
     * Content File
     */
    ContentFile(ContentFile),
    /**
     * An object describing a symlink
     */
    ContentSubmodule(ContentSubmodule),
    /**
     * An object describing a symlink
     */
    SymlinkContent(SymlinkContent),
    EntriesVector(Vec<Entries>),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateUpdateFileContentsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<Tagger>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branch: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committer: Option<Tagger>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposDeleteFileRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<Author>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branch: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committer: Option<Author>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateDeploymentRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub environment: String,
    /**
     * One of the following types:
     *  
     *  - `Data`
     *  - `String`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<PayloadOneOf>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub production_environment: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_contexts: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub task: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transient_environment: Option<bool>,
}

/**
 * Name for the target deployment environment, which can be changed when setting a deploy status. For example, `production`, `staging`, or `qa`. **Note:** This parameter requires you to use the [`application/vnd.github.flash-preview+json`](https://docs.github.com/rest/overview/api-previews#deployment-statuses) custom media type.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ReposCreateDeploymentStatusRequestEnvironment {
    #[serde(rename = "production")]
    Production,
    #[serde(rename = "qa")]
    Qa,
    #[serde(rename = "staging")]
    Staging,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for ReposCreateDeploymentStatusRequestEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReposCreateDeploymentStatusRequestEnvironment::Production => "production",
            ReposCreateDeploymentStatusRequestEnvironment::Qa => "qa",
            ReposCreateDeploymentStatusRequestEnvironment::Staging => "staging",
            ReposCreateDeploymentStatusRequestEnvironment::Noop => "",
            ReposCreateDeploymentStatusRequestEnvironment::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for ReposCreateDeploymentStatusRequestEnvironment {
    fn default() -> ReposCreateDeploymentStatusRequestEnvironment {
        ReposCreateDeploymentStatusRequestEnvironment::Noop
    }
}
impl ReposCreateDeploymentStatusRequestEnvironment {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReposCreateDeploymentStatusRequestEnvironment::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateDeploymentStatusRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_inactive: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Name for the target deployment environment, which can be changed when setting a deploy status. For example, `production`, `staging`, or `qa`. \*\*Note:\*\* This parameter requires you to use the [`application/vnd.github.flash-preview+json`](https://docs.github.com/rest/overview/api-previews#deployment-statuses) custom media type.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<ReposCreateDeploymentStatusRequestEnvironment>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub environment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub log_url: String,
    /**
     * The state of the status.
     */
    #[serde(default, skip_serializing_if = "DeploymentStatusState::is_noop")]
    pub state: DeploymentStatusState,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateDispatchEventRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_payload: Option<Data>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposGetAllEnvironmentsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub environments: Vec<EnvironmentData>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateUpdateEnvironmentRequestReviewers {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The type of reviewer. Must be one of: `User` or `Team`
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<DeploymentReviewerType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateUpdateEnvironmentRequest {
    /**
     * The type of deployment branch policy for this environment. To allow all branches to deploy, set to `null`.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_branch_policy: Option<DeploymentBranchPolicy>,
    /**
     * The people or teams that may review jobs that reference the environment. You can list up to six users or teams as reviewers. The reviewers must have at least read access to the repository. Only one of the required reviewers needs to approve the job for it to proceed.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewers: Vec<ReposCreateUpdateEnvironmentRequestReviewers>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub wait_timer: i64,
}

/**
 * The sort order. Can be either `newest`, `oldest`, or `stargazers`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ReposListForksSort {
    #[serde(rename = "newest")]
    Newest,
    #[serde(rename = "oldest")]
    Oldest,
    #[serde(rename = "stargazers")]
    Stargazers,
    #[serde(rename = "watchers")]
    Watchers,
    FallthroughString(String),
}

impl std::fmt::Display for ReposListForksSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReposListForksSort::Newest => "newest",
            ReposListForksSort::Oldest => "oldest",
            ReposListForksSort::Stargazers => "stargazers",
            ReposListForksSort::Watchers => "watchers",
            ReposListForksSort::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for ReposListForksSort {
    fn default() -> ReposListForksSort {
        ReposListForksSort::Newest
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateForkRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organization: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCreateBlobRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub encoding: String,
}

/// Information about the author of the commit. By default, the `author` will be the authenticated user and the current date. See the `author` and `committer` object below for details.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCreateCommitRequestAuthor {
    /**
     * Information about the author of the commit. By default, the `author` will be the authenticated user and the current date. See the `author` and `committer` object below for details.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// Information about the person who is making the commit. By default, `committer` will use the information set in `author`. See the `author` and `committer` object below for details.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCreateCommitRequestCommitter {
    /**
     * Information about the person who is making the commit. By default, `committer` will use the information set in `author`. See the `author` and `committer` object below for details.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCreateCommitRequest {
    /**
     * Information about the author of the commit. By default, the `author` will be the authenticated user and the current date. See the `author` and `committer` object below for details.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<GitCreateCommitRequestAuthor>,
    /**
     * Information about the person who is making the commit. By default, `committer` will use the information set in `author`. See the `author` and `committer` object below for details.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committer: Option<GitCreateCommitRequestCommitter>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parents: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub signature: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tree: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCreateRefRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitUpdateRefRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
}

/**
 * The type of the object we're tagging. Normally this is a `commit` but it can also be a `tree` or a `blob`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum GitCreateTagRequestType {
    #[serde(rename = "blob")]
    Blob,
    #[serde(rename = "commit")]
    Commit,
    #[serde(rename = "tree")]
    Tree,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for GitCreateTagRequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GitCreateTagRequestType::Blob => "blob",
            GitCreateTagRequestType::Commit => "commit",
            GitCreateTagRequestType::Tree => "tree",
            GitCreateTagRequestType::Noop => "",
            GitCreateTagRequestType::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for GitCreateTagRequestType {
    fn default() -> GitCreateTagRequestType {
        GitCreateTagRequestType::Noop
    }
}
impl GitCreateTagRequestType {
    pub fn is_noop(&self) -> bool {
        matches!(self, GitCreateTagRequestType::Noop)
    }
}

/// An object with information about the individual creating the tag.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCreateTagRequestTagger {
    /**
     * An object with information about the individual creating the tag.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCreateTagRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub object: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tag: String,
    /**
     * An object with information about the individual creating the tag.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tagger: Option<GitCreateTagRequestTagger>,
    /**
     * The type of the object we're tagging. Normally this is a `commit` but it can also be a `tree` or a `blob`.
     */
    #[serde(
        default,
        skip_serializing_if = "GitCreateTagRequestType::is_noop",
        rename = "type"
    )]
    pub type_: GitCreateTagRequestType,
}

/**
 * The file mode; one of `100644` for file (blob), `100755` for executable (blob), `040000` for subdirectory (tree), `160000` for submodule (commit), or `120000` for a blob that specifies the path of a symlink.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum GitCreateTreeRequestMode {
    #[serde(rename = "040000")]
    SubdirectoryTree,
    #[serde(rename = "100644")]
    FileBlob,
    #[serde(rename = "100755")]
    ExecutableBlob,
    #[serde(rename = "120000")]
    SymlinkPathBlob,
    #[serde(rename = "160000")]
    SubmoduleCommit,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for GitCreateTreeRequestMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GitCreateTreeRequestMode::SubdirectoryTree => "040000",
            GitCreateTreeRequestMode::FileBlob => "100644",
            GitCreateTreeRequestMode::ExecutableBlob => "100755",
            GitCreateTreeRequestMode::SymlinkPathBlob => "120000",
            GitCreateTreeRequestMode::SubmoduleCommit => "160000",
            GitCreateTreeRequestMode::Noop => "",
            GitCreateTreeRequestMode::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for GitCreateTreeRequestMode {
    fn default() -> GitCreateTreeRequestMode {
        GitCreateTreeRequestMode::Noop
    }
}
impl GitCreateTreeRequestMode {
    pub fn is_noop(&self) -> bool {
        matches!(self, GitCreateTreeRequestMode::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCreateTreeRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content: String,
    /**
     * The file mode; one of `100644` for file (blob), `100755` for executable (blob), `040000` for subdirectory (tree), `160000` for submodule (commit), or `120000` for a blob that specifies the path of a symlink.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<GitCreateTreeRequestMode>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    /**
     * The type of the object we're tagging. Normally this is a `commit` but it can also be a `tree` or a `blob`.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<GitCreateTagRequestType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCreateTreeRequestData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub base_tree: String,
    /**
     * Objects (of `path`, `mode`, `type`, and `sha`) specifying a tree structure.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tree: Vec<GitCreateTreeRequest>,
}

/// Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateWebhookRequestConfig {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub digest: String,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSslOneOf>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateWebhookRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<ReposCreateWebhookRequestConfig>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateWebhookRequestConfig {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSslOneOf>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub room: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateWebhookRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_events: Vec<String>,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<ReposUpdateWebhookRequestConfig>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove_events: Vec<String>,
}

/**
 * The originating VCS type. Can be one of `subversion`, `git`, `mercurial`, or `tfvc`. Please be aware that without this parameter, the import job will take additional time to detect the VCS type before beginning the import. This detection step will be reflected in the response.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Vcs {
    #[serde(rename = "git")]
    Git,
    #[serde(rename = "mercurial")]
    Mercurial,
    #[serde(rename = "subversion")]
    Subversion,
    #[serde(rename = "tfvc")]
    Tfvc,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Vcs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Vcs::Git => "git",
            Vcs::Mercurial => "mercurial",
            Vcs::Subversion => "subversion",
            Vcs::Tfvc => "tfvc",
            Vcs::Noop => "",
            Vcs::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Vcs {
    fn default() -> Vcs {
        Vcs::Noop
    }
}
impl Vcs {
    pub fn is_noop(&self) -> bool {
        matches!(self, Vcs::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MigrationsStartImportRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tfvc_project: String,
    /**
     * The originating VCS type. Can be one of `subversion`, `git`, `mercurial`, or `tfvc`. Please be aware that without this parameter, the import job will take additional time to detect the VCS type before beginning the import. This detection step will be reflected in the response.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vcs: Option<Vcs>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vcs_password: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vcs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vcs_username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MigrationsUpdateImportRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tfvc_project: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vcs: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vcs_password: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vcs_username: String,
}

/**
 * Can be one of `opt_in` (large files will be stored using Git LFS) or `opt_out` (large files will be removed during the import).
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum UseLfs {
    #[serde(rename = "opt_in")]
    OptIn,
    #[serde(rename = "opt_out")]
    OptOut,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for UseLfs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            UseLfs::OptIn => "opt_in",
            UseLfs::OptOut => "opt_out",
            UseLfs::Noop => "",
            UseLfs::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for UseLfs {
    fn default() -> UseLfs {
        UseLfs::Noop
    }
}
impl UseLfs {
    pub fn is_noop(&self) -> bool {
        matches!(self, UseLfs::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MigrationsSetLfsPreferenceRequest {
    /**
     * Can be one of `opt_in` (large files will be stored using Git LFS) or `opt_out` (large files will be removed during the import).
     */
    #[serde(default, skip_serializing_if = "UseLfs::is_noop")]
    pub use_lfs: UseLfs,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateInvitationRequest {
    /**
     * The permission associated with the invitation.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<RepositoryInvitationPermissions>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct LabelsDataType {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub color: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// One of the following types:
///
/// - `String`
/// - `LabelsDataType`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum IssuesCreateRequestLabelsOneOf {
    LabelsDataType(LabelsDataType),
    String(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesCreateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignee: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * Labels to associate with this issue. _NOTE: Only users with push access can set labels for new issues. Labels are silently dropped otherwise._
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<IssuesCreateRequestLabelsOneOf>,
    /**
     * One of the following types:
     *  
     *  - `String`
     *  - `i64`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestone: Option<TitleOneOf>,
    /**
     * One of the following types:
     *  
     *  - `String`
     *  - `i64`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde()]
    pub title: TitleOneOf,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesUpdateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignee: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * Labels to associate with this issue. _NOTE: Only users with push access can set labels for new issues. Labels are silently dropped otherwise._
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<IssuesCreateRequestLabelsOneOf>,
    /**
     * One of the following types:
     *  
     *  - `String`
     *  - `i64`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestone: Option<TitleOneOf>,
    /**
     * The state of the milestone.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /**
     * One of the following types:
     *  
     *  - `String`
     *  - `i64`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<TitleOneOf>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesAddAssigneesRequest {
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesAddLabelsRequest {
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesSetLabelsRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<ProjectsUpdateColumnRequest>,
}

/// One of the following types:
///
/// - `IssuesAddLabelsRequest`
/// - `Vec<String>`
/// - `IssuesAddLabelsRequestData`
/// - `Vec<ProjectsUpdateColumnRequest>`
/// - `String`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum IssuesAddLabelsRequestOneOf {
    IssuesAddLabelsRequest(IssuesAddLabelsRequest),
    IssuesAddLabelsRequestData(IssuesAddLabelsRequestData),
    String(String),
    ProjectsUpdateColumnRequestVector(Vec<ProjectsUpdateColumnRequest>),
    /**
     * The list of events for the GitHub app
     */
    StringVector(Vec<String>),
}

/// Any of the following types:
///
/// - `IssuesAddLabelsRequest`
/// - `Vec<String>`
/// - `IssuesSetLabelsRequest`
/// - `Vec<ProjectsUpdateColumnRequest>`
/// - `String`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum IssuesSetLabelsRequestAnyOf {
    IssuesAddLabelsRequest(IssuesAddLabelsRequest),
    IssuesSetLabelsRequest(IssuesSetLabelsRequest),
    String(String),
    ProjectsUpdateColumnRequestVector(Vec<ProjectsUpdateColumnRequest>),
    /**
     * The list of events for the GitHub app
     */
    StringVector(Vec<String>),
}

/**
 * The reason for locking the issue or pull request conversation. Lock will fail if you don't use one of these reasons:  
 *   \* `off-topic`  
 *   \* `too heated`  
 *   \* `resolved`  
 *   \* `spam`
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum LockReason {
    #[serde(rename = "off-topic")]
    OffTopic,
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "spam")]
    Spam,
    #[serde(rename = "too heated")]
    TooHeated,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for LockReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            LockReason::OffTopic => "off-topic",
            LockReason::Resolved => "resolved",
            LockReason::Spam => "spam",
            LockReason::TooHeated => "too heated",
            LockReason::Noop => "",
            LockReason::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for LockReason {
    fn default() -> LockReason {
        LockReason::Noop
    }
}
impl LockReason {
    pub fn is_noop(&self) -> bool {
        matches!(self, LockReason::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesLockRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_reason: Option<LockReason>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateDeployKeyRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesCreateLabelRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub color: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesUpdateLabelRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub color: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub new_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposMergeRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub base: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head: String,
}

/**
 * What to sort results by. Either `due_on` or `completeness`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum IssuesListMilestonesSort {
    #[serde(rename = "completeness")]
    Completeness,
    #[serde(rename = "due_on")]
    DueOn,
    FallthroughString(String),
}

impl std::fmt::Display for IssuesListMilestonesSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            IssuesListMilestonesSort::Completeness => "completeness",
            IssuesListMilestonesSort::DueOn => "due_on",
            IssuesListMilestonesSort::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for IssuesListMilestonesSort {
    fn default() -> IssuesListMilestonesSort {
        IssuesListMilestonesSort::DueOn
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesCreateMilestoneRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub due_on: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The state of the milestone.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActivityMarkRepoNotificationsAsReadRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_read_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsUpdateBranchResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The repository directory that includes the source files for the Pages site. Allowed paths are `/` or `/docs`. Default: `/`
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Path {
    #[serde(rename = "/")]
    Root,
    #[serde(rename = "/docs")]
    Docs,
    FallthroughString(String),
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Path::Root => "/",
            Path::Docs => "/docs",
            Path::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Path {
    fn default() -> Path {
        Path::Root
    }
}

/// The source branch and directory used to publish your Pages site.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreatePagesSiteRequestSource {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branch: String,
    /**
     * The source branch and directory used to publish your Pages site.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<Path>,
}

/// The source branch and directory used to publish your Pages site.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreatePagesSiteRequest {
    /**
     * The source branch and directory used to publish your Pages site.
     */
    #[serde()]
    pub source: ReposCreatePagesSiteRequestSource,
}

/**
 * Update the source for the repository. Must include the branch name, and may optionally specify the subdirectory `/docs`. Possible values are `"gh-pages"`, `"master"`, and `"master /docs"`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum SourceData {
    #[serde(rename = "gh-pages")]
    GhPages,
    #[serde(rename = "master")]
    Master,
    #[serde(rename = "master /docs")]
    MasterDocs,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for SourceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SourceData::GhPages => "gh-pages",
            SourceData::Master => "master",
            SourceData::MasterDocs => "master /docs",
            SourceData::Noop => "",
            SourceData::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for SourceData {
    fn default() -> SourceData {
        SourceData::Noop
    }
}
impl SourceData {
    pub fn is_noop(&self) -> bool {
        matches!(self, SourceData::Noop)
    }
}

/// Update the source for the repository. Must include the branch name and path.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SourceDataType {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branch: String,
    /**
     * The repository directory that includes the source files for the Pages site. Allowed paths are `/` or `/docs`. Default: `/`
     */
    #[serde(default)]
    pub path: Path,
}

/// Any of the following types:
///
/// - `SourceData`
/// - `SourceDataType`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum SourceAnyOf {
    /**
     * Update the source for the repository. Must include the branch name, and may optionally specify the subdirectory `/docs`. Possible values are `"gh-pages"`, `"master"`, and `"master /docs"`.
     */
    SourceData(SourceData),
    /**
     * Update the source for the repository. Must include the branch name and path.
     */
    SourceDataType(SourceDataType),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateInformationAboutPagesSiteRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub cname: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub https_enforced: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    /**
     * Any of the following types:
     *  
     *  - `SourceData`
     *  - `SourceDataType`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceAnyOf>,
}

/**
 * What to sort results by. Can be either `created`, `updated`, `popularity` (comment count) or `long-running` (age, filtering by pulls updated in the last month).
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum PullsListSort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "long-running")]
    LongRunning,
    #[serde(rename = "popularity")]
    Popularity,
    #[serde(rename = "updated")]
    Updated,
    FallthroughString(String),
}

impl std::fmt::Display for PullsListSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PullsListSort::Created => "created",
            PullsListSort::LongRunning => "long-running",
            PullsListSort::Popularity => "popularity",
            PullsListSort::Updated => "updated",
            PullsListSort::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for PullsListSort {
    fn default() -> PullsListSort {
        PullsListSort::Created
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsCreateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub base: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub issue: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintainer_can_modify: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum PullsListReviewCommentsRepoSort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for PullsListReviewCommentsRepoSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PullsListReviewCommentsRepoSort::Created => "created",
            PullsListReviewCommentsRepoSort::CreatedAt => "created_at",
            PullsListReviewCommentsRepoSort::Updated => "updated",
            PullsListReviewCommentsRepoSort::Noop => "",
            PullsListReviewCommentsRepoSort::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for PullsListReviewCommentsRepoSort {
    fn default() -> PullsListReviewCommentsRepoSort {
        PullsListReviewCommentsRepoSort::Noop
    }
}
impl PullsListReviewCommentsRepoSort {
    pub fn is_noop(&self) -> bool {
        matches!(self, PullsListReviewCommentsRepoSort::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsUpdateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub base: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintainer_can_modify: Option<bool>,
    /**
     * The state of the milestone.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

/**
 * **Required when using multi-line comments**. To create multi-line comments, you must use the `comfort-fade` preview header. The `start_side` is the starting side of the diff that the comment applies to. Can be `LEFT` or `RIGHT`. To learn more about multi-line comments, see "[Commenting on a pull request](https://help.github.com/en/articles/commenting-on-a-pull-request#adding-line-comments-to-a-pull-request)" in the GitHub Help documentation. See `side` in this table for additional context.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum PullsCreateReviewCommentRequestStartSide {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
    #[serde(rename = "side")]
    Side,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for PullsCreateReviewCommentRequestStartSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PullsCreateReviewCommentRequestStartSide::Left => "LEFT",
            PullsCreateReviewCommentRequestStartSide::Right => "RIGHT",
            PullsCreateReviewCommentRequestStartSide::Side => "side",
            PullsCreateReviewCommentRequestStartSide::Noop => "",
            PullsCreateReviewCommentRequestStartSide::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for PullsCreateReviewCommentRequestStartSide {
    fn default() -> PullsCreateReviewCommentRequestStartSide {
        PullsCreateReviewCommentRequestStartSide::Noop
    }
}
impl PullsCreateReviewCommentRequestStartSide {
    pub fn is_noop(&self) -> bool {
        matches!(self, PullsCreateReviewCommentRequestStartSide::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsCreateReviewCommentRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub in_reply_to: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub line: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub position: i64,
    /**
     * The side of the diff to which the comment applies. The side of the last line of the range for a multi-line comment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub start_line: i64,
    /**
     * \*\*Required when using multi-line comments\*\*. To create multi-line comments, you must use the `comfort-fade` preview header. The `start_side` is the starting side of the diff that the comment applies to. Can be `LEFT` or `RIGHT`. To learn more about multi-line comments, see "[Commenting on a pull request](https://help.github.com/en/articles/commenting-on-a-pull-request#adding-line-comments-to-a-pull-request)" in the GitHub Help documentation. See `side` in this table for additional context.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_side: Option<PullsCreateReviewCommentRequestStartSide>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsMergeRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_method: Option<MergeMethod>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsRequestReviewers {
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewers: Vec<String>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub team_reviewers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsRemoveRequestedReviewersRequest {
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewers: Vec<String>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub team_reviewers: Vec<String>,
}

/**
 * The review action you want to perform. The review actions include: `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. By leaving this blank, you set the review action state to `PENDING`, which means you will need to [submit the pull request review](https://docs.github.com/rest/reference/pulls#submit-a-review-for-a-pull-request) when you are ready.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum PullsCreateReviewRequestEvent {
    #[serde(rename = "APPROVE")]
    Approve,
    #[serde(rename = "COMMENT")]
    Comment,
    #[serde(rename = "REQUEST_CHANGES")]
    RequestChanges,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for PullsCreateReviewRequestEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PullsCreateReviewRequestEvent::Approve => "APPROVE",
            PullsCreateReviewRequestEvent::Comment => "COMMENT",
            PullsCreateReviewRequestEvent::RequestChanges => "REQUEST_CHANGES",
            PullsCreateReviewRequestEvent::Noop => "",
            PullsCreateReviewRequestEvent::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for PullsCreateReviewRequestEvent {
    fn default() -> PullsCreateReviewRequestEvent {
        PullsCreateReviewRequestEvent::Noop
    }
}
impl PullsCreateReviewRequestEvent {
    pub fn is_noop(&self) -> bool {
        matches!(self, PullsCreateReviewRequestEvent::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Comments {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub line: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub position: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub side: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub start_line: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub start_side: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsCreateReviewRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * Use the following table to specify the location, destination, and contents of the draft review comment.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<Comments>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    /**
     * The review action you want to perform. The review actions include: `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. By leaving this blank, you set the review action state to `PENDING`, which means you will need to [submit the pull request review](https://docs.github.com/rest/reference/pulls#submit-a-review-for-a-pull-request) when you are ready.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<PullsCreateReviewRequestEvent>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsDismissReviewRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsSubmitReviewRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * The review action you want to perform. The review actions include: `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. By leaving this blank, you set the review action state to `PENDING`, which means you will need to [submit the pull request review](https://docs.github.com/rest/reference/pulls#submit-a-review-for-a-pull-request) when you are ready.
     */
    #[serde(
        default,
        skip_serializing_if = "PullsCreateReviewRequestEvent::is_noop"
    )]
    pub event: PullsCreateReviewRequestEvent,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsUpdateBranchRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub expected_head_sha: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateReleaseRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub discussion_category_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tag_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target_commitish: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateReleaseAssetRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
}

/**
 * The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the release.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ReactionsCreateReleaseRequestContent {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "eyes")]
    Eyes,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for ReactionsCreateReleaseRequestContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReactionsCreateReleaseRequestContent::PlusOne => "+1",
            ReactionsCreateReleaseRequestContent::Eyes => "eyes",
            ReactionsCreateReleaseRequestContent::Heart => "heart",
            ReactionsCreateReleaseRequestContent::Hooray => "hooray",
            ReactionsCreateReleaseRequestContent::Laugh => "laugh",
            ReactionsCreateReleaseRequestContent::Rocket => "rocket",
            ReactionsCreateReleaseRequestContent::Noop => "",
            ReactionsCreateReleaseRequestContent::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for ReactionsCreateReleaseRequestContent {
    fn default() -> ReactionsCreateReleaseRequestContent {
        ReactionsCreateReleaseRequestContent::Noop
    }
}
impl ReactionsCreateReleaseRequestContent {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReactionsCreateReleaseRequestContent::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReactionsCreateReleaseRequest {
    /**
     * The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the release.
     */
    #[serde(
        default,
        skip_serializing_if = "ReactionsCreateReleaseRequestContent::is_noop"
    )]
    pub content: ReactionsCreateReleaseRequestContent,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SecretScanningUpdateAlertRequest {
    /**
     * \*\*Required when the `state` is `resolved`.\*\* The reason for resolving the alert. Can be one of `false_positive`, `wont_fix`, `revoked`, or `used_in_tests`.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<SecretScanningAlertResolution>,
    /**
     * Sets the state of the secret scanning alert. Can be either `open` or `resolved`. You must provide `resolution` when you set the state to `resolved`.
     */
    #[serde()]
    pub state: SecretScanningAlertState,
}

/// Any of the following types:
///
/// - `Vec<SimpleUser>`
/// - `Vec<Stargazer>`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ActivityListStargazersRepoResponseAnyOf {
    SimpleUserVector(Vec<SimpleUser>),
    StargazerVector(Vec<Stargazer>),
}

/**
 * The state of the status. Can be one of `error`, `failure`, `pending`, or `success`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ReposCreateCommitStatusRequestState {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for ReposCreateCommitStatusRequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReposCreateCommitStatusRequestState::Error => "error",
            ReposCreateCommitStatusRequestState::Failure => "failure",
            ReposCreateCommitStatusRequestState::Pending => "pending",
            ReposCreateCommitStatusRequestState::Success => "success",
            ReposCreateCommitStatusRequestState::Noop => "",
            ReposCreateCommitStatusRequestState::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for ReposCreateCommitStatusRequestState {
    fn default() -> ReposCreateCommitStatusRequestState {
        ReposCreateCommitStatusRequestState::Noop
    }
}
impl ReposCreateCommitStatusRequestState {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReposCreateCommitStatusRequestState::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateCommitStatusRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub context: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * The state of the status. Can be one of `error`, `failure`, `pending`, or `success`.
     */
    #[serde(
        default,
        skip_serializing_if = "ReposCreateCommitStatusRequestState::is_noop"
    )]
    pub state: ReposCreateCommitStatusRequestState,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActivitySetRepoSubscriptionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignored: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposTransferRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub new_owner: String,
    /**
     * Code Frequency Stat
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub team_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateUsingTemplateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_all_branches: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub owner: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminProvisionInviteGroupRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<ScimUserListEnterpriseResourcesGroups>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum EnterpriseAdminUpdateAttributeGroupRequestOperationsOp {
    #[serde(rename = "Add")]
    Add,
    #[serde(rename = "Remove")]
    Remove,
    #[serde(rename = "Replace")]
    Replace,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for EnterpriseAdminUpdateAttributeGroupRequestOperationsOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EnterpriseAdminUpdateAttributeGroupRequestOperationsOp::Add => "Add",
            EnterpriseAdminUpdateAttributeGroupRequestOperationsOp::Remove => "Remove",
            EnterpriseAdminUpdateAttributeGroupRequestOperationsOp::Replace => "Replace",
            EnterpriseAdminUpdateAttributeGroupRequestOperationsOp::Noop => "",
            EnterpriseAdminUpdateAttributeGroupRequestOperationsOp::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for EnterpriseAdminUpdateAttributeGroupRequestOperationsOp {
    fn default() -> EnterpriseAdminUpdateAttributeGroupRequestOperationsOp {
        EnterpriseAdminUpdateAttributeGroupRequestOperationsOp::Noop
    }
}
impl EnterpriseAdminUpdateAttributeGroupRequestOperationsOp {
    pub fn is_noop(&self) -> bool {
        matches!(
            self,
            EnterpriseAdminUpdateAttributeGroupRequestOperationsOp::Noop
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminUpdateAttributeGroupRequestOperations {
    #[serde(
        default,
        skip_serializing_if = "EnterpriseAdminUpdateAttributeGroupRequestOperationsOp::is_noop"
    )]
    pub op: EnterpriseAdminUpdateAttributeGroupRequestOperationsOp,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    /**
     * One of the following types:
     *  
     *  - `String`
     *  - `Data`
     *  - `Vec<String>`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<ScimUserOperationsValueOneOf>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminUpdateAttributeGroupRequest {
    /**
     * Array of [SCIM operations](https://tools.ietf.org/html/rfc7644#section-3.5.2).
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<EnterpriseAdminUpdateAttributeGroupRequestOperations>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminProvisionInviteUserRequestEmails {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub primary: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminProvisionInviteUserRequest {
    /**
     * List of user emails.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<EnterpriseAdminProvisionInviteUserRequestEmails>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<ScimUserListEnterpriseResourcesGroups>,
    #[serde()]
    pub name: Name,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminUpdateAttributeUserRequest {
    /**
     * Array of [SCIM operations](https://tools.ietf.org/html/rfc7644#section-3.5.2).
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<Data>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimProvisionInviteUserRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<Emails>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_id: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
    #[serde()]
    pub name: ScimUserName,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Value {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub active: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub family_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub given_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_name: String,
}

/// One of the following types:
///
/// - `Value`
/// - `Vec<ScimUserEmails>`
/// - `String`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ScimUpdateAttributeUserRequestOperationsValueOneOf {
    String(String),
    Value(Value),
    /**
     * user emails
     */
    ScimUserEmailsVector(Vec<ScimUserEmails>),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimUpdateAttributeUserRequestOperations {
    #[serde(default, skip_serializing_if = "Op::is_noop")]
    pub op: Op,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    /**
     * One of the following types:
     *  
     *  - `Value`
     *  - `Vec<ScimUserEmails>`
     *  - `String`
     *  
     *  You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<ScimUpdateAttributeUserRequestOperationsValueOneOf>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimUpdateAttributeUserRequest {
    /**
     * Set of operations to be performed
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<ScimUpdateAttributeUserRequestOperations>,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
}

/**
 * Sorts the results of your query. Can only be `indexed`, which indicates how recently a file has been indexed by the GitHub search infrastructure. Default: [best match](https://docs.github.com/rest/reference/search#ranking-search-results)
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum SearchCodeSort {
    #[serde(rename = "indexed")]
    Indexed,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for SearchCodeSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SearchCodeSort::Indexed => "indexed",
            SearchCodeSort::Noop => "",
            SearchCodeSort::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for SearchCodeSort {
    fn default() -> SearchCodeSort {
        SearchCodeSort::Noop
    }
}
impl SearchCodeSort {
    pub fn is_noop(&self) -> bool {
        matches!(self, SearchCodeSort::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SearchCodeResponse {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub incomplete_results: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CodeSearchResultItem>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
}

/**
 * Sorts the results of your query by `author-date` or `committer-date`. Default: [best match](https://docs.github.com/rest/reference/search#ranking-search-results)
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum SearchCommitsSort {
    #[serde(rename = "author-date")]
    AuthorDate,
    #[serde(rename = "committer-date")]
    CommitterDate,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for SearchCommitsSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SearchCommitsSort::AuthorDate => "author-date",
            SearchCommitsSort::CommitterDate => "committer-date",
            SearchCommitsSort::Noop => "",
            SearchCommitsSort::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for SearchCommitsSort {
    fn default() -> SearchCommitsSort {
        SearchCommitsSort::Noop
    }
}
impl SearchCommitsSort {
    pub fn is_noop(&self) -> bool {
        matches!(self, SearchCommitsSort::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SearchCommitsResponse {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub incomplete_results: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CommitSearchResultItemData>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
}

/**
 * Sorts the results of your query by the number of `comments`, `reactions`, `reactions-+1`, `reactions--1`, `reactions-smile`, `reactions-thinking_face`, `reactions-heart`, `reactions-tada`, or `interactions`. You can also sort results by how recently the items were `created` or `updated`, Default: [best match](https://docs.github.com/rest/reference/search#ranking-search-results)
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum SearchIssuesPullRequestsSort {
    #[serde(rename = "comments")]
    Comments,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "interactions")]
    Interactions,
    #[serde(rename = "reactions")]
    Reactions,
    #[serde(rename = "reactions-+1")]
    ReactionsPlusOne,
    #[serde(rename = "reactions--1")]
    ReactionsMinusOne,
    #[serde(rename = "reactions-heart")]
    ReactionsHeart,
    #[serde(rename = "reactions-smile")]
    ReactionsSmile,
    #[serde(rename = "reactions-tada")]
    ReactionsTada,
    #[serde(rename = "reactions-thinking_face")]
    ReactionsThinkingFace,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for SearchIssuesPullRequestsSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SearchIssuesPullRequestsSort::Comments => "comments",
            SearchIssuesPullRequestsSort::Created => "created",
            SearchIssuesPullRequestsSort::Interactions => "interactions",
            SearchIssuesPullRequestsSort::Reactions => "reactions",
            SearchIssuesPullRequestsSort::ReactionsPlusOne => "reactions-+1",
            SearchIssuesPullRequestsSort::ReactionsMinusOne => "reactions--1",
            SearchIssuesPullRequestsSort::ReactionsHeart => "reactions-heart",
            SearchIssuesPullRequestsSort::ReactionsSmile => "reactions-smile",
            SearchIssuesPullRequestsSort::ReactionsTada => "reactions-tada",
            SearchIssuesPullRequestsSort::ReactionsThinkingFace => "reactions-thinking_face",
            SearchIssuesPullRequestsSort::Updated => "updated",
            SearchIssuesPullRequestsSort::Noop => "",
            SearchIssuesPullRequestsSort::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for SearchIssuesPullRequestsSort {
    fn default() -> SearchIssuesPullRequestsSort {
        SearchIssuesPullRequestsSort::Noop
    }
}
impl SearchIssuesPullRequestsSort {
    pub fn is_noop(&self) -> bool {
        matches!(self, SearchIssuesPullRequestsSort::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SearchIssuesPullRequestsResponse {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub incomplete_results: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<IssueSearchResultItem>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SearchLabelsResponse {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub incomplete_results: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LabelSearchResultItem>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
}

/**
 * Sorts the results of your query by number of `stars`, `forks`, or `help-wanted-issues` or how recently the items were `updated`. Default: [best match](https://docs.github.com/rest/reference/search#ranking-search-results)
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum SearchReposSort {
    #[serde(rename = "forks")]
    Forks,
    #[serde(rename = "help-wanted-issues")]
    HelpWantedIssues,
    #[serde(rename = "stars")]
    Stars,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for SearchReposSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SearchReposSort::Forks => "forks",
            SearchReposSort::HelpWantedIssues => "help-wanted-issues",
            SearchReposSort::Stars => "stars",
            SearchReposSort::Updated => "updated",
            SearchReposSort::Noop => "",
            SearchReposSort::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for SearchReposSort {
    fn default() -> SearchReposSort {
        SearchReposSort::Noop
    }
}
impl SearchReposSort {
    pub fn is_noop(&self) -> bool {
        matches!(self, SearchReposSort::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SearchReposResponse {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub incomplete_results: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RepoSearchResultItem>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SearchTopicsResponse {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub incomplete_results: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TopicSearchResultItem>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
}

/**
 * Sorts the results of your query by number of `followers` or `repositories`, or when the person `joined` GitHub. Default: [best match](https://docs.github.com/rest/reference/search#ranking-search-results)
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum SearchUsersSort {
    #[serde(rename = "followers")]
    Followers,
    #[serde(rename = "joined")]
    Joined,
    #[serde(rename = "repositories")]
    Repositories,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for SearchUsersSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SearchUsersSort::Followers => "followers",
            SearchUsersSort::Joined => "joined",
            SearchUsersSort::Repositories => "repositories",
            SearchUsersSort::Noop => "",
            SearchUsersSort::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for SearchUsersSort {
    fn default() -> SearchUsersSort {
        SearchUsersSort::Noop
    }
}
impl SearchUsersSort {
    pub fn is_noop(&self) -> bool {
        matches!(self, SearchUsersSort::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SearchUsersResponse {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub incomplete_results: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<UserSearchResultItem>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsAddUpdateProjectPermissionsLegacyRequest {
    /**
     * The level of permission to grant the access token to manage repository projects, columns, and cards. Can be one of: `read`, `write`, or `admin`.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<RepositoryProjects>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsAddUpdateRepoPermissionsLegacyRequest {
    /**
     * \*\*Deprecated\*\*. The permission that new repositories will be added to the team with when none is specified. Can be one of:  
     *  \\* `pull` - team members can pull, but not push to or administer newly-added repositories.  
     *  \\* `push` - team members can pull and push, but not administer newly-added repositories.  
     *  \\* `admin` - team members can pull, push and administer newly-added repositories.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<Permission>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsCreateUpdateIdpGroupConnectionsLegacyRequestGroups {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsCreateUpdateIdpGroupConnectionsLegacyRequest {
    /**
     * The IdP groups you want to connect to a GitHub team. When updating, the new `groups` object will replace the original one. You must include any existing groups that you don't want to remove.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<TeamsCreateUpdateIdpGroupConnectionsLegacyRequestGroups>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub synced_at: String,
}

/// One of the following types:
///
/// - `PrivateUser`
/// - `PublicUser`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum UsersGetByUsernameResponseOneOf {
    /**
     * Private User
     */
    PrivateUser(PrivateUser),
    /**
     * Public User
     */
    PublicUser(PublicUser),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct UsersUpdateAuthenticatedRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub bio: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blog: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hireable: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub twitter_username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct UsersSetPrimaryEmailVisibilityAuthenticatedRequest {
    #[serde(default, skip_serializing_if = "PackageVisibility::is_noop")]
    pub visibility: PackageVisibility,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct UsersAddEmailAuthenticatedRequest {
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<String>,
}

/// One of the following types:
///
/// - `UsersAddEmailAuthenticatedRequest`
/// - `Vec<String>`
/// - `String`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum UsersAddEmailAuthenticatedRequestOneOf {
    String(String),
    UsersAddEmailAuthenticatedRequest(UsersAddEmailAuthenticatedRequest),
    /**
     * The list of events for the GitHub app
     */
    StringVector(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct UsersCreateGpgKeyAuthenticatedRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub armored_public_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct UsersCreatePublicSshKeyAuthenticatedRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

/**
 * The state that the membership should be in. Only `"active"` will be accepted.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum OrgsUpdateMembershipRequestState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for OrgsUpdateMembershipRequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            OrgsUpdateMembershipRequestState::Active => "active",
            OrgsUpdateMembershipRequestState::Noop => "",
            OrgsUpdateMembershipRequestState::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for OrgsUpdateMembershipRequestState {
    fn default() -> OrgsUpdateMembershipRequestState {
        OrgsUpdateMembershipRequestState::Noop
    }
}
impl OrgsUpdateMembershipRequestState {
    pub fn is_noop(&self) -> bool {
        matches!(self, OrgsUpdateMembershipRequestState::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgsUpdateMembershipRequest {
    /**
     * The state that the membership should be in. Only `"active"` will be accepted.
     */
    #[serde(
        default,
        skip_serializing_if = "OrgsUpdateMembershipRequestState::is_noop"
    )]
    pub state: OrgsUpdateMembershipRequestState,
}

/**
 * Can be one of `all`, `public`, or `private`. Note: For GitHub AE, can be one of `all`, `internal`, or `private`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ReposListVisibility {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
    FallthroughString(String),
}

impl std::fmt::Display for ReposListVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReposListVisibility::All => "all",
            ReposListVisibility::Private => "private",
            ReposListVisibility::Public => "public",
            ReposListVisibility::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for ReposListVisibility {
    fn default() -> ReposListVisibility {
        ReposListVisibility::All
    }
}

/**
 * Can be one of `all`, `owner`, `public`, `private`, `member`. Note: For GitHub AE, can be one of `all`, `owner`, `internal`, `private`, `member`. Default: `all`  
 *     
 *   Will cause a `422` error if used in the same request as **visibility** or **affiliation**. Will cause a `422` error if used in the same request as **visibility** or **affiliation**.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ReposListType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
    FallthroughString(String),
}

impl std::fmt::Display for ReposListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReposListType::All => "all",
            ReposListType::Member => "member",
            ReposListType::Owner => "owner",
            ReposListType::Private => "private",
            ReposListType::Public => "public",
            ReposListType::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for ReposListType {
    fn default() -> ReposListType {
        ReposListType::All
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_auto_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_init: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gitignore_template: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_downloads: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_issues: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_projects: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_wiki: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub license_template: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub team_id: i64,
}

/**
 * Identifies which additional information you'd like to receive about the person's hovercard. Can be `organization`, `repository`, `issue`, `pull_request`. **Required** when using `subject_id`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum SubjectType {
    #[serde(rename = "issue")]
    Issue,
    #[serde(rename = "organization")]
    Organization,
    #[serde(rename = "pull_request")]
    PullRequest,
    #[serde(rename = "repository")]
    Repository,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for SubjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SubjectType::Issue => "issue",
            SubjectType::Organization => "organization",
            SubjectType::PullRequest => "pull_request",
            SubjectType::Repository => "repository",
            SubjectType::Noop => "",
            SubjectType::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for SubjectType {
    fn default() -> SubjectType {
        SubjectType::Noop
    }
}
impl SubjectType {
    pub fn is_noop(&self) -> bool {
        matches!(self, SubjectType::Noop)
    }
}

/**
 * Can be one of `all`, `owner`, `member`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ReposListUserType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "owner")]
    Owner,
    FallthroughString(String),
}

impl std::fmt::Display for ReposListUserType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReposListUserType::All => "all",
            ReposListUserType::Member => "member",
            ReposListUserType::Owner => "owner",
            ReposListUserType::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for ReposListUserType {
    fn default() -> ReposListUserType {
        ReposListUserType::Owner
    }
}

/// Any of the following types:
///
/// - `Vec<StarredRepository>`
/// - `Vec<Repository>`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ActivityListReposStarredByUserResponseAnyOf {
    RepositoryVector(Vec<Repository>),
    StarredRepositoryVector(Vec<StarredRepository>),
}
