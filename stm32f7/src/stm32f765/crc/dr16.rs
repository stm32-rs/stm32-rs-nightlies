#[doc = "Reader of register DR16"]
pub type R = crate::R<u16, super::DR16>;
#[doc = "Writer for register DR16"]
pub type W = crate::W<u16, super::DR16>;
#[doc = "Register DR16 `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::DR16 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `DR16`"]
pub type DR16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DR16`"]
pub struct DR16_W<'a> {
    w: &'a mut W,
}
impl<'a> DR16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Data register bits"]
    #[inline(always)]
    pub fn dr16(&self) -> DR16_R {
        DR16_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data register bits"]
    #[inline(always)]
    pub fn dr16(&mut self) -> DR16_W {
        DR16_W { w: self }
    }
}
