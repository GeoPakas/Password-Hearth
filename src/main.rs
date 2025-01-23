
mod menu;
mod password_mngr;
mod database;

use menu::Menu;
use menu::MenuOption;

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    database::init_db()?;
    let mut manager = password_mngr::PasswordManager::new()?;

    let main_menu = Menu::new("Main menu", 0, vec![
        MenuOption::new(1, "Log in"),
        MenuOption::new(2, "Sign up"),
        MenuOption::new(3, "Exit"),
        ]);
    loop {
        main_menu.display();
        let continue_menu = main_menu.get_choice(&mut manager)?;
        if !continue_menu {
            break;
        }
    }
    Ok(())
}
