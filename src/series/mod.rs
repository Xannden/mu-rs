use crate::{AuthorId, GroupId, PublisherId};

mod parsing;

pub use parsing::{parse_search_results, parse_series};

/// The Id for a series on MangaUpdates
///
/// The id can be found by searching by name with [MuClient](crate::MuClient::search())
/// or by going to the page for the series and looking for `id=` in the url
#[derive(Debug, Clone, Copy, Default)]
pub struct SeriesId(pub usize);

#[derive(Debug)]
pub struct SeriesSearchResult {
    id: SeriesId,
    title: String,
    description: Option<String>,
    image: Option<String>,
    year: Option<String>,
    rating: Option<String>,
}

impl SeriesSearchResult {
    pub fn id(&self) -> SeriesId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn image(&self) -> Option<&str> {
        self.image.as_deref()
    }

    pub fn year(&self) -> Option<&str> {
        self.year.as_deref()
    }

    pub fn rating(&self) -> Option<&str> {
        self.rating.as_deref()
    }
}

///All the information on the series page
#[derive(Debug, Default)]
pub struct Series {
    id: SeriesId,
    title: String,
    description: Option<String>,
    ty: Option<String>,
    related_series: Vec<(SeriesId, String, String)>,
    associated_names: Vec<String>,
    groups_scanlating: Vec<(GroupId, String)>,
    status_in_coo: Vec<String>,
    completely_scanlated: bool,
    anime_start_end: Vec<String>,
    image: Option<String>,
    genre: Vec<String>,
    categories: Vec<String>,
    category_recommendations: Vec<(SeriesId, String)>,
    recommendations: Vec<(SeriesId, String)>,
    authors: Vec<(AuthorId, String)>,
    artists: Vec<(AuthorId, String)>,
    year: Option<String>,
    original_publisher: Option<(PublisherId, String)>,
    serialized_in: Vec<(String, String)>,
    licensed: bool,
    english_publisher: Vec<(PublisherId, String, Option<String>)>,
}

impl Series {
    /// Get a reference to the series's id.
    pub fn id(&self) -> &SeriesId {
        &self.id
    }

    /// Get a reference to the series's title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get a reference to the series's description.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Get a reference to the series's ty.
    pub fn ty(&self) -> Option<&str> {
        self.ty.as_deref()
    }

    /// Get a reference to the series's related series.
    pub fn related_series(&self) -> impl Iterator<Item = (SeriesId, &str, &str)> {
        self.related_series
            .iter()
            .map(|i| (i.0, i.1.as_str(), i.2.as_str()))
    }

    /// Get a reference to the series's associated names.
    pub fn associated_names(&self) -> impl Iterator<Item = &str> {
        self.associated_names.iter().map(|i| i.as_str())
    }

    /// Get a reference to the series's groups scanlating.
    pub fn groups_scanlating(&self) -> impl Iterator<Item = (GroupId, &str)> {
        self.groups_scanlating.iter().map(|i| (i.0, i.1.as_str()))
    }

    /// Get a reference to the series's status in coo.
    pub fn status_in_coo(&self) -> impl Iterator<Item = &str> {
        self.status_in_coo.iter().map(|i| i.as_str())
    }

    /// Get a reference to the series's completely scanlated.
    pub fn completely_scanlated(&self) -> bool {
        self.completely_scanlated
    }

    /// Get a reference to the series's anime start end.
    pub fn anime_start_end(&self) -> impl Iterator<Item = &str> {
        self.anime_start_end.iter().map(|i| i.as_str())
    }

    /// Get a reference to the series's image.
    pub fn image(&self) -> Option<&str> {
        self.image.as_deref()
    }

    /// Get a reference to the series's genre.
    pub fn genre(&self) -> impl Iterator<Item = &str> {
        self.genre.iter().map(|i| i.as_str())
    }

    /// Get a reference to the series's categories.
    pub fn categories(&self) -> impl Iterator<Item = &str> {
        self.categories.iter().map(|i| i.as_str())
    }

    /// Get a reference to the series's category recommendations.
    pub fn category_recommendations(&self) -> &[(SeriesId, String)] {
        &self.category_recommendations
    }

    /// Get a reference to the series's recommendations.
    pub fn recommendations(&self) -> impl Iterator<Item = (SeriesId, &str)> {
        self.recommendations.iter().map(|i| (i.0, i.1.as_str()))
    }

    /// Get a reference to the series's authors.
    pub fn authors(&self) -> impl Iterator<Item = (AuthorId, &str)> {
        self.authors.iter().map(|i| (i.0, i.1.as_str()))
    }

    /// Get a reference to the series's artists.
    pub fn artists(&self) -> impl Iterator<Item = (AuthorId, &str)> {
        self.artists.iter().map(|i| (i.0, i.1.as_str()))
    }

    /// Get a reference to the series's year.
    pub fn year(&self) -> Option<&str> {
        self.year.as_deref()
    }

    /// Get a reference to the series's original publisher.
    pub fn original_publisher(&self) -> Option<(PublisherId, &str)> {
        self.original_publisher
            .as_ref()
            .map(|i| (i.0, i.1.as_str()))
    }

    /// Get a reference to the series's serialized in.
    pub fn serialized_in(&self) -> impl Iterator<Item = (&str, &str)> {
        self.serialized_in
            .iter()
            .map(|i| (i.0.as_str(), i.1.as_str()))
    }

    /// Get a reference to the series's licensed.
    pub fn licensed(&self) -> bool {
        self.licensed
    }

    /// Get a reference to the series's english publisher.
    pub fn english_publisher(&self) -> impl Iterator<Item = (PublisherId, &str, Option<&str>)> {
        self.english_publisher
            .iter()
            .map(|i| (i.0, i.1.as_str(), i.2.as_deref()))
    }
}
