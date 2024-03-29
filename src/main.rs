use dotenv::dotenv;

struct Position {
    x: i32,
}

impl Position {
    pub fn new(x: i32) -> Position {
        Position { x }
    }
}

struct Environment {
    map: Vec<bool>,
}

impl Environment {
    pub fn new(map: Vec<bool>) -> Environment {
        Environment { map }
    }

    pub fn print(&self) {
        println!("Environment: ");
        for i in 0..self.map.len() {
            print!("{} ", self.map[i]);
        }
        println!("\n");
    }
}

struct PositionSensor {
    position: Position,
}

impl PositionSensor {

    pub fn new(position: Position) -> PositionSensor {
        PositionSensor { position }
    }

    pub fn update_position(&mut self, new_position: i32) {
        self.position.x = new_position;
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }
}

struct DirtySensor {
    position: Position,
}

impl DirtySensor {

    pub fn new(position: Position) -> DirtySensor {
        DirtySensor { position }
    }

    pub fn update_position(&mut self, new_position: i32) {
        self.position.x = new_position;
    }

    pub fn is_dirty(&self, env: &Environment) -> bool {
        env.map[self.position.x as usize]
    }
}


struct Actuator {
    position: Position,
    position_sensor: PositionSensor,
    dirty_sensor: DirtySensor,
    environment: Environment,
    historical: Vec<i32>,
    direction: bool
}

impl Actuator{
    pub fn new(position_sensor: PositionSensor, dirty_sensor: DirtySensor, environment: Environment) -> Actuator {
        Actuator {
            position: Position { x: position_sensor.position.x.clone() },
            position_sensor,
            dirty_sensor,
            environment,
            historical: Vec::new(),
            direction: true
        }
    }

    pub fn choose_side(&mut self) {
        if self.position_sensor.position.x == self.environment.map.len() as i32 - 1 {
            self.direction = false;
        } else if self.position_sensor.position.x == 0 {
            self.direction = true;
        }
    }

    pub fn move_robot(&mut self) {
        if self.direction {
            self.move_right();
        } else {
            self.move_left();
        }
    }
    fn move_right(&mut self){
        self.move_to(self.position_sensor.get_position().x + 1)
    }

    fn move_left(&mut self){
        self.move_to(self.position_sensor.get_position().x - 1)
    }

    pub fn move_to(&mut self, new_position: i32){

        self.position_sensor.update_position(new_position);
        self.dirty_sensor.update_position(new_position);
        self.position.x = self.position_sensor.get_position().x;

        if !self.historical.contains(&new_position) {
            self.historical.push(new_position);
        }
    }

    pub fn suck(&mut self) {
        if self.dirty_sensor.is_dirty(&self.environment) {
            self.environment.map[self.position_sensor.get_position().x as usize] = false;
        }
    }

    pub fn finished(&self) -> bool {
        self.historical.len() as i32 == self.environment.map.len() as i32
        &&
        !self.environment.map.contains(&true)
    }
}

fn main() {
    dotenv().ok();

    let position =  std::env::var("ROBOT_POSITION").unwrap().parse::<i32>().unwrap();
    let env_map = std::env::var("ENVIRONMENT_MAP").ok();

    let mut robot: Actuator = Actuator::new(PositionSensor::new(Position::new(position.clone())),
                                            DirtySensor::new(Position::new(position.clone())),
                                            Environment::new(Vec::from(env_map.clone().
                                                unwrap().split(",").map(|x| x.parse::<bool>().
                                                unwrap()).collect::<Vec<bool>>())));

    println!("Robot started");
    println!("Initial pos: {}", robot.position.x);
    robot.environment.print();

    while !robot.finished() {
        println!("Robot is at {}", robot.position.x);
        if robot.dirty_sensor.is_dirty(&robot.environment) {
            robot.suck();
            println!("Robot sucked {}", robot.position.x)
        }
        
        robot.choose_side();
        robot.move_robot();
        
    }
    println!("\nRobot finished");
    println!("Last pos: {}", robot.position.x);
    robot.environment.print();
}