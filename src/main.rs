use mu_rs::MuClient;
use mu_rs::SeriesId;

fn main() {
    let client = MuClient::new();

    let s = client.search("Naruto");
    // let s = client.series(SeriesId(85181)).unwrap();

    println!("{:#?}", s);
}
