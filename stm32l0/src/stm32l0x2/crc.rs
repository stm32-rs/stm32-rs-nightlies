#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_dr: [u8; 0x04],
    idr: IDR,
    cr: CR,
    _reserved3: [u8; 0x04],
    init: INIT,
    pol: POL,
}
impl RegisterBlock {
    #[doc = "0x00 - Data register - half-word sized"]
    #[inline(always)]
    pub const fn dr16(&self) -> &DR16 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Data register - byte sized"]
    #[inline(always)]
    pub const fn dr8(&self) -> &DR8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - Independent data register"]
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    #[doc = "0x08 - Control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x10 - Initial CRC value"]
    #[inline(always)]
    pub const fn init(&self) -> &INIT {
        &self.init
    }
    #[doc = "0x14 - polynomial"]
    #[inline(always)]
    pub const fn pol(&self) -> &POL {
        &self.pol
    }
}
#[doc = "DR (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "Data register"]
pub mod dr;
#[doc = "IDR (rw) register accessor: Independent data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
pub type IDR = crate::Reg<idr::IDRrs>;
#[doc = "Independent data register"]
pub mod idr;
#[doc = "CR (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Control register"]
pub mod cr;
#[doc = "INIT (rw) register accessor: Initial CRC value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@init`]
module"]
pub type INIT = crate::Reg<init::INITrs>;
#[doc = "Initial CRC value"]
pub mod init;
#[doc = "POL (rw) register accessor: polynomial\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pol`]
module"]
pub type POL = crate::Reg<pol::POLrs>;
#[doc = "polynomial"]
pub mod pol;
#[doc = "DR8 (rw) register accessor: Data register - byte sized\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr8`]
module"]
pub type DR8 = crate::Reg<dr8::DR8rs>;
#[doc = "Data register - byte sized"]
pub mod dr8;
#[doc = "DR16 (rw) register accessor: Data register - half-word sized\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr16`]
module"]
pub type DR16 = crate::Reg<dr16::DR16rs>;
#[doc = "Data register - half-word sized"]
pub mod dr16;
