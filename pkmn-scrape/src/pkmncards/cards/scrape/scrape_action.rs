use crate::pkmncards::Client;
use pkmn_core::clean::{clean_damage, display_to_name, energy_type};
use pkmn_schema::cards::action::{Ability, Action, Attack, Text};
use pkmn_schema::cards::meta::EnergyType;
use scraper::{ElementRef, Node};
use web_scrape::scrape::{ScrapeError, Scraper};

impl Client {
    //! Scrape: Action

    /// Scrapes & cleans the action paragraphs. (abilities, attacks & rule texts)
    pub(crate) fn scrape_actions(&self, article: Scraper) -> Result<Vec<Action>, ScrapeError> {
        article.all("div.text p", |p| self.scrape_action(p))
    }

    /// Scrapes & cleans one paragraph `p` into an [Action].
    ///
    /// - Abilities: `<a href="/has/...">kind</a> ⇢ name<br/>text`
    /// - Attacks: `<abbr title="cost">..</abbr> → <span>name</span> : damage<br/>text`
    /// - Rule texts: plain content. (energy symbol `abbr` elements read as `{P}`)
    fn scrape_action(&self, p: Scraper) -> Result<Action, ScrapeError> {
        let mut cost: Vec<String> = Vec::new();
        let mut kind: Option<String> = None;
        let mut name: Option<String> = None;
        let mut cost_text: String = String::new();
        let mut pre: String = String::new();
        let mut post: String = String::new();
        let mut body: String = String::new();
        let mut in_body: bool = false;
        for child in p.element().children() {
            match child.value() {
                Node::Element(element) if element.name() == "br" => {
                    if in_body {
                        body.push('\n');
                    } else {
                        in_body = true;
                    }
                }
                Node::Element(element) => {
                    let text: String = ElementRef::wrap(child)
                        .map(|e| e.text().collect::<String>())
                        .unwrap_or_default();
                    if in_body {
                        body.push_str(text.as_str());
                    } else if element.name() == "abbr"
                        && kind.is_none()
                        && name.is_none()
                        && pre.trim().is_empty()
                    {
                        if let Some(title) = element.attr("title") {
                            cost.push(title.to_string());
                            cost_text.push_str(text.as_str());
                        }
                    } else if element.name() == "a"
                        && element
                            .attr("href")
                            .is_some_and(|href| href.contains("/has/"))
                        && kind.is_none()
                        && name.is_none()
                        && cost.is_empty()
                        && pre.trim().is_empty()
                    {
                        kind = Some(text.trim().to_string());
                    } else if element.name() == "span" && kind.is_none() && name.is_none() {
                        name = Some(text.trim().to_string());
                    } else if name.is_some() {
                        post.push_str(text.as_str());
                    } else {
                        pre.push_str(text.as_str());
                    }
                }
                Node::Text(text) => {
                    if in_body {
                        body.push_str(text);
                    } else if name.is_some() {
                        post.push_str(text);
                    } else {
                        pre.push_str(text);
                    }
                }
                _ => {}
            }
        }
        let mut damage: Option<String> = None;
        let text: Option<String>;
        if kind.is_some() {
            // ability: the name is the pre-break text after the `⇢`
            let ability_name: &str = pre.trim().trim_start_matches('⇢').trim();
            if !ability_name.is_empty() {
                name = Some(ability_name.to_string());
            }
            text = Self::non_empty(body.trim());
        } else if name.is_some() {
            // attack: the damage is the post-name text after the final `:` and the name may
            // continue past its span (ex: `<span>C</span> :O.D.E.: Protect: 40`)
            let post: &str = post.trim();
            let (extension, found): (&str, &str) = match post.rsplit_once(':') {
                Some((extension, found)) => (extension, found.trim()),
                None => (post, ""),
            };
            if found.is_empty() || self.is_damage(found) {
                if !extension.trim().is_empty()
                    && let Some(current) = name.take()
                {
                    let joined: String = format!("{}{}", current, extension.trim_end());
                    name = Some(joined.trim().to_string());
                }
                damage = Self::non_empty(found);
                text = Self::non_empty(body.trim());
            } else {
                // same-line attack: the effect text follows the damage without a break and the
                // damage renders with unit words (ex: `: 20 damage damage. This attack does...`)
                let rest: &str = post.trim_start_matches(':').trim_start();
                let (found, mut effect): (&str, &str) = rest.split_once(' ').unwrap_or((rest, ""));
                for _ in 0..2 {
                    match effect.trim_start().split_once(' ') {
                        Some((word, after))
                            if word.eq_ignore_ascii_case("damage")
                                || word.eq_ignore_ascii_case("damage.") =>
                        {
                            effect = after;
                        }
                        _ => break,
                    }
                }
                damage = Some(found.to_string());
                text = Self::join(effect.trim(), body.trim());
            }
        } else {
            // rule text: leading symbols are part of the text, not an attack cost
            // (ex: `δ Rainbow Energy provides {C} Energy...`)
            cost.clear();
            let pre: String = format!("{cost_text}{pre}");
            text = Self::join(pre.trim(), body.trim());
        }
        self.clean_action(cost, kind, name, damage, text)
    }

