#[doc = "Reader of register RCC_AXIDIVR"]
pub type R = crate::R<u32, super::RCC_AXIDIVR>;
#[doc = "Writer for register RCC_AXIDIVR"]
pub type W = crate::W<u32, super::RCC_AXIDIVR>;
#[doc = "Register RCC_AXIDIVR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_AXIDIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `AXIDIV`"]
pub type AXIDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AXIDIV`"]
pub struct AXIDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `AXIDIVRDY`"]
pub type AXIDIVRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:2 - AXIDIV"]
    #[inline(always)]
    pub fn axidiv(&self) -> AXIDIV_R {
        AXIDIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - AXIDIVRDY"]
    #[inline(always)]
    pub fn axidivrdy(&self) -> AXIDIVRDY_R {
        AXIDIVRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AXIDIV"]
    #[inline(always)]
    pub fn axidiv(&mut self) -> AXIDIV_W {
        AXIDIV_W { w: self }
    }
}
