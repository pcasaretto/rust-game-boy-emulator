use super::super::*;

pub fn nop() -> impl Fn(&mut CPU) {
    move |_: &mut CPU| {}
}

#[cfg(test)]
mod tests {}
