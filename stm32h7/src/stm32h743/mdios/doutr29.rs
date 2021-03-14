#[doc = "Reader of register DOUTR29"]
pub type R = crate::R<u32, super::DOUTR29>;
#[doc = "Writer for register DOUTR29"]
pub type W = crate::W<u32, super::DOUTR29>;
#[doc = "Register DOUTR29 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUTR29 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT29`"]
pub type DOUT29_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT29`"]
pub struct DOUT29_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT29_W<'a> {
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
    pub fn dout29(&self) -> DOUT29_R {
        DOUT29_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout29(&mut self) -> DOUT29_W {
        DOUT29_W { w: self }
    }
}
