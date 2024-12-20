/*
 * Clerk Frontend API
 *
 * The Clerk REST Frontend API, meant to be accessed from a browser or native environment.  This is a Form Based API and all the data must be sent and formatted according to the `application/x-www-form-urlencoded` content type.  ### Versions  When the API changes in a way that isn't compatible with older versions, a new version is released. Each version is identified by its release date, e.g. `2021-02-05`. For more information, please see [Clerk API Versions](https://clerk.com/docs/backend-requests/versioning/overview).  ### Using the Try It Console  The `Try It` feature of the docs only works for **Development Instances** when using the `DevBrowser` security scheme. To use it, first generate a dev instance token from the `/v1/dev_browser` endpoint.  Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSettingsPeriodSocial {
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "required")]
    pub required: bool,
    #[serde(rename = "authenticatable")]
    pub authenticatable: bool,
    #[serde(
        rename = "block_email_subaddresses",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_email_subaddresses: Option<bool>,
    #[serde(rename = "strategy")]
    pub strategy: String,
    #[serde(rename = "not_selectable", skip_serializing_if = "Option::is_none")]
    pub not_selectable: Option<bool>,
    #[serde(rename = "deprecated", skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
}

impl UserSettingsPeriodSocial {
    pub fn new(
        enabled: bool,
        required: bool,
        authenticatable: bool,
        strategy: String,
    ) -> UserSettingsPeriodSocial {
        UserSettingsPeriodSocial {
            enabled,
            required,
            authenticatable,
            block_email_subaddresses: None,
            strategy,
            not_selectable: None,
            deprecated: None,
        }
    }
}
