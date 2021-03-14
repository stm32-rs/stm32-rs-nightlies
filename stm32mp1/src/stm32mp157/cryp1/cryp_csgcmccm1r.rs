#[doc = "Reader of register CRYP_CSGCMCCM1R"]
pub type R = crate::R<u32, super::CRYP_CSGCMCCM1R>;
#[doc = "Writer for register CRYP_CSGCMCCM1R"]
pub type W = crate::W<u32, super::CRYP_CSGCMCCM1R>;
#[doc = "Register CRYP_CSGCMCCM1R `reset()`'s with value 0"]
impl crate::ResetValue for super::CRYP_CSGCMCCM1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSGCMCCM1`"]
pub type CSGCMCCM1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSGCMCCM1`"]
pub struct CSGCMCCM1_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCMCCM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM1"]
    #[inline(always)]
    pub fn csgcmccm1(&self) -> CSGCMCCM1_R {
        CSGCMCCM1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM1"]
    #[inline(always)]
    pub fn csgcmccm1(&mut self) -> CSGCMCCM1_W {
        CSGCMCCM1_W { w: self }
    }
}
