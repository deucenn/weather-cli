use serde::Deserialize;
//TODO: build api call
// build city to coordinates api call from meteo
#[derive(Deserialize, Debug)]
struct Location {
    name: String,
    #[serde(alias = "latitude")]
    lat: f64,
    #[serde(alias = "longitude")]
    lon: f64,
}

#[derive(Deserialize, Debug)]
struct GeoApiAnswer {
    results: Option<Vec<Location>>,
}

pub async fn get_coordinates(city: String, country: String) -> Result<(), reqwest::Error> {
    let url_city = city.replace(' ', "%20");
    let res: GeoApiAnswer = reqwest::get(format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={url_city}"
    ))
    .await?
    .json()
    .await?;

    if let Some(ref locations) = res.results {
        if let Some(first_hit) = locations.first() {
            println!(
                "city: {}, lat: {}, lon: {}",
                first_hit.name, first_hit.lat, first_hit.lon
            );
        } else {
            println!("cant find city: {}", city);
        }
    } else {
        println!("cant find city: {}", city);
    }

    //let body = res.text().await?;

    println!("response: {:?}", res);

    Ok(())
}
