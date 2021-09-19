fn main() {

    //とりあえず定義
    let gs = vec![17, 23, 24, 5434, 3];
    let mut fes = Vec::<FieldElement>::new();
    for g in gs {
        let fe_result = FieldElement::new(g, 25);
        match fe_result {
            Some(fe) => fes.push(fe),
            None => println!("{}", " NON"),
        }
    }
    //出力
    for fe in &fes {
        println!("{}", fe.to_str())
    }

    //NONEの時には終了
    let r1 = FieldElement::new(4, 5).expect("Err!!1");
    let r2 = FieldElement::new(3, 5).expect("NONE invalid fe");

    let r12 = FieldElement::new(4, 5);
    let r22 = FieldElement::new(24, 5);

    //println!(" {}=={} is {}", r12.to_str(),r22.to_str() , r12 == r22);
    
    //特に定義しなくても Option<T>のイコールで比較は可能のようだ
    println!(" check {}",  r12 == r22);

    let r3 = (&r1 + &r2).expect("eee");
    let r4 = (&r1 - &r2).expect("eee");
    let r5 = (&r1 * &r2).expect("eee");
    println!("{} + {} = {} ", r1, r2, r3);
    println!("{} - {} = {} ", r1, r2, r4);
    println!("{} * {} = {} ", r1, r2, r5);
    println!("{} * {} = {} ", r1, 7, &r1 * 7 );
    println!("{} * {} = {} ", 17, r1, 17 * &r1  );
    //println!(" heyhey {} " , r1+r2);

    println!(
        "FiledElement size_of is {}",
        std::mem::size_of::<FieldElement>()
    );
}

//Prime　割る数
//NUM割られる数
struct FieldElement {
    num: isize,
    prime: isize,
}


impl FieldElement {
    //文字列を表示させる
    fn to_str(&self) -> String {
        format!("f_{}({})",  self.prime ,self.num)
    }
    //条件に満たない場合はエラー文言を出すようにします。
    pub fn new(num: isize, prime: isize) -> Option<FieldElement> {
        if num >= prime || num < 0 {
            None
        } else {
            let fe = FieldElement {
                num: num,
                prime: prime,
            };
            Some(fe)
        }
    }
}

use std::cmp::PartialEq;

impl PartialEq<FieldElement> for FieldElement {
    fn eq(&self, rhs: &FieldElement) -> bool {
        let res = self.num == rhs.num && self.prime == rhs.prime;
        res
    }
}


use std::fmt;
impl fmt::Display for FieldElement {
    // This trait requires `fmt` with this exact signature.
    // このトレイトは`fmt`が想定通りのシグネチャであることを要求します。
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        // 必ず、第一の要素が出力されるようにしています。
        // `f`は`fmt::Result`を返します。これはオペレーションが成功したか否か
        // を表します。
        // `write!`は`println!`に非常によく似た文法を使用していることに注目。
        write!(f, "{}", self.to_str())
    }
}

use std::ops::Add;
impl Add<&FieldElement> for &FieldElement {
    type Output = Option<FieldElement>;

    fn add<'a>(self, rhs: &'a FieldElement) -> Self::Output {
        if self.prime != rhs.prime {
            //let _s = "cant add two numbers in diffrent Fields";
            //Err(&_s)
            None
        } else {
            let _num = (self.num + rhs.num) % self.prime;
            FieldElement::new(_num, self.prime)
            //let fe_result = FieldElement::new(g, 25);
        }
    }
}

use std::ops::Sub;
impl Sub<&FieldElement> for &FieldElement {
    type Output = Option<FieldElement>;


    fn sub<'a>(self, rhs: &'a FieldElement) -> Self::Output {
        if self.prime != rhs.prime {
           // let _s = "cant add two numbers in diffrent Fields";
            //Err(&_s)
            None
        } else {
            let _num = (self.num - rhs.num) % self.prime;
            let _num2 = if _num < 0 { self.prime + _num } else { _num };
            FieldElement::new(_num2, self.prime)
        }
    }
}

//掛け算
use std::ops::Mul;
impl Mul<&FieldElement> for &FieldElement {
    type Output = Option<FieldElement>;

    fn mul<'a>(self, rhs: &'a FieldElement) -> Self::Output {
        if self.prime != rhs.prime {
            //let _s = "cant add two numbers in diffrent Fields";
            None
        } else {
            let _num = (self.num * rhs.num) % self.prime;
            FieldElement::new(_num, self.prime)
        }
    }
}

// FieldElement * 3
impl Mul<isize> for &FieldElement {
    type Output = FieldElement;
    
    fn mul(self, rhs: isize) -> Self::Output {
        let _num = (self.num * rhs) % self.prime;
        FieldElement::new(_num, self.prime).expect("test")
    }
    
}
//3*FieldElement
impl Mul<&FieldElement> for isize {
    type Output = FieldElement;
    
    fn mul(self, rhs: &FieldElement) -> Self::Output {
        let _num = (self * rhs.num) % rhs.prime;
        FieldElement::new(_num, rhs.prime).expect("test")
    }
    
}

//use std::ops::BitXor

//OPTIONで演算を定義したい
//タプル構造体？


struct OpFileldElement(Option<FieldElement>);
use std::ops::Deref;
//use std::ops::DerefMut;

impl Deref for OpFileldElement {
    type Target = Option<FieldElement>;
    fn deref(&self) -> &Self::Target {
        //&self.
        let ss = &self.0;
        ss
    }
}

/*
impl OpFileldElement<FieldElement> {
    //文字列を表示させる
    fn to_str(&self) -> String {
        let ss = &self.0;
        match ss {
            None => format!("None"),
            Some(fe) =>         format!("f_{}({})",  fe.prime ,fe.num),
        }
        

    }
}
*/