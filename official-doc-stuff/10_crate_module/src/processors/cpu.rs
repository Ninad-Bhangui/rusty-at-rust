use super::gpu; //Using Super for relative paths. Super refers to parent scope => use crate::processors::gpu;

pub struct Process {
    pub pid: u32,
}

impl Process {
    pub fn pass_process(self) {
        gpu::assign_to_gpu();
    }
}
