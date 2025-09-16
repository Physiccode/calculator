use std::f64::consts::PI;
use std::io;
use std::error::Error;
#[allow(unused_variables)]

fn secdeg_delta(a:f64,b:f64,c:f64)->f64{
    b*b-4.0*a*c
}
fn secdeg_posroot(a:f64,b:f64,c:f64,delta:&f64)->f64{
    //i dont know what dereferencing does well
    let dedet=*delta;
    if dedet  >= 0.0 {
        (-1.0*b+delta.powf(1.0/2.0))/(2.0*a)
    }
    else {
        panic!("Delta value:{} is 0.Equation:{}x^2+{}x+{}=0",delta,a,b,c);
    }
}
fn secdeg_negroot(a:f64,b:f64,c:f64,delta:&f64)->f64{
    let dedet=*delta;
    if dedet  >= 0.0 {
        (-1.0*b-delta.powf(1.0/2.0))/(2.0*a)
    }
    else {
        panic!("Delta value:{} is 0.Equation:{}x^2+{}x+{}=0",delta,a,b,c);
    }
}
fn twosys_delta(a_1:&f64,b_1:&f64,a_2:&f64,b_2:&f64)->f64{
    a_1*b_2-a_2-b_1
}
fn twosys_deltax(c_1:&f64,b_1:&f64,c_2:&f64,b_2:&f64)->f64{
    c_1*b_2-c_2*b_1
}
fn twosys_deltay(a_1:&f64,a_2:&f64,c_1:&f64,c_2:&f64)->f64{
    a_1*c_2*a_2*c_1
}
fn twosys_calcdeltx(deltax:&f64,delta:&f64)->f64{
    deltax/delta
}
fn twosys_calcdelty(deltay:&f64,delta:&f64)->f64{
    deltay/delta
}
fn add(x:f64,y:f64){
    println!("{}+{}={}",x.clone(),y.clone(),x+y);
}
fn subtract(x:f64,y:f64){
    println!("{}-{}={}",x.clone(),y.clone(),x-y);
}
fn multiply(x:f64,y:f64){
    println!("{}*{}={}",x.clone(),y.clone(),x*y);
}
fn divide(x:f64,y:f64){
    if y != 0.0 {
        println!("{}/{}={}",x,y,x/y)
    }
    panic!("Can't divide by 0\noperation:{}/{}",x,y)
}
fn floordivide(x:f64,y:f64){
    let p=x/y;
    println!("{}//{}={}",x.clone(),y.clone(),p.floor());
}
fn power(x:f64,r:f64){
    let res=x.powf(1.0/r);
    println!("{}nd/rd/th root of {}={}",r.clone(),x.clone(),res);
}
enum Shape {
    Circle{radius:f64},

    Square{side:f64},

    Rectangle{
        base:f64,
        height:f64,
    },

    RightAngledTriangle{
        base:f64,
        height:f64,
        hypothenuse:f64
    },

    OtherTypeTriangle{
        a:f64,
        b:f64,
        c:f64
    }
}

impl Shape {
    fn area(&self,sum:&Shape)->Result<f64,String>{
        match sum{
            Shape::RightAngledTriangle{base,height,hypothenuse}=>{
                //check if the triangle is really a right angled triangle with pythagorean theorem
                let sides=base.powi(2)+height.powi(2);
                if sides==hypothenuse.powi(2){
                    return Ok((base*height)/2.0);
                }
                else {
                    return Err("This triangle's shape is abnormal,it's not a right angled triangle or a polygon".to_string());
                    /*
                    Example of a right angled triangle:
                    base:8cm
                    height:6cm
                    hypothenuse:10cm
                     */
                }
            }

            Shape::OtherTypeTriangle{a,b,c}=>{
                let s=(a+b+c)/2.0;
                let expression=(s-a)+(s-b)+(s-c);
                let area=expression.powf(1.0/2.0);
                return Ok(area);
            }

            Shape::Circle{radius}=>{
                return  Ok(radius*PI);
            }

            Shape::Square{side}=>{
                return Ok(side.powi(2));
            }

            Shape::Rectangle {
                base,
                height,
            }=>{
                return Ok(base*height);
            }

        }

    }

