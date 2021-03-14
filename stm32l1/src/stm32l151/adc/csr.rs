#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Reader of field `AWD1`"]
pub type AWD1_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC1`"]
pub type EOC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `JEOC1`"]
pub type JEOC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `JSTRT1`"]
pub type JSTRT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `STRT1`"]
pub type STRT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVR1`"]
pub type OVR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADONS1`"]
pub type ADONS1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Analog watchdog flag of the ADC"]
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of conversion of the ADC"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Injected channel end of conversion of the ADC"]
    #[inline(always)]
    pub fn jeoc1(&self) -> JEOC1_R {
        JEOC1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Injected channel Start flag of the ADC"]
    #[inline(always)]
    pub fn jstrt1(&self) -> JSTRT1_R {
        JSTRT1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Regular channel Start flag of the ADC"]
    #[inline(always)]
    pub fn strt1(&self) -> STRT1_R {
        STRT1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overrun flag of the ADC"]
    #[inline(always)]
    pub fn ovr1(&self) -> OVR1_R {
        OVR1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADON Status of ADC1"]
    #[inline(always)]
    pub fn adons1(&self) -> ADONS1_R {
        ADONS1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
