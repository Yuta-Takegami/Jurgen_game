mod lib;

fn main() {
    let mut jurgen = crate::lib::Operator{
        name: String::from("Jurgen"),
        job: crate::lib::Job::Attacker,
        hit_point: 2000,
        attack: 700.0,
        deffence: 500.0,
        magic_resistance: 10.0,
        speed: 100.0,
        target: 0,
        paramator: crate::lib::Paramator::Normal
    };

    let mut killer = crate::lib::Enemy{
        name: String::from("Killer"),
        hit_point: 1500,
        attack: 900.0,
        deffence: 200.0,
        magic_resistance: 0.0,
        speed: 300.0,
        target: 0,
        paramator: crate::lib::Paramator::Normal
    };



    println!("{}が現れた", killer.name);
    while true {
        if(jurgen.speed > killer.speed){
            println!("{}はどうする?", jurgen.name);
        }
        
    }
    


    
}