    fn perimeter(&self,sum:&Shape)->f64{
        match sum{
            
            Shape::RightAngledTriangle{base,height,hypothenuse}=>{
                return base+height+hypothenuse;
            }

            Shape::OtherTypeTriangle { a, b, c }=>{
                return a+b+c;
            }

            Shape::Circle{radius}=>{
                return 2.0*PI*radius;
            }

            Shape::Square{side}=>{
                return 4.0*side;
            }

            Shape::Rectangle{
                base,
                height
            }=>{
                return 2.0*(base+height);
            }

        }

    }

}
fn square(x:f64)->f64{
    x.powf(1.0/2.0)   
}
fn plotx(mval:f64,cval:f64,xval:f64)->f64{
    //y=mx+c
    let y=(mval*xval)+cval;
    return y;
}
fn ploty(mval:f64,cval:f64,yval:f64,calculatefraction:bool)->String{
    //formula to calculate x from y:x=(y-c)/m
    let yminusc=yval-cval;
    if calculatefraction {
        if mval==0.0 {
            panic!("value of m cannot be 0");
        }
        return (yminusc/mval).to_string();
    }
    else{
        let expression=format!("{}/{}",yminusc,mval);
        return expression;
    }
}
fn radtodeg(sum:String)->f64{
    //ex:2pi/3
    //ex3:2/3
    //ex4:12.5
    //(rad*180º)/pi
    if let Some(indexofpi)=sum.find("p") {
        if let Some(indexofbar)=sum.find("/") {
            let bef:String=sum[..indexofpi].to_string();
            let aft:String=sum[indexofbar+1..].to_string();
            let mut bef:f64=bef.trim().parse::<f64>().expect("parsing of nominator error");
            let mut aft=aft.trim().parse::<f64>().expect("Couldnt parse denominator error");
            let input=(bef*PI)/aft;
            let eq=(input*180.0)/PI;
            return eq;
        }
        //at this point input is something like 2pi
        let num:String=sum[..indexofpi].to_string();
        let mut num:f64=num.trim().parse::<f64>().expect("Couldnt parse the number before pi");
        let tot=num*PI;
        let eq=(tot*180.0)/PI;
        return eq;
    }
    else if let Some(indexofbar)=sum.find("/") {
        let bef:String=sum[..indexofbar].to_string();
        let aft:String=sum[indexofbar+1..].to_string();
        let mut bef:f64=bef.trim().parse::<f64>().expect("Couldnt parse nominator");
        let mut aft=aft.trim().parse::<f64>().expect("Couldnt parse denominator");
        let tot=bef/aft;
        let eq=(tot*180.0)/PI;
        return eq;
    }
    else {
        /*if input survived until here,then it's probably something like '2',so ill parse the input  as f64,if its anything else then its probably invalid
        if its invalid ill return an error
        */
        let mut sum:f64=sum.trim().parse::<f64>().expect("Couldnt parse  the result");
        let eq=(sum*180.0)/PI;
        return eq;
    }
}

