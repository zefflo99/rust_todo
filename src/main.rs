use std::io;

fn main() {
    // Demander le nom de l'utilisateur
    println!("Quel est votre nom ?");
    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name).unwrap();
    let user_name = user_name.trim();

    let mut tasks: Vec<String> = Vec::new();
    let mut archived_tasks: Vec<String> = Vec::new(); // Nouvelle variable pour les tâches archivées

    // Affiche le message de début
    println!("Bonjour, {} ! ", user_name);

    // Variable pour gérer l'état de l'application
    let mut is_archived_view = false;

    // boucle principal
    loop {
        if is_archived_view {
            // Affiche la liste des tache archivé
            println!("Tâches archivées :");
            for (index, task) in archived_tasks.iter().enumerate() {
                println!("{}. {}", index + 1, task);
            }

            // j'affiche le menu
            println!("Appuyez sur 'Q' pour revenir à la vue de base");
        } else {
            // Affiche la liste des tache non archivé
            println!("Voici votre liste de tâches :");
            for (index, task) in tasks.iter().enumerate() {
                println!("{}. {}", index + 1, task);
            }

            // j'affiche le menu
            println!("Appuyez sur 'N' pour ajouter une tâche, 'A' pour afficher les tâches archivé ou entrez le numéro de la tâche pour la déplacer, 'E' pour quitter");
        }

        // initialisation d'une nouvel variable mutable
        let mut choice = String::new();

        // lire le choix de l'utilisateur et le stocker dans la variable choice
        io::stdin().read_line(&mut choice).unwrap();

        // supprime les espaces inutiles, convertit les lettres en majuscules
        let choice = choice.trim().to_uppercase();

        match choice.as_str() {
            "N" => {
                // Permet d'ajouter une nouvelle tâche à la liste
                println!("Entrez une nouvelle tâche :");

                // initialise une nouvelle variable mutable
                let mut new_task = String::new();

                // lire la nouvelle tâche de l'utilisateur et le stocker dans la variable new_task
                io::stdin().read_line(&mut new_task).unwrap();

                // push ajoute la new_task dans tasks
                tasks.push(new_task);
                println!("Tâche ajoutée !");
            }
            "A" => {
                // Afficher les tâches archivés
                is_archived_view = true;
            }
            "Q" => {
                // Revenir à la vue de base
                is_archived_view = false;
            }
            "E" => {
                // Quitte le programme
                println!("Au revoir !");
                break;
            }
            //  déplacer une tâche avec son num
            _ => {
                // j'essaye de convertir l'entré de l'utilisateur en un nombre de type usize
                if let Ok(index) = choice.parse::<usize>() {
                    // je vérifie si l'index est valide
                    if index > 0 && index <= tasks.len() {
                        // je retire la tâche de la liste et la stocke dans la variable task
                        let task = tasks.remove(index - 1);

                        // Ajoute la tâche à la liste des tâche archivé
                        archived_tasks.push(task);

                        // Affiche un message de confirmation
                        println!("Tâche déplacée vers les tâches archivées !");
                    } else {
                        // Si l'index est invalide on afficher un message d'erreur
                        println!("Numéro de tâche invalide.");
                    }
                } else {
                    // Si la conversion en usize échoue j'affiche un message d'erreur
                    println!("Choix invalide. Appuyez sur 'N' pour ajouter une tâche, 'A' pour afficher les tâches archivé ou entrez le numéro de la tâche pour la déplacer, 'E' pour quitter");
                }
            }

        }
    }
}
