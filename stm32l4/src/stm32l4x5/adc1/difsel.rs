#[doc = "Reader of register DIFSEL"]
pub type R = crate::R<u32, super::DIFSEL>;
#[doc = "Writer for register DIFSEL"]
pub type W = crate::W<u32, super::DIFSEL>;
#[doc = "Register DIFSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::DIFSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIFSEL_1_15`"]
pub type DIFSEL_1_15_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DIFSEL_1_15`"]
pub struct DIFSEL_1_15_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_1_15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 1)) | (((value as u32) & 0x7fff) << 1);
        self.w
    }
}
#[doc = "Reader of field `DIFSEL_16_18`"]
pub type DIFSEL_16_18_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 1:15 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_1_15(&self) -> DIFSEL_1_15_R {
        DIFSEL_1_15_R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bits 16:18 - Differential mode for channels 18 to 16"]
    #[inline(always)]
    pub fn difsel_16_18(&self) -> DIFSEL_16_18_R {
        DIFSEL_16_18_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 1:15 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_1_15(&mut self) -> DIFSEL_1_15_W {
        DIFSEL_1_15_W { w: self }
    }
}
