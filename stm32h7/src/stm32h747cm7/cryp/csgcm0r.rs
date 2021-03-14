#[doc = "Reader of register CSGCM0R"]
pub type R = crate::R<u32, super::CSGCM0R>;
#[doc = "Writer for register CSGCM0R"]
pub type W = crate::W<u32, super::CSGCM0R>;
#[doc = "Register CSGCM0R `reset()`'s with value 0"]
impl crate::ResetValue for super::CSGCM0R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSGCM0R`"]
pub type CSGCM0R_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSGCM0R`"]
pub struct CSGCM0R_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCM0R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCM0R"]
    #[inline(always)]
    pub fn csgcm0r(&self) -> CSGCM0R_R {
        CSGCM0R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM0R"]
    #[inline(always)]
    pub fn csgcm0r(&mut self) -> CSGCM0R_W {
        CSGCM0R_W { w: self }
    }
}
