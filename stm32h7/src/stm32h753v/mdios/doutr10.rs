#[doc = "Reader of register DOUTR10"]
pub type R = crate::R<u32, super::DOUTR10>;
#[doc = "Writer for register DOUTR10"]
pub type W = crate::W<u32, super::DOUTR10>;
#[doc = "Register DOUTR10 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUTR10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT10`"]
pub type DOUT10_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT10`"]
pub struct DOUT10_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT10_W<'a> {
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
    pub fn dout10(&self) -> DOUT10_R {
        DOUT10_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout10(&mut self) -> DOUT10_W {
        DOUT10_W { w: self }
    }
}
