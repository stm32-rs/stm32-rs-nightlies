#[doc = "Reader of register MDIOS_CRDFR"]
pub type R = crate::R<u32, super::MDIOS_CRDFR>;
#[doc = "Writer for register MDIOS_CRDFR"]
pub type W = crate::W<u32, super::MDIOS_CRDFR>;
#[doc = "Register MDIOS_CRDFR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDIOS_CRDFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRDF`"]
pub type CRDF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CRDF`"]
pub struct CRDF_W<'a> {
    w: &'a mut W,
}
impl<'a> CRDF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRDF"]
    #[inline(always)]
    pub fn crdf(&self) -> CRDF_R {
        CRDF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRDF"]
    #[inline(always)]
    pub fn crdf(&mut self) -> CRDF_W {
        CRDF_W { w: self }
    }
}
