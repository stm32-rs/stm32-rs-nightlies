#[doc = "Reader of register DOUTR30"]
pub type R = crate::R<u32, super::DOUTR30>;
#[doc = "Writer for register DOUTR30"]
pub type W = crate::W<u32, super::DOUTR30>;
#[doc = "Register DOUTR30 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUTR30 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT30`"]
pub type DOUT30_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT30`"]
pub struct DOUT30_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT30_W<'a> {
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
    pub fn dout30(&self) -> DOUT30_R {
        DOUT30_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout30(&mut self) -> DOUT30_W {
        DOUT30_W { w: self }
    }
}
