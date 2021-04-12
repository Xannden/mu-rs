use select::document::Document;
use ureq::{Agent, AgentBuilder};

use crate::{
    series::{parse_search_results, parse_series, SeriesSearchResult},
    Series, SeriesId,
};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

///Used to interact with MangaUpdates pages
///
/// MuClient has a internal Arc so cloning is cheap
#[derive(Clone)]
pub struct MuClient {
    pub(crate) client: Agent,
}

impl MuClient {
    pub fn new() -> MuClient {
        MuClient {
            client: AgentBuilder::new().user_agent(APP_USER_AGENT).build(),
        }
    }

    pub(crate) fn get(&self, url: &str) -> Document {
        let response = self.client.get(url).call().unwrap();

        Document::from_read(response.into_reader()).unwrap()
    }

    pub fn search(&self, query: &str) -> Vec<SeriesSearchResult> {
        let doc = self.get(&format!(
            "https://www.mangaupdates.com/series.html?search={}",
            query
        ));

        parse_search_results(doc)
    }

    pub fn series(&self, id: SeriesId) -> Option<Series> {
        let doc = self.get(&format!(
            "https://www.mangaupdates.com/series.html?id={}",
            id.0
        ));

        parse_series(self, doc, id)
    }
}

impl Default for MuClient {
    fn default() -> Self {
        Self::new()
    }
}
