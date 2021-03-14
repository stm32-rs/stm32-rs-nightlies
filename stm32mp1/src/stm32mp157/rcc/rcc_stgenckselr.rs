#[doc = "Reader of register RCC_STGENCKSELR"]
pub type R = crate::R<u32, super::RCC_STGENCKSELR>;
#[doc = "Writer for register RCC_STGENCKSELR"]
pub type W = crate::W<u32, super::RCC_STGENCKSELR>;
#[doc = "Register RCC_STGENCKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_STGENCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STGENSRC`"]
pub type STGENSRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STGENSRC`"]
pub struct STGENSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - STGENSRC"]
    #[inline(always)]
    pub fn stgensrc(&self) -> STGENSRC_R {
        STGENSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - STGENSRC"]
    #[inline(always)]
    pub fn stgensrc(&mut self) -> STGENSRC_W {
        STGENSRC_W { w: self }
    }
}
