use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select};
use console::{style, Term};

use super::{accounts, proxy_menu, factory_setup};

pub async fn show_main_menu() -> Result<()> {
    let term = Term::stdout();
    
    loop {
        term.clear_screen()?;
        
        // Print header
        println!("{}", style("========================================").cyan());
        println!("{}", style("        DROVITY - Main Menu").cyan().bold());
        println!("{}", style("  Gemini API Proxy for Factory Droid").cyan());
        println!("{}", style("========================================").cyan());
        println!();
        println!("{}", style("Author: @onexv (https://t.me/onexv)").dim());
        println!();

        let choices = vec![
            "1. Accounts",
            "2. API Proxy",
            "3. Droid Settings Setup",
            "4. Hide Mode",
            "5. Exit",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option")
            .items(&choices)
            .default(0)
            .interact_on(&term)?;

        match selection {
            0 => {
                // Accounts menu
                accounts::show_accounts_menu().await?;
            }
            1 => {
                // API Proxy menu
                proxy_menu::show_proxy_menu().await?;
            }
            2 => {
                // Factory Droid setup
                factory_setup::show_factory_setup().await?;
            }
            3 => {
                // Hide mode - start daemon and exit
                term.clear_screen()?;
                println!("{}", style("Starting proxy in background...").green());
                crate::daemon::start_background(false).await?;
                break;
            }
            4 => {
                // Exit
                term.clear_screen()?;
                println!("{}", style("Goodbye!").green());
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
