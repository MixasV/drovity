use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select};
use console::{style, Term};

pub async fn show_accounts_menu() -> Result<()> {
    let term = Term::stdout();
    
    loop {
        term.clear_screen()?;
        
        println!("{}", style("========================================").cyan());
        println!("{}", style("       Account Management").cyan().bold());
        println!("{}", style("========================================").cyan());
        println!();

        // Load and display accounts
        let accounts = crate::config::account::list_accounts()?;
        
        if !accounts.is_empty() {
            println!("{}", style("Current Accounts:").yellow());
            for (i, account) in accounts.iter().enumerate() {
                let status = if account.disabled {
                    style("[DISABLED]").red()
                } else {
                    style("[ACTIVE]").green()
                };
                println!("  {}. {} - {}", i + 1, account.email, status);
            }
            println!();
        } else {
            println!("{}", style("No accounts added yet.").yellow());
            println!();
        }

        let choices = vec![
            "1. Add New Account",
            "2. Remove Account",
            "3. Back to Main Menu",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option")
            .items(&choices)
            .default(0)
            .interact_on(&term)?;

        match selection {
            0 => {
                // Add new account
                add_account_oauth().await?;
            }
            1 => {
                // Remove account
                if !accounts.is_empty() {
                    remove_account(&accounts).await?;
                } else {
                    println!("{}", style("No accounts to remove!").red());
                    term.read_key()?;
                }
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

async fn add_account_oauth() -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", style("========================================").cyan());
    println!("{}", style("        Add Google Account").cyan().bold());
    println!("{}", style("========================================").cyan());
    println!();

    // Use manual callback method (works on servers)
    match crate::oauth::authorize_with_manual_callback().await {
        Ok(account) => {
            println!();
            println!("{}", style("╔══════════════════════════════════════════════════════════════╗").green());
            println!("{}", style("║              ✅ Account Added Successfully!                  ║").green().bold());
            println!("{}", style("╚══════════════════════════════════════════════════════════════╝").green());
            println!();
            println!("📧 Email: {}", style(&account.email).cyan().bold());
            if let Some(name) = &account.display_name {
                println!("👤 Name: {}", style(name).cyan());
            }
            println!();
            println!("{}", style("Press any key to continue...").dim());
            term.read_key()?;
        }
        Err(e) => {
            println!();
            println!("{}", style("╔══════════════════════════════════════════════════════════════╗").red());
            println!("{}", style("║                   ❌ Authorization Failed                    ║").red().bold());
            println!("{}", style("╚══════════════════════════════════════════════════════════════╝").red());
            println!();
            println!("{}", style(format!("Error: {}", e)).red());
            println!();
            println!("{}", style("Press any key to continue...").dim());
            term.read_key()?;
        }
    }

    Ok(())
}

async fn remove_account(accounts: &[crate::config::account::Account]) -> Result<()> {
    let term = Term::stdout();
    
    let account_choices: Vec<String> = accounts
        .iter()
        .enumerate()
        .map(|(i, acc)| format!("{}. {}", i + 1, acc.email))
        .collect();
    
    let mut choices = account_choices.clone();
    choices.push("Cancel".to_string());

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select account to remove")
        .items(&choices)
        .default(0)
        .interact_on(&term)?;

    if selection < accounts.len() {
        let account = &accounts[selection];
        crate::config::account::delete_account(&account.id)?;
        
        println!();
        println!("{}", style(format!("[SUCCESS] Account {} removed", account.email)).green());
        println!();
        println!("{}", style("Press any key to continue...").dim());
        term.read_key()?;
    }

    Ok(())
}
