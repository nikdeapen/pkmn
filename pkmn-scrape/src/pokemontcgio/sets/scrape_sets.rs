use crate::pokemontcgio::client::Client;
use crate::pokemontcgio::model::{RawPage, RawSet};
use web_scrape::Error;

impl Client {
    //! Scrape Sets

    /// The number of sets per page. (the api max is `250`)
    const SETS_PAGE_SIZE: usize = 250;

    /// Scrapes the card sets.
    pub fn scrape_sets(&self) -> Result<Vec<RawSet>, Error> {
        let mut sets: Vec<RawSet> = Vec::new();
        let mut page: usize = 1;
        loop {
            let extension: String = format!("sets?page={}&pageSize={}", page, Self::SETS_PAGE_SIZE);
            let response: RawPage<RawSet> = self.scrape(extension.as_str())?;
            let done: bool = response.data.is_empty()
                || sets.len() + response.data.len() >= response.total_count;
            sets.extend(response.data);
            if done {
                return Ok(sets);
            }
            page += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn scrape_sets() {
        let client: Client = Client::with_local_cache();
        let sets: Vec<RawSet> = client.scrape_sets().unwrap();
        for set in &sets {
            println!("{:?}", set);
        }
        println!("scraped {} sets", sets.len());
    }
}
