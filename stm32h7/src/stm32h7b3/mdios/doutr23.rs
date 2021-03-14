#[doc = "Reader of register DOUTR23"]
pub type R = crate::R<u32, super::DOUTR23>;
#[doc = "Writer for register DOUTR23"]
pub type W = crate::W<u32, super::DOUTR23>;
#[doc = "Register DOUTR23 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUTR23 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT23`"]
pub type DOUT23_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT23`"]
pub struct DOUT23_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT23_W<'a> {
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
    pub fn dout23(&self) -> DOUT23_R {
        DOUT23_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout23(&mut self) -> DOUT23_W {
        DOUT23_W { w: self }
    }
}
