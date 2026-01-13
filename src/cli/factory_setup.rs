use anyhow::{Result, Context};
use dialoguer::{theme::ColorfulTheme, Select};
use console::{style, Term};
use std::path::PathBuf;

pub async fn show_factory_setup() -> Result<()> {
    let term = Term::stdout();
    
    loop {
        term.clear_screen()?;
        
        println!("{}", style("========================================").cyan());
        println!("{}", style("    Factory Droid Settings Setup").cyan().bold());
        println!("{}", style("========================================").cyan());
        println!();

        let choices = vec![
            "1. Auto Config Droid Settings",
            "2. Manual Setup (Show Config)",
            "3. Back to Main Menu",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option")
            .items(&choices)
            .default(0)
            .interact_on(&term)?;

        match selection {
            0 => {
                // Auto config
                auto_configure_factory().await?;
                term.read_key()?;
            }
            1 => {
                // Manual setup
                show_manual_config().await?;
                term.read_key()?;
            }
            2 => {
                // Back
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

async fn auto_configure_factory() -> Result<()> {
    println!();
    println!("{}", style("Searching for Factory Droid settings...").yellow());
    
    let settings_path = find_factory_settings()?;
    
    match settings_path {
        Some(path) => {
            println!("{}", style(format!("[SUCCESS] Found: {}", path.display())).green());
            println!();
            println!("{}", style("Updating settings...").yellow());
            
            crate::factory::auto_configure(&path).await?;
            
            println!();
            println!("{}", style("[SUCCESS] Factory Droid settings updated successfully!").green().bold());
            println!();
            println!("{}", style("You can now use drovity models in Factory Droid:").cyan());
            println!("  - Type {} in Factory Droid CLI", style("/model").cyan().bold());
            println!("  - Select a model starting with {}", style("[drovity]").cyan().bold());
            println!();
            println!("{}", style("Press any key to continue...").dim());
        }
        None => {
            println!("{}", style("[ERROR] Could not find Factory Droid settings file.").red());
            println!();
            println!("{}", style("Expected location:").yellow());
            println!("  Linux/macOS: ~/.factory/settings.json");
            println!("  Windows: C:\\Users\\<USER>\\.factory\\settings.json");
            println!();
            println!("{}", style("Please use 'Manual Setup' to view and copy configuration.").yellow());
            println!();
            println!("{}", style("Press any key to continue...").dim());
        }
    }

    Ok(())
}

async fn show_manual_config() -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", style("========================================").cyan());
    println!("{}", style("       Manual Configuration").cyan().bold());
    println!("{}", style("========================================").cyan());
    println!();
    
    let config_json = crate::factory::generate_config_json()?;
    
    println!("{}", style("Copy the configuration below and add it to your Factory settings:").yellow().bold());
    println!();
    println!("{}", style("File location:").cyan());
    println!("  Linux/macOS: {}/.factory/settings.json", dirs::home_dir().unwrap().display());
    println!("  Windows: {}/.factory/settings.json", dirs::home_dir().unwrap().display());
    println!();
    println!("{}", style("─".repeat(60)).dim());
    println!("{}", style(&config_json).green());
    println!("{}", style("─".repeat(60)).dim());
    println!();
    println!("{}", style("Instructions:").yellow().bold());
    println!("1. Open your Factory Droid settings.json file");
    println!("2. Find the \"customModels\" array (or create it if missing)");
    println!("3. Add the models from above to the array");
    println!("4. Save the file");
    println!("5. Restart Factory Droid CLI");
    println!();
    println!("{}", style("Press any key to continue...").dim());

    Ok(())
}

fn find_factory_settings() -> Result<Option<PathBuf>> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    let settings_path = home.join(".factory").join("settings.json");
    
    if settings_path.exists() {
        Ok(Some(settings_path))
    } else {
        Ok(None)
    }
}
