#[doc = "Reader of register DOUTR9"]
pub type R = crate::R<u32, super::DOUTR9>;
#[doc = "Writer for register DOUTR9"]
pub type W = crate::W<u32, super::DOUTR9>;
#[doc = "Register DOUTR9 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUTR9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT9`"]
pub type DOUT9_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT9`"]
pub struct DOUT9_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT9_W<'a> {
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
    pub fn dout9(&self) -> DOUT9_R {
        DOUT9_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout9(&mut self) -> DOUT9_W {
        DOUT9_W { w: self }
    }
}
