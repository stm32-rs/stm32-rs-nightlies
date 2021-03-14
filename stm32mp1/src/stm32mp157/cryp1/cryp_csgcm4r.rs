#[doc = "Reader of register CRYP_CSGCM4R"]
pub type R = crate::R<u32, super::CRYP_CSGCM4R>;
#[doc = "Writer for register CRYP_CSGCM4R"]
pub type W = crate::W<u32, super::CRYP_CSGCM4R>;
#[doc = "Register CRYP_CSGCM4R `reset()`'s with value 0"]
impl crate::ResetValue for super::CRYP_CSGCM4R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSGCM4`"]
pub type CSGCM4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSGCM4`"]
pub struct CSGCM4_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCM4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCM4"]
    #[inline(always)]
    pub fn csgcm4(&self) -> CSGCM4_R {
        CSGCM4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM4"]
    #[inline(always)]
    pub fn csgcm4(&mut self) -> CSGCM4_W {
        CSGCM4_W { w: self }
    }
}
