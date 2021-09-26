use std::time::Instant;

struct Pascal {
    rows: u32,
    left: Vec<u32>,
    series: Vec<u32>,
}

impl Pascal {
    /*
     * the 'triangle' represented here as a vector for every row
     * push x new u32 integers where x is one plus the row index
     */
    fn create(&mut self) {
        let mut start = 1;
        for i in 1..self.rows-1 {
            self.new_row();
            start += (i as usize)+1;
            self.left = (&self.series[start+1..]).to_vec();
        }
    }
    
    /*
     * pushes a new row onto the '
    */
    fn new_row(&mut self) {
        let left_slice_length = self.left.len();
        self.series.push(1);
        for i in 0..left_slice_length { 
            self.series.push({
                self.left[i]+self.left[left_slice_length-1-i]
            });

        }
        self.series.push(1);
    }

    // print as a right triangle, aligned to left side
    fn pretty_print(&self) {
        // index in the series that marks the end of the line
        // continuing the next
        let mut row_length = 1;
        let mut row_end = 0;
        for (i, x) in self.series.iter().enumerate() {
            print!("{} ", x);
            if i == row_end {
                row_length += 1;
                row_end += row_length;
                println!();
            }
        }
    }
}

fn main() {
    let (run_fn, n) = (std::env::args().nth(1).expect("run function not found"),
                       std::env::args().nth(2).expect("row count argument not found"));
    // try to parse, if get an error 3 will be the iteration count
    let n: u32 = match n.trim().parse() {
        Ok(x) => {
            if x < 2 {
                println!("minimum input of {}", 2);
                2
            } else {
                x
            }
        },
        Err(_) => {
            let default_rows = 2;
            println!("n is not valid, defaulting to {}",
                     default_rows);
            default_rows
        },
    };

    let mut pascal = Pascal {
        rows: n,
        left: vec![1],
        series: vec![1, 1, 1],
    };
    
    match run_fn.trim() {
        "pascal" => {
            println!("generating a {}-row triangle...", n);
            pascal.create();
            pascal.pretty_print();
        }
        "time" => {
            println!("running a single proceedure for a {}-row triangle...", n);
            let now = Instant::now();
            pascal.create();
            println!("operation took: {:?}", now.elapsed());
        }
        _ => {
            println!("command not available");
        }
    }
}
