#[doc = "Reader of register ITLINE12"]
pub type R = crate::R<u32, super::ITLINE12>;
#[doc = "Reader of field `ADC`"]
pub type ADC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ADC"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new((self.bits & 0x01) != 0)
    }
}
