use series_tools::partition::{PartitionGenerator, num_partitions, num_partitions_fast};
use std::time::Instant;

pub fn main() {
    let n = Instant::now();
    println!("{}", num_partitions(30, 15));
    println!("{:?}", n.elapsed());

    let n3 = Instant::now();
    println!("{}", num_partitions_fast(30, 15));
    println!("{:?}", n3.elapsed());

    let n2 = Instant::now();
    let pgen = PartitionGenerator::new(30, 15);
    println!("{} {:?}", pgen.count(), n2.elapsed());
}
