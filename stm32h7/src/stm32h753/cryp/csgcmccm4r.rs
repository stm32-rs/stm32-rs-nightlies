#[doc = "Reader of register CSGCMCCM4R"]
pub type R = crate::R<u32, super::CSGCMCCM4R>;
#[doc = "Writer for register CSGCMCCM4R"]
pub type W = crate::W<u32, super::CSGCMCCM4R>;
#[doc = "Register CSGCMCCM4R `reset()`'s with value 0"]
impl crate::ResetValue for super::CSGCMCCM4R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSGCMCCM4`"]
pub type CSGCMCCM4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSGCMCCM4`"]
pub struct CSGCMCCM4_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCMCCM4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM4"]
    #[inline(always)]
    pub fn csgcmccm4(&self) -> CSGCMCCM4_R {
        CSGCMCCM4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM4"]
    #[inline(always)]
    pub fn csgcmccm4(&mut self) -> CSGCMCCM4_W {
        CSGCMCCM4_W { w: self }
    }
}
