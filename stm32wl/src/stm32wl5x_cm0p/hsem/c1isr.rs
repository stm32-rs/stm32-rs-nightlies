#[doc = "Register `C1ISR` reader"]
pub struct R(crate::R<C1ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Interrupt(N) semaphore n status bit before enable (mask)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF0_A {
    #[doc = "0: No interrupt pending"]
    NOTPENDING = 0,
    #[doc = "1: Interrupt pending"]
    PENDING = 1,
}
impl From<ISF0_A> for bool {
    #[inline(always)]
    fn from(variant: ISF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF0` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISF0_R(crate::FieldReader<bool, ISF0_A>);
impl ISF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF0_A {
        match self.bits {
            false => ISF0_A::NOTPENDING,
            true => ISF0_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == ISF0_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == ISF0_A::PENDING
    }
}
impl core::ops::Deref for ISF0_R {
    type Target = crate::FieldReader<bool, ISF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF1_A = ISF0_A;
#[doc = "Field `ISF1` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF1_R = ISF0_R;
#[doc = "Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF2_A = ISF0_A;
#[doc = "Field `ISF2` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF2_R = ISF0_R;
#[doc = "Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF3_A = ISF0_A;
#[doc = "Field `ISF3` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF3_R = ISF0_R;
#[doc = "Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF4_A = ISF0_A;
#[doc = "Field `ISF4` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF4_R = ISF0_R;
#[doc = "Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF5_A = ISF0_A;
#[doc = "Field `ISF5` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF5_R = ISF0_R;
#[doc = "Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF6_A = ISF0_A;
#[doc = "Field `ISF6` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF6_R = ISF0_R;
#[doc = "Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF7_A = ISF0_A;
#[doc = "Field `ISF7` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF7_R = ISF0_R;
#[doc = "Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF8_A = ISF0_A;
#[doc = "Field `ISF8` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF8_R = ISF0_R;
#[doc = "Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF9_A = ISF0_A;
#[doc = "Field `ISF9` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF9_R = ISF0_R;
#[doc = "Interrupt(N) semaphore n status bit before enable (mask)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF10_A {
    #[doc = "0: No interrupt pending"]
    NOTPENDING = 0,
    #[doc = "1: Interrupt pending"]
    PENDING = 1,
}
impl From<ISF10_A> for bool {
    #[inline(always)]
    fn from(variant: ISF10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF10` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISF10_R(crate::FieldReader<bool, ISF10_A>);
impl ISF10_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISF10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF10_A {
        match self.bits {
            false => ISF10_A::NOTPENDING,
            true => ISF10_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == ISF10_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == ISF10_A::PENDING
    }
}
impl core::ops::Deref for ISF10_R {
    type Target = crate::FieldReader<bool, ISF10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF11_A = ISF10_A;
#[doc = "Field `ISF11` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF11_R = ISF10_R;
#[doc = "Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF12_A = ISF10_A;
#[doc = "Field `ISF12` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF12_R = ISF10_R;
#[doc = "Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF13_A = ISF10_A;
#[doc = "Field `ISF13` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF13_R = ISF10_R;
#[doc = "Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF14_A = ISF10_A;
#[doc = "Field `ISF14` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF14_R = ISF10_R;
#[doc = "Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF15_A = ISF10_A;
#[doc = "Field `ISF15` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISF15_R = ISF10_R;
impl R {
    #[doc = "Bit 0 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf0(&self) -> ISF0_R {
        ISF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf1(&self) -> ISF1_R {
        ISF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf2(&self) -> ISF2_R {
        ISF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf3(&self) -> ISF3_R {
        ISF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf4(&self) -> ISF4_R {
        ISF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf5(&self) -> ISF5_R {
        ISF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf6(&self) -> ISF6_R {
        ISF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf7(&self) -> ISF7_R {
        ISF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf8(&self) -> ISF8_R {
        ISF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf9(&self) -> ISF9_R {
        ISF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf10(&self) -> ISF10_R {
        ISF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf11(&self) -> ISF11_R {
        ISF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf12(&self) -> ISF12_R {
        ISF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf13(&self) -> ISF13_R {
        ISF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf14(&self) -> ISF14_R {
        ISF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf15(&self) -> ISF15_R {
        ISF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "HSEM Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1isr](index.html) module"]
pub struct C1ISR_SPEC;
impl crate::RegisterSpec for C1ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1isr::R](R) reader structure"]
impl crate::Readable for C1ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets C1ISR to value 0"]
impl crate::Resettable for C1ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
