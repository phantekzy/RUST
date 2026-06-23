fn main() {
    // The Array type
    let _months = [
        "janvier",
        "fevrier",
        "mars",
        "avril",
        "mai",
        "june",
        "july",
        "August",
        "September",
        "Octobre",
        "November",
        "November",
        "december",
    ];

    // The fist way
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    // the second way
    let b = [3; 5];

    // Accessing an Array
    let one = [1, 2, 3, 4, 5];
    let first = one[0];
    println!("first : {}", first);
    let second = one[2];
    println!("second : {}", second);
}
