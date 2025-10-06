use crate::SubscriberEmail;
use reqwest::{Client, header};
use secrecy::{ExposeSecret, SecretString};

pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
    authorization_token: SecretString,
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
        authorization_token: SecretString,
    ) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
            authorization_token,
        }
    }

    pub async fn send_email(
        &self,
        recepient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), reqwest::Error> {
        let _ = self
            .http_client
            .post(format!("{}/mail", self.base_url))
            .header(
                "Authorization",
                format!("Basic {}", self.authorization_token.expose_secret()),
            )
            .query(&[
                ("from", self.sender.as_ref()),
                ("to", recepient.as_ref()),
                ("subject", subject),
                ("text", text_content),
                ("html", html_content),
            ])
            .send()
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use fake::{
        Fake, Faker,
        faker::{
            internet::en::SafeEmail,
            lorem::en::{Paragraph, Sentence},
        },
    };
    use secrecy::SecretString;
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::any};

    use crate::{EmailClient, SubscriberEmail};

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_ursl() {
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(
            mock_server.uri(),
            sender,
            SecretString::new(Faker.fake::<String>().into()),
        );

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        let _ = email_client
            .send_email(subscriber_email, &subject, &content, &content)
            .await;
    }
}
