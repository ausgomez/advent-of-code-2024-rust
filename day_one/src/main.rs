mod data;

fn findDifference(value1: i32, value2: i32) -> i32 {
    if value1 > value2 {
        return value1 - value2;
    } else {
        return value2 - value1;
    }
}

fn main() {
    let mut totalDiff: i32 = 0;
    let mut list1 = data::LIST_1;
    let mut list2 = data::LIST_2;

    list1.sort();
    list2.sort();

    for (index, _) in list1.iter().enumerate() {
        let diff = findDifference(list1[index], list2[index]);
        totalDiff += diff;
    }

    println!("total difference {:?}", totalDiff);
}
