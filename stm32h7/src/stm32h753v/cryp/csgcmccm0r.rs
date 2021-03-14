#[doc = "Reader of register CSGCMCCM0R"]
pub type R = crate::R<u32, super::CSGCMCCM0R>;
#[doc = "Writer for register CSGCMCCM0R"]
pub type W = crate::W<u32, super::CSGCMCCM0R>;
#[doc = "Register CSGCMCCM0R `reset()`'s with value 0"]
impl crate::ResetValue for super::CSGCMCCM0R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSGCMCCM0`"]
pub type CSGCMCCM0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSGCMCCM0`"]
pub struct CSGCMCCM0_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCMCCM0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM0"]
    #[inline(always)]
    pub fn csgcmccm0(&self) -> CSGCMCCM0_R {
        CSGCMCCM0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM0"]
    #[inline(always)]
    pub fn csgcmccm0(&mut self) -> CSGCMCCM0_W {
        CSGCMCCM0_W { w: self }
    }
}
