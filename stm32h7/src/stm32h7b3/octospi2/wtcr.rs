#[doc = "Reader of register WTCR"]
pub type R = crate::R<u32, super::WTCR>;
#[doc = "Writer for register WTCR"]
pub type W = crate::W<u32, super::WTCR>;
#[doc = "Register WTCR `reset()`'s with value 0"]
impl crate::ResetValue for super::WTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCYC`"]
pub type DCYC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCYC`"]
pub struct DCYC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCYC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - DCYC"]
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DCYC"]
    #[inline(always)]
    pub fn dcyc(&mut self) -> DCYC_W {
        DCYC_W { w: self }
    }
}
