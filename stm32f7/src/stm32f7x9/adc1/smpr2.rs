#[doc = "Reader of register SMPR2"]
pub type R = crate::R<u32, super::SMPR2>;
#[doc = "Writer for register SMPR2"]
pub type W = crate::W<u32, super::SMPR2>;
#[doc = "Register SMPR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SMPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sample time bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SMPX_X_A {
    #[doc = "0: 3 cycles"]
    CYCLES3 = 0,
    #[doc = "1: 15 cycles"]
    CYCLES15 = 1,
    #[doc = "2: 28 cycles"]
    CYCLES28 = 2,
    #[doc = "3: 56 cycles"]
    CYCLES56 = 3,
    #[doc = "4: 84 cycles"]
    CYCLES84 = 4,
    #[doc = "5: 112 cycles"]
    CYCLES112 = 5,
    #[doc = "6: 144 cycles"]
    CYCLES144 = 6,
    #[doc = "7: 480 cycles"]
    CYCLES480 = 7,
}
impl From<SMPX_X_A> for u32 {
    #[inline(always)]
    fn from(variant: SMPX_X_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMPx_x`"]
pub type SMPX_X_R = crate::R<u32, SMPX_X_A>;
impl SMPX_X_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, SMPX_X_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SMPX_X_A::CYCLES3),
            1 => Val(SMPX_X_A::CYCLES15),
            2 => Val(SMPX_X_A::CYCLES28),
            3 => Val(SMPX_X_A::CYCLES56),
            4 => Val(SMPX_X_A::CYCLES84),
            5 => Val(SMPX_X_A::CYCLES112),
            6 => Val(SMPX_X_A::CYCLES144),
            7 => Val(SMPX_X_A::CYCLES480),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES3`"]
    #[inline(always)]
    pub fn is_cycles3(&self) -> bool {
        *self == SMPX_X_A::CYCLES3
    }
    #[doc = "Checks if the value of the field is `CYCLES15`"]
    #[inline(always)]
    pub fn is_cycles15(&self) -> bool {
        *self == SMPX_X_A::CYCLES15
    }
    #[doc = "Checks if the value of the field is `CYCLES28`"]
    #[inline(always)]
    pub fn is_cycles28(&self) -> bool {
        *self == SMPX_X_A::CYCLES28
    }
    #[doc = "Checks if the value of the field is `CYCLES56`"]
    #[inline(always)]
    pub fn is_cycles56(&self) -> bool {
        *self == SMPX_X_A::CYCLES56
    }
    #[doc = "Checks if the value of the field is `CYCLES84`"]
    #[inline(always)]
    pub fn is_cycles84(&self) -> bool {
        *self == SMPX_X_A::CYCLES84
    }
    #[doc = "Checks if the value of the field is `CYCLES112`"]
    #[inline(always)]
    pub fn is_cycles112(&self) -> bool {
        *self == SMPX_X_A::CYCLES112
    }
    #[doc = "Checks if the value of the field is `CYCLES144`"]
    #[inline(always)]
    pub fn is_cycles144(&self) -> bool {
        *self == SMPX_X_A::CYCLES144
    }
    #[doc = "Checks if the value of the field is `CYCLES480`"]
    #[inline(always)]
    pub fn is_cycles480(&self) -> bool {
        *self == SMPX_X_A::CYCLES480
    }
}
#[doc = "Write proxy for field `SMPx_x`"]
pub struct SMPX_X_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPX_X_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPX_X_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn cycles3(self) -> &'a mut W {
        self.variant(SMPX_X_A::CYCLES3)
    }
    #[doc = "15 cycles"]
    #[inline(always)]
    pub fn cycles15(self) -> &'a mut W {
        self.variant(SMPX_X_A::CYCLES15)
    }
    #[doc = "28 cycles"]
    #[inline(always)]
    pub fn cycles28(self) -> &'a mut W {
        self.variant(SMPX_X_A::CYCLES28)
    }
    #[doc = "56 cycles"]
    #[inline(always)]
    pub fn cycles56(self) -> &'a mut W {
        self.variant(SMPX_X_A::CYCLES56)
    }
    #[doc = "84 cycles"]
    #[inline(always)]
    pub fn cycles84(self) -> &'a mut W {
        self.variant(SMPX_X_A::CYCLES84)
    }
    #[doc = "112 cycles"]
    #[inline(always)]
    pub fn cycles112(self) -> &'a mut W {
        self.variant(SMPX_X_A::CYCLES112)
    }
    #[doc = "144 cycles"]
    #[inline(always)]
    pub fn cycles144(self) -> &'a mut W {
        self.variant(SMPX_X_A::CYCLES144)
    }
    #[doc = "480 cycles"]
    #[inline(always)]
    pub fn cycles480(self) -> &'a mut W {
        self.variant(SMPX_X_A::CYCLES480)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Sample time bits"]
    #[inline(always)]
    pub fn smpx_x(&self) -> SMPX_X_R {
        SMPX_X_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sample time bits"]
    #[inline(always)]
    pub fn smpx_x(&mut self) -> SMPX_X_W {
        SMPX_X_W { w: self }
    }
}
