#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    crh: CRH,
    crl: CRL,
    prlh: PRLH,
    prll: PRLL,
    divh: DIVH,
    divl: DIVL,
    cnth: CNTH,
    cntl: CNTL,
    alrh: ALRH,
    alrl: ALRL,
}
impl RegisterBlock {
    ///0x00 - RTC Control Register High
    #[inline(always)]
    pub const fn crh(&self) -> &CRH {
        &self.crh
    }
    ///0x04 - RTC Control Register Low
    #[inline(always)]
    pub const fn crl(&self) -> &CRL {
        &self.crl
    }
    ///0x08 - RTC Prescaler Load Register High
    #[inline(always)]
    pub const fn prlh(&self) -> &PRLH {
        &self.prlh
    }
    ///0x0c - RTC Prescaler Load Register Low
    #[inline(always)]
    pub const fn prll(&self) -> &PRLL {
        &self.prll
    }
    ///0x10 - RTC Prescaler Divider Register High
    #[inline(always)]
    pub const fn divh(&self) -> &DIVH {
        &self.divh
    }
    ///0x14 - RTC Prescaler Divider Register Low
    #[inline(always)]
    pub const fn divl(&self) -> &DIVL {
        &self.divl
    }
    ///0x18 - RTC Counter Register High
    #[inline(always)]
    pub const fn cnth(&self) -> &CNTH {
        &self.cnth
    }
    ///0x1c - RTC Counter Register Low
    #[inline(always)]
    pub const fn cntl(&self) -> &CNTL {
        &self.cntl
    }
    ///0x20 - RTC Alarm Register High
    #[inline(always)]
    pub const fn alrh(&self) -> &ALRH {
        &self.alrh
    }
    ///0x24 - RTC Alarm Register Low
    #[inline(always)]
    pub const fn alrl(&self) -> &ALRL {
        &self.alrl
    }
}
/**CRH (rw) register accessor: RTC Control Register High

You can [`read`](crate::Reg::read) this register and get [`crh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#RTC:CRH)

For information about available fields see [`mod@crh`] module*/
pub type CRH = crate::Reg<crh::CRHrs>;
///RTC Control Register High
pub mod crh;
/**CRL (rw) register accessor: RTC Control Register Low

You can [`read`](crate::Reg::read) this register and get [`crl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#RTC:CRL)

For information about available fields see [`mod@crl`] module*/
pub type CRL = crate::Reg<crl::CRLrs>;
///RTC Control Register Low
pub mod crl;
/**PRLH (w) register accessor: RTC Prescaler Load Register High

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prlh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#RTC:PRLH)

For information about available fields see [`mod@prlh`] module*/
pub type PRLH = crate::Reg<prlh::PRLHrs>;
///RTC Prescaler Load Register High
pub mod prlh;
/**PRLL (w) register accessor: RTC Prescaler Load Register Low

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prll::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#RTC:PRLL)

For information about available fields see [`mod@prll`] module*/
pub type PRLL = crate::Reg<prll::PRLLrs>;
///RTC Prescaler Load Register Low
pub mod prll;
/**DIVH (r) register accessor: RTC Prescaler Divider Register High

You can [`read`](crate::Reg::read) this register and get [`divh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#RTC:DIVH)

For information about available fields see [`mod@divh`] module*/
pub type DIVH = crate::Reg<divh::DIVHrs>;
///RTC Prescaler Divider Register High
pub mod divh;
/**DIVL (r) register accessor: RTC Prescaler Divider Register Low

You can [`read`](crate::Reg::read) this register and get [`divl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#RTC:DIVL)

For information about available fields see [`mod@divl`] module*/
pub type DIVL = crate::Reg<divl::DIVLrs>;
///RTC Prescaler Divider Register Low
pub mod divl;
/**CNTH (rw) register accessor: RTC Counter Register High

You can [`read`](crate::Reg::read) this register and get [`cnth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#RTC:CNTH)

For information about available fields see [`mod@cnth`] module*/
pub type CNTH = crate::Reg<cnth::CNTHrs>;
///RTC Counter Register High
pub mod cnth;
/**CNTL (rw) register accessor: RTC Counter Register Low

You can [`read`](crate::Reg::read) this register and get [`cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#RTC:CNTL)

For information about available fields see [`mod@cntl`] module*/
pub type CNTL = crate::Reg<cntl::CNTLrs>;
///RTC Counter Register Low
pub mod cntl;
/**ALRH (w) register accessor: RTC Alarm Register High

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#RTC:ALRH)

For information about available fields see [`mod@alrh`] module*/
pub type ALRH = crate::Reg<alrh::ALRHrs>;
///RTC Alarm Register High
pub mod alrh;
/**ALRL (w) register accessor: RTC Alarm Register Low

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#RTC:ALRL)

For information about available fields see [`mod@alrl`] module*/
pub type ALRL = crate::Reg<alrl::ALRLrs>;
///RTC Alarm Register Low
pub mod alrl;
