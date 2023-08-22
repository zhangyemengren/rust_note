/// newtype 习惯用法在编译时保证向程序提供正确类型的值。
/// 例如，检查年龄的年龄验证函数必须指定 Years 类型的值。

struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}


impl Days {
    /// truncates partial years
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // println!("Old enough {}", old_enough(&age_days));

    let years = Years(42);
    let years_as_primitive_1: i64 = years.0; // 元组
    let Years(years_as_primitive_2) = years; // 解构
    println!("years_as_primitive_1: {}", years_as_primitive_1);
    println!("years_as_primitive_2: {}", years_as_primitive_2);
}