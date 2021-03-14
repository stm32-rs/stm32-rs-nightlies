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
#[doc = "Reader of field `CSGCMCCM5`"]
pub type CSGCMCCM5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSGCMCCM5`"]
pub struct CSGCMCCM5_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCMCCM5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM5"]
    #[inline(always)]
    pub fn csgcmccm5(&self) -> CSGCMCCM5_R {
        CSGCMCCM5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM5"]
    #[inline(always)]
    pub fn csgcmccm5(&mut self) -> CSGCMCCM5_W {
        CSGCMCCM5_W { w: self }
    }
}
