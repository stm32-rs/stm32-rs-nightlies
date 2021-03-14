#[doc = "Reader of register DOUTR26"]
pub type R = crate::R<u32, super::DOUTR26>;
#[doc = "Writer for register DOUTR26"]
pub type W = crate::W<u32, super::DOUTR26>;
#[doc = "Register DOUTR26 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUTR26 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT26`"]
pub type DOUT26_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT26`"]
pub struct DOUT26_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT26_W<'a> {
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
    pub fn dout26(&self) -> DOUT26_R {
        DOUT26_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout26(&mut self) -> DOUT26_W {
        DOUT26_W { w: self }
    }
}
