use std::io::{self};

pub enum Paramator {
    Normal,
    Dead,
    Burning,
    Stuned,
    Sleep
}

pub enum Job {
    Attacker,
    Specialist,
    Deffencer,
    Shooter,
    Caster,
    Healer
}

pub struct Operator {
    pub name: String,
    pub job: Job,
    pub hit_point: i32,
    pub attack: f32,
    pub deffence: f32,
    pub magic_resistance: f32,
    pub speed: f32,
    pub target: u8,
    pub paramator: Paramator
}

pub struct Enemy {
    pub name: String,
    pub hit_point: i32,
    pub attack: f32,
    pub deffence: f32,
    pub magic_resistance: f32,
    pub speed: f32,
    pub target: u8,
    pub paramator: Paramator
}

pub struct Situation {
    layer: u8,
    coin: u32,
    fame: u32,
    exp: u32,
    leadersLevel: u8,
    num_of_tresure: u32,
    num_of_operator: u32
}

pub struct Tresure {
    name: String,
    id: u32,

}


impl Operator {
    pub fn command(&mut self, enemy :&mut Enemy ){
        println!("{}はどうする?", self.name);
        println!("1. 攻撃\n2. なにもしない");
        let mut select = String::new();
        io::stdin().read_line(&mut select).unwrap();
        if (select == "1".to_string()){

            self.attack(enemy);
        }
        if (select == "2".to_string()){

        }
    }

    pub fn attack(&mut self, enemy :&mut Enemy){
        println!("{}の攻撃！", self.name);
        match self.job {
            Job::Attacker => {
                let damage = self.attack - enemy.deffence;
                if (damage > self.attack * 0.05) {enemy.hit_point -= damage as i32;}
                else {enemy.hit_point -= (self.attack * 0.05) as i32;}
                self.death_judge();
            }
            Job::Caster => {
                let damage = self.attack * (1.0 - enemy.magic_resistance);
                if (damage > self.attack * 0.05) {enemy.hit_point -= damage as i32;}
                else {enemy.hit_point -= (self.attack * 0.05) as i32;}
                self.death_judge();
            }

            _ => {

            }
        }
    }

    fn death_judge(&mut self){
        if(self.hit_point < 0){
            println!("{}は倒れた！", self.name);
            self.hit_point = 0;
            self.paramator = Paramator::Dead;
        }
    }
}

