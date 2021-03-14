#[doc = "Reader of register MDIOS_DOUTR20"]
pub type R = crate::R<u32, super::MDIOS_DOUTR20>;
#[doc = "Writer for register MDIOS_DOUTR20"]
pub type W = crate::W<u32, super::MDIOS_DOUTR20>;
#[doc = "Register MDIOS_DOUTR20 `reset()`'s with value 0"]
impl crate::ResetValue for super::MDIOS_DOUTR20 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT20`"]
pub type DOUT20_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT20`"]
pub struct DOUT20_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT20_W<'a> {
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
    pub fn dout20(&self) -> DOUT20_R {
        DOUT20_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout20(&mut self) -> DOUT20_W {
        DOUT20_W { w: self }
    }
}
