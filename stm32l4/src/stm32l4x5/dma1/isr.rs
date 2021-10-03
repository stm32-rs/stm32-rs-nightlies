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
#[doc = "Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF7_A = TEIF1_A;
#[doc = "Field `TEIF7` reader - Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF7_R = TEIF1_R;
#[doc = "Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF7_A = HTIF1_A;
#[doc = "Field `HTIF7` reader - Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF7_R = HTIF1_R;
#[doc = "Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF7_A = TCIF1_A;
#[doc = "Field `TCIF7` reader - Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF7_R = TCIF1_R;
#[doc = "Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF7_A = GIF1_A;
#[doc = "Field `GIF7` reader - Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF7_R = GIF1_R;
#[doc = "Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF6_A = TEIF1_A;
#[doc = "Field `TEIF6` reader - Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF6_R = TEIF1_R;
#[doc = "Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF6_A = HTIF1_A;
#[doc = "Field `HTIF6` reader - Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF6_R = HTIF1_R;
#[doc = "Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF6_A = TCIF1_A;
#[doc = "Field `TCIF6` reader - Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF6_R = TCIF1_R;
#[doc = "Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF6_A = GIF1_A;
#[doc = "Field `GIF6` reader - Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF6_R = GIF1_R;
#[doc = "Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF5_A = TEIF1_A;
#[doc = "Field `TEIF5` reader - Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF5_R = TEIF1_R;
#[doc = "Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF5_A = HTIF1_A;
#[doc = "Field `HTIF5` reader - Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF5_R = HTIF1_R;
#[doc = "Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF5_A = TCIF1_A;
#[doc = "Field `TCIF5` reader - Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF5_R = TCIF1_R;
#[doc = "Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF5_A = GIF1_A;
#[doc = "Field `GIF5` reader - Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF5_R = GIF1_R;
#[doc = "Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF4_A = TEIF1_A;
#[doc = "Field `TEIF4` reader - Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF4_R = TEIF1_R;
#[doc = "Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF4_A = HTIF1_A;
#[doc = "Field `HTIF4` reader - Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF4_R = HTIF1_R;
#[doc = "Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF4_A = TCIF1_A;
#[doc = "Field `TCIF4` reader - Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF4_R = TCIF1_R;
#[doc = "Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF4_A = GIF1_A;
#[doc = "Field `GIF4` reader - Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF4_R = GIF1_R;
#[doc = "Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF3_A = TEIF1_A;
#[doc = "Field `TEIF3` reader - Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF3_R = TEIF1_R;
#[doc = "Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF3_A = HTIF1_A;
#[doc = "Field `HTIF3` reader - Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF3_R = HTIF1_R;
#[doc = "Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF3_A = TCIF1_A;
#[doc = "Field `TCIF3` reader - Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF3_R = TCIF1_R;
#[doc = "Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF3_A = GIF1_A;
#[doc = "Field `GIF3` reader - Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF3_R = GIF1_R;
#[doc = "Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF2_A = TEIF1_A;
#[doc = "Field `TEIF2` reader - Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF2_R = TEIF1_R;
#[doc = "Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF2_A = HTIF1_A;
#[doc = "Field `HTIF2` reader - Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF2_R = HTIF1_R;
#[doc = "Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF2_A = TCIF1_A;
#[doc = "Field `TCIF2` reader - Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF2_R = TCIF1_R;
#[doc = "Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF2_A = GIF1_A;
#[doc = "Field `GIF2` reader - Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF2_R = GIF1_R;
#[doc = "Channel x transfer error flag (x = 1 ..7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF1_A {
    #[doc = "0: No transfer error"]
    NOERROR = 0,
    #[doc = "1: A transfer error has occured"]
    ERROR = 1,
}
impl From<TEIF1_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIF1` reader - Channel x transfer error flag (x = 1 ..7)"]
pub struct TEIF1_R(crate::FieldReader<bool, TEIF1_A>);
impl TEIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF1_A {
        match self.bits {
            false => TEIF1_A::NOERROR,
            true => TEIF1_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == TEIF1_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == TEIF1_A::ERROR
    }
}
impl core::ops::Deref for TEIF1_R {
    type Target = crate::FieldReader<bool, TEIF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Channel x half transfer flag (x = 1 ..7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIF1_A {
    #[doc = "0: No half transfer event"]
    NOTHALF = 0,
    #[doc = "1: A half transfer event has occured"]
    HALF = 1,
}
impl From<HTIF1_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIF1` reader - Channel x half transfer flag (x = 1 ..7)"]
pub struct HTIF1_R(crate::FieldReader<bool, HTIF1_A>);
impl HTIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTIF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTIF1_A {
        match self.bits {
            false => HTIF1_A::NOTHALF,
            true => HTIF1_A::HALF,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHALF`"]
    #[inline(always)]
    pub fn is_not_half(&self) -> bool {
        **self == HTIF1_A::NOTHALF
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        **self == HTIF1_A::HALF
    }
}
impl core::ops::Deref for HTIF1_R {
    type Target = crate::FieldReader<bool, HTIF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Channel x transfer complete flag (x = 1 ..7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF1_A {
    #[doc = "0: No transfer complete event"]
    NOTCOMPLETE = 0,
    #[doc = "1: A transfer complete event has occured"]
    COMPLETE = 1,
}
impl From<TCIF1_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIF1` reader - Channel x transfer complete flag (x = 1 ..7)"]
pub struct TCIF1_R(crate::FieldReader<bool, TCIF1_A>);
impl TCIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF1_A {
        match self.bits {
            false => TCIF1_A::NOTCOMPLETE,
            true => TCIF1_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        **self == TCIF1_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == TCIF1_A::COMPLETE
    }
}
impl core::ops::Deref for TCIF1_R {
    type Target = crate::FieldReader<bool, TCIF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Channel x global interrupt flag (x = 1 ..7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF1_A {
    #[doc = "0: No transfer error, half event, complete event"]
    NOEVENT = 0,
    #[doc = "1: A transfer error, half event or complete event has occured"]
    EVENT = 1,
}
impl From<GIF1_A> for bool {
    #[inline(always)]
    fn from(variant: GIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIF1` reader - Channel x global interrupt flag (x = 1 ..7)"]
pub struct GIF1_R(crate::FieldReader<bool, GIF1_A>);
impl GIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        GIF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF1_A {
        match self.bits {
            false => GIF1_A::NOEVENT,
            true => GIF1_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == GIF1_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == GIF1_A::EVENT
    }
}
impl core::ops::Deref for GIF1_R {
    type Target = crate::FieldReader<bool, GIF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 27 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif7(&self) -> HTIF7_R {
        HTIF7_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif7(&self) -> GIF7_R {
        GIF7_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif6(&self) -> GIF6_R {
        GIF6_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
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
