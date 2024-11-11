use std::fmt;

#[derive(Debug)]
pub enum ClerkFapiEndpoint {
    Get(ClerkFapiGetEndpoint),
    Post(ClerkFapiPostEndpoint),
    Delete(ClerkFapiDeleteEndpoint),
    Put(ClerkFapiPutEndpoint),
    Patch(ClerkFapiPatchEndpoint),
    DynamicGet(ClerkFapiDynamicGetEndpoint),
    DynamicPost(ClerkFapiDynamicPostEndpoint),
    DynamicDelete(ClerkFapiDynamicDeleteEndpoint),
    DynamicPut(ClerkFapiDynamicPutEndpoint),
    DynamicPatch(ClerkFapiDynamicPatchEndpoint),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkFapiDynamicGetEndpoint {
    GetOrganization,
    GetOrganizationMembership,
    GetOrganizationInvitation,
    GetOrganizationSuggestions,
    GetSession,
    GetUser,
    GetUserOrganizationInvitations,
    GetUserOrganizationMemberships,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkFapiDynamicPostEndpoint {
    AcceptOrganizationInvitation,
    CreateOrganization,
    CreateOrganizationInvitation,
    CreateOrganizationMembership,
    SignOut,
    SignUp,
    VerifyEmail,
    VerifyPhoneNumber,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkFapiDynamicDeleteEndpoint {
    DeleteOrganization,
    DeleteOrganizationMembership,
    RejectOrganizationInvitation,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkFapiDynamicPutEndpoint {
    UpdateOrganization,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkFapiDynamicPatchEndpoint {
    UpdateOrganizationMembership,
    UpdateOrganizationMetadata,
    UpdateUser,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkFapiGetEndpoint {
    ListOrganizations,
    GetActiveSession,
    GetClientToken,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkFapiPostEndpoint {
    SignIn,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkFapiDeleteEndpoint {}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkFapiPatchEndpoint {}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkFapiPutEndpoint {}

impl ClerkFapiGetEndpoint {
    pub fn as_str(&self) -> &str {
        match self {
            ClerkFapiGetEndpoint::ListOrganizations => "/organizations",
            ClerkFapiGetEndpoint::GetActiveSession => "/session",
            ClerkFapiGetEndpoint::GetClientToken => "/client",
        }
    }
}

impl ClerkFapiPostEndpoint {
    pub fn as_str(&self) -> &str {
        match self {
            ClerkFapiPostEndpoint::SignIn => "/sign_in",
        }
    }
}

impl ClerkFapiDynamicGetEndpoint {
    pub fn as_str(&self) -> &str {
        match self {
            ClerkFapiDynamicGetEndpoint::GetOrganization => "/organizations/{organization_id}",
            ClerkFapiDynamicGetEndpoint::GetOrganizationMembership => "/organizations/{organization_id}/memberships/{user_id}",
            ClerkFapiDynamicGetEndpoint::GetOrganizationInvitation => "/organizations/{organization_id}/invitations/{invitation_id}",
            ClerkFapiDynamicGetEndpoint::GetOrganizationSuggestions => "/organizations/{organization_id}/suggestions",
            ClerkFapiDynamicGetEndpoint::GetSession => "/sessions/{session_id}",
            ClerkFapiDynamicGetEndpoint::GetUser => "/users/{user_id}",
            ClerkFapiDynamicGetEndpoint::GetUserOrganizationInvitations => "/users/{user_id}/organization_invitations",
            ClerkFapiDynamicGetEndpoint::GetUserOrganizationMemberships => "/users/{user_id}/organization_memberships",
        }
    }
}

impl ClerkFapiDynamicPostEndpoint {
    pub fn as_str(&self) -> &str {
        match self {
            ClerkFapiDynamicPostEndpoint::AcceptOrganizationInvitation => "/organizations/{organization_id}/invitations/{invitation_id}/accept",
            ClerkFapiDynamicPostEndpoint::CreateOrganization => "/organizations",
            ClerkFapiDynamicPostEndpoint::CreateOrganizationInvitation => "/organizations/{organization_id}/invitations",
            ClerkFapiDynamicPostEndpoint::CreateOrganizationMembership => "/organizations/{organization_id}/memberships",
            ClerkFapiDynamicPostEndpoint::SignOut => "/sessions/{session_id}/end",
            ClerkFapiDynamicPostEndpoint::SignUp => "/sign_up",
            ClerkFapiDynamicPostEndpoint::VerifyEmail => "/verify_email",
            ClerkFapiDynamicPostEndpoint::VerifyPhoneNumber => "/verify_phone_number",
        }
    }
}

impl ClerkFapiDynamicDeleteEndpoint {
    pub fn as_str(&self) -> &str {
        match self {
            ClerkFapiDynamicDeleteEndpoint::DeleteOrganization => "/organizations/{organization_id}",
            ClerkFapiDynamicDeleteEndpoint::DeleteOrganizationMembership => "/organizations/{organization_id}/memberships/{user_id}",
            ClerkFapiDynamicDeleteEndpoint::RejectOrganizationInvitation => "/organizations/{organization_id}/invitations/{invitation_id}",
        }
    }
}

impl ClerkFapiDynamicPutEndpoint {
    pub fn as_str(&self) -> &str {
        match self {
            ClerkFapiDynamicPutEndpoint::UpdateOrganization => "/organizations/{organization_id}",
        }
    }
}

impl ClerkFapiDynamicPatchEndpoint {
    pub fn as_str(&self) -> &str {
        match self {
            ClerkFapiDynamicPatchEndpoint::UpdateOrganizationMembership => "/organizations/{organization_id}/memberships/{user_id}",
            ClerkFapiDynamicPatchEndpoint::UpdateOrganizationMetadata => "/organizations/{organization_id}/metadata",
            ClerkFapiDynamicPatchEndpoint::UpdateUser => "/users/{user_id}",
        }
    }
}

// Implement Display traits for all enums
impl fmt::Display for ClerkFapiGetEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for ClerkFapiPostEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for ClerkFapiDeleteEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for ClerkFapiPutEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for ClerkFapiPatchEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for ClerkFapiDynamicGetEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for ClerkFapiDynamicPostEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for ClerkFapiDynamicDeleteEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for ClerkFapiDynamicPutEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for ClerkFapiDynamicPatchEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
} 