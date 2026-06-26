use pkmn_schema::api::v0::cards::{
    CardQuery, CardService, GetCardRequest, GetCardResponse, GetCardsRequest, GetCardsResponse,
    GetSetsRequest, GetSetsResponse,
};
use pkmn_schema::cards::card::Card;
use pkmn_schema::cards::set::CardSet;
use pkmn_schema::core::api::{PageRequest, PageResponse};
use proto_packet::service::{ServiceError, ServiceErrorReason};

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

    async fn get_card(&self, request: GetCardRequest) -> Result<GetCardResponse, ServiceError> {
        let id: &str = request.id().unwrap_or_default();
        let card: &Card = pkmn_data::cards::cards_by_id()
            .get(id)
            .copied()
            .ok_or(ServiceError::new(ServiceErrorReason::NotFound))?;
        Ok(GetCardResponse::default().with_card(card.clone()))
    }

    async fn get_cards(&self, request: GetCardsRequest) -> Result<GetCardsResponse, ServiceError> {
        let cards: Vec<Card> = match request.query() {
            Some(CardQuery::SetId(set_id)) => pkmn_data::cards::cards_by_set()
                .get(set_id.as_str())
                .map(|cards| cards.iter().map(|card| (**card).clone()).collect())
                .unwrap_or_default(),
            Some(CardQuery::Search(_)) => {
                return Err(ServiceError::new(ServiceErrorReason::NotImplemented));
            }
            None => return Err(ServiceError::new(ServiceErrorReason::BadRequest)),
        };
        let (cards, paging): (Vec<Card>, PageResponse) = self.paginate(cards, request.paging());
        Ok(GetCardsResponse::default()
            .with_cards(cards)
            .with_paging(paging))
    }
}

impl Cards {
    //! Paging

    /// Applies the `request` paging to `cards`. (page is 0-based; an absent or zero page size
    /// returns every card)
    fn paginate(
        &self,
        cards: Vec<Card>,
        request: Option<PageRequest>,
    ) -> (Vec<Card>, PageResponse) {
        let total: u32 = cards.len() as u32;
        let page: u32 = request.map(|page| page.page()).unwrap_or(0);
        let page_size: u32 = request.map(|page| page.page_size()).unwrap_or(0);
        let cards: Vec<Card> = if page_size == 0 {
            cards
        } else {
            cards
                .into_iter()
                .skip((page * page_size) as usize)
                .take(page_size as usize)
                .collect()
        };
        (cards, PageResponse::new(page, page_size, total))
    }
}
