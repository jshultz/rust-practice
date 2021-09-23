use std::io;

struct Pascal {
    rows: u64,
    series: Vec<u64>,
}

impl Pascal {
    fn next_row(&self, prev_row: u64) -> Vec<u64> {
        let length = prev_row+1;
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
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("somehow cannot read the input");
    
    // try to parse, if get an error 3 will be the iteration count
    let n: u64 = match n.trim().parse() {
        Ok(x) => x,
        Err(_) => {
            3
        },
    };
    
    // clear line buffer
    //io::stdout().flush().unrwap();
    let pascal = Pascal {
        rows: n,
        series: vec![1, 1, 1],
    };

    for i in 0..pascal.rows {
        let new_row = pascal.next_row(i);
        for x in pascal.series.iter() {
            pascal.series.push(x);
        }
    }

    pascal.pretty_print();
}
