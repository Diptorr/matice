use std::io;

fn main(){
    let mut size_intp = String::new();
    let mut smootho_perator_inpt = String::new();

    println!("jak ma byt matice velka?");
    io::stdin().read_line(&mut size_intp).expect("error");
    let size = size_intp.trim().parse().expect("error");

    let mut matrix_1 = vec![vec![0; size]; size];

    println!("napiste hodnoti matice po radcich odeleny mezerou nebo podtrzitkem\npriklad:\n 2 2 (enter)\n 2 2");
    for i in 0..size {
        let mut matrix_inpt = String::new();
        io::stdin()
            .read_line(&mut matrix_inpt)
            .expect("error");
        let matrix_values: Vec<i32> = matrix_inpt
            .split_whitespace()
            .map(|s| s.parse().expect("error"))
            .collect();
        if matrix_values.len() != size {
            println!("error");
            return;
        }
        matrix_1[i] = matrix_values;
    }
    //its to ez
    println!("Matice_1:");
    for i in 0..size {
        for j in 0..size {
            print!("{:4} ", matrix_1[i][j]);
        }
        println!();
    }

    println!("druha matice");

    let mut matrix_2 = vec![vec![0; size]; size];

    for i in 0..size {
        let mut matrix_inpt = String::new();
        io::stdin()
            .read_line(&mut matrix_inpt)
            .expect("error");
        let matrix_values: Vec<i32> = matrix_inpt
            .split_whitespace()
            .map(|s| s.parse().expect("error"))
            .collect();
        if matrix_values.len() != size {
            println!("error");
            return;
        }
        matrix_2[i] = matrix_values;
    }

    println!("Matice_2:");
    for i in 0..size {
        for j in 0..size {
            print!("{:4} ", matrix_2[i][j]);
        }
        println!();
    }

    println!("jakou operaci by jste chteli provest\n- pro odecitani\n + pro scitani");
    io::stdin().read_line(&mut smootho_perator_inpt).expect("error");
    let smootho_perator = smootho_perator_inpt.trim();

    match smootho_perator {
        "+" => {let result_matrix: Vec<Vec<i32>> = matrix_1
        .iter()
        .zip(matrix_2.iter())
        .map(|(row1, row2)| {
            row1.iter().zip(row2.iter()).map(|(a, b)| a - b).collect()
        })
        .collect();
    println!("Result Matrix:");
    for i in 0..size {
        for j in 0..size {
            print!("{:4} ", result_matrix[i][j]);
        }
        println!();
        }
        }
        "-" => {let result_matrix: Vec<Vec<i32>> = matrix_1
            .iter()
            .zip(matrix_2.iter())
            .map(|(row1, row2)| {
                row1.iter().zip(row2.iter()).map(|(a, b)| a - b).collect()
            })
            .collect();
        println!("Result Matrix:");
        for i in 0..size {
            for j in 0..size {
                print!("{:4} ", result_matrix[i][j]);
            }
            println!();
            }
            }
            _ => {println!("prace na silnicich")}
            }   
            }

    

        // Jsem stracen co se deje zacharnte mne prosim

