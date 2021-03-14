#[doc = "Reader of register CSGCM6R"]
pub type R = crate::R<u32, super::CSGCM6R>;
#[doc = "Writer for register CSGCM6R"]
pub type W = crate::W<u32, super::CSGCM6R>;
#[doc = "Register CSGCM6R `reset()`'s with value 0"]
impl crate::ResetValue for super::CSGCM6R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSGCM6R`"]
pub type CSGCM6R_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSGCM6R`"]
pub struct CSGCM6R_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCM6R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCM6R"]
    #[inline(always)]
    pub fn csgcm6r(&self) -> CSGCM6R_R {
        CSGCM6R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM6R"]
    #[inline(always)]
    pub fn csgcm6r(&mut self) -> CSGCM6R_W {
        CSGCM6R_W { w: self }
    }
}
