#[doc = "Reader of register RCC_CPERCKSELR"]
pub type R = crate::R<u32, super::RCC_CPERCKSELR>;
#[doc = "Writer for register RCC_CPERCKSELR"]
pub type W = crate::W<u32, super::RCC_CPERCKSELR>;
#[doc = "Register RCC_CPERCKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_CPERCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CKPERSRC`"]
pub type CKPERSRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKPERSRC`"]
pub struct CKPERSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPERSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CKPERSRC"]
    #[inline(always)]
    pub fn ckpersrc(&self) -> CKPERSRC_R {
        CKPERSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CKPERSRC"]
    #[inline(always)]
    pub fn ckpersrc(&mut self) -> CKPERSRC_W {
        CKPERSRC_W { w: self }
    }
}
