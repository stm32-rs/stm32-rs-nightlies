#[doc = "Reader of register MDIOS_DOUTR22"]
pub type R = crate::R<u32, super::MDIOS_DOUTR22>;
#[doc = "Writer for register MDIOS_DOUTR22"]
pub type W = crate::W<u32, super::MDIOS_DOUTR22>;
#[doc = "Register MDIOS_DOUTR22 `reset()`'s with value 0"]
impl crate::ResetValue for super::MDIOS_DOUTR22 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT22`"]
pub type DOUT22_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT22`"]
pub struct DOUT22_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT22_W<'a> {
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
    pub fn dout22(&self) -> DOUT22_R {
        DOUT22_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout22(&mut self) -> DOUT22_W {
        DOUT22_W { w: self }
    }
}
