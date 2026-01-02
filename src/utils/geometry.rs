use std::f64::consts::PI;
pub enum Shape {
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
    pub fn area(&self,sum:&Shape)->Result<f64,String>{
        match sum{
            Shape::RightAngledTriangle{base,height,hypothenuse}=>{
                //check if the triangle is really a right angled triangle with pythagorean theorem
                let sides=base.powi(2)+height.powi(2);
                if sides==hypothenuse.powi(2){
                    return Ok((base*height)/2.0);
                }
                else {
                    return Err("Not a right angled triangle".to_string());
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

    pub fn perimeter(&self,sum:&Shape)->f64{
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

    pub fn detect_type_by_sides(&self,sum:&Shape)->Result<String,String>{
        match sum {
            Shape::RightAngledTriangle{base,height,hypothenuse}=>{
                //all sides must be different on a right angle triangle
                return Ok("all sides are different".to_string());
            }
            Shape::OtherTypeTriangle{a,b,c}=>{
                //see if equilateral
                if a==b && b==c {
                    return Ok("Equilateral/All sides equal".to_string());
                }
                else if a==b || b==c || a==c {
                    return Ok("2 sides equal".to_string());
                }
                else {
                    return Ok("all sides different".to_string());
                }
            }
            _ =>{
                return Err("Cant be classified".to_string());
            }
        }

    }

    pub fn detect_type_by_angles(&self,sum:&Shape)->String{
        match sum {
            Shape::RightAngledTriangle{base,height,hypothenuse}=>{
                return "Right angled triangle".to_string();
            }
            Shape::OtherTypeTriangle { a, b, c }=>{
                if a==b && b==c {
                    return "Acutangle".to_string();
                    /*if all sides are equal then all angles must be equal
                    since α+θ+β=180º =>  α=θ=β => 3θ=180º => θ=60º
                    */
                }

                //use law of cosines to find all angles corresponding to each side
                let cos_a=(b.powi(2)+c.powi(2)-a.powi(2))/(2.0*b*c);
                let cos_b=(a.powi(2)+c.powi(2)-b.powi(2))/(2.0*a*b);
                let rad_a=cos_a.acos();
                let rad_b=cos_b.acos();
                let angle_a=rad_a.to_degrees();
                let angle_b=rad_b.to_degrees();
                let angle_c=180.0-(angle_a+angle_b);
                if  angle_a.floor()==90.0 || angle_b.floor()==90.0 || angle_c.floor()==90.0 {
                    return "Right angled triangle".to_string();
                }
                else if angle_a.floor()>90.0 || angle_b.floor()>90.0 || angle_c.floor()>90.0 {
                    return "Obtuse triangle".to_string();
                }
                else {
                    panic!("Something went wrong while classifying the triangle's angle by sides");
                }
            }
            _=>{
                return "Cant classify".to_string();
            }
        }
    }

}

/*
|\
|_\
|x \ k
|   \
|_ __\
 d
*/
