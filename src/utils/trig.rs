use std::f64::consts::PI;
use std::error::Error;

pub fn radtodeg(sum:String)->Result<f64,Box<dyn Error>>{
    //ex:2pi/3
    //ex3:2/3
    //ex4:12.5
    //(rad*180º)/pi
    if let Some(indexofpi)=sum.find("p") {
        if let Some(indexofbar)=sum.find("/") {
            let bef:String   = sum[..indexofpi].to_string();
            let aft:String   = sum[indexofbar+1..].to_string();
            let bef:f64  = bef.trim().parse::<f64>()?;
            let aft = aft.trim().parse::<f64>()?;
            let input   = (bef*PI)/aft;
            let eq      = (input*180.0)/PI;
            return Ok(eq);
        }
        //at this point input is something like 2pi
        let num:String  = sum[..indexofpi].to_string();
        let num:f64 = num.trim().parse::<f64>()?;
        let tot    = num*PI;
        let eq     = (tot*180.0)/PI;
        return Ok(eq);
    }
    else if let Some(indexofbar)=sum.find("/") {
        let bef:String = sum[..indexofbar].to_string();
        let aft:String = sum[indexofbar+1..].to_string();
        let bef:f64    = bef.trim().parse::<f64>()?;
        let aft   = aft.trim().parse::<f64>()?;
        let tot   = bef/aft;
        let eq    = (tot*180.0)/PI;
        return Ok(eq);
    }
    else {
        /*if input survived until here,then it's probably something like '2',so ill parse the input  as f64,if its anything else then its probably invalid
        if its invalid ill return an error
        */
        let sum:f64 = sum.trim().parse::<f64>()?;
        let eq = (sum*180.0)/PI;
        return Ok(eq);
    }
}

//pretty degrees to radians function
pub fn degreestorad(x:&f64,calcfrac:bool)->String {
    //π
    if calcfrac {
        let eq     = x/180.0;
        let tot = format!("{}π",eq);
        return tot.to_string(); 
    }
    else {
        let eq = format!("{}π/{}",x,180.0);
        return eq.to_string();
    }
}

//simple degrees to radians function
pub fn drad(x:&f64)->f64{
    x*180.0/PI
}