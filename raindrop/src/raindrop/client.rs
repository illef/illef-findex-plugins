use super::types::*;
use std::iter::repeat;

const BASE_URL: &str = "https://api.raindrop.io/rest/v1";

pub struct Client {
    token: String,
}

impl Client {
    pub fn new(token: &str) -> Self {
        Client {
            token: token.to_string(),
        }
    }

    pub fn get_all_items(&self) -> Result<Vec<Item>, ureq::Error> {
        let url = format!("{}/raindrops/0", BASE_URL);
        let token = format!("Bearer {}", self.token);

        let mut items = Vec::new();

        for (i, _) in repeat(1).enumerate() {
            let resp = ureq::get(&url)
                .query("page", &i.to_string())
                .query("perpage", "50")
                .set("Authorization", &token)
                .call()?
                .into_json::<RainDropResponse>()?;
            if resp.items.len() == 0 {
                break;
            }
            items.extend(resp.items);
        }

        Ok(items)
    }
}

// write test
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::env;

    #[test]
    fn it_gets_all_items() {
        let token = env!("RAINDROP_TOKEN");
        let client = Client::new(token);

        let items = client.get_all_items().unwrap();

        assert!(!items.is_empty());
        let unique_ids: HashSet<_> = items.iter().map(|item| &item.id).collect();
        assert_eq!(unique_ids.len(), items.len());
    }
}
