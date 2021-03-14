#[doc = "Reader of register ITLINE12"]
pub type R = crate::R<u32, super::ITLINE12>;
#[doc = "Reader of field `ADC`"]
pub type ADC_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMP1`"]
pub type COMP1_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMP2`"]
pub type COMP2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ADC"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - COMP1"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - COMP2"]
    #[inline(always)]
    pub fn comp2(&self) -> COMP2_R {
        COMP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
