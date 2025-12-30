/* src/utils/validation.rs */

use crate::error::{CatoolsError, Result};
use regex::Regex;

pub fn validate_ip(ip: &str) -> Result<()> {
    let pattern = r"^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})$";
    let re = Regex::new(pattern).unwrap();

    if !re.is_match(ip) {
        return Err(CatoolsError::ValidationError(format!(
            "Invalid IP format: {}",
            ip
        )));
    }

    // Check each octet is in range 0-255
    for cap in re.captures_iter(ip) {
        for i in 1..=4 {
            let octet: u8 = cap[i]
                .parse()
                .map_err(|_| CatoolsError::ValidationError("Invalid IP octet".to_string()))?;
            if i == 4 && (octet < 1 || octet > 254) {
                return Err(CatoolsError::ValidationError(
                    "Last octet must be 1-254".to_string(),
                ));
            }
        }
    }

    Ok(())
}

pub fn validate_version(ver: &str) -> Result<()> {
    let pattern = r"^v(\d{2})\.(\d{2})$";
    let re = Regex::new(pattern).unwrap();

    if !re.is_match(ver) {
        return Err(CatoolsError::ValidationError(format!(
            "Invalid version format: {}",
            ver
        )));
    }

    Ok(())
}

pub fn parse_version(ver: &str) -> Result<(u32, u32)> {
    validate_version(ver)?;

    let pattern = r"^v(\d{2})\.(\d{2})$";
    let re = Regex::new(pattern).unwrap();

    if let Some(caps) = re.captures(ver) {
        let year: u32 = caps[1].parse().unwrap();
        let month: u32 = caps[2].parse().unwrap();
        Ok((year, month))
    } else {
        Err(CatoolsError::ValidationError(
            "Cannot parse version".to_string(),
        ))
    }
}

pub fn compare_version(v1: &str, v2: &str) -> Result<i8> {
    let (y1, m1) = parse_version(v1)?;
    let (y2, m2) = parse_version(v2)?;

    if y1 > y2 || (y1 == y2 && m1 > m2) {
        Ok(1)
    } else if y1 < y2 || (y1 == y2 && m1 < m2) {
        Ok(-1)
    } else {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_ip() {
        assert!(validate_ip("192.168.1.1").is_ok());
        assert!(validate_ip("10.0.0.254").is_ok());
        assert!(validate_ip("256.1.1.1").is_err());
        assert!(validate_ip("192.168.1").is_err());
    }

    #[test]
    fn test_version_compare() {
        assert_eq!(compare_version("v25.08", "v24.12").unwrap(), 1);
        assert_eq!(compare_version("v24.12", "v25.08").unwrap(), -1);
        assert_eq!(compare_version("v25.08", "v25.08").unwrap(), 0);
    }
}
