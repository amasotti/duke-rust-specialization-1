use validator::{Validate, ValidationErrors};

// -------------------------- USER --------------------------

#[derive(Debug)]
pub struct User {
    pub username: String,
    email: Email,
    uri: String,
    pub active: bool,
}

impl User {
    /// Create a new user account
    pub fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email: Email::new(&email).unwrap(),
            uri,
            active: true,
        }
    }

    pub fn from_email(email: &str) -> Self {
        let email = Email::new(email).unwrap();

        Self {
            username: email.extract_username(),
            email: email.clone(),
            uri: format!("https://{}", email.extract_domain()),
            active: true,
        }
    }

    /// Deactivate the user account
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    
    /// Update URI
    pub fn update_uri(&mut self, uri: &str) {
        self.uri = uri.to_string();
    }
}

// -------------------------- EMAIL --------------------------

#[derive(Debug, Clone, Validate)]
pub struct Email {
    #[validate(email)]
    pub inner: String,
}

impl Email {
    /// Create a new email address; validate on creation
    pub fn new(value: &str) -> Result<Self, ValidationErrors> {
        let email = Self {
            inner: value.to_owned(),
        };

        email.validate()?;

        Ok(email)
    }

    /// Extract the username from the email address
    pub fn extract_username(&self) -> String {
        self.inner.split('@').next().unwrap().to_string()
    }

    /// Extract the domain from the email address
    pub fn extract_domain(&self) -> String {
        self.inner.split('@').last().unwrap().to_string()
    }
}
