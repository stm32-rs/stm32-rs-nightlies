#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Field `AWD1` reader - Analog watchdog flag of the ADC"]
pub type AWD1_R = crate::BitReader;
#[doc = "Field `EOC1` reader - End of conversion of the ADC"]
pub type EOC1_R = crate::BitReader;
#[doc = "Field `JEOC1` reader - Injected channel end of conversion of the ADC"]
pub type JEOC1_R = crate::BitReader;
#[doc = "Field `JSTRT1` reader - Injected channel Start flag of the ADC"]
pub type JSTRT1_R = crate::BitReader;
#[doc = "Field `STRT1` reader - Regular channel Start flag of the ADC"]
pub type STRT1_R = crate::BitReader;
#[doc = "Field `OVR1` reader - Overrun flag of the ADC"]
pub type OVR1_R = crate::BitReader;
#[doc = "Field `ADONS1` reader - ADON Status of ADC1"]
pub type ADONS1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Analog watchdog flag of the ADC"]
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of conversion of the ADC"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected channel end of conversion of the ADC"]
    #[inline(always)]
    pub fn jeoc1(&self) -> JEOC1_R {
        JEOC1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Injected channel Start flag of the ADC"]
    #[inline(always)]
    pub fn jstrt1(&self) -> JSTRT1_R {
        JSTRT1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Regular channel Start flag of the ADC"]
    #[inline(always)]
    pub fn strt1(&self) -> STRT1_R {
        STRT1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun flag of the ADC"]
    #[inline(always)]
    pub fn ovr1(&self) -> OVR1_R {
        OVR1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADON Status of ADC1"]
    #[inline(always)]
    pub fn adons1(&self) -> ADONS1_R {
        ADONS1_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "ADC common status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
