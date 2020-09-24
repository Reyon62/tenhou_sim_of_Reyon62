extern crate rand;

use std::io;
use std::io::Write;
use std::collections::HashSet;
use rand::Rng;

struct Tehai {
    m: [i16; 9], 
    s: [i16; 9],
    p: [i16; 9],
    w: [i16; 4],
    y: [i16; 3],
}

impl Tehai {
    fn new() -> Self {
        Self {
            m: [0;9],
            s: [0;9],
            p: [0;9],
            w: [0;4],
            y: [0;3],
        }
    }
    
    fn cp(&self) -> Tehai {
        let mut copied = Tehai::new();
        copied.m = self.m.clone();
        copied.s = self.s.clone();
        copied.p = self.p.clone();
        copied.w = self.w.clone();
        copied.y = self.y.clone();
        copied
    }
    
    fn insert(&mut self, ins: i16) {
        if ins < 36_i16 {
            self.m[(ins%9) as usize] += 1; 
        } else if ins < 72_i16 {
            self.s[((ins-36)%9) as usize] += 1;
        } else if ins < 108_i16 {
            self.p[((ins-72)%9) as usize] += 1;
        } else if ins < 124_i16 {
            self.w[((ins-108)%4) as usize] += 1;
        } else {
            self.y[((ins-124)%3) as usize] += 1;
        }
    }
    
    fn sum(a: &[i16]) -> usize {
        let mut r = 0_i16;
        for i in a {
            r += *i;
        }
        r as usize
    }
    
    fn len(&self) -> usize {
        Tehai::sum(&self.m)+Tehai::sum(&self.s)+Tehai::sum(&self.p)+Tehai::sum(&self.w)+Tehai::sum(&self.y)
    }
    
    fn print(&self) {
        if Tehai::sum(&self.m) != 0 {
            for i in 0..9 as usize {
                for j in 0..self.m[i] as usize {
                    print!("{}", i + 1);
                }
            }
            print!("m");
        }
        
        if Tehai::sum(&self.s) != 0 {
            for i in 0..9 as usize {
                for j in 0..self.s[i] as usize {
                    print!("{}", i + 1);
                }
            }
            print!("s");
        }
        
        if Tehai::sum(&self.p) != 0 {
            for i in 0..9 as usize {
                for j in 0..self.p[i] as usize {
                    print!("{}", i + 1);
                }
            }
            print!("p");
        }
        
        if Tehai::sum(&self.w) != 0 {
                for j in 0..self.w[0] as usize {
                    print!("E"); // 東
                }
                for j in 0..self.w[1] as usize {
                    print!("S"); // 南
                }
                for j in 0..self.w[2] as usize {
                    print!("W"); // 西
                }
                for j in 0..self.w[3] as usize {
                    print!("N"); // 北
                }
        }
        
        if Tehai::sum(&self.y) != 0 {
            for j in 0..self.y[0] as usize {
                    print!("P"); // 東
                }
                for j in 0..self.y[1] as usize {
                    print!("F"); // 南
                }
                for j in 0..self.y[2] as usize {
                    print!("C"); // 西
                }
        }
        println!("");
        io::stdout().flush().unwrap();
    }
    
    fn iswh0(hand: &[i16]) -> bool {
        let mut flag = true;
        let mut a = hand[0];
        let mut b = hand[1];
        
        for i in 0..7 as usize {
            let r = a%3;
            if b >= r && hand[i+2] >= r {
                a = b-r;
                b = hand[i+2] - r;
            } else { 
                flag = false;
                break;
            }
        }
        
        if flag {
            if a%3 == 0 && b%3 == 0 {
                flag = true;
            } else {
                flag = false;
            }
        }
        flag
    }
    
    fn iswh2(hand: &mut [i16]) -> bool {
        let mut flag = false;
        
        let mut p = 0_i16;
        let mut ind = 0 as usize;
        while ind < hand.len() {
            p += (ind as i16)*hand[ind];
            ind += 1;
        }
        
        ind = ((p*2)%3) as usize;
        
        while ind < 9 {
            hand[ind] -= 2;
            if hand[ind] >= 0 {
                if Tehai::iswh0(hand) {
                    hand[ind] += 2;
                    flag = true;
                    break;
                }
            }
            hand[ind] += 2;
            ind += 3;
        }
        
        flag
    }
    
