

/** fn borrow_sum(v: &Vec<i32>) -> i32 {
//   let mut sum = 0;
//   for val in v  {
//       sum += *val;
//   }
//   return sum;
// }
//
// fn cap_values_own(v: &mut Vec<i32>, max: i32) {
//     for i in 0..v.len() {
//         if v[i] > max {
//             v[i]= max;
//         }
//     }
// }
//
// ////////////////
//
//#[derive(Debug)]
//enum UserType {
//    USER,
//    MODERATOR,
//    ADMIN
//}
//
//#[derive(Debug)]
//struct User {
//    id: i32,
//    name: String,
//    user_type: UserType
//}
//
///////////////////////////////
//
//#[derive(Debug)]
//enum Action {
//    Drive,
//    Turn(Direction),
//    Stop
//}
//
//#[derive(Debug)]
//enum Direction{
//    Left,
//    Right
//}
//
//
//fn print_action(a: Action) {
//    match a {
//      Action::Drive => println!("Driving forward"),
//      Action::Turn(direction) => match direction {
//        Direction::Left => println!("Driving left"),
//        Direction::Right => println!("Driving right")
//      },
//      Action::Stop => println!("Stopping"),
//    }
//}
*/

////////////////////////
#[derive(Copy, Clone)]
enum MachineState {
    Normal,
    Comment,
    Upper,
    Lower
}

fn machine_cycle(state: MachineState, c: char) -> (Option<char>, MachineState) {
    use self::MachineState::*;

    match (state, c) {
        (Normal, '#') => (None, Comment),
        (Normal, '^') => (None, Upper),
        (Normal, '_') => (None, Lower),
        (Normal, other) => (Some(other), Normal),

        (Comment, '#') => (None, Normal),
        (Comment, _) => (None, Comment),

        (Upper, '^') => (None, Normal),
        (Upper, other) => (Some(other.to_uppercase().nth(0).unwrap()), Comment),

        (Lower, '_') => (None, Normal),
        (Lower, other) => (Some(other.to_lowercase().nth(0).unwrap()), Lower)
    }
}


fn main() {
    /**
    // let mut vec = vec![1,2,3,5,6,7,8];
    //   let res = borrow_sum(&vec);
    //   println!("Sum of {} values: {}", vec.len(), res );
    // cap_values_own(&mut vec, 3);
    // println!("{:?}", vec);

    // let user1 = User {
    //     id: 32, 
    //     name: "Test User".to_string(),
    //     user_type: UserType::ADMIN
    // };

    // println!("User: {:?}", user1)

    // let protocol: Vec<Action> = vec![
    //     Action::Drive, 
    //     Action::Turn(Direction::Left), 
    //     Action::Drive, Action::Stop
    // ];

    // for a in protocol {
    //     print_action(a)
    // }
    **/

    let mut state = MachineState::Normal;

    let mut processed_string = String::new();

    let input = "This _Is_ some ^Input^. #we want thi processed without this comment#";
    for c in input.chars() {
        let (output, new_state) = machine_cycle(state, c);
        if let Some(c_) = output {
            processed_string.push(c_);
        }

        state = new_state;

    }

    println!("{}", processed_string);

    /////////




    /////////


}
