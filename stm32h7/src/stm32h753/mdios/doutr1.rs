#[doc = "Reader of register DOUTR1"]
pub type R = crate::R<u32, super::DOUTR1>;
#[doc = "Writer for register DOUTR1"]
pub type W = crate::W<u32, super::DOUTR1>;
#[doc = "Register DOUTR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUTR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT1`"]
pub type DOUT1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT1`"]
pub struct DOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT1_W<'a> {
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
    pub fn dout1(&self) -> DOUT1_R {
        DOUT1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout1(&mut self) -> DOUT1_W {
        DOUT1_W { w: self }
    }
}
