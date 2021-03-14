#[doc = "Reader of register MDIOS_DOUTR4"]
pub type R = crate::R<u32, super::MDIOS_DOUTR4>;
#[doc = "Writer for register MDIOS_DOUTR4"]
pub type W = crate::W<u32, super::MDIOS_DOUTR4>;
#[doc = "Register MDIOS_DOUTR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::MDIOS_DOUTR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT4`"]
pub type DOUT4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT4`"]
pub struct DOUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT4_W<'a> {
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
    pub fn dout4(&self) -> DOUT4_R {
        DOUT4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout4(&mut self) -> DOUT4_W {
        DOUT4_W { w: self }
    }
}