fn classify(deltax:f64,deltay:f64,delta:f64,calcx:f64,calcy:f64,a_1:f64,b_1:f64,a_2:f64,b_2:f64,c_1:f64,c_2:f64){
    if delta.floor() !=0.0{   
        println!("
        Type:Possible and determinated two equations system
        X={}
        Y={}
        S=({},{})
        Delta={}
        Delta_x={}
        Delta_y={}
        form of equation:
        |{}x+{}y={}
        |{}x+{}y={}
        ",&calcx,&calcy,calcx,calcy,delta,deltax,deltay,a_1,b_1,c_1,a_2,b_2,c_2)
    }
    else if delta.floor()==0.0{
        if deltax==0.0 && deltay==0.0{
        println!("
        Type:Possible and indeterminated two equations system
        X=Any real number
        Y=Any real number
        S=(-inf,inf)
        Delta={}
        Delta_x={}
        Delta_y={}
        form of equation:
        |{}x+{}y={}
        |{}x+{}y={}
        ",delta,deltax,deltay,a_1,b_1,c_1,a_2,b_2,c_2);
        }
        if deltax.floor() !=0.0 && deltay.floor() !=0.0{
        println!("
        Type:Impossible system of two equations
        X=Not defined
        Y=Not defined
        S=(Not defined,Not defined)
        Delta={}
        Delta_x={}
        Delta_y={}
        form of equation:
        |{}x+{}y={}
        |{}x+{}y={}
        ",delta,deltax,deltay,a_1,b_1,c_1,a_2,b_2,c_2)
        }
    }
}
fn main()->Result<(),Box<dyn Error>>{
    let mut brake=String::new();
    println!("Do you wish to break after each answer[yes/no]?:");
    io::stdin().read_line(&mut brake).expect("Couldnt read line");
    let _operation=loop{
        let mut exit=String::new();
        println!("
        Select 1 mode:
        1-exit
        2-second degree equation solver
        3-system of equations solver
        4-Area
        5-addition
        6-subtraction
        7-multiplication
        8-division
        9-floor division
        10-power
        11-radians to degrees
        12-degrees to radians
        13-Functions
            ");
            io::stdin().read_line(&mut exit).expect("Couldnt erad line.Sorry");
            if exit.trim()=="exit"||exit.trim()=="1"{
                break 0
            }
            else if exit.trim()=="2"{
                let mut a=String::new();
                let mut b=String::new();
                let mut c=String::new();
                println!("Whats the value of a in ax^2 :");
                io::stdin().read_line(&mut a).expect("Couldnt read line sorry");
                println!("Whats the value of a in bx :");
                io::stdin().read_line(&mut b).expect("Couldnt read line sorry");
                println!("Whats the value of a in c :");
                io::stdin().read_line(&mut c).expect("Couldnt read line sorry");
                let a:f64=match a.trim().parse::<f64>(){
                    Ok(num)=>num,
                    Err(_)=>break 0,
                };
                let b:f64=match b.trim().parse::<f64>(){
                    Ok(num)=>num,
                    Err(_)=>break 0,
                };

                let c:f64=match c.trim().parse::<f64>(){
                    Ok(num)=>num,
                    Err(_)=>break 0,
                };
                let delta=secdeg_delta(a,b,c);
                let posroot=secdeg_posroot(a,b,c,&delta);
                let negroot=secdeg_negroot(a,b,c,&delta);
                println!("
                Delta:{}
                1st root(positive):{}
                2nd root(negative):{}
                Equation:[{}]x^2+[{}]x+[{}]=0
                    ",delta,posroot,negroot,a,b,c);
                    if brake.trim()=="yes"{
                        break 0
                    }
    }
            else if exit.trim()=="3"{
                let mut a_1=String::new();
                let mut a_2=String::new();
                let mut b_1=String::new();
                let mut b_2=String::new();
                let mut c_1=String::new();
                let mut c_2=String::new();
                println!("a_1 as in a1x:");
                io::stdin().read_line(&mut a_1).expect("Couldnt read line");
                println!("a_2 as in a2x:");
                io::stdin().read_line(&mut a_2).expect("Couldnt read line");
                println!("b_1 as in b1x:");
                io::stdin().read_line(&mut b_1).expect("Couldnt read line");
                println!("b_2 as in b2x:");
                io::stdin().read_line(&mut b_2).expect("Couldnt read line");
                println!("c_1 as in c1:");
                io::stdin().read_line(&mut c_1).expect("Couldnt read line");
                println!("c_2 as in c2:");
                io::stdin().read_line(&mut c_2).expect("Couldnt read line");
                let a_1:f64=match a_1.trim().parse::<f64>(){
                    Ok(num)=>num,
                    Err(_)=>{println!("Couldnt parse a_1 to f64");break 0}
            };

                let a_2:f64=match a_2.trim().parse::<f64>(){
                    Ok(num)=>num,
                    Err(_)=>{println!("Couldnt parse a_1 to f64");break 0}
            };
                let b_1:f64=match b_1.trim().parse::<f64>(){
                    Ok(num)=>num,
                    Err(_)=>{println!("Couldnt parse a_1 to f64");break 0}
            };

                let b_2:f64=match b_2.trim().parse::<f64>(){
                    Ok(num)=>num,
                    Err(_)=>{println!("Couldnt parse a_1 to f64");break 0}
            };

                let c_1:f64=match c_1.trim().parse::<f64>(){
                    Ok(num)=>num,
                    Err(_)=>{println!("Couldnt parse a_1 to f64");break 0}
            };
                let c_2:f64=match c_2.trim().parse::<f64>(){
                    Ok(num)=>num,
                    Err(_)=>{println!("Couldnt parse a_1 to f64");break 0}
            };
                let delta=twosys_delta(&a_1,&b_1,&a_2,&b_2);
                let deltax=twosys_deltax(&c_1,&b_1,&c_2,&b_2);
                let deltay=twosys_deltay(&a_1,&a_2,&c_1,&c_2);
                let calcx=twosys_calcdeltx(&deltax,&delta);
                let calcy=twosys_calcdelty(&deltay,&delta);
                classify(deltax,deltay,delta,calcx,calcy,a_1,b_1,a_2,b_2,c_1,c_2);
                if brake.trim()=="yes"{
                    break 0
                }

      }
      else if exit.trim()=="4"{
        let mut choice=String::new();
        println!("Select your figure:
        1-Area of a square
        2-Area of a circle
        3-Area of  rectangle
        4-Area of right angled triangle
        5-Area of other type of triangle
        --perimeters--
        a-perimeter of square
        b-perimeter of circle
        c-perimter of rectangle
        d-perimeter of right angled triangle
        e-perimeter of other type of triangle
        ");
        io::stdin().read_line(&mut choice).expect("Couldnt read line sorry");
        if choice.trim()=="1"{
            let mut stside=String::new();
            println!("Whats the measurement of one side? :");
            io::stdin().read_line(&mut stside).expect("Couldnt read line");
            let stside:f64=match stside.trim().parse::<f64>(){
                Ok(num)=>num,
                Err(_)=>{
                    println!("Couldnt parse first side from string to float[Hint:try typing a correct number]");
                    break 0
                }
            };
            let square=Shape::Square{side:stside.clone()};
            let res=square.area(&square)?;
            println!("Dimensions of square:({}) x ({})
                      Area:{}",&stside,stside,res);
            if brake.trim()=="yes"{break 0}
        }
        else if choice.trim()=="2"{
            let mut radius=String::new();
            println!("Whats the radius of the circle? :");
            io::stdin().read_line(&mut radius).expect("Couldnt read line");
            let radius:f64=radius.trim().parse::<f64>()?;
            let circle=Shape::Circle{radius:radius};
            let res=circle.area(&circle)?;
            println!("Circle's radius:{}\nCircle's Area:{}\nCircle's area formula:pi*(radius^2)",radius,res);
            if brake.trim()=="yes"{break 0}
        }
        else if choice.trim()=="3"{
            let mut width=String::new();
            let mut length=String::new();
            println!("Whats the width of the rectangle? :");
            io::stdin().read_line(&mut width).expect("Couldnt read line");
            let width:f64=width.trim().parse::<f64>()?;
            println!("Whats the length of the rectangle? :");
            io::stdin().read_line(&mut length).expect("Couldnt read line");
            let length:f64=length.trim().parse::<f64>()?;
            let rect=Shape::Rectangle { base: width, height: length };
            let res=rect.area(&rect)?;
            println!("Rectangles length:{}\nRectangle's width:{}\nRectangle's Area:{}",length.clone(),width.clone(),res);
            if brake.trim()=="yes"{break 0}
        }
        else if choice.trim()=="4"{
            println!("[4-4] Mode:Área of right angled triangle");
            let mut base=String::new();
            let mut height=String::new();
            let mut hypothenuse=String::new();
            println!("Whats the measurement of the base? :");
            io::stdin().read_line(&mut base).expect("Couldnt read line");
            println!("Whats the measurement of the height? :");
            io::stdin().read_line(&mut height).expect("Couldnt read line");
            println!("Whats the measurement of the hypothenuse? :");
            io::stdin().read_line(&mut hypothenuse).expect("Couldnt read line");
            let base:f64=base.trim().parse::<f64>()?;
            let height:f64=height.trim().parse::<f64>()?;
            let hypothenuse:f64=hypothenuse.trim().parse::<f64>()?;
            //construct triangle
            let rightangledtriangle=Shape::RightAngledTriangle{ base:base.clone(), height:height.clone(), hypothenuse:hypothenuse.clone(),};
            let res=rightangledtriangle.area(&rightangledtriangle)?;
            println!("
            triangle's type:Right angled
            triangle's base:{}
            triangle'height:{}
            triangle's hypothenuse:{}
            triangle's area:{}
            ",base,height,hypothenuse,res);
        }
      }
      else if exit.trim()=="5"{
        let mut x=String::new();
        let mut y=String::new();
        println!("Select the value of x:");
        io::stdin().read_line(&mut x).expect("Couldnt read line");
        println!("Select the value of y:");
        io::stdin().read_line(&mut y).expect("Couldnt read line");
        let x:f64=match x.trim().parse::<f64>(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Please type a valid number,breaking");
                break 0
            }
        };
        let y:f64=match y.trim().parse::<f64>(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Please type a valid number,breaking");
                break 0
            }
        };
        add(x,y);
        if brake.trim()=="yes" {
            break 0
        }
      }
      else if exit.trim()=="6"{
        let mut x=String::new();
        let mut y=String::new();
        println!("Select the value of x:");
        io::stdin().read_line(&mut x).expect("Couldnt read line");
        println!("Select the value of y:");
        io::stdin().read_line(&mut y).expect("Couldnt read line");
        let x:f64=match x.trim().parse::<f64>(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Please type a valid number,breaking");
                break 0
            }
        };
        let y:f64=match y.trim().parse::<f64>(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Please type a valid number,breaking");
                break 0
            }
        };
        subtract(x,y);
        if brake.trim()=="yes" {
            break 0
        }
      }
      else if exit.trim()=="7"{
        let mut x=String::new();
        let mut y=String::new();
        println!("Select the value of x:");
        io::stdin().read_line(&mut x).expect("Couldnt read line");
        println!("Select the value of y:");
        io::stdin().read_line(&mut y).expect("Couldnt read line");
        let x:f64=match x.trim().parse::<f64>(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Please type a valid number,breaking");
                break 0
            }
        };
        let y:f64=match y.trim().parse::<f64>(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Please type a valid number,breaking");
                break 0
            }
        };
        multiply(x,y);
        if brake.trim()=="yes" {
            break 0
        }
      }
      else if exit.trim()=="8"{
        let mut x=String::new();
        let mut y=String::new();
        println!("Select the value of x:");
        io::stdin().read_line(&mut x).expect("Couldnt read line");
        println!("Select the value of y:");
        io::stdin().read_line(&mut y).expect("Couldnt read line");
        let x:f64=match x.trim().parse::<f64>(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Please type a valid number,breaking");
                break 0
            }
        };
        let y:f64=match y.trim().parse::<f64>(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Please type a valid number,breaking");
                break 0
            }
        };
        divide(x,y);
        if brake.trim()=="yes" {
            break 0
        }
      }
      else if exit.trim()=="9"{
        let mut x=String::new();
        let mut y=String::new();
        println!("Select the value of x:");
        io::stdin().read_line(&mut x).expect("Couldnt read line");
        println!("Select the value of y:");
        io::stdin().read_line(&mut y).expect("Couldnt read line");
        let x:f64=match x.trim().parse::<f64>(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Please type a valid number,breaking");
                break 0
            }
        };
        let y:f64=match y.trim().parse::<f64>(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Please type a valid number,breaking");
                break 0
            }
        };
        floordivide(x,y);
        if brake.trim()=="yes" {
            break 0
        }
      }
      else if exit.trim()=="10"{
        let mut x=String::new();
        let mut r=String::new();
        println!("Select the value of x:");
        io::stdin().read_line(&mut x).expect("Couldnt read line");
        println!("Select the value of r:");
        io::stdin().read_line(&mut r).expect("Couldnt read line");
        let x:f64=match x.trim().parse::<f64>(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Please type a valid number,breaking");
                break 0
            }
        };
        let r:f64=match r.trim().parse::<f64>(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Please type a valid number,breaking");
                break 0
            }
        };
        power(x,r);
        if brake.trim()=="yes" {
            break 0
        }
      }
      else if exit.trim()=="11"{
        let mut rad=String::new();
        println!("[11] Mode:Radians to degrees");
        println!("How many radians do you wish to convert to degrees? enter them in the form of '2pi/3' :");
        io::stdin().read_line(&mut rad).expect("Couldnt read line");
        let res=radtodeg(rad.clone());
        println!("{} rad -> {} degrees",rad.trim(),res);
        if brake.trim()=="yes" {
            break 0
        }
      }
      else if exit.trim()=="12"{
        let mut deg=String::new();
        println!("[12] Mode:Degrees to radians");
        println!("How many degrees do you wish to convert to radians? enter them in the form of '150' :");
        io::stdin().read_line(&mut deg).expect("Couldnt read line");
        println!("this mode is deprecated");
      }
      else if exit.trim()=="13" {
        println!("[12] Mode:Functions");
        let _funcloop:u8=loop {
            let mut funcchoice=String::new();
            println!("Select your objective:
            1-go back
            2-Plotting ranges of x in y=mx+c
            3-plotting ranges of y in y=mx+c
            4-finding x and y axis in y=mx+c
            ");
            io::stdin().read_line(&mut funcchoice).expect("Couldnt read line");
            if funcchoice.trim() == "1" {
                break 0
            }
            else if funcchoice.trim() == "2"{
                let mut mval=String::new();
                let mut cval=String::new();
                let mut stval=String::new();
                let mut ndval=String::new();
                println!("Enter value of m:");
                io::stdin().read_line(&mut mval).expect("Couldnt read line");
                println!("Enter value of c:");
                io::stdin().read_line(&mut cval).expect("Couldnt read line");
                let mval:f64=mval.trim().parse::<f64>()?;
                let cval:f64=cval.trim().parse::<f64>()?;
                println!("Enter the first value of x :");
                io::stdin().read_line(&mut stval).expect("Couldnt read line");
                println!("Enter the second value of x :");
                io::stdin().read_line(&mut ndval).expect("Couldnt read line");
                let mut stval:f64=stval.trim().parse::<f64>()?;
                let ndval:f64=ndval.trim().parse::<f64>()?;
                let mut valsy:Vec<f64>=Vec::new();
                while stval<ndval {
                    let i=plotx(mval,cval,stval);
                    valsy.push(i);
                    stval +=1.0;
                }
                println!("Equation:y=({})x+({}) for x in range=[({}),({})]",mval,cval,stval,ndval);
                println!("Values of y:{:#?}",valsy);
                if brake.trim()=="yes"{
                    panic!("ignore this error,it was made so the program could break from both loops");
                }
            }
            else if funcchoice.trim() =="3" {
                let mut mval=String::new();
                let mut cval=String::new();
                let mut stval=String::new();
                let mut ndval=String::new();
                let mut calcfrac=String::new();
                let mut valsx:Vec<String>=Vec::new();
                println!("Enter value of m:");
                io::stdin().read_line(&mut mval).expect("Couldnt read line");
                println!("Enter value of c:");
                io::stdin().read_line(&mut cval).expect("Couldnt read line");
                println!("Enter the first value of y:");
                io::stdin().read_line(&mut stval).expect("Couldnt read line");
                println!("Enter the last value of y:");
                io::stdin().read_line(&mut ndval).expect("Couldnt read line");
                println!("Do you want to calculate fraction? :");
                io::stdin().read_line(&mut calcfrac).expect("Couldnt read line");
                let mval:f64=mval.trim().parse::<f64>()?;
                let cval:f64=cval.trim().parse::<f64>()?;
                let mut stval:f64=stval.trim().parse::<f64>()?;
                let ndval:f64=ndval.trim().parse::<f64>()?;
                while stval<ndval {
                if calcfrac.trim() == "yes" || calcfrac.trim() == "calculate" || calcfrac.trim() =="accept" {
                    let i=ploty(mval,cval,stval,true);
                    valsx.push(i);
                    stval +=1.0;
                }
                else {
                    let i=ploty(mval,cval,stval,false);
                    valsx.push(i);
                    stval +=1.0;
                }
                }
                println!("Equation:y={}x+{} for y in range=[{},{}]",mval,cval,stval,ndval);
                println!("Values of y as x varies:{:#?}",valsx);
                if brake.trim()=="yes"{
                    break 0
                }
            }

        };
      }
      else {
        println!("Invalid choice");
        if brake.trim()=="yes"{
            break 0;
        }
      }
    };
    Ok(())
}
