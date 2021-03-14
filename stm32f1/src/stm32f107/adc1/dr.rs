#[doc = "Reader of register DR"]
pub type R = crate::R<u32, super::DR>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u16, u16>;
#[doc = "Reader of field `ADC2DATA`"]
pub type ADC2DATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Regular data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADC2 data"]
    #[inline(always)]
    pub fn adc2data(&self) -> ADC2DATA_R {
        ADC2DATA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
