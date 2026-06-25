use crate::cards::copy_images;
use crate::cards::remove_set;
use crate::cards::rename_set;
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
            Command::ScrapeUnknownSets => scrape_unknown_sets(),
            Command::Validate => validate(),
            Command::ValidateImages => validate_images(),
        }
    }
}
