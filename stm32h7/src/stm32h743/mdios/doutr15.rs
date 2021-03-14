#[doc = "Reader of register DOUTR15"]
pub type R = crate::R<u32, super::DOUTR15>;
#[doc = "Writer for register DOUTR15"]
pub type W = crate::W<u32, super::DOUTR15>;
#[doc = "Register DOUTR15 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUTR15 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT15`"]
pub type DOUT15_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT15`"]
pub struct DOUT15_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT15_W<'a> {
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
    pub fn dout15(&self) -> DOUT15_R {
        DOUT15_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout15(&mut self) -> DOUT15_W {
        DOUT15_W { w: self }
    }
}
