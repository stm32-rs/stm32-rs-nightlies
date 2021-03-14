#[doc = "Reader of register DOUTR5"]
pub type R = crate::R<u32, super::DOUTR5>;
#[doc = "Writer for register DOUTR5"]
pub type W = crate::W<u32, super::DOUTR5>;
#[doc = "Register DOUTR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUTR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT5`"]
pub type DOUT5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT5`"]
pub struct DOUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT5_W<'a> {
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
    pub fn dout5(&self) -> DOUT5_R {
        DOUT5_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout5(&mut self) -> DOUT5_W {
        DOUT5_W { w: self }
    }
}
