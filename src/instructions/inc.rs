use crate::CPU;

pub fn inc_sp() -> impl Fn(&mut CPU) {
    move |_: &mut CPU| {}
}

#[cfg(test)]
mod tests {
    use super::*;
}
