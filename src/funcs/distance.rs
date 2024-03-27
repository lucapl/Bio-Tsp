
pub fn manhattan(x1:f64,y1:f64,x2:f64,y2:f64) -> f64{
    (x1-x2).abs()+(y1-y2).abs()
}

pub fn euclidean(x1:f64,y1:f64,x2:f64,y2:f64) -> f64{
    ((x1-x2).powf(2.0)+(y1-y2).powf(2.0)).sqrt()
}

pub fn chebyshev(x1:f64,y1:f64,x2:f64,y2:f64) -> f64{
    (x1-x2).abs().max((y1-y2).abs())
}