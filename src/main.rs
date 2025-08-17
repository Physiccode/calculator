use std::io;
#[allow(unused_variables)]

fn secdeg_delta(a:f64,b:f64,c:f64)->f64{
    b*b-4.0*a*c
}
fn secdeg_posroot(a:f64,b:f64,c:f64,delta:&f64)->f64{
    (-1.0*b+delta.powf(1.0/2.0))/(2.0*a)
}
fn secdeg_negroot(a:f64,b:f64,c:f64,delta:&f64)->f64{
    (-1.0*b-delta.powf(1.0/2.0))/(2.0*a)
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
    let p=x/y;
    println!("{}/{}={}",x.clone(),y.clone(),p);
}
fn floordivide(x:f64,y:f64){
    let p=x/y;
    println!("{}//{}={}",x.clone(),y.clone(),p.floor());
}
fn power(x:f64,r:f64){
    let res=x.powf(1.0/r);
    println!("{}nd/rd/th root of {}={}",r.clone(),x.clone(),res);
}
fn areasquare(stside:&f64,ndside:&f64)->f64{
    return stside*ndside;
}
fn square(x:f64)->f64{
    x.powf(1.0/2.0)   
}
fn degtorad(deg:f64)->f64{
    const PIFUNC:f64=3.1415;
    (deg/180.0)*PIFUNC
}
fn radtodeg(rad:f64)->f64{
    rad*180.0
} 
fn process(something:f64)->f64{
    something
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
fn main(){
    const PI:f64=3.1415;
    const EULER:f64=2.78;
    let mut brake=String::new();
    println!("Do you wish to break after each answer[yes/no]?:");
    io::stdin().read_line(&mut brake).expect("Couldnt read line");
    let operation=loop{
        let mut exit=String::new();
        println!("
        Select 1 mode:
        1-exit
        2-second degree equation solver
        3-system of equations solver
        4-Solve with math formulas
        5-addition
        6-subtraction
        7-multiplication
        8-division
        9-floor division
        10-power
        11-radians to degrees
        12-degrees to radians
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
        println!("Select 1 type of formula:
        1-Area of a square
        2-Area of a circle
        3-Area of  rectangle
        ");
        io::stdin().read_line(&mut choice).expect("Couldnt read line sorry");
        if choice.trim()=="1"{
            let mut stside=String::new();
            let mut ndside=String::new();
            println!("Whats the measurement of 1st side? :");
            io::stdin().read_line(&mut stside).expect("Couldnt read line");
            println!("Whats the measurement of the 2nd side? :");
            io::stdin().read_line(&mut ndside).expect("Couldnt read line");
            let stside:f64=match stside.trim().parse::<f64>(){
                Ok(num)=>num,
                Err(_)=>{
                    println!("Couldnt parse first side from string to float[Hint:try typing a correct number]");
                    break 0
                }
            };
            let ndside:f64=match ndside.trim().parse::<f64>(){
                Ok(num)=>num,
                Err(_)=>{
                    println!("Couldnt parse second side form string to float[Hint:Try typing a correct number]");
                    break 0
                }
            };
            let res=areasquare(&stside,&ndside);
            println!("Dimensions of square:{}x{}
                      Area:{}",stside,ndside,res);
            if brake.trim()=="yes"{break 0}
        }
        else if choice.trim()=="2"{
            let mut radius=String::new();
            println!("Whats the radius of the circle? :");
            io::stdin().read_line(&mut radius).expect("Couldnt read line");
            let radius:f64=match radius.trim().parse::<f64>(){
                Ok(num)=>num,
                Err(_)=>{
                    println!("Couldnt parse radius from string to float,breaking");
                    break 0
                }
            };
            let res=square(radius.clone())*PI;
            println!("Circle's radius:{}\nCircle's Area:{}\nCircle's area formula:pi*(radius^2)",radius,res);
            if brake.trim()=="yes"{break 0}
        }
        else if choice.trim()=="3"{
            let mut width=String::new();
            let mut length=String::new();
            println!("Whats the width of the rectangle? :");
            io::stdin().read_line(&mut width).expect("Couldnt read line");
            let width:f64=match width.trim().parse::<f64>(){
                Ok(num)=>num,
                Err(_)=>{
                    println!("Couldnt parse width from string to float,breaking");
                    break 0
                }
            };
            println!("Whats the length of the rectangle? :");
            io::stdin().read_line(&mut length).expect("Couldnt read line");
            let length:f64=match length.trim().parse::<f64>(){
                Ok(num)=>num,
                Err(_)=>{
                    println!("Couldnt parse length from string to float,breaking");
                    break 0
                }
            };
            println!("Rectangles length:{}\nRectangle's width:{}\nRectangle's Area:{}",length.clone(),width.clone(),length*width);
            if brake.trim()=="yes"{break 0}
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
      }
      else if exit.trim()=="11"{
        let mut deg=String::new();
        println!("[Degrees to radians mode]\nHow many degrees do you want to convert to radian?e.g:25 :");
        io::stdin().read_line(&mut deg).expect("Couldnt read line sorry");
        let deg:f64=match deg.trim().parse::<f64>(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Couldnt convert degrees variable from string to float\n[Hint:if you want to convert 25º,dont type 25º,instead type 25]");
                break 0
            }
        };
        let conv=degtorad(deg);
        println!("{}º(Degrees)->{}Rad",deg.clone(),conv);
      }
      else if exit.trim()=="12"{
        //i had a hard time here,almost gave up untill i realized i could use string slices💀
        let mut totalstring=String::new();
        println!("Input your radian value EXACTLY like this '1pi/3':");
        io::stdin().read_line(&mut totalstring).expect("Couldnt read line sorry");
        let stprocess=&totalstring.replace("pi","").to_string();
        if let Some(index)=stprocess.find("/"){
            let stpart=&stprocess[..index];
            let ndpart=&stprocess[index+1..];
            let stpart:f64=stpart.trim().parse().unwrap_or(0.0);
            let ndpart:f64=ndpart.trim().parse().unwrap_or(1.0);
            let res=(stpart/ndpart)*180.0;
            println!("{}radians to {}º",totalstring.trim(),res);
            if brake.trim()=="yes"{
                break 0
            }
        }
        else{
            println!("Please formate your answer to something like 1pi/3[Hint:we exclude pi form your answer,so if you input pi/3 we try to parse /3]");
        }

      }
      else {
        println!("Invalid choice");
        if brake.trim()=="yes"{
            break 0;
        }
      }
    };
}
