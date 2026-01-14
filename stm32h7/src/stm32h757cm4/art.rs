#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctr: CTR,
}
impl RegisterBlock {
    ///0x00 - control register
    #[inline(always)]
    pub const fn ctr(&self) -> &CTR {
        &self.ctr
    }
}
/**CTR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`ctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#ART:CTR)

For information about available fields see [`mod@ctr`] module*/
pub type CTR = crate::Reg<ctr::CTRrs>;
///control register
pub mod ctr;
