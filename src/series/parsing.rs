use select::{document::Document, node::Node, predicate::*};

use super::*;
use crate::{get_id, AuthorId, GroupId, MuClient, PublisherId};

pub fn parse_search_results(doc: Document) -> Vec<SeriesSearchResult> {
    fn parse(doc: Document, results: &mut Vec<SeriesSearchResult>) -> Option<()> {
        for node in doc.find(Class("col-lg-6")) {
            let image = node
                .find(Name("img"))
                .next()
                .map(|n| n.attr("src").unwrap().to_string()); // This unwrap should be safe because we are on a img tag
            let mut data = node
                .find(Class("flex-column"))
                .next()?
                .children()
                .filter(|n| !n.is(Text));

            let (title, id) = {
                let node = data.next()?;

                (
                    node.text(),
                    get_id(node.find(Name("a")).next()?.attr("href"))?,
                )
            };

            //Genres
            data.next();

            let desc = data.next()?.text();

            let (year, rating) = parse_year_and_rating(data.next()?);

            results.push(SeriesSearchResult {
                id: SeriesId(id),
                title,
                description: if desc.trim() == "N/A" || desc.is_empty() {
                    None
                } else {
                    Some(desc)
                },
                image,
                year,
                rating,
            })
        }

        Some(())
    }
    let mut results = Vec::new();

    parse(doc, &mut results);

    results
}

fn parse_year_and_rating(node: Node) -> (Option<String>, Option<String>) {
    let text = node.text();
    let mut text = text.as_str();

    let mut year = if let Some(pos) = text.find('-') {
        let year = &text[..(pos - 1)];
        text = &text[(pos + 1)..].trim();

        Some(year.to_string())
    } else {
        None
    };

    let rating = if let Some(pos) = text.find('/') {
        Some(text[..(pos - 1)].to_string())
    } else {
        if !text.is_empty() && year.is_none() {
            year = Some(text.trim().to_string());
        }
        None
    };

    (year, rating)
}

pub fn parse_series(client: &MuClient, doc: Document, id: SeriesId) -> Option<Series> {
    let title = doc.find(Class("releasestitle")).next()?.text();

    let mut col6 = doc.find(And(Class("col-6"), Class("p-2")));

    let mut series = Series {
        id,
        title,
        ..Default::default()
    };

    parse_col_1(col6.next()?, &mut series);
    parse_col_2(client, col6.next()?, &mut series);

    Some(series)
}

fn parse_col_1(node: Node, series: &mut Series) -> Option<()> {
    let mut contents = node.find(Class("sContent"));

    series.description = parse_description(contents.next()?);

    series.ty = Some(contents.next()?.text().trim().to_owned());

    parse_related(contents.next()?, series);

    parse_associated_names(contents.next()?, series);

    parse_scanlating(contents.next()?, series);

    // //Latest Releases
    contents.next();

    parse_status_in_coo(contents.next()?, series);

    series.completely_scanlated = contents.next()?.text().trim() == "Yes";

    parse_anime_start_end(contents.next()?, series);

    Some(())
}
fn parse_col_2(client: &MuClient, node: Node, series: &mut Series) -> Option<()> {
    let mut contents = node.find(Class("sContent"));

    series.image = parse_image(contents.next()?);

    parse_genre(contents.next()?, series);

    parse_categories(client, contents.next()?, series);

    parse_category_recommendations(contents.next()?, series);

    parse_recommendations(contents.next()?, series);

    parse_authors(contents.next()?, series);

    parse_artists(contents.next()?, series);

    series.year = parse_year(contents.next()?);

    series.original_publisher = parse_original_publisher(contents.next()?);

    parse_serialized_in(contents.next()?, series);

    series.licensed = parse_licensed(contents.next()?);

    parse_english_publisher(contents.next()?, series);

    Some(())
}

fn parse_description(node: Node) -> Option<String> {
    if node.text().trim() == "N/A" {
        return None;
    }

    let desc = if let Some(desc) = node.find(Attr("id", "div_desc_more")).next() {
        desc.first_child()?.text()
    } else {
        node.text()
    };

    let desc = desc.trim();

    if !desc.is_empty() {
        Some(desc.to_string())
    } else {
        None
    }
}

fn parse_related(node: Node, series: &mut Series) -> Option<()> {
    if node.text().trim() == "N/A" {
        return None;
    }

    let mut children = node.children();

    loop {
        let link = children.next()?;
        let extra = children.next()?.text();

        let id = get_id(link.attr("href"))?;

        series
            .related_series
            .push((SeriesId(id), link.text(), extra));

        children.next();
    }
}

fn parse_associated_names(node: Node, series: &mut Series) {
    if node.text().trim() == "N/A" {
        return;
    }

    for child in node.find(Text) {
        let name = child.text();
        let name = name.trim();

        if !name.is_empty() && name != "N/A" {
            series.associated_names.push(name.to_owned());
        }
    }
}

