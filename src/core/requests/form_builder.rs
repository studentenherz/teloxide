use reqwest::r#async::multipart::Form;
use serde::Serialize;

/// This is a convenient struct that builds `reqwest::r#async::multipart::Form`
/// from scratch.
pub struct FormBuilder {
    form: Form,
}

impl FormBuilder {
    pub fn new() -> Self {
        Self { form: Form::new() }
    }

    /// Add the supplied key-value pair to this `FormBuilder`. Don't forget to
    /// implement `serde::Serialize` for `T`!
    pub fn add<T>(self, name: &str, value: &T) -> Self
    where
        T: Serialize,
    {
        Self {
            form: self.form.text(
                name.to_owned(),
                serde_json::to_string(value)
                    .expect("serde_json::to_string failed"),
            ),
        }
    }

    /// Adds a key-value pair to the supplied `FormBuilder` if `value` is some.
    /// Don't forget to implement `serde::Serialize` for `T`!
    pub fn add_if_some<T>(self, name: &str, value: Option<&T>) -> Self
    where
        T: Serialize,
    {
        match value {
            None => Self { form: self.form },
            Some(value) => Self {
                form: self.form.text(
                    name.to_owned(),
                    serde_json::to_string(value)
                        .expect("serde_json::to_string failed"),
                ),
            },
        }
    }

    /// Add the supplied key-value pair to this `FormBuilder`.
    /// With raw str value, so `serde_json` will not add redundant `""`
    pub fn add_raw(self, name: &str, value: &str) -> Self {
        Self {
            form: self.form.text(
                name.to_owned(),
                value.to_owned(),
            ),
        }
    }

    pub fn build(self) -> Form {
        self.form
    }
}
