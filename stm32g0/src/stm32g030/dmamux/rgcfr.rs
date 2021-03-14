#[doc = "Writer for register RGCFR"]
pub type W = crate::W<u32, super::RGCFR>;
#[doc = "Register RGCFR `reset()`'s with value 0"]
impl crate::ResetValue for super::RGCFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `COF`"]
pub struct COF_W<'a> {
    w: &'a mut W,
}
impl<'a> COF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:3 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[inline(always)]
    pub fn cof(&mut self) -> COF_W {
        COF_W { w: self }
    }
}
