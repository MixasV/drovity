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
    println!("{}", style("🔍 Searching for Factory Droid settings...").yellow());
    
    let (settings_path, created) = find_or_create_factory_settings()?;
    
    match settings_path {
        Some(path) => {
            if created {
                println!("{}", style("✨ Created new settings.json").green());
            } else {
                println!("{}", style(format!("✅ Found: {}", path.display())).green());
            }
            println!();
            println!("{}", style("⚙️  Configuring drovity models...").yellow());
            
            crate::factory::auto_configure(&path).await?;
            
            println!();
            println!("{}", style("╔══════════════════════════════════════════════════════════════╗").green());
            println!("{}", style("║         ✅ Factory Droid Configured Successfully!            ║").green().bold());
            println!("{}", style("╚══════════════════════════════════════════════════════════════╝").green());
            println!();
            println!("{}", style("📝 Settings file:").cyan());
            println!("   {}", style(path.display()).cyan().bold());
            println!();
            println!("{}", style("🚀 Next steps:").yellow().bold());
            println!("   1. Start drovity proxy: {}", style("drovity start").cyan().bold());
            println!("   2. In Factory Droid CLI, type: {}", style("/model").cyan().bold());
            println!("   3. Select a model with {}", style("[drovity]").cyan().bold());
            println!();
            println!("{}", style("Press any key to continue...").dim());
        }
        None => {
            println!("{}", style("❌ Factory Droid directory not found").red().bold());
            println!();
            println!("{}", style("💡 Solutions:").yellow().bold());
            println!("   1. Install Factory Droid:");
            println!("      {}", style("curl -fsSL https://app.factory.ai/cli | sh").cyan());
            println!();
            println!("   2. Or use 'Manual Setup' to view and copy configuration");
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

/// Find Factory settings or create if .factory directory exists
fn find_or_create_factory_settings() -> Result<(Option<PathBuf>, bool)> {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return Ok((None, false)),
    };
    
    let factory_dir = home.join(".factory");
    let settings_path = factory_dir.join("settings.json");
    
    // If settings.json exists, return it
    if settings_path.exists() {
        return Ok((Some(settings_path), false));
    }
    
    // If .factory directory exists, create settings.json
    if factory_dir.exists() && factory_dir.is_dir() {
        // Create minimal settings.json structure
        let initial_settings = serde_json::json!({
            "customModels": []
        });
        
        let settings_content = serde_json::to_string_pretty(&initial_settings)?;
        std::fs::write(&settings_path, settings_content)
            .context("Failed to create settings.json")?;
        
        return Ok((Some(settings_path), true));
    }
    
    // Neither settings.json nor .factory directory exists
    Ok((None, false))
}
