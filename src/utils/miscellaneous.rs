use std::io;
pub fn print_modes(){      //this shall print the principal modes
          println!("
        Select the desired mode:
        0  - exit
        1  - third degree equation solver  [included complex roots]
        2  - second degree equation solver [included complex roots]
        3  - system of equations solver
        4  - Area calculations
        5  - addition
        6  - subtraction
        7  - multiplication
        8  - division
        9  - floor division
        10 - power
        11 - radians to degrees
        12 - degrees to radians
        13 - Functions operations
            ");
}


pub fn print_modes_secondary(){  //This shall print modes when inside mode 4
        println!("Select the desired calculation:
        1 - Area of a square
        2 - Area of a circle
        3 - Area of  rectangle
        4 - Area of right angled triangle
        5 - Area of other type of triangle
        --perimeters--
        a - perimeter of square
        b - perimeter of circle
        c - perimter of rectangle
        d - perimeter of right angled triangle
        e - perimeter of other type of triangle
        ");
}
//add:quadratic function points support,linear function,exponential function,etc...
pub fn print_objective(){
              println!("Select the desired mode:
            1 - go back
            2 - Plotting ranges of x in y=mx+c
            3 - plotting ranges of y in y=mx+c
            4 - finding x and y axis in y=mx+c
            ");
}
pub fn readvar()->String{
  let mut var=String::new();
  io::stdin().read_line(&mut var).expect("Couldnt read line!");
  var
}
