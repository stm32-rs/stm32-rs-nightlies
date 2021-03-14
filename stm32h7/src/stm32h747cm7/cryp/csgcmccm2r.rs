#[doc = "Reader of register CSGCMCCM2R"]
pub type R = crate::R<u32, super::CSGCMCCM2R>;
#[doc = "Writer for register CSGCMCCM2R"]
pub type W = crate::W<u32, super::CSGCMCCM2R>;
#[doc = "Register CSGCMCCM2R `reset()`'s with value 0"]
impl crate::ResetValue for super::CSGCMCCM2R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSGCMCCM2R`"]
pub type CSGCMCCM2R_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSGCMCCM2R`"]
pub struct CSGCMCCM2R_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCMCCM2R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM2R"]
    #[inline(always)]
    pub fn csgcmccm2r(&self) -> CSGCMCCM2R_R {
        CSGCMCCM2R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM2R"]
    #[inline(always)]
    pub fn csgcmccm2r(&mut self) -> CSGCMCCM2R_W {
        CSGCMCCM2R_W { w: self }
    }
}
