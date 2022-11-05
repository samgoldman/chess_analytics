#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
pub fn get_required_parameter(
    step_name: &str,
    param: &str,
    params: &serde_yaml::Value,
) -> Result<String, String> {
    match params.get(param) {
        Some(v) => Ok(v.as_str().unwrap().to_string()),
        None => Err(format!("{step_name}: parameter '{param}' is required")),
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
pub fn get_parameter_with_default(
    param: &str,
    default: &str,
    params: &serde_yaml::Value,
) -> String {
    match params.get(param) {
        Some(v) => v.as_str().unwrap().to_string(),
        None => default.to_string(),
    }
}
