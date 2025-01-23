use crate::password_mngr::PasswordManager;

pub struct Menu {
    title: String,
    _depth: i32,
    options: Vec<MenuOption>,
}
pub struct MenuOption {
    index: i32,
    label: String,
}

impl Menu {
    pub fn new(title: &str, _depth: i32, options: Vec<MenuOption>) -> Self {
        /*let options = options
            .into_iter()
            .enumerate()
            .map(|(i, label)| MenuOption::new((i + 1) as i32, label))
            .collect();
        */
        Self {
            title: title.to_string(),
            _depth,
            options,
        }
    }


    pub fn display(&self, ) {
        println!("{}", self.title);
        for (i, option) in self.options.iter().enumerate() {
            println!("{}. {}", i + 1, option.label);
        }
    }
    
    pub fn get_choice(&self, password_manager: &mut PasswordManager) -> color_eyre::eyre::Result<bool> {
        if let Some(index) = self.read_choice() {
            // Find the MenuOption with the matching index
            if let Some(option) = self.options.iter().find(|opt| opt.index == index) {
                self.provide_choice_feedback(option);
                match option.index {
                    1 => {
                        password_manager.login()?;
                    }
                    2 => {
                        password_manager.signup()?;
                    }
                    3 => {
                        return Ok(false);
                    }
                    _ => {}
                }
                Ok(true) // Pass the MenuOption
            } else {
                println!("Invalid choice!");
                Ok(false)
            }
        } else {
            println!("Invalid input!");
            Ok(false)
        }
    }
    
    fn read_choice(&self) -> Option<i32> {
        use std::io::{self, Write};
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<i32>().ok()
    }

    fn provide_choice_feedback(&self, option: &MenuOption) {
        // Print the feedback using the selected option's details
        println!("You chose to navigate into:");
        option.display();
    }
}

impl MenuOption {
    pub fn new(index: i32, label: &str) -> Self {
        Self {
            index,
            label: label.to_string(),
        }
    }

    fn display(&self) {
        println!("{}. {}", self.index, self.label);
    }
    
}