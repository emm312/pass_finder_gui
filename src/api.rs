use rayon::prelude::*;

const ROOT_URL: &str = "https://api.n2yo.com/rest/v1/";

pub struct N2YOApi {
    api_key: String,
    batched_reqs: Vec<String>,
}

impl N2YOApi {
    pub fn new(api_key: String) -> N2YOApi {
        N2YOApi {
            api_key,
            batched_reqs: Vec::new(),
        }
    }

    pub fn get_radiopasses(
        &mut self,
        id: usize,
        lat: f64,
        long: f64,
        min_elevation: usize,
        days: usize,
    ) {
        self.batched_reqs.push(
            ROOT_URL.to_owned()
                + &format!(
                    "satellite/radiopasses/{id}/{lat}/{long}/0/{days}/{min_elevation}/&apiKey={}",
                    self.api_key
                ),
        )
    }

    pub fn dispatch_reqs(&mut self) -> Vec<RadioPasses> {
        let ret = self.batched_reqs
            .par_iter()
            .map(|elem| {
                serde_json::from_str(&reqwest::blocking::get(elem).unwrap().text().unwrap())
                    .unwrap()
            })
            .collect();
        self.batched_reqs = Vec::new();
        ret
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
    #[serde(alias = "startUTC")]
    pub start_utc: usize,
    #[serde(alias = "endUTC")]
    pub end_utc: usize,
    #[serde(alias = "maxEl")]
    pub max_el: f64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RadioPassInfo {
    pub satname: String,
}
