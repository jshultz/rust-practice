use std::io;

struct Pascal {
    rows: u32,
    left: Vec<u32>,
    right: Vec<u32>,
    series: Vec<u32>,
}

impl Pascal {
    fn create(&mut self) {
        let mut start = 1;
        for i in 1..self.rows-1 {
            println!("[DEBUG] {:?} {:?}", self.left, self.right);
            for j in 0..self.left.len() { 
                self.series.push(1);
                self.series.push({
                    self.left[j]+self.right[j]
                });
                self.series.push(1);
            }
            start += (i as usize)+1;
            let end = start+(i as usize);
            self.left = (self.series[start..end-1]).to_vec();
            self.right = (self.series[start+1..end]).to_vec();
        }
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
    let n: u32 = match n.trim().parse() {
        Ok(x) => x,
        Err(_) => {
            3
        },
    };
    
    // clear line buffer
    //io::stdout().flush().unrwap();
    let mut pascal = Pascal {
        rows: n,
        left: vec![1],
        right: vec![1],
        series: vec![1, 1, 1],
    };

    pascal.create();

    pascal.pretty_print();
}
