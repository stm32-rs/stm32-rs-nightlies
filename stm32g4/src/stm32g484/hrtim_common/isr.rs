#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Reader of field `BMPER`"]
pub type BMPER_R = crate::R<bool, bool>;
#[doc = "Reader of field `DLLRDY`"]
pub type DLLRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLT6`"]
pub type FLT6_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYSFLT`"]
pub type SYSFLT_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLT5`"]
pub type FLT5_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLT4`"]
pub type FLT4_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLT3`"]
pub type FLT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLT2`"]
pub type FLT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLT1`"]
pub type FLT1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 17 - Burst mode Period Interrupt Flag"]
    #[inline(always)]
    pub fn bmper(&self) -> BMPER_R {
        BMPER_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Flag"]
    #[inline(always)]
    pub fn dllrdy(&self) -> DLLRDY_R {
        DLLRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fault 6 Interrupt Flag"]
    #[inline(always)]
    pub fn flt6(&self) -> FLT6_R {
        FLT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - System Fault Interrupt Flag"]
    #[inline(always)]
    pub fn sysflt(&self) -> SYSFLT_R {
        SYSFLT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Flag"]
    #[inline(always)]
    pub fn flt5(&self) -> FLT5_R {
        FLT5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Flag"]
    #[inline(always)]
    pub fn flt4(&self) -> FLT4_R {
        FLT4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Flag"]
    #[inline(always)]
    pub fn flt3(&self) -> FLT3_R {
        FLT3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Flag"]
    #[inline(always)]
    pub fn flt2(&self) -> FLT2_R {
        FLT2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Fault 1 Interrupt Flag"]
    #[inline(always)]
    pub fn flt1(&self) -> FLT1_R {
        FLT1_R::new((self.bits & 0x01) != 0)
    }
}
