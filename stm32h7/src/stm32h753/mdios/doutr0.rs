#[doc = "Reader of register DOUTR0"]
pub type R = crate::R<u32, super::DOUTR0>;
#[doc = "Writer for register DOUTR0"]
pub type W = crate::W<u32, super::DOUTR0>;
#[doc = "Register DOUTR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUTR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT0`"]
pub type DOUT0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT0`"]
pub struct DOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT0_W<'a> {
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
    pub fn dout0(&self) -> DOUT0_R {
        DOUT0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout0(&mut self) -> DOUT0_W {
        DOUT0_W { w: self }
    }
}
