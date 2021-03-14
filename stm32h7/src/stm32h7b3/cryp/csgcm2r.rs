#[doc = "Reader of register CSGCM2R"]
pub type R = crate::R<u32, super::CSGCM2R>;
#[doc = "Writer for register CSGCM2R"]
pub type W = crate::W<u32, super::CSGCM2R>;
#[doc = "Register CSGCM2R `reset()`'s with value 0"]
impl crate::ResetValue for super::CSGCM2R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSGCM2R`"]
pub type CSGCM2R_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSGCM2R`"]
pub struct CSGCM2R_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCM2R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCM2R"]
    #[inline(always)]
    pub fn csgcm2r(&self) -> CSGCM2R_R {
        CSGCM2R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM2R"]
    #[inline(always)]
    pub fn csgcm2r(&mut self) -> CSGCM2R_W {
        CSGCM2R_W { w: self }
    }
}
