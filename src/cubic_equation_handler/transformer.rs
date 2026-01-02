pub struct Cubiceqn{  //ax**3+bx**2+cx+d=0
  pub a:f64,
  pub b:f64,
  pub c:f64,
  pub d:f64
}
pub struct Normalized_cube{ //x**3+Ax**2+Bx+C=0
  pub a:f64,
  pub b:f64,
  pub c:f64
}
pub struct Depressed_cube{ //y**3+Py+q=0
  pub p:f64,
  pub q:f64,
  pub a:f64 //i need A to recover roots
}

pub trait Transform{
  fn normalize(&self)->Normalized_cube;
}


impl Transform for Cubiceqn{

  //step 1:divide everything by a if a !=1
  fn normalize(&self)->Normalized_cube{  //normalized equation:x**3+Ax**2+Bx+C=0
    if self.a != 1.0{
      Normalized_cube{
        a:self.b/self.a,   //A
        b:self.c/self.a,  //B
        c:self.d/self.a  //C
      }
    }
    else{
      Normalized_cube{
        a:self.b,
        b:self.c,
        c:self.d
      }
    }
  }

}

//step 2:find p and q:
pub trait Coefficients{
  fn p(&self)->f64;
  fn q(&self)->f64;
  fn depress(&self)->Depressed_cube;
}
impl Coefficients for Normalized_cube{
  
  fn p(&self)->f64{
    return self.b-(self.a.powi(2)/3.0);
  }

  fn q(&self)->f64{
    let first_part  = (2.0*self.a.powi(3))/27.0;
    let second_part = -((self.a*self.b)/3.0);
    return first_part+second_part+self.c;
  }

  fn depress(&self)->Depressed_cube{
    Depressed_cube{
      p:self.p(),
      q:self.q(),
      a:self.a
    }
  }

}

pub trait Discriminant{
  fn discriminant(&self)->f64;
}
impl Discriminant for Depressed_cube{
  fn discriminant(&self)->f64{
    let first_part=(self.q/2.0).powi(2);
    let second_part=(self.p/3.0).powi(3);
    return first_part+second_part;
  }
}
