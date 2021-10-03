#[doc = "Register `C2MISR` reader"]
pub struct R(crate::R<C2MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2MISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "masked interrupt(N) semaphore n status bit after enable (mask)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MISF0_A {
    #[doc = "0: No interrupt pending after masking"]
    NOTPENDING = 0,
    #[doc = "1: Interrupt pending after masking"]
    PENDING = 1,
}
impl From<MISF0_A> for bool {
    #[inline(always)]
    fn from(variant: MISF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MISF0` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub struct MISF0_R(crate::FieldReader<bool, MISF0_A>);
impl MISF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        MISF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MISF0_A {
        match self.bits {
            false => MISF0_A::NOTPENDING,
            true => MISF0_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == MISF0_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == MISF0_A::PENDING
    }
}
impl core::ops::Deref for MISF0_R {
    type Target = crate::FieldReader<bool, MISF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF1_A = MISF0_A;
#[doc = "Field `MISF1` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF1_R = MISF0_R;
#[doc = "masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF2_A = MISF0_A;
#[doc = "Field `MISF2` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF2_R = MISF0_R;
#[doc = "masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF3_A = MISF0_A;
#[doc = "Field `MISF3` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF3_R = MISF0_R;
#[doc = "masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF4_A = MISF0_A;
#[doc = "Field `MISF4` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF4_R = MISF0_R;
#[doc = "masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF5_A = MISF0_A;
#[doc = "Field `MISF5` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF5_R = MISF0_R;
#[doc = "masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF6_A = MISF0_A;
#[doc = "Field `MISF6` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF6_R = MISF0_R;
#[doc = "masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF7_A = MISF0_A;
#[doc = "Field `MISF7` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF7_R = MISF0_R;
#[doc = "masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF8_A = MISF0_A;
#[doc = "Field `MISF8` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF8_R = MISF0_R;
#[doc = "masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF9_A = MISF0_A;
#[doc = "Field `MISF9` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF9_R = MISF0_R;
#[doc = "masked interrupt(N) semaphore n status bit after enable (mask)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MISF10_A {
    #[doc = "0: No interrupt pending after masking"]
    NOTPENDING = 0,
    #[doc = "1: Interrupt pending after masking"]
    PENDING = 1,
}
impl From<MISF10_A> for bool {
    #[inline(always)]
    fn from(variant: MISF10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MISF10` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub struct MISF10_R(crate::FieldReader<bool, MISF10_A>);
impl MISF10_R {
    pub(crate) fn new(bits: bool) -> Self {
        MISF10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MISF10_A {
        match self.bits {
            false => MISF10_A::NOTPENDING,
            true => MISF10_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == MISF10_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == MISF10_A::PENDING
    }
}
impl core::ops::Deref for MISF10_R {
    type Target = crate::FieldReader<bool, MISF10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF11_A = MISF10_A;
#[doc = "Field `MISF11` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF11_R = MISF10_R;
#[doc = "masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF12_A = MISF10_A;
#[doc = "Field `MISF12` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF12_R = MISF10_R;
#[doc = "masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF13_A = MISF10_A;
#[doc = "Field `MISF13` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF13_R = MISF10_R;
#[doc = "masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF14_A = MISF10_A;
#[doc = "Field `MISF14` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF14_R = MISF10_R;
#[doc = "masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF15_A = MISF10_A;
#[doc = "Field `MISF15` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF15_R = MISF10_R;
impl R {
    #[doc = "Bit 0 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf0(&self) -> MISF0_R {
        MISF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf1(&self) -> MISF1_R {
        MISF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf2(&self) -> MISF2_R {
        MISF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf3(&self) -> MISF3_R {
        MISF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf4(&self) -> MISF4_R {
        MISF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf5(&self) -> MISF5_R {
        MISF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf6(&self) -> MISF6_R {
        MISF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf7(&self) -> MISF7_R {
        MISF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf8(&self) -> MISF8_R {
        MISF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf9(&self) -> MISF9_R {
        MISF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf10(&self) -> MISF10_R {
        MISF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf11(&self) -> MISF11_R {
        MISF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf12(&self) -> MISF12_R {
        MISF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf13(&self) -> MISF13_R {
        MISF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf14(&self) -> MISF14_R {
        MISF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf15(&self) -> MISF15_R {
        MISF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "HSEM Masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2misr](index.html) module"]
pub struct C2MISR_SPEC;
impl crate::RegisterSpec for C2MISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2misr::R](R) reader structure"]
impl crate::Readable for C2MISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets C2MISR to value 0"]
impl crate::Resettable for C2MISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
