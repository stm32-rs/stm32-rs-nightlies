#[doc = "Reader of register STGENC_CNTFID0"]
pub type R = crate::R<u32, super::STGENC_CNTFID0>;
#[doc = "Writer for register STGENC_CNTFID0"]
pub type W = crate::W<u32, super::STGENC_CNTFID0>;
#[doc = "Register STGENC_CNTFID0 `reset()`'s with value 0"]
impl crate::ResetValue for super::STGENC_CNTFID0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FREQ`"]
pub type FREQ_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FREQ`"]
pub struct FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FREQ"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FREQ"]
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W {
        FREQ_W { w: self }
    }
}
