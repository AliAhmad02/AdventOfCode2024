use std::fs;

fn main() {
    let input_vec: Vec<_>;
    {
        let input = fs::read_to_string("data.txt").unwrap();
        input_vec = input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();
    }

    count(&input_vec);
}

fn count(input_vec: &Vec<Vec<char>>) {
    let m = input_vec.len();
    let n = input_vec[0].len();

    let mut mas_count: u32 = 0;
    let mut xmas_count: u32 = 0;

    let xmas = "XMAS";
    let samx = "SAMX";
    let mas = "MAS";
    let sam = "SAM";

    for i in 0..m {
        for j in 0..n {
            let mut mas_diag_bool = false;
            let mut xmas_diag_bool = false;
            let mut xmas_diag_rev_bool = false;
            let mut xmas_hor_bool = false;
            let mut xmas_ver_bool = false;

            if i < (m - 2) && j < (n - 2) {
                let diag_mas: String = vec![
                    input_vec[i][j],
                    input_vec[i + 1][j + 1],
                    input_vec[i + 2][j + 2],
                ]
                .iter()
                .collect();
                let diag_mas_rev: String = vec![
                    input_vec[i][j + 2],
                    input_vec[i + 1][j + 1],
                    input_vec[i + 2][j],
                ]
                .iter()
                .collect();

                mas_diag_bool = ((&diag_mas == mas) || (&diag_mas == sam))
                    && ((&diag_mas_rev == mas) || (&diag_mas_rev == sam));
            }

            if i < (m - 3) && j < (n - 3) {
                let diag_xmas: String = vec![
                    input_vec[i][j],
                    input_vec[i + 1][j + 1],
                    input_vec[i + 2][j + 2],
                    input_vec[i + 3][j + 3],
                ]
                .iter()
                .collect();

                xmas_diag_bool = (&diag_xmas == xmas) || (&diag_xmas == samx);
            }
            if i < (m - 3) && (j > 2) {
                let diag_xmas_rev: String = vec![
                    input_vec[i][j],
                    input_vec[i + 1][j - 1],
                    input_vec[i + 2][j - 2],
                    input_vec[i + 3][j - 3],
                ]
                .iter()
                .collect();

                xmas_diag_rev_bool = (&diag_xmas_rev == xmas) || (&diag_xmas_rev == samx);
            }

            if j < (n - 3) {
                let hor_xmas: String = vec![
                    input_vec[i][j],
                    input_vec[i][j + 1],
                    input_vec[i][j + 2],
                    input_vec[i][j + 3],
                ]
                .iter()
                .collect();

                xmas_hor_bool = (&hor_xmas == xmas) || (&hor_xmas == samx);
            }

            if i < (m - 3) {
                let ver_xmas: String = vec![
                    input_vec[i][j],
                    input_vec[i + 1][j],
                    input_vec[i + 2][j],
                    input_vec[i + 3][j],
                ]
                .iter()
                .collect();

                xmas_ver_bool = (&ver_xmas == xmas) || (&ver_xmas == samx);
            }

            mas_count += mas_diag_bool as u32;
            xmas_count += [
                xmas_diag_bool,
                xmas_diag_rev_bool,
                xmas_hor_bool,
                xmas_ver_bool,
            ]
            .into_iter()
            .filter(|b| *b)
            .count() as u32;
        }
    }

    println!("Number XMAS: {}", xmas_count);
    println!("Number MAS: {}", mas_count);
}
