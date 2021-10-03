#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Transfer error (TE) flag for channel x"]
pub type TEIF7_A = TEIF0_A;
#[doc = "Field `TEIF7` reader - Transfer error (TE) flag for channel x"]
pub type TEIF7_R = TEIF0_R;
#[doc = "Half transfer (HT) flag for channel x"]
pub type HTIF7_A = HTIF0_A;
#[doc = "Field `HTIF7` reader - Half transfer (HT) flag for channel x"]
pub type HTIF7_R = HTIF0_R;
#[doc = "Transfer complete (TC) flag for channel x"]
pub type TCIF7_A = TCIF0_A;
#[doc = "Field `TCIF7` reader - Transfer complete (TC) flag for channel x"]
pub type TCIF7_R = TCIF0_R;
#[doc = "Global interrupt flag for channel x"]
pub type GIF7_A = GIF0_A;
#[doc = "Field `GIF7` reader - Global interrupt flag for channel x"]
pub type GIF7_R = GIF0_R;
#[doc = "Transfer error (TE) flag for channel x"]
pub type TEIF6_A = TEIF0_A;
#[doc = "Field `TEIF6` reader - Transfer error (TE) flag for channel x"]
pub type TEIF6_R = TEIF0_R;
#[doc = "Half transfer (HT) flag for channel x"]
pub type HTIF6_A = HTIF0_A;
#[doc = "Field `HTIF6` reader - Half transfer (HT) flag for channel x"]
pub type HTIF6_R = HTIF0_R;
#[doc = "Transfer complete (TC) flag for channel x"]
pub type TCIF6_A = TCIF0_A;
#[doc = "Field `TCIF6` reader - Transfer complete (TC) flag for channel x"]
pub type TCIF6_R = TCIF0_R;
#[doc = "Global interrupt flag for channel x"]
pub type GIF6_A = GIF0_A;
#[doc = "Field `GIF6` reader - Global interrupt flag for channel x"]
pub type GIF6_R = GIF0_R;
#[doc = "Transfer error (TE) flag for channel x"]
pub type TEIF5_A = TEIF0_A;
#[doc = "Field `TEIF5` reader - Transfer error (TE) flag for channel x"]
pub type TEIF5_R = TEIF0_R;
#[doc = "Half transfer (HT) flag for channel x"]
pub type HTIF5_A = HTIF0_A;
#[doc = "Field `HTIF5` reader - Half transfer (HT) flag for channel x"]
pub type HTIF5_R = HTIF0_R;
#[doc = "Transfer complete (TC) flag for channel x"]
pub type TCIF5_A = TCIF0_A;
#[doc = "Field `TCIF5` reader - Transfer complete (TC) flag for channel x"]
pub type TCIF5_R = TCIF0_R;
#[doc = "Global interrupt flag for channel x"]
pub type GIF5_A = GIF0_A;
#[doc = "Field `GIF5` reader - Global interrupt flag for channel x"]
pub type GIF5_R = GIF0_R;
#[doc = "Transfer error (TE) flag for channel x"]
pub type TEIF4_A = TEIF0_A;
#[doc = "Field `TEIF4` reader - Transfer error (TE) flag for channel x"]
pub type TEIF4_R = TEIF0_R;
#[doc = "Half transfer (HT) flag for channel x"]
pub type HTIF4_A = HTIF0_A;
#[doc = "Field `HTIF4` reader - Half transfer (HT) flag for channel x"]
pub type HTIF4_R = HTIF0_R;
#[doc = "Transfer complete (TC) flag for channel x"]
pub type TCIF4_A = TCIF0_A;
#[doc = "Field `TCIF4` reader - Transfer complete (TC) flag for channel x"]
pub type TCIF4_R = TCIF0_R;
#[doc = "Global interrupt flag for channel x"]
pub type GIF4_A = GIF0_A;
#[doc = "Field `GIF4` reader - Global interrupt flag for channel x"]
pub type GIF4_R = GIF0_R;
#[doc = "Transfer error (TE) flag for channel x"]
pub type TEIF3_A = TEIF0_A;
#[doc = "Field `TEIF3` reader - Transfer error (TE) flag for channel x"]
pub type TEIF3_R = TEIF0_R;
#[doc = "Half transfer (HT) flag for channel x"]
pub type HTIF3_A = HTIF0_A;
#[doc = "Field `HTIF3` reader - Half transfer (HT) flag for channel x"]
pub type HTIF3_R = HTIF0_R;
#[doc = "Transfer complete (TC) flag for channel x"]
pub type TCIF3_A = TCIF0_A;
#[doc = "Field `TCIF3` reader - Transfer complete (TC) flag for channel x"]
pub type TCIF3_R = TCIF0_R;
#[doc = "Global interrupt flag for channel x"]
pub type GIF3_A = GIF0_A;
#[doc = "Field `GIF3` reader - Global interrupt flag for channel x"]
pub type GIF3_R = GIF0_R;
#[doc = "Transfer error (TE) flag for channel x"]
pub type TEIF2_A = TEIF0_A;
#[doc = "Field `TEIF2` reader - Transfer error (TE) flag for channel x"]
pub type TEIF2_R = TEIF0_R;
#[doc = "Half transfer (HT) flag for channel x"]
pub type HTIF2_A = HTIF0_A;
#[doc = "Field `HTIF2` reader - Half transfer (HT) flag for channel x"]
pub type HTIF2_R = HTIF0_R;
#[doc = "Transfer complete (TC) flag for channel x"]
pub type TCIF2_A = TCIF0_A;
#[doc = "Field `TCIF2` reader - Transfer complete (TC) flag for channel x"]
pub type TCIF2_R = TCIF0_R;
#[doc = "Global interrupt flag for channel x"]
pub type GIF2_A = GIF0_A;
#[doc = "Field `GIF2` reader - Global interrupt flag for channel x"]
pub type GIF2_R = GIF0_R;
#[doc = "Transfer error (TE) flag for channel x"]
pub type TEIF1_A = TEIF0_A;
#[doc = "Field `TEIF1` reader - Transfer error (TE) flag for channel x"]
pub type TEIF1_R = TEIF0_R;
#[doc = "Half transfer (HT) flag for channel x"]
pub type HTIF1_A = HTIF0_A;
#[doc = "Field `HTIF1` reader - Half transfer (HT) flag for channel x"]
pub type HTIF1_R = HTIF0_R;
#[doc = "Transfer complete (TC) flag for channel x"]
pub type TCIF1_A = TCIF0_A;
#[doc = "Field `TCIF1` reader - Transfer complete (TC) flag for channel x"]
pub type TCIF1_R = TCIF0_R;
#[doc = "Global interrupt flag for channel x"]
pub type GIF1_A = GIF0_A;
#[doc = "Field `GIF1` reader - Global interrupt flag for channel x"]
pub type GIF1_R = GIF0_R;
#[doc = "Transfer error (TE) flag for channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF0_A {
    #[doc = "0: No transfer error on channel x"]
    NOERROR = 0,
    #[doc = "1: A transfer error occurred on channel x"]
    ERROR = 1,
}
impl From<TEIF0_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIF0` reader - Transfer error (TE) flag for channel x"]
pub struct TEIF0_R(crate::FieldReader<bool, TEIF0_A>);
impl TEIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF0_A {
        match self.bits {
            false => TEIF0_A::NOERROR,
            true => TEIF0_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == TEIF0_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == TEIF0_A::ERROR
    }
}
impl core::ops::Deref for TEIF0_R {
    type Target = crate::FieldReader<bool, TEIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Half transfer (HT) flag for channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIF0_A {
    #[doc = "0: No half transfer event on channel x"]
    NOTHALF = 0,
    #[doc = "1: A half transfer event occurred on channel x"]
    HALF = 1,
}
impl From<HTIF0_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIF0` reader - Half transfer (HT) flag for channel x"]
pub struct HTIF0_R(crate::FieldReader<bool, HTIF0_A>);
impl HTIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTIF0_A {
        match self.bits {
            false => HTIF0_A::NOTHALF,
            true => HTIF0_A::HALF,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHALF`"]
    #[inline(always)]
    pub fn is_not_half(&self) -> bool {
        **self == HTIF0_A::NOTHALF
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        **self == HTIF0_A::HALF
    }
}
impl core::ops::Deref for HTIF0_R {
    type Target = crate::FieldReader<bool, HTIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transfer complete (TC) flag for channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF0_A {
    #[doc = "0: No transfer complete event on channel x"]
    NOTCOMPLETE = 0,
    #[doc = "1: A transfer complete event occurred on channel x"]
    COMPLETE = 1,
}
impl From<TCIF0_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIF0` reader - Transfer complete (TC) flag for channel x"]
pub struct TCIF0_R(crate::FieldReader<bool, TCIF0_A>);
impl TCIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF0_A {
        match self.bits {
            false => TCIF0_A::NOTCOMPLETE,
            true => TCIF0_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        **self == TCIF0_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == TCIF0_A::COMPLETE
    }
}
impl core::ops::Deref for TCIF0_R {
    type Target = crate::FieldReader<bool, TCIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Global interrupt flag for channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF0_A {
    #[doc = "0: No TE, HT or TC event on channel x"]
    NOEVENT = 0,
    #[doc = "1: A TE, HT or TC event occurred on channel x"]
    EVENT = 1,
}
impl From<GIF0_A> for bool {
    #[inline(always)]
    fn from(variant: GIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIF0` reader - Global interrupt flag for channel x"]
pub struct GIF0_R(crate::FieldReader<bool, GIF0_A>);
impl GIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        GIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF0_A {
        match self.bits {
            false => GIF0_A::NOEVENT,
            true => GIF0_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == GIF0_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == GIF0_A::EVENT
    }
}
impl core::ops::Deref for GIF0_R {
    type Target = crate::FieldReader<bool, GIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31 - Transfer error (TE) flag for channel x"]
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Half transfer (HT) flag for channel x"]
    #[inline(always)]
    pub fn htif7(&self) -> HTIF7_R {
        HTIF7_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Transfer complete (TC) flag for channel x"]
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Global interrupt flag for channel x"]
    #[inline(always)]
    pub fn gif7(&self) -> GIF7_R {
        GIF7_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Transfer error (TE) flag for channel x"]
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Half transfer (HT) flag for channel x"]
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Transfer complete (TC) flag for channel x"]
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Global interrupt flag for channel x"]
    #[inline(always)]
    pub fn gif6(&self) -> GIF6_R {
        GIF6_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Transfer error (TE) flag for channel x"]
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Half transfer (HT) flag for channel x"]
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Transfer complete (TC) flag for channel x"]
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Global interrupt flag for channel x"]
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Transfer error (TE) flag for channel x"]
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Half transfer (HT) flag for channel x"]
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transfer complete (TC) flag for channel x"]
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Global interrupt flag for channel x"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Transfer error (TE) flag for channel x"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Half transfer (HT) flag for channel x"]
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transfer complete (TC) flag for channel x"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Global interrupt flag for channel x"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transfer error (TE) flag for channel x"]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Half transfer (HT) flag for channel x"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transfer complete (TC) flag for channel x"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Global interrupt flag for channel x"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transfer error (TE) flag for channel x"]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Half transfer (HT) flag for channel x"]
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transfer complete (TC) flag for channel x"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Global interrupt flag for channel x"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transfer error (TE) flag for channel x"]
    #[inline(always)]
    pub fn teif0(&self) -> TEIF0_R {
        TEIF0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Half transfer (HT) flag for channel x"]
    #[inline(always)]
    pub fn htif0(&self) -> HTIF0_R {
        HTIF0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer complete (TC) flag for channel x"]
    #[inline(always)]
    pub fn tcif0(&self) -> TCIF0_R {
        TCIF0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Global interrupt flag for channel x"]
    #[inline(always)]
    pub fn gif0(&self) -> GIF0_R {
        GIF0_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
