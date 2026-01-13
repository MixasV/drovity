use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select, Input};
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

    // Generate OAuth URL
    let auth_url = crate::oauth::generate_auth_url()?;
    
    println!("{}", style("Step 1: Open this URL in your browser:").yellow().bold());
    println!();
    println!("{}", style(&auth_url).blue().underlined());
    println!();
    println!("{}", style("Step 2: After authorization, Google will show you a CODE.").yellow().bold());
    println!("{}", style("Copy that code and paste it below:").yellow().bold());
    println!();

    let code: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter authorization code")
        .interact_text()?;

    println!();
    println!("{}", style("Processing...").yellow());

    // Exchange code for tokens
    match crate::oauth::exchange_code_for_tokens(&code).await {
        Ok(account) => {
            println!();
            println!("{}", style("[SUCCESS] Account added successfully!").green().bold());
            println!("Email: {}", style(&account.email).cyan());
            println!();
            println!("{}", style("Press any key to continue...").dim());
            term.read_key()?;
        }
        Err(e) => {
            println!();
            println!("{}", style(format!("[ERROR] {}", e)).red().bold());
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
