fn main() {
    //mise en place de la boucle sauf si exit
    loop {
        println!("bienvenue dans la calculatrice !");
        //connaitre l'opération arithmétique
        println!("veuillez choisir une opération : +, -, *, / ou quit (pour quitter)");
        let mut operation = String::new();
        std::io::stdin().read_line(&mut operation).expect("erreur lors de la saisie");
        let operation = operation.trim();
        if operation == "quit"{
            break;
        }
        let valid_operation = ["+", "-", "*", "/"];
        let mut is_valid_operation = false;

        for &valid_op in &valid_operation{
            if operation == valid_op {
                is_valid_operation = true;
            }
        }
        if !is_valid_operation {
            println!("operation non reconnu");
            continue
        }

        //connaitre le premier nombre choisi
        println!("entrez le premier nombre : ");
        let num1 = read_number();
        println!("entrez le second nombre : ");
        let num2 = read_number();

        let result: f64;

        if operation == "+" {
            result = num1 + num2;
        }
        else if operation == "-"{
            result = num1 - num2;
        }
        else if operation == "*" {
            result = num1 * num2;
        }
        else{
            result = num1 / num2;
        }

        println!("Result : {num1} {operation} {num2} = {result}");
    }
}

fn read_number() -> f64 {
    let mut res = 0.0;
    while res == 0.0 {let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("erreur lors de la saisie");
        match input.trim().parse::<f64>(){
            Ok(value) => {
                res = value;
            }
            Err(_) => {
                println!("ce n'est pas un nombre valide");
            }

        }
    }
    res
}