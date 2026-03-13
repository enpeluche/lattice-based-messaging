pub fn extended_gcd(a: i64, b:i64) -> (i64, i64, i64){
    let mut x = a;
    let mut y = b;
    
    if a==0 {
        return (b, 0, 1)
    }

    let mut ux = 1; let mut vx = 0;
    let mut uy = 0; let mut vy = 1;
    
    

    while y != 0 {
        let q = x / y;
        let r = x % y;
        

        let tmp = ux;
        ux = uy;
        uy = tmp-q*uy;

        let tmp = vx;
        vx = vy;
        vy = tmp - q*vy;
        
        x = y;
        y = r;
    }

    (x, ux, vx)
}