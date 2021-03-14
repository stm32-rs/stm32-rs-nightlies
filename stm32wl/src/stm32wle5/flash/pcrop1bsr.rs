#[doc = "Reader of register PCROP1BSR"]
pub type R = crate::R<u32, super::PCROP1BSR>;
#[doc = "Writer for register PCROP1BSR"]
pub type W = crate::W<u32, super::PCROP1BSR>;
#[doc = "Register PCROP1BSR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PCROP1BSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `PCROP1B_STRT`"]
pub type PCROP1B_STRT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCROP1B_STRT`"]
pub struct PCROP1B_STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP1B_STRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Bank 1 WRP second area B end offset"]
    #[inline(always)]
    pub fn pcrop1b_strt(&self) -> PCROP1B_STRT_R {
        PCROP1B_STRT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bank 1 WRP second area B end offset"]
    #[inline(always)]
    pub fn pcrop1b_strt(&mut self) -> PCROP1B_STRT_W {
        PCROP1B_STRT_W { w: self }
    }
}
