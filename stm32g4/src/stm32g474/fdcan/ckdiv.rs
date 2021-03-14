#[doc = "Reader of register CKDIV"]
pub type R = crate::R<u32, super::CKDIV>;
#[doc = "Writer for register CKDIV"]
pub type W = crate::W<u32, super::CKDIV>;
#[doc = "Register CKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::CKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDIV`"]
pub type PDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDIV`"]
pub struct PDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - input clock divider. the APB clock could be divided prior to be used by the CAN sub"]
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - input clock divider. the APB clock could be divided prior to be used by the CAN sub"]
    #[inline(always)]
    pub fn pdiv(&mut self) -> PDIV_W {
        PDIV_W { w: self }
    }
}
