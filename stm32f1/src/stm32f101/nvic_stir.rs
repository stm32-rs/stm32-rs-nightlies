#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    stir: STIR,
}
impl RegisterBlock {
    ///0x00 - Software trigger interrupt register
    #[inline(always)]
    pub const fn stir(&self) -> &STIR {
        &self.stir
    }
}
/**STIR (rw) register accessor: Software trigger interrupt register

You can [`read`](crate::Reg::read) this register and get [`stir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#NVIC_STIR:STIR)

For information about available fields see [`mod@stir`] module*/
pub type STIR = crate::Reg<stir::STIRrs>;
///Software trigger interrupt register
pub mod stir;
