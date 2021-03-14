#[doc = "Reader of register MDIOS_DOUTR13"]
pub type R = crate::R<u32, super::MDIOS_DOUTR13>;
#[doc = "Writer for register MDIOS_DOUTR13"]
pub type W = crate::W<u32, super::MDIOS_DOUTR13>;
#[doc = "Register MDIOS_DOUTR13 `reset()`'s with value 0"]
impl crate::ResetValue for super::MDIOS_DOUTR13 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT13`"]
pub type DOUT13_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT13`"]
pub struct DOUT13_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout13(&self) -> DOUT13_R {
        DOUT13_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout13(&mut self) -> DOUT13_W {
        DOUT13_W { w: self }
    }
}
