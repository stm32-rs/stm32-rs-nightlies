#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    actrl: ACTRL,
}
impl RegisterBlock {
    ///0x00 - Auxiliary control register
    #[inline(always)]
    pub const fn actrl(&self) -> &ACTRL {
        &self.actrl
    }
}
/**ACTRL (rw) register accessor: Auxiliary control register

You can [`read`](crate::Reg::read) this register and get [`actrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#SCB_ACTRL:ACTRL)

For information about available fields see [`mod@actrl`] module*/
pub type ACTRL = crate::Reg<actrl::ACTRLrs>;
///Auxiliary control register
pub mod actrl;
