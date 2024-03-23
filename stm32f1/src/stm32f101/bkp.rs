#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dr: [DR; 10],
    rtccr: RTCCR,
    cr: CR,
    csr: CSR,
    _reserved4: [u8; 0x08],
    bkp_dr: [BKP_DR; 32],
}
impl RegisterBlock {
    #[doc = "0x00..0x28 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr(&self, n: usize) -> &DR {
        &self.dr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x28 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn dr_iter(&self) -> impl Iterator<Item = &DR> {
        self.dr.iter()
    }
    #[doc = "0x00 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr1(&self) -> &DR {
        self.dr(0)
    }
    #[doc = "0x04 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr2(&self) -> &DR {
        self.dr(1)
    }
    #[doc = "0x08 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr3(&self) -> &DR {
        self.dr(2)
    }
    #[doc = "0x0c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr4(&self) -> &DR {
        self.dr(3)
    }
    #[doc = "0x10 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr5(&self) -> &DR {
        self.dr(4)
    }
    #[doc = "0x14 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr6(&self) -> &DR {
        self.dr(5)
    }
    #[doc = "0x18 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr7(&self) -> &DR {
        self.dr(6)
    }
    #[doc = "0x1c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr8(&self) -> &DR {
        self.dr(7)
    }
    #[doc = "0x20 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr9(&self) -> &DR {
        self.dr(8)
    }
    #[doc = "0x24 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr10(&self) -> &DR {
        self.dr(9)
    }
    #[doc = "0x28 - RTC clock calibration register (BKP_RTCCR)"]
    #[inline(always)]
    pub const fn rtccr(&self) -> &RTCCR {
        &self.rtccr
    }
    #[doc = "0x2c - Backup control register (BKP_CR)"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x30 - BKP_CSR control/status register (BKP_CSR)"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x3c..0xbc - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr(&self, n: usize) -> &BKP_DR {
        &self.bkp_dr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3c..0xbc - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr_iter(&self) -> impl Iterator<Item = &BKP_DR> {
        self.bkp_dr.iter()
    }
    #[doc = "0x3c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr11(&self) -> &BKP_DR {
        self.bkp_dr(0)
    }
    #[doc = "0x40 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr12(&self) -> &BKP_DR {
        self.bkp_dr(1)
    }
    #[doc = "0x44 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr13(&self) -> &BKP_DR {
        self.bkp_dr(2)
    }
    #[doc = "0x48 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr14(&self) -> &BKP_DR {
        self.bkp_dr(3)
    }
    #[doc = "0x4c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr15(&self) -> &BKP_DR {
        self.bkp_dr(4)
    }
    #[doc = "0x50 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr16(&self) -> &BKP_DR {
        self.bkp_dr(5)
    }
    #[doc = "0x54 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr17(&self) -> &BKP_DR {
        self.bkp_dr(6)
    }
    #[doc = "0x58 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr18(&self) -> &BKP_DR {
        self.bkp_dr(7)
    }
    #[doc = "0x5c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr19(&self) -> &BKP_DR {
        self.bkp_dr(8)
    }
    #[doc = "0x60 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr20(&self) -> &BKP_DR {
        self.bkp_dr(9)
    }
    #[doc = "0x64 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr21(&self) -> &BKP_DR {
        self.bkp_dr(10)
    }
    #[doc = "0x68 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr22(&self) -> &BKP_DR {
        self.bkp_dr(11)
    }
    #[doc = "0x6c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr23(&self) -> &BKP_DR {
        self.bkp_dr(12)
    }
    #[doc = "0x70 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr24(&self) -> &BKP_DR {
        self.bkp_dr(13)
    }
    #[doc = "0x74 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr25(&self) -> &BKP_DR {
        self.bkp_dr(14)
    }
    #[doc = "0x78 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr26(&self) -> &BKP_DR {
        self.bkp_dr(15)
    }
    #[doc = "0x7c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr27(&self) -> &BKP_DR {
        self.bkp_dr(16)
    }
    #[doc = "0x80 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr28(&self) -> &BKP_DR {
        self.bkp_dr(17)
    }
    #[doc = "0x84 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr29(&self) -> &BKP_DR {
        self.bkp_dr(18)
    }
    #[doc = "0x88 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr30(&self) -> &BKP_DR {
        self.bkp_dr(19)
    }
    #[doc = "0x8c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr31(&self) -> &BKP_DR {
        self.bkp_dr(20)
    }
    #[doc = "0x90 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr32(&self) -> &BKP_DR {
        self.bkp_dr(21)
    }
    #[doc = "0x94 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr33(&self) -> &BKP_DR {
        self.bkp_dr(22)
    }
    #[doc = "0x98 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr34(&self) -> &BKP_DR {
        self.bkp_dr(23)
    }
    #[doc = "0x9c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr35(&self) -> &BKP_DR {
        self.bkp_dr(24)
    }
    #[doc = "0xa0 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr36(&self) -> &BKP_DR {
        self.bkp_dr(25)
    }
    #[doc = "0xa4 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr37(&self) -> &BKP_DR {
        self.bkp_dr(26)
    }
    #[doc = "0xa8 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr38(&self) -> &BKP_DR {
        self.bkp_dr(27)
    }
    #[doc = "0xac - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr39(&self) -> &BKP_DR {
        self.bkp_dr(28)
    }
    #[doc = "0xb0 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr40(&self) -> &BKP_DR {
        self.bkp_dr(29)
    }
    #[doc = "0xb4 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr41(&self) -> &BKP_DR {
        self.bkp_dr(30)
    }
    #[doc = "0xb8 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn bkp_dr42(&self) -> &BKP_DR {
        self.bkp_dr(31)
    }
}
#[doc = "DR (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr;
#[doc = "BKP_DR (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp_dr`]
module"]
pub type BKP_DR = crate::Reg<bkp_dr::BKP_DRrs>;
#[doc = "Backup data register (BKP_DR)"]
pub mod bkp_dr;
#[doc = "RTCCR (rw) register accessor: RTC clock calibration register (BKP_RTCCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccr`]
module"]
pub type RTCCR = crate::Reg<rtccr::RTCCRrs>;
#[doc = "RTC clock calibration register (BKP_RTCCR)"]
pub mod rtccr;
#[doc = "CR (rw) register accessor: Backup control register (BKP_CR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Backup control register (BKP_CR)"]
pub mod cr;
#[doc = "CSR (rw) register accessor: BKP_CSR control/status register (BKP_CSR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSRrs>;
#[doc = "BKP_CSR control/status register (BKP_CSR)"]
pub mod csr;
