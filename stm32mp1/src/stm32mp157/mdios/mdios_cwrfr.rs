#[doc = "Reader of register MDIOS_CWRFR"]
pub type R = crate::R<u32, super::MDIOS_CWRFR>;
#[doc = "Writer for register MDIOS_CWRFR"]
pub type W = crate::W<u32, super::MDIOS_CWRFR>;
#[doc = "Register MDIOS_CWRFR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDIOS_CWRFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CWRF`"]
pub type CWRF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CWRF`"]
pub struct CWRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CWRF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CWRF"]
    #[inline(always)]
    pub fn cwrf(&self) -> CWRF_R {
        CWRF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CWRF"]
    #[inline(always)]
    pub fn cwrf(&mut self) -> CWRF_W {
        CWRF_W { w: self }
    }
}
