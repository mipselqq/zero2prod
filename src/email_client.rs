use crate::SubscriberEmail;

pub struct EmailClient {
    sender: SubscriberEmail,
}

impl EmailClient {
    pub async fn send_email(
        &self,
        recepient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        todo!()
    }
}
