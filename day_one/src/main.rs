mod data;

fn count_occurrences(list: [i32; 1000], value: i32) -> i32 {
    let mut count: i32 = 0;
    for (_, &item) in list.iter().enumerate() {
        if item == value {
            count += 1;
        }
    }
    return count;
}

fn find_difference(value1: i32, value2: i32) -> i32 {
    if value1 > value2 {
        return value1 - value2;
    } else {
        return value2 - value1;
    }
}

fn main() {
    let mut list1 = data::LIST_1;
    let mut list2 = data::LIST_2;

    list1.sort();
    list2.sort();

    println!("--- PART ONE ---");
    part_one(list1, list2);
    println!();
    println!("--- PART TWO ---");
    part_two(list1, list2);
}


fn part_one(list1: [i32; 1000], list2: [i32; 1000]) {
    let mut total_diff: i32 = 0;
    for (index, _) in list1.iter().enumerate() {
        let diff = find_difference(list1[index], list2[index]);
        total_diff += diff;
    }

    println!("total difference {:?}", total_diff);
}

fn part_two(list1: [i32; 1000], list2: [i32; 1000]) {

    let mut similarity: i32 = 0;
    
    for(x, _) in list1.iter().enumerate() {
        similarity += count_occurrences(list2, list1[x]) * list1[x];
    }

    println!("similarity {:?}", similarity);
}