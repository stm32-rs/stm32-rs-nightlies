#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dr: [DR; 10],
    rtccr: RTCCR,
    cr: CR,
    csr: CSR,
    _reserved4: [u8; 0x08],
    bkp_dr: [BKP_DR; 32],
}
impl RegisterBlock {
    ///0x00..0x28 - Backup data register (BKP_DR)
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `DR1` register.</div>
    #[inline(always)]
    pub const fn dr(&self, n: usize) -> &DR {
        &self.dr[n]
    }
    ///Iterator for array of:
    ///0x00..0x28 - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn dr_iter(&self) -> impl Iterator<Item = &DR> {
        self.dr.iter()
    }
    ///0x00 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn dr1(&self) -> &DR {
        self.dr(0)
    }
    ///0x04 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn dr2(&self) -> &DR {
        self.dr(1)
    }
    ///0x08 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn dr3(&self) -> &DR {
        self.dr(2)
    }
    ///0x0c - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn dr4(&self) -> &DR {
        self.dr(3)
    }
    ///0x10 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn dr5(&self) -> &DR {
        self.dr(4)
    }
    ///0x14 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn dr6(&self) -> &DR {
        self.dr(5)
    }
    ///0x18 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn dr7(&self) -> &DR {
        self.dr(6)
    }
    ///0x1c - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn dr8(&self) -> &DR {
        self.dr(7)
    }
    ///0x20 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn dr9(&self) -> &DR {
        self.dr(8)
    }
    ///0x24 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn dr10(&self) -> &DR {
        self.dr(9)
    }
    ///0x28 - RTC clock calibration register (BKP_RTCCR)
    #[inline(always)]
    pub const fn rtccr(&self) -> &RTCCR {
        &self.rtccr
    }
    ///0x2c - Backup control register (BKP_CR)
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x30 - BKP_CSR control/status register (BKP_CSR)
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x3c..0xbc - Backup data register (BKP_DR)
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BKP_DR11` register.</div>
    #[inline(always)]
    pub const fn bkp_dr(&self, n: usize) -> &BKP_DR {
        &self.bkp_dr[n]
    }
    ///Iterator for array of:
    ///0x3c..0xbc - Backup data register (BKP_DR)
    #[inline(always)]
    pub fn bkp_dr_iter(&self) -> impl Iterator<Item = &BKP_DR> {
        self.bkp_dr.iter()
    }
    ///0x3c - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr11(&self) -> &BKP_DR {
        self.bkp_dr(0)
    }
    ///0x40 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr12(&self) -> &BKP_DR {
        self.bkp_dr(1)
    }
    ///0x44 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr13(&self) -> &BKP_DR {
        self.bkp_dr(2)
    }
    ///0x48 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr14(&self) -> &BKP_DR {
        self.bkp_dr(3)
    }
    ///0x4c - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr15(&self) -> &BKP_DR {
        self.bkp_dr(4)
    }
    ///0x50 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr16(&self) -> &BKP_DR {
        self.bkp_dr(5)
    }
    ///0x54 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr17(&self) -> &BKP_DR {
        self.bkp_dr(6)
    }
    ///0x58 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr18(&self) -> &BKP_DR {
        self.bkp_dr(7)
    }
    ///0x5c - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr19(&self) -> &BKP_DR {
        self.bkp_dr(8)
    }
    ///0x60 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr20(&self) -> &BKP_DR {
        self.bkp_dr(9)
    }
    ///0x64 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr21(&self) -> &BKP_DR {
        self.bkp_dr(10)
    }
    ///0x68 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr22(&self) -> &BKP_DR {
        self.bkp_dr(11)
    }
    ///0x6c - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr23(&self) -> &BKP_DR {
        self.bkp_dr(12)
    }
    ///0x70 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr24(&self) -> &BKP_DR {
        self.bkp_dr(13)
    }
    ///0x74 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr25(&self) -> &BKP_DR {
        self.bkp_dr(14)
    }
    ///0x78 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr26(&self) -> &BKP_DR {
        self.bkp_dr(15)
    }
    ///0x7c - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr27(&self) -> &BKP_DR {
        self.bkp_dr(16)
    }
    ///0x80 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr28(&self) -> &BKP_DR {
        self.bkp_dr(17)
    }
    ///0x84 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr29(&self) -> &BKP_DR {
        self.bkp_dr(18)
    }
    ///0x88 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr30(&self) -> &BKP_DR {
        self.bkp_dr(19)
    }
    ///0x8c - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr31(&self) -> &BKP_DR {
        self.bkp_dr(20)
    }
    ///0x90 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr32(&self) -> &BKP_DR {
        self.bkp_dr(21)
    }
    ///0x94 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr33(&self) -> &BKP_DR {
        self.bkp_dr(22)
    }
    ///0x98 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr34(&self) -> &BKP_DR {
        self.bkp_dr(23)
    }
    ///0x9c - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr35(&self) -> &BKP_DR {
        self.bkp_dr(24)
    }
    ///0xa0 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr36(&self) -> &BKP_DR {
        self.bkp_dr(25)
    }
    ///0xa4 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr37(&self) -> &BKP_DR {
        self.bkp_dr(26)
    }
    ///0xa8 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr38(&self) -> &BKP_DR {
        self.bkp_dr(27)
    }
    ///0xac - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr39(&self) -> &BKP_DR {
        self.bkp_dr(28)
    }
    ///0xb0 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr40(&self) -> &BKP_DR {
        self.bkp_dr(29)
    }
    ///0xb4 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr41(&self) -> &BKP_DR {
        self.bkp_dr(30)
    }
    ///0xb8 - Backup data register (BKP_DR)
    #[inline(always)]
    pub const fn bkp_dr42(&self) -> &BKP_DR {
        self.bkp_dr(31)
    }
}
/**DR (rw) register accessor: Backup data register (BKP_DR)

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#BKP:DR[1])

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///Backup data register (BKP_DR)
pub mod dr;
/**BKP_DR (rw) register accessor: Backup data register (BKP_DR)

You can [`read`](crate::Reg::read) this register and get [`bkp_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#BKP:BKP_DR[11])

For information about available fields see [`mod@bkp_dr`] module*/
pub type BKP_DR = crate::Reg<bkp_dr::BKP_DRrs>;
///Backup data register (BKP_DR)
pub mod bkp_dr;
/**RTCCR (rw) register accessor: RTC clock calibration register (BKP_RTCCR)

You can [`read`](crate::Reg::read) this register and get [`rtccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#BKP:RTCCR)

For information about available fields see [`mod@rtccr`] module*/
pub type RTCCR = crate::Reg<rtccr::RTCCRrs>;
///RTC clock calibration register (BKP_RTCCR)
pub mod rtccr;
/**CR (rw) register accessor: Backup control register (BKP_CR)

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#BKP:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Backup control register (BKP_CR)
pub mod cr;
/**CSR (rw) register accessor: BKP_CSR control/status register (BKP_CSR)

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#BKP:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///BKP_CSR control/status register (BKP_CSR)
pub mod csr;
