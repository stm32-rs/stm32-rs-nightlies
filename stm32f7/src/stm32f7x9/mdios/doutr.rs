#[doc = "Reader of register DOUTR%s"]
pub type R = crate::R<u32, super::DOUTR>;
#[doc = "Writer for register DOUTR%s"]
pub type W = crate::W<u32, super::DOUTR>;
#[doc = "Register DOUTR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT`"]
pub type DOUT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT`"]
pub struct DOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT_W<'a> {
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
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout(&mut self) -> DOUT_W {
        DOUT_W { w: self }
    }
}
