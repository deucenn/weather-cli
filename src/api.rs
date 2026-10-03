//TODO: build api call
// build city to coordinates api call from meteo

pub async fn get_coordinates(city: String, country: String) -> Result<(), reqwest::Error> {
    let url_city = city.replace(' ', "%20");
    let res = reqwest::get(format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={url_city}"
    ))
    .await?;
    let body = res.text().await?;

    println!("response: {:?}", body);
    Ok(())
}
