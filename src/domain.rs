use unicode_segmentation::UnicodeSegmentation;

pub struct SubscriberName(String);

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

impl SubscriberName {
    pub fn parse(name: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = name.trim().is_empty();
        let is_too_long = name.graphemes(true).count() > 256;
        let forbidden_charactores = "/()<>\\{}";
        let contains_forbideen_characters = name
            .chars()
            .any(|char| forbidden_charactores.contains(char));

        let is_invalid = is_empty_or_whitespace || is_too_long || contains_forbideen_characters;

        if is_invalid {
            Err(format!("{name} is not a valid subscriber name"))
        } else {
            Ok(Self(name))
        }
    }
}

impl AsRef<String> for SubscriberName {
    fn as_ref(&self) -> &String {
        &self.0
    }
}
