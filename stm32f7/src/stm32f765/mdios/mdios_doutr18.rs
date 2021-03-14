#[doc = "Reader of register MDIOS_DOUTR18"]
pub type R = crate::R<u32, super::MDIOS_DOUTR18>;
#[doc = "Writer for register MDIOS_DOUTR18"]
pub type W = crate::W<u32, super::MDIOS_DOUTR18>;
#[doc = "Register MDIOS_DOUTR18 `reset()`'s with value 0"]
impl crate::ResetValue for super::MDIOS_DOUTR18 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT18`"]
pub type DOUT18_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT18`"]
pub struct DOUT18_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT18_W<'a> {
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
    pub fn dout18(&self) -> DOUT18_R {
        DOUT18_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout18(&mut self) -> DOUT18_W {
        DOUT18_W { w: self }
    }
}
