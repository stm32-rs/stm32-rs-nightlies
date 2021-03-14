#[doc = "Reader of register CONFCHR2"]
pub type R = crate::R<u32, super::CONFCHR2>;
#[doc = "Writer for register CONFCHR2"]
pub type W = crate::W<u32, super::CONFCHR2>;
#[doc = "Register CONFCHR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFCHR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONFCH8`"]
pub type CONFCH8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CONFCH8`"]
pub struct CONFCH8_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFCH8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel 8 configuration"]
    #[inline(always)]
    pub fn confch8(&self) -> CONFCH8_R {
        CONFCH8_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 8 configuration"]
    #[inline(always)]
    pub fn confch8(&mut self) -> CONFCH8_W {
        CONFCH8_W { w: self }
    }
}