    fn islh(&self) -> bool {
        let mut flag = true;
        let mut cl = self.cp();
        let mut head:&str = "";
        
        for i in cl.m.iter() {
            if i%2 != 0 {
                flag = false;
            }
        }
        
        for i in cl.s.iter() {
            if i%2 != 0 {
                flag = false;
            }
        }
        
        for i in cl.p.iter() {
            if i%2 != 0 {
                flag = false;
            }
        }
        
        for i in cl.w.iter() {
            if i%2 != 0 {
                flag = false;
            }
        }
        
        for i in cl.y.iter() {
            if i%2 != 0 {
                flag = false;
            }
        }
        
        if !flag {
            flag = true;
            if Tehai::sum(&cl.m)%3 == 2 {
                if head == "" {
                    head = "m"
                } else {
                    flag = false;
                }
            } else if Tehai::sum(&cl.m)%3 == 1 {
                flag = false;
            }

            if Tehai::sum(&cl.s)%3 == 2 {
                if head == "" {
                    head = "s"
                } else {
                    flag = false;
                }
            } else if Tehai::sum(&cl.s)%3 == 1 {
                flag = false;
            }

            if Tehai::sum(&cl.p)%3 == 2 {
                if head == "" {
                    head = "p"
                } else {
                    flag = false;
                }
            } else if Tehai::sum(&cl.p)%3 == 1 {
                flag = false;
            }

            for i in 0..4 as usize {
                if cl.w[i]%3 == 2 {
                    if head == "" {
                        head = "w";
                    } else {
                        flag = false;
                    }
                } else if cl.w[i]%3 == 1 {
                    flag = false;
                }
            }

            for i in 0..3 as usize {
                if cl.y[i]%3 == 2 {
                    if head == "" {
                        head = "y";
                    } else {
                        flag = false;
                    }
                } else if cl.y[i]%3 == 1 {
                    flag = false;
                }
            }

            if head == "m" {
                if !(Tehai::iswh2(&mut cl.m)) {
                    flag = false;
                }
            } else {
                if !(Tehai::iswh0(&mut cl.m)) {
                    flag = false;
                }
            }

            if head == "s" {
                if !(Tehai::iswh2(&mut cl.s)) {
                    flag = false;
                }
            } else {
                if !(Tehai::iswh0(&mut cl.s)) {
                    flag = false;
                }
            }

            if head == "p" {
                if !(Tehai::iswh2(&mut cl.p)) {
                    flag = false;
                }
            } else {
                if !(Tehai::iswh0(&mut cl.p)) {
                    flag = false;
                }
            }
        }

        flag
    }
}

fn main() {
    let mut cnt:u128 = 1_u128;
    let mut rng = rand::thread_rng();
    loop {
        let mut set:HashSet<i16> = HashSet::new();
        let mut hai:Vec<i16> = Vec::new();
        loop {
            let num:i16 = rng.gen_range(0, 136);
            if set.insert(num) == true {
                hai.push(num);
            }
            if set.len() == 136 {
                break;
            }
        }
        
        let mut card = Tehai::new();
    
        for j in 0..3 {
            for i in 0..4 {
                card.insert(hai.pop().unwrap());
            }
            for i in 0..12 {
                hai.pop();
            }
        }

        for i in 0..2 {
            card.insert(hai.pop().unwrap());
        }
        if card.islh() {
            println!("{}回目で出ました。", cnt);
            card.print();
            break;
        }
        if cnt == u128::MAX {
            println!("失敗");
            break;
        }
        cnt+=1;
    }
    /*
    let mut card = Tehai::new();
    for i in 0..2 {
        card.insert(0_i16);
        card.insert(1_i16);
        card.insert(2_i16);
        card.insert(3_i16);
        card.insert(4_i16);
        card.insert(5_i16);
        card.insert(6_i16);
    }
    card.print();
    if card.islh() {
        card.print();
    }*/
}