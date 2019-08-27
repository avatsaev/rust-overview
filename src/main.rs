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
enum State {
    Normal,
    Comment,
    Upper,
    Lower
}

fn machine_cycle(state: &State, c: char) -> (Option<char>, State) {

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

    /////////




    /////////


}
