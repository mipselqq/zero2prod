use validator::ValidateEmail;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(email: String) -> Result<SubscriberEmail, String> {
        if ValidateEmail::validate_email(&email) {
            Ok(Self(email))
        } else {
            Err(format!("{email} is not a valid subscriber email"))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::SubscriberEmail;
    use claims::assert_err;
    use fake::{Fake, faker::internet::en::SafeEmail};
    use proptest::prelude::*;

    #[test]
    fn empty_string_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn missing_at_symbol_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn missing_subject_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    fn arb_valid_email() -> impl Strategy<Value = String> {
        any::<()>().prop_map(|_| SafeEmail().fake())
    }

    proptest! {
        #[test]
        fn valid_emails_accepted(email in arb_valid_email()) {
            prop_assert!(SubscriberEmail::parse(email).is_ok());
        }
    }
}
