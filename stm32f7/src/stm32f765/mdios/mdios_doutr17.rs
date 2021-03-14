#[doc = "Reader of register MDIOS_DOUTR17"]
pub type R = crate::R<u32, super::MDIOS_DOUTR17>;
#[doc = "Writer for register MDIOS_DOUTR17"]
pub type W = crate::W<u32, super::MDIOS_DOUTR17>;
#[doc = "Register MDIOS_DOUTR17 `reset()`'s with value 0"]
impl crate::ResetValue for super::MDIOS_DOUTR17 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT17`"]
pub type DOUT17_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT17`"]
pub struct DOUT17_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT17_W<'a> {
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
    pub fn dout17(&self) -> DOUT17_R {
        DOUT17_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout17(&mut self) -> DOUT17_W {
        DOUT17_W { w: self }
    }
}
