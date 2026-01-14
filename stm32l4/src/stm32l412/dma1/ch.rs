#[repr(C)]
#[derive(Debug)]
///Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
pub struct CH {
    cr: CR,
    ndtr: NDTR,
    par: PAR,
    mar: MAR,
    _reserved_end: [u8; 0x04],
}
impl CH {
    ///0x00 - channel x configuration register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - channel x number of data register
    #[inline(always)]
    pub const fn ndtr(&self) -> &NDTR {
        &self.ndtr
    }
    ///0x08 - channel x peripheral address register
    #[inline(always)]
    pub const fn par(&self) -> &PAR {
        &self.par
    }
    ///0x0c - channel x memory address register
    #[inline(always)]
    pub const fn mar(&self) -> &MAR {
        &self.mar
    }
}
/**CR (rw) register accessor: channel x configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///channel x configuration register
pub mod cr;
/**NDTR (rw) register accessor: channel x number of data register

You can [`read`](crate::Reg::read) this register and get [`ndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ndtr`] module*/
pub type NDTR = crate::Reg<ndtr::NDTRrs>;
///channel x number of data register
pub mod ndtr;
/**PAR (rw) register accessor: channel x peripheral address register

You can [`read`](crate::Reg::read) this register and get [`par::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`par::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@par`] module*/
pub type PAR = crate::Reg<par::PARrs>;
///channel x peripheral address register
pub mod par;
/**MAR (rw) register accessor: channel x memory address register

You can [`read`](crate::Reg::read) this register and get [`mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mar`] module*/
pub type MAR = crate::Reg<mar::MARrs>;
///channel x memory address register
pub mod mar;
