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
#[doc = "Reader of field `CSGCM6`"]
pub type CSGCM6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSGCM6`"]
pub struct CSGCM6_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCM6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCM6"]
    #[inline(always)]
    pub fn csgcm6(&self) -> CSGCM6_R {
        CSGCM6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM6"]
    #[inline(always)]
    pub fn csgcm6(&mut self) -> CSGCM6_W {
        CSGCM6_W { w: self }
    }
}
