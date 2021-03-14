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
#[doc = "Reader of field `CSGCMCCM2`"]
pub type CSGCMCCM2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSGCMCCM2`"]
pub struct CSGCMCCM2_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCMCCM2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM2"]
    #[inline(always)]
    pub fn csgcmccm2(&self) -> CSGCMCCM2_R {
        CSGCMCCM2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM2"]
    #[inline(always)]
    pub fn csgcmccm2(&mut self) -> CSGCMCCM2_W {
        CSGCMCCM2_W { w: self }
    }
}
