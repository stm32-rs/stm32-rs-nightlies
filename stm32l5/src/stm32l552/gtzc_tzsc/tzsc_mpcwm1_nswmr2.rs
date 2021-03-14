#[doc = "Reader of register TZSC_MPCWM1_NSWMR2"]
pub type R = crate::R<u32, super::TZSC_MPCWM1_NSWMR2>;
#[doc = "Writer for register TZSC_MPCWM1_NSWMR2"]
pub type W = crate::W<u32, super::TZSC_MPCWM1_NSWMR2>;
#[doc = "Register TZSC_MPCWM1_NSWMR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TZSC_MPCWM1_NSWMR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NSWM2STRT`"]
pub type NSWM2STRT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NSWM2STRT`"]
pub struct NSWM2STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> NSWM2STRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `NSWM2LGTH`"]
pub type NSWM2LGTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NSWM2LGTH`"]
pub struct NSWM2LGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> NSWM2LGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - NSWM2STRT"]
    #[inline(always)]
    pub fn nswm2strt(&self) -> NSWM2STRT_R {
        NSWM2STRT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:27 - NSWM2LGTH"]
    #[inline(always)]
    pub fn nswm2lgth(&self) -> NSWM2LGTH_R {
        NSWM2LGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - NSWM2STRT"]
    #[inline(always)]
    pub fn nswm2strt(&mut self) -> NSWM2STRT_W {
        NSWM2STRT_W { w: self }
    }
    #[doc = "Bits 16:27 - NSWM2LGTH"]
    #[inline(always)]
    pub fn nswm2lgth(&mut self) -> NSWM2LGTH_W {
        NSWM2LGTH_W { w: self }
    }
}
