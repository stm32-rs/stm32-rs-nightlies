#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x02],
    cfr: CFR,
    _reserved2: [u8; 0x02],
    sr: SR,
}
impl RegisterBlock {
    ///0x00 - Control register (WWDG_CR)
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - Configuration register (WWDG_CFR)
    #[inline(always)]
    pub const fn cfr(&self) -> &CFR {
        &self.cfr
    }
    ///0x08 - Status register (WWDG_SR)
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
}
/**CR (rw) register accessor: Control register (WWDG_CR)

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#WWDG:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Control register (WWDG_CR)
pub mod cr;
/**CFR (rw) register accessor: Configuration register (WWDG_CFR)

You can [`read`](crate::Reg::read) this register and get [`cfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#WWDG:CFR)

For information about available fields see [`mod@cfr`] module*/
pub type CFR = crate::Reg<cfr::CFRrs>;
///Configuration register (WWDG_CFR)
pub mod cfr;
/**SR (rw) register accessor: Status register (WWDG_SR)

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#WWDG:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///Status register (WWDG_SR)
pub mod sr;
