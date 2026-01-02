//two variables system of equation


pub fn classify(deltax:f64,deltay:f64,delta:f64,calcx:f64,calcy:f64,a_1:f64,b_1:f64,a_2:f64,b_2:f64,c_1:f64,c_2:f64){
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

pub fn twosys_delta(a_1:&f64,b_1:&f64,a_2:&f64,b_2:&f64)->f64{
    a_1*b_2-a_2-b_1
}
pub fn twosys_deltax(c_1:&f64,b_1:&f64,c_2:&f64,b_2:&f64)->f64{
    c_1*b_2-c_2*b_1
}
pub fn twosys_deltay(a_1:&f64,a_2:&f64,c_1:&f64,c_2:&f64)->f64{
    a_1*c_2*a_2*c_1
}
pub fn twosys_calcdeltx(deltax:&f64,delta:&f64)->f64{
    deltax/delta
}
pub fn twosys_calcdelty(deltay:&f64,delta:&f64)->f64{
    deltay/delta
}


//functions


pub fn plotx(mval:f64,cval:f64,xval:f64)->f64{
    //y=mx+c
    let y=(mval*xval)+cval;
    return y;
}
pub fn ploty(mval:f64,cval:f64,yval:f64,calculatefraction:bool)->String{
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


//second degree equations


pub fn secdeg_delta(a:f64,b:f64,c:f64)->f64{
    b*b-4.0*a*c
}
pub fn secdeg_posroot(a:f64,b:f64,delta:&f64,calcfrac:bool)->String{
    let dedet=*delta;
    if dedet  >= 0.0 {
        if calcfrac==true {
            let eq=(-1.0*b+delta.powf(1.0/2.0))/(2.0*a);
            return eq.to_string();
        }
        else {
            let eqp1=-1.0*b;
            let eqp2=2.0*a;
            let eqt=format!("({}+√{})/({})",eqp1,eqp2,delta);
            return eqt;
        }
    }
    //handle negative roots
    else {
        if calcfrac==true {
            let parseddelta=delta*-1.0;
            let eqp1=(-1.0*b)/(2.0*a);
            let eqp2=(parseddelta.powf(1.0/2.0))/(2.0*a);
            let eqt=format!("{}+({})i",eqp1,eqp2);
            return eqt;
        }
        else {
            let parseddelta=delta*-1.0;
            let eqp1=-1.0*b;
            let eqp2=2.0*a;
            let eqt=format!("({}+i√{})/({})",eqp1,eqp2,parseddelta);
            return eqt;
        }

    }
}

pub fn secdeg_negroot(a:f64,b:f64,delta:&f64,calcfrac:bool)->String{
    let dedet=*delta;
    if dedet  >= 0.0 {
        if calcfrac==true {
            let eq=(-1.0*b-delta.powf(1.0/2.0))/(2.0*a);
            return eq.to_string();
        }
        else {
            let eqp1=-1.0*b;
            let eqp2=2.0*a;
            let eqt=format!("({}-√{})/({})",eqp1,eqp2,delta);
            return eqt;
        }
    }
    //handle negative roots
    else {
        if calcfrac==true {
            let parseddelta=delta*-1.0;
            let eqp1=(-1.0*b)/(2.0*a);
            let eqp2=(parseddelta.powf(1.0/2.0))/(2.0*a);
            let eqt=format!("({})-({})i",eqp1,eqp2);
            return eqt;
        }
        else {
            let parseddelta=delta*-1.0;
            let eqp1=-1.0*b;
            let eqp2=2.0*a;
            let eqt=format!("({}-i√{})/({})",eqp1,eqp2,parseddelta);
            return eqt;
        }

    }
}


//Essentials


pub fn add(x:f64,y:f64)->f64{
    x+y
}
pub fn subtract(x:f64,y:f64)->f64{
    x-y
}
pub fn multiply(x:f64,y:f64){
    println!("{}*{}={}",x.clone(),y.clone(),x*y);
}
pub fn divide(x:f64,y:f64){
    if y != 0.0 {
        println!("{}/{}={}",x,y,x/y)
    }
    panic!("Can't divide by 0\noperation:{}/{}",x,y)
}
pub fn floordivide(x:f64,y:f64){
    let p=x/y;
    println!("{}//{}={}",x.clone(),y.clone(),p.floor());
}
pub fn power(x:f64,r:f64)->f64{
    x.powf(r)
}

pub fn square(x:f64)->f64{ //re-check this function and test it's logic
    x.powf(1.0/2.0)   
}
pub fn factorial(n:u64)->u64{
    (1..=n).product()
}
