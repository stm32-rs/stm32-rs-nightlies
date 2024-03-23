#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crc_dr: CRC_DR,
    crc_idr: CRC_IDR,
    crc_cr: CRC_CR,
    _reserved3: [u8; 0x04],
    crc_init: CRC_INIT,
    crc_pol: CRC_POL,
}
impl RegisterBlock {
    #[doc = "0x00 - CRC data register"]
    #[inline(always)]
    pub const fn crc_dr(&self) -> &CRC_DR {
        &self.crc_dr
    }
    #[doc = "0x04 - CRC independent data register"]
    #[inline(always)]
    pub const fn crc_idr(&self) -> &CRC_IDR {
        &self.crc_idr
    }
    #[doc = "0x08 - CRC control register"]
    #[inline(always)]
    pub const fn crc_cr(&self) -> &CRC_CR {
        &self.crc_cr
    }
    #[doc = "0x10 - CRC initial value"]
    #[inline(always)]
    pub const fn crc_init(&self) -> &CRC_INIT {
        &self.crc_init
    }
    #[doc = "0x14 - CRC polynomial"]
    #[inline(always)]
    pub const fn crc_pol(&self) -> &CRC_POL {
        &self.crc_pol
    }
}
#[doc = "CRC_DR (rw) register accessor: CRC data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_dr`]
module"]
pub type CRC_DR = crate::Reg<crc_dr::CRC_DRrs>;
#[doc = "CRC data register"]
pub mod crc_dr;
#[doc = "CRC_IDR (rw) register accessor: CRC independent data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_idr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_idr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_idr`]
module"]
pub type CRC_IDR = crate::Reg<crc_idr::CRC_IDRrs>;
#[doc = "CRC independent data register"]
pub mod crc_idr;
#[doc = "CRC_CR (rw) register accessor: CRC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_cr`]
module"]
pub type CRC_CR = crate::Reg<crc_cr::CRC_CRrs>;
#[doc = "CRC control register"]
pub mod crc_cr;
#[doc = "CRC_INIT (rw) register accessor: CRC initial value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_init`]
module"]
pub type CRC_INIT = crate::Reg<crc_init::CRC_INITrs>;
#[doc = "CRC initial value"]
pub mod crc_init;
#[doc = "CRC_POL (rw) register accessor: CRC polynomial\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_pol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_pol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_pol`]
module"]
pub type CRC_POL = crate::Reg<crc_pol::CRC_POLrs>;
#[doc = "CRC polynomial"]
pub mod crc_pol;
