#[doc = "Reader of register DR8"]
pub type R = crate::R<u8, super::DR8>;
#[doc = "Writer for register DR8"]
pub type W = crate::W<u8, super::DR8>;
#[doc = "Register DR8 `reset()`'s with value 0xff"]
impl crate::ResetValue for super::DR8 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `DR8`"]
pub type DR8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DR8`"]
pub struct DR8_W<'a> {
    w: &'a mut W,
}
impl<'a> DR8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data register bits"]
    #[inline(always)]
    pub fn dr8(&self) -> DR8_R {
        DR8_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data register bits"]
    #[inline(always)]
    pub fn dr8(&mut self) -> DR8_W {
        DR8_W { w: self }
    }
}
