#[doc = "Reader of register CRYP_CSGCM5R"]
pub type R = crate::R<u32, super::CRYP_CSGCM5R>;
#[doc = "Writer for register CRYP_CSGCM5R"]
pub type W = crate::W<u32, super::CRYP_CSGCM5R>;
#[doc = "Register CRYP_CSGCM5R `reset()`'s with value 0"]
impl crate::ResetValue for super::CRYP_CSGCM5R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSGCM5`"]
pub type CSGCM5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSGCM5`"]
pub struct CSGCM5_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCM5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCM5"]
    #[inline(always)]
    pub fn csgcm5(&self) -> CSGCM5_R {
        CSGCM5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM5"]
    #[inline(always)]
    pub fn csgcm5(&mut self) -> CSGCM5_W {
        CSGCM5_W { w: self }
    }
}
