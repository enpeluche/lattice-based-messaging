pub fn extended_gcd(mut a: i64, mut b:i64) -> (i64, i64, i64){
    if a==0 { return (b, 0, 1);}

    let (mut ua , mut va) = (1, 0);
    let (mut ub , mut vb) = (0, 1);

    while b != 0 {
        let q = a / b;
        
        (a, b) = (b, a % b);

        (ua, ub) = (ub, ua - q * ub);
        (va, vb) = (vb, va - q * vb);
    }

    (a, ua, va)
}

pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}