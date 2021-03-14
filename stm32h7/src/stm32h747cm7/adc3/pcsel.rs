#[doc = "Reader of register PCSEL"]
pub type R = crate::R<u32, super::PCSEL>;
#[doc = "Writer for register PCSEL"]
pub type W = crate::W<u32, super::PCSEL>;
#[doc = "Register PCSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::PCSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel x (VINP\\[i\\]) pre selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PCSEL_A {
    #[doc = "0: Input channel x is not pre-selected"]
    NOTPRESELECTED = 0,
    #[doc = "1: Pre-select input channel x"]
    PRESELECTED = 1,
}
impl From<PCSEL_A> for u32 {
    #[inline(always)]
    fn from(variant: PCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCSEL`"]
pub type PCSEL_R = crate::R<u32, PCSEL_A>;
impl PCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PCSEL_A::NOTPRESELECTED),
            1 => Val(PCSEL_A::PRESELECTED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESELECTED`"]
    #[inline(always)]
    pub fn is_not_preselected(&self) -> bool {
        *self == PCSEL_A::NOTPRESELECTED
    }
    #[doc = "Checks if the value of the field is `PRESELECTED`"]
    #[inline(always)]
    pub fn is_preselected(&self) -> bool {
        *self == PCSEL_A::PRESELECTED
    }
}
#[doc = "Write proxy for field `PCSEL`"]
pub struct PCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input channel x is not pre-selected"]
    #[inline(always)]
    pub fn not_preselected(self) -> &'a mut W {
        self.variant(PCSEL_A::NOTPRESELECTED)
    }
    #[doc = "Pre-select input channel x"]
    #[inline(always)]
    pub fn preselected(self) -> &'a mut W {
        self.variant(PCSEL_A::PRESELECTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - Channel x (VINP\\[i\\]) pre selection"]
    #[inline(always)]
    pub fn pcsel(&self) -> PCSEL_R {
        PCSEL_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Channel x (VINP\\[i\\]) pre selection"]
    #[inline(always)]
    pub fn pcsel(&mut self) -> PCSEL_W {
        PCSEL_W { w: self }
    }
}
