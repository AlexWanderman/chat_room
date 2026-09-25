use anyhow::{Result, anyhow};

const MAX_NAME_LEN: usize = 30;
const MAX_DESCRIPTION_LEN: usize = 120;
const MAX_PASSWORD_LEN: usize = 120;
const MAX_MESSAGE_LEN: usize = 500;

pub fn validate_name(name: &str) -> Result<()> {
    let len = name.len();

    if len < 3 {
        Err(anyhow!("Name must be longer ({len} < 3)"))
    } else if len > MAX_NAME_LEN {
        Err(anyhow!("Name must be shorter ({len} > {MAX_NAME_LEN})"))
    } else if name
        .chars()
        .any(|c| !['-', '_'].contains(&c) && !c.is_alphanumeric())
    {
        Err(anyhow!("Symbols and spaces are forbidden ('_' '-' OK)"))
    } else {
        Ok(())
    }
}

pub fn validate_description(description: &str) -> Result<()> {
    let len = description.len();

    if description.is_empty() {
        Err(anyhow!("Description must contain something"))
    } else if len > MAX_DESCRIPTION_LEN {
        Err(anyhow!(
            "Description must be shorter ({len} > {MAX_DESCRIPTION_LEN})"
        ))
    } else {
        Ok(())
    }
}

pub fn validate_password(password: &str) -> Result<()> {
    let len = password.len();

    if password.is_empty() {
        Err(anyhow!("Password must contain something"))
    } else if len > MAX_PASSWORD_LEN {
        Err(anyhow!(
            "Password must be shorter ({len} > {MAX_MESSAGE_LEN})"
        ))
    } else {
        Ok(())
    }
}

pub fn validate_message(message: &str) -> Result<()> {
    let len = message.len();

    if message.is_empty() {
        Err(anyhow!("Message must contain something"))
    } else if len > MAX_MESSAGE_LEN {
        Err(anyhow!(
            "Message must be shorter ({len} > {MAX_MESSAGE_LEN})"
        ))
    } else {
        Ok(())
    }
}
