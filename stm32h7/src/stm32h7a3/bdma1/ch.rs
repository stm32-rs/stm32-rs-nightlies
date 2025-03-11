#[repr(C)]
#[derive(Debug)]
///Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR? registers
pub struct CH {
    cr: CR,
    ndtr: NDTR,
    par: PAR,
    m0ar: M0AR,
    m1ar: M1AR,
}
impl CH {
    ///0x00 - DMA channel x configuration register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - DMA channel x number of data register
    #[inline(always)]
    pub const fn ndtr(&self) -> &NDTR {
        &self.ndtr
    }
    ///0x08 - This register must not be written when the channel is enabled.
    #[inline(always)]
    pub const fn par(&self) -> &PAR {
        &self.par
    }
    ///0x0c - This register must not be written when the channel is enabled.
    #[inline(always)]
    pub const fn m0ar(&self) -> &M0AR {
        &self.m0ar
    }
    ///0x10 - This register must not be written when the channel is enabled
    #[inline(always)]
    pub const fn m1ar(&self) -> &M1AR {
        &self.m1ar
    }
}
/**CR (rw) register accessor: DMA channel x configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///DMA channel x configuration register
pub mod cr;
/**NDTR (rw) register accessor: DMA channel x number of data register

You can [`read`](crate::Reg::read) this register and get [`ndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ndtr`] module*/
pub type NDTR = crate::Reg<ndtr::NDTRrs>;
///DMA channel x number of data register
pub mod ndtr;
/**PAR (rw) register accessor: This register must not be written when the channel is enabled.

You can [`read`](crate::Reg::read) this register and get [`par::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`par::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@par`] module*/
pub type PAR = crate::Reg<par::PARrs>;
///This register must not be written when the channel is enabled.
pub mod par;
/**M0AR (rw) register accessor: This register must not be written when the channel is enabled.

You can [`read`](crate::Reg::read) this register and get [`m0ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@m0ar`] module*/
pub type M0AR = crate::Reg<m0ar::M0ARrs>;
///This register must not be written when the channel is enabled.
pub mod m0ar;
/**M1AR (rw) register accessor: This register must not be written when the channel is enabled

You can [`read`](crate::Reg::read) this register and get [`m1ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@m1ar`] module*/
pub type M1AR = crate::Reg<m1ar::M1ARrs>;
///This register must not be written when the channel is enabled
pub mod m1ar;
