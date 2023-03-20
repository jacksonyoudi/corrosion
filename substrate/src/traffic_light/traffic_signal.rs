// 枚举类
enum TrafficSignal {
    Red,
    Yellow,
    Green,
}

// 特质
trait TrafficLight {
    // 返回时间
    fn duration(&self) -> u8;
    // 返回什么信号灯
    fn signal(&self) -> TrafficSignal;
}


// 具体实现
impl TrafficLight for TrafficSignal {
    fn duration(&self) -> u8 {
        match self {
            TrafficSignal::Red => 30,
            TrafficSignal::Yellow => 3,
            TrafficSignal::Green => 45,
        }
    }

    // 当前是什么灯亮
    fn signal(&self) -> TrafficSignal {
        todo!()
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_test() {
        let red_light: TrafficSignal = TrafficSignal::Red;
        let yellow_light: TrafficSignal = TrafficSignal::Yellow;
        let green_light: TrafficSignal = TrafficSignal::Green;

        println!("Red light duration: {} seconds", red_light.duration());
        println!("Yellow light duration: {} seconds", yellow_light.duration());
        println!("Green light duration: {} seconds", green_light.duration());
    }
}
