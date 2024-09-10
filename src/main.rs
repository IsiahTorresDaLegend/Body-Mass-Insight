/*

main.rs
Body Mass Insight
Written by Isiah Torres

*/

// Add libraries to use
use std::io; 

#[derive(Debug)] // syntax to add the Debug trait for this struct
struct Person { // Created custom object Person with attributes with struct. 
    name: String,
    age: u8,
    gender: Gender, // all enum types must also have the Debug trait.
    height_unit: HeightUnit,
    weight_unit: WeightUnit,
    height: f64,
    weight: f64,
}

#[derive(Debug)] 
enum Gender { // Created new data types with enum to make code clearer.
    Male,
    Female,
}

#[derive(Debug)]
enum WeightUnit { // Will be used to allow user to choose their preferred unit. 
    Kilograms,
    Pounds,
}

#[derive(Debug)]
enum HeightUnit { // Will be used to allow user to choose their preferred unit.
    Centimeters,
    Inches,
}

#[derive(Debug)]
enum BmiCategory { // Will be used to display what category the user's BMI falls on.
    Underweight,
    Healthyweight,
    Overweight,
    Obese, 
}

impl Person { // impl statement to create methods for Person types. 
    fn calculate_bmi(&self) -> f64 { // this function will calculate the BMI. &self is the object. 
        // must convert the height to meters for accuracy. 
        let height_m = match self.height_unit { // use match statement to proceed based on input.
            // If height was entered as centimeters, must divide by 100.0 to convert to meters.
            HeightUnit::Centimeters => self.height / 100.0,
            // If height was entered as inches, must multiply by 0.0254 to convert to inches.
            HeightUnit::Inches => self.height * 0.0254,
        };
        // must convert weight to kilograms for accuracy.
        let weight_kg = match self.weight_unit {
            // no conversion needed if entered as kilograms.
            WeightUnit::Kilograms => self.weight,
            // if weight was entered as pounds, must multiply weight by 0.453592 to convert to kg.
            WeightUnit::Pounds => self.weight * 0.453592,
        };

        // BMI is weight in kg divded by the height in meters squared.
        weight_kg / (height_m * height_m)
    }

    // Determine what category the BMI falls in.
    fn get_bmi_category(&self) -> BmiCategory { // Takes a Person type. Returns a BmiCategory type.
        let bmi = self.calculate_bmi(); // The current Person objects calls calculate_bmi() method.
        if  bmi < 18.5 { // if bmi is less than 18.5, the person is Underweight
            BmiCategory::Underweight
        } else if bmi < 25.0 { // if bmi is less than 25 but not 18.5, the person is Healthyweight
            BmiCategory::Healthyweight
        } else if bmi < 30.0 { // if bmi is less than 30 but not 25, the person is Overweight
            BmiCategory::Overweight
        } else { // if bmi is over 30, the person is obese. 
            BmiCategory::Obese
        }
    }

}

impl Gender { // Create methods for Gender types
    fn set_gender() -> Self { // returns a Gender type
        loop { // loop so the program does not crash when receiving invalid input. 
            println!("\nEnter your gender: (Please enter male or female)");
            let mut gender_input = String::new();
            // read user input
            io::stdin().read_line(&mut gender_input).expect("Failed to read line.");
            // always trim() to ensure buffer is cleared of any extra newlines. used to_lowercase()
            // to make the string easier to compare. used as_str() to convert String to str. 
            let gender = match gender_input.trim().to_lowercase().as_str() {
                "male" => Gender::Male, // if user typed 'male' gender = Gender::Male
                "female" => Gender::Female, // if user typed 'female' gender = Gender::female
                _ => { // if user enters anything other than the two options above...
                    println!("Invalid input. Please choose 'male' or 'female'.");
                    continue; // loop again
                }
            };
            // if successful, break loop and return the value of 'gender'
            break gender;
        }
        
    }
}

// allow user to choose height units.
fn choose_height_unit() -> HeightUnit { // return HeightUnit type
    loop {
        println!("\nChoose height unit:");
        println!("1. Centimeters");
        println!("2. Inches");

        // these variables cease to exist out of scope unless returned. 
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line.");
        let height_unit = match choice.trim() {
            "1" => HeightUnit::Centimeters, // assign the HeightUnit type according to users choice
            "2" => HeightUnit::Inches,
            _ => { // otherwise loop again if input is invalid.
                println!("Invalid input. Please enter '1' or '2'.");
                continue;
            }
        };
        // break loop if successful and return the height_unit value. 
        break height_unit;
    }
    
}

