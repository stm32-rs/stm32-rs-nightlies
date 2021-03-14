#[doc = "Reader of register TZSC_MPCWM2_NSWMR1"]
pub type R = crate::R<u32, super::TZSC_MPCWM2_NSWMR1>;
#[doc = "Writer for register TZSC_MPCWM2_NSWMR1"]
pub type W = crate::W<u32, super::TZSC_MPCWM2_NSWMR1>;
#[doc = "Register TZSC_MPCWM2_NSWMR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TZSC_MPCWM2_NSWMR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NSWM1STRT`"]
pub type NSWM1STRT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NSWM1STRT`"]
pub struct NSWM1STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> NSWM1STRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `NSWM1LGTH`"]
pub type NSWM1LGTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NSWM1LGTH`"]
pub struct NSWM1LGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> NSWM1LGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - NSWM1STRT"]
    #[inline(always)]
    pub fn nswm1strt(&self) -> NSWM1STRT_R {
        NSWM1STRT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:27 - NSWM1LGTH"]
    #[inline(always)]
    pub fn nswm1lgth(&self) -> NSWM1LGTH_R {
        NSWM1LGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - NSWM1STRT"]
    #[inline(always)]
    pub fn nswm1strt(&mut self) -> NSWM1STRT_W {
        NSWM1STRT_W { w: self }
    }
    #[doc = "Bits 16:27 - NSWM1LGTH"]
    #[inline(always)]
    pub fn nswm1lgth(&mut self) -> NSWM1LGTH_W {
        NSWM1LGTH_W { w: self }
    }
}
