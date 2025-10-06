use crate::{SubscriberEmail, domain::subscriber_name::SubscriberName};

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
