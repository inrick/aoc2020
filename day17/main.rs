use std::io::BufRead;
use std::io::BufReader;

type Space3d = Vec<Vec<Vec<u8>>>;
type Space4d = Vec<Vec<Vec<Vec<u8>>>>;

fn main() {
    let initial: Vec<Vec<u8>> = BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().into())
        .collect();

    let rows = initial.len();
    let cols = initial[0].len();
    let offset = 10;
    let expanse = 2 * offset;

    let a = {
        let mut state: Space3d = vec![vec![vec![b'.'; cols + expanse]; rows + expanse]; 1 + expanse];
        for y in 0..rows {
            for x in 0..cols {
                state[offset][y + offset][x + offset] = initial[y][x];
            }
        }

        let hz = 1 + expanse;
        let hy = rows + expanse;
        let hx = cols + expanse;
        for _ in 0..6 {
            let state0 = state.clone();
            for i in 0..hz {
                for j in 0..hy {
                    for k in 0..hx {
                        state[i][j][k] = transition_3d(&state0, i, j, k);
                    }
                }
            }
        }
        count_active_3d(&state)
    };

    let b = {
        let mut state: Space4d =
            vec![vec![vec![vec![b'.'; cols + expanse]; rows + expanse]; 1 + expanse]; 1 + expanse];
        for y in 0..rows {
            for x in 0..cols {
                state[offset][offset][y + offset][x + offset] = initial[y][x];
            }
        }

        let hz = 1 + expanse;
        let hy = rows + expanse;
        let hx = cols + expanse;
        for _ in 0..6 {
            let state0 = state.clone();
            for i in 0..hz {
                for j in 0..hz {
                    for k in 0..hy {
                        for l in 0..hx {
                            state[i][j][k][l] = transition_4d(&state0, i, j, k, l);
                        }
                    }
                }
            }
        }
        count_active_4d(&state)
    };

    println!("a) {}", a);
    println!("b) {}", b);
}

fn count_active_3d(state: &Space3d) -> usize {
    state
        .iter()
        .flatten()
        .flatten()
        .filter(|&&c| c == b'#')
        .count()
}

fn count_active_4d(state: &Space4d) -> usize {
    state
        .iter()
        .flatten()
        .flatten()
        .flatten()
        .filter(|&&c| c == b'#')
        .count()
}

fn transition_3d(state: &Space3d, i: usize, j: usize, k: usize) -> u8 {
    let mut n = 0;
    for dz in 0..3 {
        for dy in 0..3 {
            for dx in 0..3 {
                if (dz, dy, dx) == (1, 1, 1) || (i <= dz || j <= dy || k <= dx) {
                    continue;
                }
                let (z, y, x) = (i + dz - 1, j + dy - 1, k + dx - 1);
                if state.len() <= z || state[0].len() <= y || state[0][0].len() <= x {
                    continue;
                }
                n += (state[z][y][x] == b'#') as usize;
            }
        }
    }
    match (state[i][j][k], n) {
        (b'#', 2) | (b'#', 3) | (b'.', 3) => b'#',
        _ => b'.',
    }
}

fn transition_4d(state: &Space4d, i: usize, j: usize, k: usize, l: usize) -> u8 {
    let mut n = 0;
    for dw in 0..3 {
        for dz in 0..3 {
            for dy in 0..3 {
                for dx in 0..3 {
                    if (dw, dz, dy, dx) == (1, 1, 1, 1)
                        || (i <= dw || j <= dz || k <= dy || l <= dx)
                    {
                        continue;
                    }
                    let (w, z, y, x) = (i + dw - 1, j + dz - 1, k + dy - 1, l + dx - 1);
                    if state.len() <= w
                        || state[0].len() <= z
                        || state[0][0].len() <= y
                        || state[0][0][0].len() <= x
                    {
                        continue;
                    }
                    n += (state[w][z][y][x] == b'#') as usize;
                }
            }
        }
    }
    match (state[i][j][k][l], n) {
        (b'#', 2) | (b'#', 3) | (b'.', 3) => b'#',
        _ => b'.',
    }
}