fn parse_scanlating(node: Node, series: &mut Series) -> Option<()> {
    if node.text().trim() == "N/A" {
        return None;
    }

    for group in node.find(Attr("title", "Group Info")) {
        let id = get_id(group.attr("href"))?;
        let name = group.text();

        series.groups_scanlating.push((GroupId(id), name));
    }

    Some(())
}

fn parse_status_in_coo(node: Node, series: &mut Series) {
    if node.text() == "N/A" {
        return;
    }

    for node in node.find(Text) {
        series.status_in_coo.push(node.text());
    }
}

fn parse_anime_start_end(node: Node, series: &mut Series) {
    if node.text() == "N/A" {
        return;
    }

    for node in node.find(Text) {
        series.anime_start_end.push(node.text());
    }
}

fn parse_image(node: Node) -> Option<String> {
    let img = node.find(Name("img")).next()?;

    Some(img.attr("src")?.to_string())
}

fn parse_genre(node: Node, series: &mut Series) {
    series.genre = node.find(Name("a")).map(|a| a.text()).collect();

    series.genre.pop();
}

fn parse_categories(client: &MuClient, node: Node, series: &mut Series) -> Option<()> {
    if node.text().trim() == "N/A" {
        return None;
    }

    let doc = client.get(&format!(
        "https://www.mangaupdates.com/ajax/show_categories.php?s={}&type=1",
        series.id.0
    ));

    for node in doc.find(Name("a")) {
        let text = node.text();

        if text.trim().is_empty() || node.attr("href")?.starts_with("javascript") {
            continue;
        }
        series.categories.push(node.text());
    }

    Some(())
}

fn parse_category_recommendations(node: Node, series: &mut Series) -> Option<()> {
    if node.text().trim().starts_with("N/A") {
        return None;
    }

    for node in node.find(Name("a")) {
        let id = get_id(node.attr("href"))?;
        series
            .category_recommendations
            .push((SeriesId(id), node.text()));
    }

    Some(())
}

fn parse_recommendations(node: Node, series: &mut Series) -> Option<()> {
    if node.text().trim() == "N/A" {
        return None;
    }

    let node = if let Some(more) = node.find(Attr("id", "div_recom_more")).next() {
        more
    } else {
        node
    };

    for node in node.find(Name("a")) {
        if node.attr("href")?.starts_with("javascript") {
            continue;
        }

        let name = node.text();
        let id = get_id(node.attr("href"))?;

        series.recommendations.push((SeriesId(id), name));
    }

    Some(())
}

fn parse_authors(node: Node, series: &mut Series) -> Option<()> {
    if node.text().trim() == "N/A" {
        return None;
    }

    for node in node.find(Name("a")) {
        let id = get_id(node.attr("href"))?;
        let name = node.text();

        series.authors.push((AuthorId(id), name));
    }

    Some(())
}

fn parse_artists(node: Node, series: &mut Series) -> Option<()> {
    if node.text().trim() == "N/A" {
        return None;
    }

    for node in node.find(Name("a")) {
        let id = get_id(node.attr("href"))?;
        let name = node.text();

        series.artists.push((AuthorId(id), name));
    }

    Some(())
}

fn parse_year(node: Node) -> Option<String> {
    if node.text().trim() == "N/A" {
        return None;
    }

    Some(node.text().trim().to_string())
}

fn parse_original_publisher(node: Node) -> Option<(PublisherId, String)> {
    if node.text().trim() == "N/A" {
        return None;
    }

    let node = node.find(Name("a")).next()?;

    let name = node.text();
    let id = get_id(node.attr("href"))?;

    Some((PublisherId(id), name))
}

fn parse_serialized_in(node: Node, series: &mut Series) -> Option<()> {
    if node.text().trim() == "N/A" {
        return None;
    }

    let mut children = node.children();

    loop {
        let name = children.next()?.text();
        let extra = children.next()?.text();

        series.serialized_in.push((name, extra.trim().to_string()));

        children.next();
    }
}

fn parse_licensed(node: Node) -> bool {
    node.text().trim() == "Yes"
}

fn parse_english_publisher(node: Node, series: &mut Series) -> Option<()> {
    if node.text().trim() == "N/A" {
        return None;
    }

    let mut children = node.children();

    #[allow(clippy::clippy::while_let_loop)]
    loop {
        let link = children.next()?;
        let extra = children.next()?.text();

        let id = get_id(link.attr("href"))?;

        series.english_publisher.push((
            PublisherId(id),
            link.text(),
            if extra.trim().is_empty() {
                None
            } else {
                Some(extra.trim().to_string())
            },
        ));

        children.next();
    }
}
