use anyhow::Error;

pub fn decode_resp(s: &str) -> Result<Vec<String>, Error> {
    let mut res: Vec<String> = Vec::new();
    s.split("\r\n").for_each(|x| {
        res.push(x.to_string());
    });
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_resp_valid() {
        let res = decode_resp("*2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n");
        assert!(res.is_ok());
    }
}
