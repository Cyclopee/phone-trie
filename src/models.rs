use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Contact {
    pub nb: String,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_one_contact() {
        let json = r#"{ "nb": "0412578440", "name": "Alice" }"#;
        let contact: Contact = serde_json::from_str(json).unwrap();
        assert_eq!(contact.nb, "0412578440");
        assert_eq!(contact.name, "Alice");
    }

    #[test]
    fn deserialize_contact_list() {
        let json = r#"[
            { "nb": "112", "name": "Urgences" },
            { "nb": "15",  "name": "SAMU" }
        ]"#;
        let contacts: Vec<Contact> = serde_json::from_str(json).unwrap();
        assert_eq!(contacts.len(), 2);
        assert_eq!(contacts[0].name, "Urgences");
    }

    #[test]
    fn json5_accepts_trailing_comma() {
        let json = r#"[
            { "nb": "15", "name": "SAMU" },
        ]"#;
        let contacts: Vec<Contact> = json5::from_str(json).unwrap();
        assert_eq!(contacts.len(), 1);
    }
}