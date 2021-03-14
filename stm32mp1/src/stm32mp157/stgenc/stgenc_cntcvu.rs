#[doc = "Reader of register STGENC_CNTCVU"]
pub type R = crate::R<u32, super::STGENC_CNTCVU>;
#[doc = "Writer for register STGENC_CNTCVU"]
pub type W = crate::W<u32, super::STGENC_CNTCVU>;
#[doc = "Register STGENC_CNTCVU `reset()`'s with value 0"]
impl crate::ResetValue for super::STGENC_CNTCVU {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNTCVU_U_32`"]
pub type CNTCVU_U_32_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CNTCVU_U_32`"]
pub struct CNTCVU_U_32_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTCVU_U_32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CNTCVU_U_32"]
    #[inline(always)]
    pub fn cntcvu_u_32(&self) -> CNTCVU_U_32_R {
        CNTCVU_U_32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CNTCVU_U_32"]
    #[inline(always)]
    pub fn cntcvu_u_32(&mut self) -> CNTCVU_U_32_W {
        CNTCVU_U_32_W { w: self }
    }
}
