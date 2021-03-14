#[doc = "Reader of register RCC_QSPICKSELR"]
pub type R = crate::R<u32, super::RCC_QSPICKSELR>;
#[doc = "Writer for register RCC_QSPICKSELR"]
pub type W = crate::W<u32, super::RCC_QSPICKSELR>;
#[doc = "Register RCC_QSPICKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_QSPICKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QSPISRC`"]
pub type QSPISRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `QSPISRC`"]
pub struct QSPISRC_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPISRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - QSPISRC"]
    #[inline(always)]
    pub fn qspisrc(&self) -> QSPISRC_R {
        QSPISRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - QSPISRC"]
    #[inline(always)]
    pub fn qspisrc(&mut self) -> QSPISRC_W {
        QSPISRC_W { w: self }
    }
}