    /// Builds an [Action] from the cleaned paragraph parts.
    fn clean_action(
        &self,
        cost: Vec<String>,
        kind: Option<String>,
        name: Option<String>,
        damage: Option<String>,
        text: Option<String>,
    ) -> Result<Action, ScrapeError> {
        if let Some(kind) = kind {
            let name: &str = name
                .as_deref()
                .ok_or_else(|| ScrapeError::Other(format!("ability '{kind}' missing a name")))?;
            let mut ability: Ability = Ability::default()
                .with_kind(display_to_name(kind.as_str()).map_err(ScrapeError::Other)?)
                .with_name(display_to_name(name).map_err(ScrapeError::Other)?);
            if let Some(text) = text {
                ability.set_text(text);
            }
            Ok(Action::Ability(ability))
        } else if name.is_some() || !cost.is_empty() {
            let name: &str = name
                .as_deref()
                .ok_or_else(|| ScrapeError::Other("attack missing a name".to_string()))?;
            let mut attack: Attack =
                Attack::default().with_name(display_to_name(name).map_err(ScrapeError::Other)?);
            // free attacks have an explicit empty cost (ex: `No Energy Cost` symbols)
            let cost: Vec<EnergyType> = cost
                .iter()
                .filter(|cost| cost.as_str() != "No Energy Cost")
                .map(|cost| energy_type(cost.as_str()))
                .collect::<Result<_, String>>()
                .map_err(ScrapeError::Other)?;
            attack.set_cost(cost);
            if let Some(damage) = damage {
                attack.set_damage(clean_damage(damage.as_str()).map_err(ScrapeError::Other)?);
            }
            if let Some(text) = text {
                attack.set_text(text);
            }
            Ok(Action::Attack(attack))
        } else {
            let text: String =
                text.ok_or_else(|| ScrapeError::Other("empty text paragraph".to_string()))?;
            Ok(Action::Text(Text::default().with_text(text)))
        }
    }

    /// Wraps the trimmed `text` as `Some` when it is not empty.
    #[must_use]
    fn non_empty(text: &str) -> Option<String> {
        if text.is_empty() {
            None
        } else {
            Some(text.to_string())
        }
    }

    /// Joins the `head` and `tail` texts with a newline, dropping empties.
    #[must_use]
    fn join(head: &str, tail: &str) -> Option<String> {
        match (head.is_empty(), tail.is_empty()) {
            (false, false) => Some(format!("{head}\n{tail}")),
            (false, true) => Some(head.to_string()),
            (true, false) => Some(tail.to_string()),
            (true, true) => None,
        }
    }

    /// Whether the `damage` is a valid damage value. (ex: `30`, `30+`, `30×`, `?`)
    #[must_use]
    fn is_damage(&self, damage: &str) -> bool {
        let scalar: &str = damage.strip_suffix(['+', '-', '×', '?']).unwrap_or(damage);
        !damage.is_empty() && scalar.chars().all(|c| c.is_ascii_digit())
    }
}
