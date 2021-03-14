#[doc = "Reader of register DAC_SHSR1"]
pub type R = crate::R<u32, super::DAC_SHSR1>;
#[doc = "Writer for register DAC_SHSR1"]
pub type W = crate::W<u32, super::DAC_SHSR1>;
#[doc = "Register DAC_SHSR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC_SHSR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSAMPLE1`"]
pub type TSAMPLE1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSAMPLE1`"]
pub struct TSAMPLE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TSAMPLE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - DAC Channel 1 sample Time (only valid in sample &amp; hold mode) These bits can be written when the DAC channel1 is disabled or also during normal operation. in the latter case, the write can be done only when BWSTx of DAC_SR register is low, If BWSTx=1, the write operation is ignored."]
    #[inline(always)]
    pub fn tsample1(&self) -> TSAMPLE1_R {
        TSAMPLE1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC Channel 1 sample Time (only valid in sample &amp; hold mode) These bits can be written when the DAC channel1 is disabled or also during normal operation. in the latter case, the write can be done only when BWSTx of DAC_SR register is low, If BWSTx=1, the write operation is ignored."]
    #[inline(always)]
    pub fn tsample1(&mut self) -> TSAMPLE1_W {
        TSAMPLE1_W { w: self }
    }
}
