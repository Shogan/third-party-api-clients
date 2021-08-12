//! The data types sent to and returned from the API client.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Acl {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub etag: String,
    /**
     * List of rules on the access control list.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AclRule>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page_token: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_sync_token: String,
}

/// The extent to which calendar access is granted by this ACL rule.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Scope {
    /**
     * The extent to which calendar access is granted by this ACL rule.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    /**
     * The extent to which calendar access is granted by this ACL rule.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AclRule {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub etag: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub role: String,
    /**
     * The extent to which calendar access is granted by this ACL rule.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Calendar {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conference_properties: Option<ConferenceProperties>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub etag: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub summary: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub time_zone: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CalendarList {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub etag: String,
    /**
     * Calendars that are present on the user's calendar list.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CalendarListEntry>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page_token: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_sync_token: String,
}

/// The notifications that the authenticated user is receiving for this calendar.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct NotificationSettings {
    /**
     * The notifications that the authenticated user is receiving for this calendar.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notifications: Vec<CalendarNotification>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CalendarListEntry {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_role: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub background_color: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub color_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conference_properties: Option<ConferenceProperties>,
    /**
     * The default reminders that the authenticated user has for this calendar.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub default_reminders: Vec<EventReminder>,
    /**
     * Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub deleted: bool,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub etag: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub foreground_color: String,
    /**
     * Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub hidden: bool,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    /**
     * The notifications that the authenticated user is receiving for this calendar.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_settings: Option<NotificationSettings>,
    /**
     * Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub primary: bool,
    /**
     * Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub selected: bool,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub summary: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub summary_override: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub time_zone: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CalendarNotification {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub method: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

/// A global palette of event colors, mapping from the color ID to its definition. An event resource may refer to one of these color IDs in its colorId field. Read-only.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Event {}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Channel {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address: String,
    /**
     * Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub expiration: i64,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * A global palette of event colors, mapping from the color ID to its definition. An event resource may refer to one of these color IDs in its colorId field. Read-only.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Event>,
    /**
     * Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub payload: bool,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub resource_id: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub resource_uri: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ColorDefinition {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub background: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub foreground: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Colors {
    /**
     * A global palette of event colors, mapping from the color ID to its definition. An event resource may refer to one of these color IDs in its colorId field. Read-only.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar: Option<Event>,
    /**
     * A global palette of event colors, mapping from the color ID to its definition. An event resource may refer to one of these color IDs in its colorId field. Read-only.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<Event>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * Last modification time of the color palette (as a RFC3339 timestamp). Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ConferenceData {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub conference_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conference_solution: Option<ConferenceSolution>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_request: Option<CreateConferenceRequest>,
    /**
     * Information about individual conference entry points, such as URLs or phone numbers.
     *  All of them must belong to the same conference.
     *  Either conferenceSolution and at least one entryPoint, or createRequest is required.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry_points: Vec<EntryPoint>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notes: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ConferenceParameters>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ConferenceParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_on_parameters: Option<ConferenceParametersAddOn>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ConferenceParametersAddOn {
    /**
     * A global palette of event colors, mapping from the color ID to its definition. An event resource may refer to one of these color IDs in its colorId field. Read-only.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Event>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ConferenceProperties {
    /**
     * The types of conference solutions that are supported for this calendar.
     *  The possible values are:  
     *  - "eventHangout"
     *  - "eventNamedHangout"
     *  - "hangoutsMeet"  Optional.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_conference_solution_types: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ConferenceRequestStatus {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status_code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ConferenceSolution {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub icon_uri: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<ConferenceSolutionKey>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ConferenceSolutionKey {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CreateConferenceRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conference_solution_key: Option<ConferenceSolutionKey>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub request_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ConferenceRequestStatus>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EntryPoint {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_code: String,
    /**
     * The types of conference solutions that are supported for this calendar.
     *  The possible values are:  
     *  - "eventHangout"
     *  - "eventNamedHangout"
     *  - "hangoutsMeet"  Optional.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry_point_features: Vec<String>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub entry_point_type: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub meeting_code: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub passcode: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub password: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pin: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub region_code: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Error {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub domain: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
}

/// The creator of the event. Read-only.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Creator {
    /**
     * The creator of the event. Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    /**
     * The creator of the event. Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The creator of the event. Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * The creator of the event. Read-only.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "self"
    )]
    pub self_: bool,
}

/// Extended properties of the event.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ExtendedProperties {
    /**
     * Extended properties of the event.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<Event>,
    /**
     * Extended properties of the event.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared: Option<Event>,
}

/// A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Gadget {
    /**
     * A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display: String,
    /**
     * A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub height: i64,
    /**
     * A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub icon_link: String,
    /**
     * A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub link: String,
    /**
     * A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferences: Option<Event>,
    /**
     * A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
     * A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    /**
     * A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub width: i64,
}

/// The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Organizer {
    /**
     * The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    /**
     * The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "self"
    )]
    pub self_: bool,
}

/// Information about the event's reminders for the authenticated user.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Reminders {
    /**
     * Information about the event's reminders for the authenticated user.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub overrides: Vec<EventReminder>,
    /**
     * Information about the event's reminders for the authenticated user.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub use_default: bool,
}

/// Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Source {
    /**
     * Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
     * Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EventData {
    /**
     * Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub anyone_can_add_self: bool,
    /**
     * File attachments for the event. Currently only Google Drive attachments are supported.
     *  In order to modify attachments the supportsAttachments request parameter should be set to true.
     *  There can be at most 25 attachments per event,
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<EventAttachment>,
    /**
     * The attendees of the event. See the Events with attendees guide for more information on scheduling events with other calendar users. Service accounts need to use domain-wide delegation of authority to populate the attendee list.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attendees: Vec<EventAttendee>,
    /**
     * Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub attendees_omitted: bool,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub color_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conference_data: Option<ConferenceData>,
    /**
     * Last modification time of the color palette (as a RFC3339 timestamp). Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The creator of the event. Read-only.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<Creator>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<EventDateTime>,
    /**
     * Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub end_time_unspecified: bool,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub etag: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event_type: String,
    /**
     * Extended properties of the event.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<ExtendedProperties>,
    /**
     * A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gadget: Option<Gadget>,
    /**
     * Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub guests_can_invite_others: bool,
    /**
     * Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub guests_can_modify: bool,
    /**
     * Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub guests_can_see_other_guests: bool,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hangout_link: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_link: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub i_cal_uid: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    /**
     * Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub locked: bool,
    /**
     * The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizer: Option<Organizer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_start_time: Option<EventDateTime>,
    /**
     * Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub private_copy: bool,
    /**
     * The types of conference solutions that are supported for this calendar.
     *  The possible values are:  
     *  - "eventHangout"
     *  - "eventNamedHangout"
     *  - "hangoutsMeet"  Optional.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recurrence: Vec<String>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub recurring_event_id: String,
    /**
     * Information about the event's reminders for the authenticated user.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reminders: Option<Reminders>,
    /**
     * Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub sequence: i64,
    /**
     * Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<EventDateTime>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub summary: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub transparency: String,
    /**
     * Last modification time of the color palette (as a RFC3339 timestamp). Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EventAttachment {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub file_id: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub file_url: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub icon_link: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mime_type: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EventAttendee {
    /**
     * Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub additional_guests: i64,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comment: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub optional: bool,
    /**
     * Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub organizer: bool,
    /**
     * Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub resource: bool,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub response_status: String,
    /**
     * Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "self"
    )]
    pub self_: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EventDateTime {
    /**
     * The date, in the format "yyyy-mm-dd", if this is an all-day event.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<chrono::NaiveDate>,
    /**
     * Last modification time of the color palette (as a RFC3339 timestamp). Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub date_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub time_zone: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EventReminder {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub method: String,
    /**
     * Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub minutes: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Events {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_role: String,
    /**
     * The default reminders that the authenticated user has for this calendar.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub default_reminders: Vec<EventReminder>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub etag: String,
    /**
     * List of events on the calendar.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<EventData>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page_token: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_sync_token: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub summary: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub time_zone: String,
    /**
     * Last modification time of the color palette (as a RFC3339 timestamp). Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FreeBusyCalendar {
    /**
     * List of time ranges during which this calendar should be regarded as busy.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub busy: Vec<TimePeriod>,
    /**
     * Optional error(s) (if computation for the calendar failed).
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<Error>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FreeBusyGroup {
    /**
     * The types of conference solutions that are supported for this calendar.
     *  The possible values are:  
     *  - "eventHangout"
     *  - "eventNamedHangout"
     *  - "hangoutsMeet"  Optional.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub calendars: Vec<String>,
    /**
     * Optional error(s) (if computation for the calendar failed).
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<Error>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FreeBusyRequest {
    /**
     * Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub calendar_expansion_max: i64,
    /**
     * Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub group_expansion_max: i64,
    /**
     * List of calendars and/or groups to query.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<FreeBusyRequestItem>,
    /**
     * Last modification time of the color palette (as a RFC3339 timestamp). Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_max: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Last modification time of the color palette (as a RFC3339 timestamp). Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_min: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub time_zone: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FreeBusyRequestItem {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FreeBusyResponse {
    /**
     * A global palette of event colors, mapping from the color ID to its definition. An event resource may refer to one of these color IDs in its colorId field. Read-only.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendars: Option<Event>,
    /**
     * A global palette of event colors, mapping from the color ID to its definition. An event resource may refer to one of these color IDs in its colorId field. Read-only.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Event>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * Last modification time of the color palette (as a RFC3339 timestamp). Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_max: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Last modification time of the color palette (as a RFC3339 timestamp). Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_min: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Setting {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub etag: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Settings {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub etag: String,
    /**
     * List of user settings.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Setting>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page_token: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_sync_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimePeriod {
    /**
     * Last modification time of the color palette (as a RFC3339 timestamp). Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub end: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Last modification time of the color palette (as a RFC3339 timestamp). Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub start: Option<chrono::DateTime<chrono::Utc>>,
}

/**
 * Data format for the response.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Alt {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Alt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Alt::Json => "json",
            Alt::Noop => "",
            Alt::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Alt {
    fn default() -> Alt {
        Alt::Noop
    }
}
impl Alt {
    pub fn is_noop(&self) -> bool {
        matches!(self, Alt::Noop)
    }
}

/**
 * The order of the events returned in the result. Optional. The default is an unspecified, stable order.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum OrderBy {
    #[serde(rename = "startTime")]
    StartTime,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for OrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            OrderBy::StartTime => "startTime",
            OrderBy::Updated => "updated",
            OrderBy::Noop => "",
            OrderBy::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for OrderBy {
    fn default() -> OrderBy {
        OrderBy::Noop
    }
}
impl OrderBy {
    pub fn is_noop(&self) -> bool {
        matches!(self, OrderBy::Noop)
    }
}

/**
 * Whether to send notifications about the creation of the new event. Note that some emails might still be sent. The default is false.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum SendUpdates {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "externalOnly")]
    ExternalOnly,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for SendUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SendUpdates::All => "all",
            SendUpdates::ExternalOnly => "externalOnly",
            SendUpdates::None => "none",
            SendUpdates::Noop => "",
            SendUpdates::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for SendUpdates {
    fn default() -> SendUpdates {
        SendUpdates::Noop
    }
}
impl SendUpdates {
    pub fn is_noop(&self) -> bool {
        matches!(self, SendUpdates::Noop)
    }
}

/**
 * The minimum access role for the user in the returned entries. Optional. The default is no restriction.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum MinAccessRole {
    #[serde(rename = "freeBusyReader")]
    FreeBusyReader,
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "reader")]
    Reader,
    #[serde(rename = "writer")]
    Writer,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for MinAccessRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            MinAccessRole::FreeBusyReader => "freeBusyReader",
            MinAccessRole::Owner => "owner",
            MinAccessRole::Reader => "reader",
            MinAccessRole::Writer => "writer",
            MinAccessRole::Noop => "",
            MinAccessRole::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for MinAccessRole {
    fn default() -> MinAccessRole {
        MinAccessRole::Noop
    }
}
impl MinAccessRole {
    pub fn is_noop(&self) -> bool {
        matches!(self, MinAccessRole::Noop)
    }
}
