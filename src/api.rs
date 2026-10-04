use serde::Deserialize;
//TODO: build api call
// build city to coordinates api call from meteo
#[derive(Deserialize, Debug)]
pub struct Location {
    name: String,
    #[serde(alias = "latitude")]
    lat: f64,
    #[serde(alias = "longitude")]
    lon: f64,
    country: Option<String>,
}

#[derive(Deserialize, Debug)]
struct GeoApiAnswer {
    results: Option<Vec<Location>>,
}

pub async fn get_coordinates(
    city: String,
    country: String,
) -> Result<Option<Location>, reqwest::Error> {
    let url_city = city.replace(' ', "%20");
    let res: GeoApiAnswer = reqwest::get(format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={url_city}"
    ))
    .await?
    .json()
    .await?;

    if let Some(ref locations) = res.results {
        //if let Some(first_hit) = locations.first() {
        if let Some(first_hit) = locations.iter().find(|c| {
            let name_match = c.name.eq_ignore_ascii_case(&city);

            let country_match = c
                .country
                .as_ref()
                .map_or(true, |c_country| c_country.eq_ignore_ascii_case(&country));

            name_match && country_match
        }) {
            println!(
                "city: {}, lat: {}, lon: {}, country: {}",
                first_hit.name,
                first_hit.lat,
                first_hit.lon,
                first_hit.country.as_ref().map_or("unknown", |v| v)
            );
        } else {
            println!("cant find city: {} in {}", city, country);
        }
    } else {
        println!("cant find city: {}", city);
    }

    //let body = res.text().await?;

    println!("response: {:?}", res);

    let coords = res
        .results
        .as_ref()
        .and_then(|locations| {
            locations.iter().find(|c| {
                let name_match = c.name.eq_ignore_ascii_case(&city);

                let country_match = c
                    .country
                    .as_ref()
                    .map_or(true, |c_country| c_country.eq_ignore_ascii_case(&country));

                name_match && country_match
            })
        })
        .map(|loc| Location {
            name: loc.name.clone(),
            lat: loc.lat,
            lon: loc.lon,
            country: loc.country.clone(),
        });

    println!("coords: {:?}", coords);

    Ok(coords)
}
