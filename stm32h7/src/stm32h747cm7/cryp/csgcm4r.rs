#[doc = "Reader of register CSGCM4R"]
pub type R = crate::R<u32, super::CSGCM4R>;
#[doc = "Writer for register CSGCM4R"]
pub type W = crate::W<u32, super::CSGCM4R>;
#[doc = "Register CSGCM4R `reset()`'s with value 0"]
impl crate::ResetValue for super::CSGCM4R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSGCM4R`"]
pub type CSGCM4R_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSGCM4R`"]
pub struct CSGCM4R_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCM4R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCM4R"]
    #[inline(always)]
    pub fn csgcm4r(&self) -> CSGCM4R_R {
        CSGCM4R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM4R"]
    #[inline(always)]
    pub fn csgcm4r(&mut self) -> CSGCM4R_W {
        CSGCM4R_W { w: self }
    }
}
