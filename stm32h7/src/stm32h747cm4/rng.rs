#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    dr: DR,
}
impl RegisterBlock {
    ///0x00 - RNG control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - RNG status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0.
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
}
/**CR (rw) register accessor: RNG control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#RNG:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///RNG control register
pub mod cr;
/**SR (rw) register accessor: RNG status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#RNG:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///RNG status register
pub mod sr;
/**DR (r) register accessor: The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0.

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#RNG:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0.
pub mod dr;