// allow user to choose weight units. 
fn choose_weight_unti() -> WeightUnit { // return WeightUnit type.
    loop { // works just like choose_height_unit()
        println!("\nChoose weight unit:");
        println!("1. Kilograms");
        println!("2. Pounds");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line.");
        let weight_unit = match choice.trim() {
            "1" => WeightUnit::Kilograms,
            "2" => WeightUnit::Pounds,
            _ => {
                println!("Invalid_input. Please enter '1' or '2'.");
                continue;
            }
        };

        break weight_unit;
    }
    
}

// get the user's height from user input.
fn get_height() -> f64 { // takes a immutable reference to HeightUnit type
    println!("\nEnter your height: (Please include a decimal, Ex: 100.0)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    // used parse() to convert String to 64-bit floating point.
    let height: f64 = input.trim().parse().expect("Please enter a valid number.");
    height // return the value of height.

}

fn get_weight() -> f64 {
    println!("\nEnter your weight: (Please include a decimal, Ex: 100.0)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let weight: f64 = input.trim().parse().expect("Please enter a valid number.");
    weight //return the value of weight
}

fn print_description(){ // just provide information about BMI and the program to the user.
    println!("This is a program that calculates an adult's Body Mass Index (BMI) and displays"); 
    println!("the category their BMI falls in based on the following ranges provided by the");
    println!("Center for Disease Control and Prevention (CDC) and the National Institues of");
    println!("Health (NIH):");
    println!("\nUnderweight: Less than 18.5");
    println!("Normal: 18.5 - 24.9");
    println!("Overweight: 25.0 - 29.9");
    println!("Obese: 30.0 or greater");
    println!("\nAs stated by the CDC, at an individual level, BMI can be used as a screening");
    println!("tool, but is not diagnostic of body fatness or health of an individual. For");
    println!("instance, an indiviual who falls in the overweight range could be someone who");
    println!("participates in training and exercise which allows them to acquire more muscle");
    println!("mass, meaning that individual could be considered healthier than someone who falls");
    println!("within the healthy weight range. A trained healthcare provider should perform");
    println!("appropriate health assessments in order to evaluate an individual's health status");
    println!("and risks.")
}

fn main() {

    // Create a variable to store ASCII Art for the program's title.
    let title = r"
 ______              _           ______                        _____               _         _           
(____  \            | |         |  ___ \                      (_____)             (_)       | |     _    
 ____)  )  ___    _ | | _   _   | | _ | |  ____   ___   ___      _    ____    ___  _   ____ | | _  | |_  
|  __  (  / _ \  / || || | | |  | || || | / _  | /___) /___)    | |  |  _ \  /___)| | / _  || || \ |  _) 
| |__)  )| |_| |( (_| || |_| |  | || || |( ( | ||___ ||___ |   _| |_ | | | ||___ || |( ( | || | | || |__ 
|______/  \___/  \____| \__  |  |_||_||_| \_||_|(___/ (___/   (_____)|_| |_|(___/ |_| \_|| ||_| |_| \___)
                       (____/                                                        (_____|             
    ";
    
    // format the variable so we can add ANSI escape codes to make the text bold and green. 
    let styled_title = format!("\x1B[1;92m{}\x1B[0m", title);

    // print the styled title. 
    println!("{}", styled_title);
    
    // Display the program's description.
    print_description();
   
    // Call these functions so user can set their weight and height.
    let weight_unit = choose_weight_unti();
    let height_unit = choose_height_unit();

    // Get the user's name.
    println!("\nEnter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string(); // convert str to owned String.
    
    // Get the user's age.
    println!("\nEnter your age:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read line.");
    // use parse() to convert String to unsigned 8bit integer.
    let age: u8 = age_input.trim().parse().expect("Please enter a valid number.");

    // Get the user's gender by creating a Gender type variable and assign it with the method call.
    let gender = Gender::set_gender(); 

    // Call these functions to get the users height and weight.
    let height = get_height();
    let weight = get_weight();
    
    // Initialize a Person object and assign its values based on the variables we created. 
    let person = Person {
        name,
        age,
        gender,
        weight_unit,
        height_unit,
        height,
        weight,
    };

    // Calculate user's BMI and determine the BMI category based on the person's attributes.
    let bmi_category = person.get_bmi_category();

    // Display results to user. 
    println!("\nDisplaying {}'s results:", person.name);
    // {:.2} means display the value up to two decimal points. 
    println!("Your BMI is: {:.2}", person.calculate_bmi());
    println!("Age: {}", person.age);
    // {:?} is the debug operator, means dump whatever we have here. Necessary for the new types.
    println!("Gender: {:?}", person.gender);
    println!("BMI Category: {:?}", bmi_category);

}
