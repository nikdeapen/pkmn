use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct RawPage<T> {
    pub data: Vec<T>,
    pub page: usize,
    pub page_size: usize,
    pub count: usize,
    pub total_count: usize,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawSet {
    pub id: String,
    pub name: String,
    pub series: String,
    pub printed_total: usize,
    pub total: usize,
    pub legalities: RawLegalities,
    pub ptcgo_code: Option<String>,
    pub release_date: String,
    pub updated_at: String,
    pub images: RawSetImages,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawLegalities {
    pub unlimited: Option<String>,
    pub standard: Option<String>,
    pub expanded: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawSetImages {
    pub symbol: String,
    pub logo: String,
}
