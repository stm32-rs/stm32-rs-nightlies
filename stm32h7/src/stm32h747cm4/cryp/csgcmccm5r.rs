#[doc = "Reader of register CSGCMCCM5R"]
pub type R = crate::R<u32, super::CSGCMCCM5R>;
#[doc = "Writer for register CSGCMCCM5R"]
pub type W = crate::W<u32, super::CSGCMCCM5R>;
#[doc = "Register CSGCMCCM5R `reset()`'s with value 0"]
impl crate::ResetValue for super::CSGCMCCM5R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSGCMCCM5R`"]
pub type CSGCMCCM5R_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSGCMCCM5R`"]
pub struct CSGCMCCM5R_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCMCCM5R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM5R"]
    #[inline(always)]
    pub fn csgcmccm5r(&self) -> CSGCMCCM5R_R {
        CSGCMCCM5R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM5R"]
    #[inline(always)]
    pub fn csgcmccm5r(&mut self) -> CSGCMCCM5R_W {
        CSGCMCCM5R_W { w: self }
    }
}
