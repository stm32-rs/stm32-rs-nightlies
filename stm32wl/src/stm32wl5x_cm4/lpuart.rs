#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_cr1: [u8; 4usize],
    #[doc = "0x04 - Control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - Control register 3"]
    pub cr3: CR3,
    #[doc = "0x0c - Baud rate register"]
    pub brr: BRR,
    _reserved4: [u8; 8usize],
    #[doc = "0x18 - Request register"]
    pub rqr: RQR,
    _reserved_5_isr: [u8; 4usize],
    #[doc = "0x20 - Interrupt flag clear register"]
    pub icr: ICR,
    #[doc = "0x24 - Receive data register"]
    pub rdr: RDR,
    #[doc = "0x28 - Transmit data register"]
    pub tdr: TDR,
    #[doc = "0x2c - Prescaler register"]
    pub presc: PRESC,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    #[inline(always)]
    pub fn cr1_disabled(&self) -> &CR1_DISABLED {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const CR1_DISABLED) }
    }
    #[doc = "0x00 - Control register 1"]
    #[inline(always)]
    pub fn cr1_disabled_mut(&self) -> &mut CR1_DISABLED {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut CR1_DISABLED) }
    }
    #[doc = "0x00 - Control register 1"]
    #[inline(always)]
    pub fn cr1_enabled(&self) -> &CR1_ENABLED {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const CR1_ENABLED) }
    }
    #[doc = "0x00 - Control register 1"]
    #[inline(always)]
    pub fn cr1_enabled_mut(&self) -> &mut CR1_ENABLED {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut CR1_ENABLED) }
    }
    #[doc = "0x1c - Interrupt and status register"]
    #[inline(always)]
    pub fn isr_disabled(&self) -> &ISR_DISABLED {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const ISR_DISABLED) }
    }
    #[doc = "0x1c - Interrupt and status register"]
    #[inline(always)]
    pub fn isr_disabled_mut(&self) -> &mut ISR_DISABLED {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut ISR_DISABLED) }
    }
    #[doc = "0x1c - Interrupt and status register"]
    #[inline(always)]
    pub fn isr_enabled(&self) -> &ISR_ENABLED {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const ISR_ENABLED) }
    }
    #[doc = "0x1c - Interrupt and status register"]
    #[inline(always)]
    pub fn isr_enabled_mut(&self) -> &mut ISR_ENABLED {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut ISR_ENABLED) }
    }
}
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1_enabled](cr1_enabled) module"]
pub type CR1_ENABLED = crate::Reg<u32, _CR1_ENABLED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1_ENABLED;
#[doc = "`read()` method returns [cr1_enabled::R](cr1_enabled::R) reader structure"]
impl crate::Readable for CR1_ENABLED {}
#[doc = "`write(|w| ..)` method takes [cr1_enabled::W](cr1_enabled::W) writer structure"]
impl crate::Writable for CR1_ENABLED {}
#[doc = "Control register 1"]
pub mod cr1_enabled;
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1_disabled](cr1_disabled) module"]
pub type CR1_DISABLED = crate::Reg<u32, _CR1_DISABLED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1_DISABLED;
#[doc = "`read()` method returns [cr1_disabled::R](cr1_disabled::R) reader structure"]
impl crate::Readable for CR1_DISABLED {}
#[doc = "`write(|w| ..)` method takes [cr1_disabled::W](cr1_disabled::W) writer structure"]
impl crate::Writable for CR1_DISABLED {}
#[doc = "Control register 1"]
pub mod cr1_disabled;
#[doc = "Control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](cr2) module"]
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
#[doc = "`read()` method returns [cr2::R](cr2::R) reader structure"]
impl crate::Readable for CR2 {}
#[doc = "`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure"]
impl crate::Writable for CR2 {}
#[doc = "Control register 2"]
pub mod cr2;
#[doc = "Control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](cr3) module"]
pub type CR3 = crate::Reg<u32, _CR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR3;
#[doc = "`read()` method returns [cr3::R](cr3::R) reader structure"]
impl crate::Readable for CR3 {}
#[doc = "`write(|w| ..)` method takes [cr3::W](cr3::W) writer structure"]
impl crate::Writable for CR3 {}
#[doc = "Control register 3"]
pub mod cr3;
#[doc = "Baud rate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brr](brr) module"]
pub type BRR = crate::Reg<u32, _BRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRR;
#[doc = "`read()` method returns [brr::R](brr::R) reader structure"]
impl crate::Readable for BRR {}
#[doc = "`write(|w| ..)` method takes [brr::W](brr::W) writer structure"]
impl crate::Writable for BRR {}
#[doc = "Baud rate register"]
pub mod brr;
#[doc = "Request register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rqr](rqr) module"]
pub type RQR = crate::Reg<u32, _RQR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RQR;
#[doc = "`write(|w| ..)` method takes [rqr::W](rqr::W) writer structure"]
impl crate::Writable for RQR {}
#[doc = "Request register"]
pub mod rqr;
#[doc = "Interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr_enabled](isr_enabled) module"]
pub type ISR_ENABLED = crate::Reg<u32, _ISR_ENABLED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR_ENABLED;
#[doc = "`read()` method returns [isr_enabled::R](isr_enabled::R) reader structure"]
impl crate::Readable for ISR_ENABLED {}
#[doc = "Interrupt and status register"]
pub mod isr_enabled;
#[doc = "Interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr_disabled](isr_disabled) module"]
pub type ISR_DISABLED = crate::Reg<u32, _ISR_DISABLED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR_DISABLED;
#[doc = "`read()` method returns [isr_disabled::R](isr_disabled::R) reader structure"]
impl crate::Readable for ISR_DISABLED {}
#[doc = "Interrupt and status register"]
pub mod isr_disabled;
#[doc = "Interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "Interrupt flag clear register"]
pub mod icr;
#[doc = "Receive data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdr](rdr) module"]
pub type RDR = crate::Reg<u32, _RDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDR;
#[doc = "`read()` method returns [rdr::R](rdr::R) reader structure"]
impl crate::Readable for RDR {}
#[doc = "Receive data register"]
pub mod rdr;
#[doc = "Transmit data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdr](tdr) module"]
pub type TDR = crate::Reg<u32, _TDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDR;
#[doc = "`read()` method returns [tdr::R](tdr::R) reader structure"]
impl crate::Readable for TDR {}
#[doc = "`write(|w| ..)` method takes [tdr::W](tdr::W) writer structure"]
impl crate::Writable for TDR {}
#[doc = "Transmit data register"]
pub mod tdr;
#[doc = "Prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presc](presc) module"]
pub type PRESC = crate::Reg<u32, _PRESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESC;
#[doc = "`read()` method returns [presc::R](presc::R) reader structure"]
impl crate::Readable for PRESC {}
#[doc = "`write(|w| ..)` method takes [presc::W](presc::W) writer structure"]
impl crate::Writable for PRESC {}
#[doc = "Prescaler register"]
pub mod presc;
