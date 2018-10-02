pub mod adc_msg;
pub mod snap2_msg;

pub fn swap_phase_factor(input:&[i16])->Vec<i16>{
    let mut result=Vec::new();
    input.chunks(2).for_each(|x|{
        result.push(x[1]);
        result.push(x[2]);
    });
    result
}