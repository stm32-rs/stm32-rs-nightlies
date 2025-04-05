#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cpacr: CPACR,
}
impl RegisterBlock {
    ///0x00 - Coprocessor access control register
    #[inline(always)]
    pub const fn cpacr(&self) -> &CPACR {
        &self.cpacr
    }
}
/**CPACR (rw) register accessor: Coprocessor access control register

You can [`read`](crate::Reg::read) this register and get [`cpacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#FPU_CPACR:CPACR)

For information about available fields see [`mod@cpacr`] module*/
pub type CPACR = crate::Reg<cpacr::CPACRrs>;
///Coprocessor access control register
pub mod cpacr;
