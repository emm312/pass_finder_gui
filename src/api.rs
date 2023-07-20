const ROOT_URL: &str = "https://api.n2yo.com/rest/v1/";

pub struct N2YOApi {
    api_key: String,
}

impl N2YOApi {
    pub fn new(api_key: String) -> N2YOApi {
        N2YOApi { api_key }
    }

    pub fn get_radiopasses(
        &self,
        id: usize,
        lat: f64,
        long: f64,
        min_elevation: usize,
        days: usize,
    ) -> RadioPasses {
        let req = reqwest::blocking::get(
            ROOT_URL.to_owned()
                + &format!(
                    "satellite/radiopasses/{id}/{lat}/{long}/0/{days}/{min_elevation}/&apiKey={}",
                    self.api_key
                ),
        )
        .unwrap();
        let tct = req.text().unwrap();
        serde_json::from_str(&tct).unwrap()
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RadioPasses {
    pub info: RadioPassInfo,
    #[serde(default)]
    pub passes: Vec<RadioPass>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RadioPass {
    pub startUTC: usize,
    pub endUTC: usize,
    pub maxEl: f64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RadioPassInfo {
    pub satname: String,
}
