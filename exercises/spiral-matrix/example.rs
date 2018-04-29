pub fn get_spiral(size: usize) -> Vec<Vec<usize>> {
    let mut matrix: Vec<Vec<usize>> = Vec::with_capacity(size);
    for i in 0..size {
        matrix.push(Vec::with_capacity(size));
        for _ in 0..size {
            matrix[i].push(0);
        }
    }

    let num_concentric_squares: usize = (size as f64 / 2.0).round() as usize;
    let mut counter: usize = 1;
    let mut sidelen = size;
    for i in 0..num_concentric_squares {

        for j in 0..sidelen {
            matrix[i][i + j] = counter;
            counter += 1;
        }

        for j in 1..sidelen {
            matrix[i + j][size - 1 - i] = counter;
            counter += 1;
        }

        for j in (0..sidelen - 1).rev() {
            matrix[size - 1 - i][i + j] = counter;
            counter += 1;
        }

        for j in (1..sidelen - 1).rev() {
            matrix[i + j][i] = counter;

            counter += 1;
        }
        if sidelen >= 2 {
            sidelen -= 2;
        } else {
            sidelen -= 1;
        }
    }
    matrix
}
