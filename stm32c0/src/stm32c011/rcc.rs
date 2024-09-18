#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    icscr: ICSCR,
    cfgr: CFGR,
    _reserved3: [u8; 0x0c],
    cier: CIER,
    cifr: CIFR,
    cicr: CICR,
    ioprstr: IOPRSTR,
    ahbrstr: AHBRSTR,
    apbrstr1: APBRSTR1,
    apbrstr2: APBRSTR2,
    iopenr: IOPENR,
    ahbenr: AHBENR,
    apbenr1: APBENR1,
    apbenr2: APBENR2,
    iopsmenr: IOPSMENR,
    ahbsmenr: AHBSMENR,
    apbsmenr1: APBSMENR1,
    apbsmenr2: APBSMENR2,
    ccipr: CCIPR,
    _reserved19: [u8; 0x04],
    csr1: CSR1,
    csr2: CSR2,
}
impl RegisterBlock {
    ///0x00 - RCC clock control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - RCC internal clock source calibration register
    #[inline(always)]
    pub const fn icscr(&self) -> &ICSCR {
        &self.icscr
    }
    ///0x08 - RCC clock configuration register
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x18 - RCC clock interrupt enable register
    #[inline(always)]
    pub const fn cier(&self) -> &CIER {
        &self.cier
    }
    ///0x1c - RCC clock interrupt flag register
    #[inline(always)]
    pub const fn cifr(&self) -> &CIFR {
        &self.cifr
    }
    ///0x20 - RCC clock interrupt clear register
    #[inline(always)]
    pub const fn cicr(&self) -> &CICR {
        &self.cicr
    }
    ///0x24 - RCC I/O port reset register
    #[inline(always)]
    pub const fn ioprstr(&self) -> &IOPRSTR {
        &self.ioprstr
    }
    ///0x28 - RCC AHB peripheral reset register
    #[inline(always)]
    pub const fn ahbrstr(&self) -> &AHBRSTR {
        &self.ahbrstr
    }
    ///0x2c - RCC APB peripheral reset register 1
    #[inline(always)]
    pub const fn apbrstr1(&self) -> &APBRSTR1 {
        &self.apbrstr1
    }
    ///0x30 - RCC APB peripheral reset register 2
    #[inline(always)]
    pub const fn apbrstr2(&self) -> &APBRSTR2 {
        &self.apbrstr2
    }
    ///0x34 - RCC I/O port clock enable register
    #[inline(always)]
    pub const fn iopenr(&self) -> &IOPENR {
        &self.iopenr
    }
    ///0x38 - RCC AHB peripheral clock enable register
    #[inline(always)]
    pub const fn ahbenr(&self) -> &AHBENR {
        &self.ahbenr
    }
    ///0x3c - RCC APB peripheral clock enable register 1
    #[inline(always)]
    pub const fn apbenr1(&self) -> &APBENR1 {
        &self.apbenr1
    }
    ///0x40 - RCC APB peripheral clock enable register 2
    #[inline(always)]
    pub const fn apbenr2(&self) -> &APBENR2 {
        &self.apbenr2
    }
    ///0x44 - RCC I/O port in Sleep mode clock enable register
    #[inline(always)]
    pub const fn iopsmenr(&self) -> &IOPSMENR {
        &self.iopsmenr
    }
    ///0x48 - RCC AHB peripheral clock enable in Sleep/Stop mode register
    #[inline(always)]
    pub const fn ahbsmenr(&self) -> &AHBSMENR {
        &self.ahbsmenr
    }
    ///0x4c - RCC APB peripheral clock enable in Sleep/Stop mode register 1
    #[inline(always)]
    pub const fn apbsmenr1(&self) -> &APBSMENR1 {
        &self.apbsmenr1
    }
    ///0x50 - RCC APB peripheral clock enable in Sleep/Stop mode register 2
    #[inline(always)]
    pub const fn apbsmenr2(&self) -> &APBSMENR2 {
        &self.apbsmenr2
    }
    ///0x54 - RCC peripherals independent clock configuration register
    #[inline(always)]
    pub const fn ccipr(&self) -> &CCIPR {
        &self.ccipr
    }
    ///0x5c - RCC control/status register 1
    #[inline(always)]
    pub const fn csr1(&self) -> &CSR1 {
        &self.csr1
    }
    ///0x60 - RCC control/status register 2
    #[inline(always)]
    pub const fn csr2(&self) -> &CSR2 {
        &self.csr2
    }
}
/**CR (rw) register accessor: RCC clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:CR)

For information about available fields see [`mod@cr`]
module*/
pub type CR = crate::Reg<cr::CRrs>;
///RCC clock control register
pub mod cr;
/**ICSCR (rw) register accessor: RCC internal clock source calibration register

You can [`read`](crate::Reg::read) this register and get [`icscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:ICSCR)

For information about available fields see [`mod@icscr`]
module*/
pub type ICSCR = crate::Reg<icscr::ICSCRrs>;
///RCC internal clock source calibration register
pub mod icscr;
/**CFGR (rw) register accessor: RCC clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:CFGR)

For information about available fields see [`mod@cfgr`]
module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///RCC clock configuration register
pub mod cfgr;
/**CIER (rw) register accessor: RCC clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:CIER)

For information about available fields see [`mod@cier`]
module*/
pub type CIER = crate::Reg<cier::CIERrs>;
///RCC clock interrupt enable register
pub mod cier;
/**CIFR (r) register accessor: RCC clock interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:CIFR)

For information about available fields see [`mod@cifr`]
module*/
pub type CIFR = crate::Reg<cifr::CIFRrs>;
///RCC clock interrupt flag register
pub mod cifr;
/**CICR (w) register accessor: RCC clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:CICR)

For information about available fields see [`mod@cicr`]
module*/
pub type CICR = crate::Reg<cicr::CICRrs>;
///RCC clock interrupt clear register
pub mod cicr;
/**IOPRSTR (rw) register accessor: RCC I/O port reset register

You can [`read`](crate::Reg::read) this register and get [`ioprstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioprstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:IOPRSTR)

For information about available fields see [`mod@ioprstr`]
module*/
pub type IOPRSTR = crate::Reg<ioprstr::IOPRSTRrs>;
///RCC I/O port reset register
pub mod ioprstr;
/**AHBRSTR (rw) register accessor: RCC AHB peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:AHBRSTR)

For information about available fields see [`mod@ahbrstr`]
module*/
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTRrs>;
///RCC AHB peripheral reset register
pub mod ahbrstr;
/**APBRSTR1 (rw) register accessor: RCC APB peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`apbrstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:APBRSTR1)

For information about available fields see [`mod@apbrstr1`]
module*/
pub type APBRSTR1 = crate::Reg<apbrstr1::APBRSTR1rs>;
///RCC APB peripheral reset register 1
pub mod apbrstr1;
/**APBRSTR2 (rw) register accessor: RCC APB peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`apbrstr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrstr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:APBRSTR2)

For information about available fields see [`mod@apbrstr2`]
module*/
pub type APBRSTR2 = crate::Reg<apbrstr2::APBRSTR2rs>;
///RCC APB peripheral reset register 2
pub mod apbrstr2;
/**IOPENR (rw) register accessor: RCC I/O port clock enable register

You can [`read`](crate::Reg::read) this register and get [`iopenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:IOPENR)

For information about available fields see [`mod@iopenr`]
module*/
pub type IOPENR = crate::Reg<iopenr::IOPENRrs>;
///RCC I/O port clock enable register
pub mod iopenr;
/**AHBENR (rw) register accessor: RCC AHB peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahbenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:AHBENR)

For information about available fields see [`mod@ahbenr`]
module*/
pub type AHBENR = crate::Reg<ahbenr::AHBENRrs>;
///RCC AHB peripheral clock enable register
pub mod ahbenr;
/**APBENR1 (rw) register accessor: RCC APB peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`apbenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:APBENR1)

For information about available fields see [`mod@apbenr1`]
module*/
pub type APBENR1 = crate::Reg<apbenr1::APBENR1rs>;
///RCC APB peripheral clock enable register 1
pub mod apbenr1;
/**APBENR2 (rw) register accessor: RCC APB peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`apbenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:APBENR2)

For information about available fields see [`mod@apbenr2`]
module*/
pub type APBENR2 = crate::Reg<apbenr2::APBENR2rs>;
///RCC APB peripheral clock enable register 2
pub mod apbenr2;
/**IOPSMENR (rw) register accessor: RCC I/O port in Sleep mode clock enable register

You can [`read`](crate::Reg::read) this register and get [`iopsmenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopsmenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:IOPSMENR)

For information about available fields see [`mod@iopsmenr`]
module*/
pub type IOPSMENR = crate::Reg<iopsmenr::IOPSMENRrs>;
///RCC I/O port in Sleep mode clock enable register
pub mod iopsmenr;
/**AHBSMENR (rw) register accessor: RCC AHB peripheral clock enable in Sleep/Stop mode register

You can [`read`](crate::Reg::read) this register and get [`ahbsmenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsmenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:AHBSMENR)

For information about available fields see [`mod@ahbsmenr`]
module*/
pub type AHBSMENR = crate::Reg<ahbsmenr::AHBSMENRrs>;
///RCC AHB peripheral clock enable in Sleep/Stop mode register
pub mod ahbsmenr;
/**APBSMENR1 (rw) register accessor: RCC APB peripheral clock enable in Sleep/Stop mode register 1

You can [`read`](crate::Reg::read) this register and get [`apbsmenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbsmenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:APBSMENR1)

For information about available fields see [`mod@apbsmenr1`]
module*/
pub type APBSMENR1 = crate::Reg<apbsmenr1::APBSMENR1rs>;
///RCC APB peripheral clock enable in Sleep/Stop mode register 1
pub mod apbsmenr1;
/**APBSMENR2 (rw) register accessor: RCC APB peripheral clock enable in Sleep/Stop mode register 2

You can [`read`](crate::Reg::read) this register and get [`apbsmenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbsmenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:APBSMENR2)

For information about available fields see [`mod@apbsmenr2`]
module*/
pub type APBSMENR2 = crate::Reg<apbsmenr2::APBSMENR2rs>;
///RCC APB peripheral clock enable in Sleep/Stop mode register 2
pub mod apbsmenr2;
/**CCIPR (rw) register accessor: RCC peripherals independent clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:CCIPR)

For information about available fields see [`mod@ccipr`]
module*/
pub type CCIPR = crate::Reg<ccipr::CCIPRrs>;
///RCC peripherals independent clock configuration register
pub mod ccipr;
/**CSR1 (rw) register accessor: RCC control/status register 1

You can [`read`](crate::Reg::read) this register and get [`csr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:CSR1)

For information about available fields see [`mod@csr1`]
module*/
pub type CSR1 = crate::Reg<csr1::CSR1rs>;
///RCC control/status register 1
pub mod csr1;
/**CSR2 (rw) register accessor: RCC control/status register 2

You can [`read`](crate::Reg::read) this register and get [`csr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:CSR2)

For information about available fields see [`mod@csr2`]
module*/
pub type CSR2 = crate::Reg<csr2::CSR2rs>;
///RCC control/status register 2
pub mod csr2;
