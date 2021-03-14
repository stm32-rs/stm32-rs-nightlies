#[doc = "Reader of register DOUTR11"]
pub type R = crate::R<u32, super::DOUTR11>;
#[doc = "Writer for register DOUTR11"]
pub type W = crate::W<u32, super::DOUTR11>;
#[doc = "Register DOUTR11 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUTR11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT11`"]
pub type DOUT11_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT11`"]
pub struct DOUT11_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT11_W<'a> {
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
    pub fn dout11(&self) -> DOUT11_R {
        DOUT11_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout11(&mut self) -> DOUT11_W {
        DOUT11_W { w: self }
    }
}
