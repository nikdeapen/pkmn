use pkmn_schema::api::v0::cards::{CardService, GetSetsRequest, GetSetsResponse};
use pkmn_schema::cards::set::CardSet;
use proto_packet::service::ServiceError;

/// The card service.
pub struct Cards;

impl CardService for Cards {
    async fn get_sets(&self, request: GetSetsRequest) -> Result<GetSetsResponse, ServiceError> {
        let sets: Vec<CardSet> = match request.context() {
            Some(context) => pkmn_data::cards::sets_by_context()
                .get(&context)
                .map(|sets| sets.iter().map(|set| (**set).clone()).collect())
                .unwrap_or_default(),
            None => pkmn_data::cards::sets().to_vec(),
        };
        Ok(GetSetsResponse::default().with_sets(sets))
    }
}
