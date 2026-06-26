use crate::cards::copy_images;
use crate::cards::remove_set;
use crate::cards::rename_set;
use crate::cards::scrape_series;
use crate::cards::scrape_set;
use crate::cards::scrape_unknown_sets;
use crate::cards::validate;
use crate::cards::validate_images;
use clap::{Parser, Subcommand};
use std::error::Error;

/// The `pkmn.com` command-line interface.
#[derive(Parser)]
#[command(name = "pkmn", about = "The pkmn.com command-line interface.")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Renames a set's id, recording the mapping for scraping.
    RenameSet {
        /// The current set id.
        old_id: String,
        /// The new set id.
        new_id: String,
    },

    /// Removes a set and ignores its source for scraping.
    RemoveSet {
        /// The set id to remove.
        set_id: String,
    },

    /// Copies a source set's logo and symbol images to a target set in R2.
    CopyImages {
        /// The set id to copy images from.
        source_id: String,
        /// The set id to copy images to.
        target_id: String,
        /// Overwrite the target images if they already exist.
        #[arg(long)]
        overwrite: bool,
    },

    /// Scrapes & writes all cards for a set from a source. (currently only `pkmncards`)
    ScrapeSet {
        /// The data source. (currently only `pkmncards`)
        source: String,
        /// The set id to scrape.
        set_id: String,
    },

    /// Scrapes & writes all cards for every set in a series from a source.
    ScrapeSeries {
        /// The data source. (currently only `pkmncards`)
        source: String,
        /// The series id to scrape.
        series_id: String,
    },

    /// Scrapes the sets unknown to our local data from both sources.
    ScrapeUnknownSets,

    /// Validates the set data (unique ids across contexts).
    Validate,

    /// Prints sets missing a logo or symbol image in R2.
    ValidateImages,
}

impl Cli {
    //! Run

    /// Runs the parsed command.
    pub fn run(self) -> Result<(), Box<dyn Error>> {
        match self.command {
            Command::RenameSet { old_id, new_id } => rename_set(&old_id, &new_id),
            Command::RemoveSet { set_id } => remove_set(&set_id),
            Command::CopyImages {
                source_id,
                target_id,
                overwrite,
            } => copy_images(&source_id, &target_id, overwrite),
            Command::ScrapeSet { source, set_id } => scrape_set(&source, &set_id),
            Command::ScrapeSeries { source, series_id } => scrape_series(&source, &series_id),
            Command::ScrapeUnknownSets => scrape_unknown_sets(),
            Command::Validate => validate(),
            Command::ValidateImages => validate_images(),
        }
    }
}
