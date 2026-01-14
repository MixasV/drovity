use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select};
use console::{style, Term};

pub async fn show_proxy_menu() -> Result<()> {
    let term = Term::stdout();
    
    loop {
        term.clear_screen()?;
        
        println!("{}", style("========================================").cyan());
        println!("{}", style("          API Proxy Status").cyan().bold());
        println!("{}", style("========================================").cyan());
        println!();

        // Check proxy status
        let is_running = crate::daemon::is_running().await?;
        let config = crate::config::load_config()?;
        
        if is_running {
            println!("{} {}", style("Status:").yellow(), style("Running").green().bold());
            println!("{} {}", style("Port:").yellow(), style(config.proxy.port).cyan());
            println!("{} {}", style("Address:").yellow(), style(format!("http://127.0.0.1:{}", config.proxy.port)).cyan());
            println!("{} {}", style("API Key:").yellow(), style(&config.proxy.api_key).dim());
        } else {
            println!("{} {}", style("Status:").yellow(), style("Stopped").red().bold());
            println!("{} {}", style("Port:").yellow().dim(), style(config.proxy.port).dim());
        }
        println!();

        let choices = if is_running {
            vec!["1. Stop Proxy", "2. Restart Proxy", "3. Back to Main Menu"]
        } else {
            vec!["1. Start Proxy", "2. Back to Main Menu"]
        };

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option")
            .items(&choices)
            .default(0)
            .interact_on(&term)?;

        if is_running {
            match selection {
                0 => {
                    // Stop
                    println!();
                    println!("{}", style("Stopping proxy...").yellow());
                    crate::daemon::stop().await?;
                    println!("{}", style("[SUCCESS] Proxy stopped").green());
                    println!();
                    println!("{}", style("Press any key to continue...").dim());
                    term.read_key()?;
                }
                1 => {
                    // Restart
                    println!();
                    println!("{}", style("Restarting proxy...").yellow());
                    crate::daemon::stop().await?;
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    crate::daemon::start_background(false).await?;
                    println!("{}", style("[SUCCESS] Proxy restarted").green());
                    println!();
                    println!("{}", style("Press any key to continue...").dim());
                    term.read_key()?;
                }
                2 => {
                    // Back
                    break;
                }
                _ => {}
            }
        } else {
            match selection {
                0 => {
                    // Start
                    println!();
                    println!("{}", style("Starting proxy...").yellow());
                    crate::daemon::start_background(false).await?;
                    println!("{}", style("[SUCCESS] Proxy started").green());
                    println!();
                    println!("{}", style("Press any key to continue...").dim());
                    term.read_key()?;
                }
                1 => {
                    // Back
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
