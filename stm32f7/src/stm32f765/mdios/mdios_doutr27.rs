#[doc = "Reader of register MDIOS_DOUTR27"]
pub type R = crate::R<u32, super::MDIOS_DOUTR27>;
#[doc = "Writer for register MDIOS_DOUTR27"]
pub type W = crate::W<u32, super::MDIOS_DOUTR27>;
#[doc = "Register MDIOS_DOUTR27 `reset()`'s with value 0"]
impl crate::ResetValue for super::MDIOS_DOUTR27 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT27`"]
pub type DOUT27_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT27`"]
pub struct DOUT27_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT27_W<'a> {
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
    pub fn dout27(&self) -> DOUT27_R {
        DOUT27_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout27(&mut self) -> DOUT27_W {
        DOUT27_W { w: self }
    }
}
