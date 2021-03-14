#[doc = "Reader of register CSGCM1R"]
pub type R = crate::R<u32, super::CSGCM1R>;
#[doc = "Writer for register CSGCM1R"]
pub type W = crate::W<u32, super::CSGCM1R>;
#[doc = "Register CSGCM1R `reset()`'s with value 0"]
impl crate::ResetValue for super::CSGCM1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSGCM1R`"]
pub type CSGCM1R_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSGCM1R`"]
pub struct CSGCM1R_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCM1R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCM1R"]
    #[inline(always)]
    pub fn csgcm1r(&self) -> CSGCM1R_R {
        CSGCM1R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM1R"]
    #[inline(always)]
    pub fn csgcm1r(&mut self) -> CSGCM1R_W {
        CSGCM1R_W { w: self }
    }
}
