#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Field `ADRDY_MST` reader - master ADC ready"]
pub type ADRDY_MST_R = crate::BitReader;
#[doc = "Field `EOSMP_MST` reader - End of Sampling phase flag of the master ADC"]
pub type EOSMP_MST_R = crate::BitReader;
#[doc = "Field `EOC_MST` reader - End of regular conversion flag of the master ADC"]
pub type EOC_MST_R = crate::BitReader;
#[doc = "Field `EOS_MST` reader - End of regular sequence flag of the master ADC"]
pub type EOS_MST_R = crate::BitReader;
#[doc = "Field `OVR_MST` reader - Overrun flag of the master ADC"]
pub type OVR_MST_R = crate::BitReader;
#[doc = "Field `JEOC_MST` reader - End of injected conversion flag of the master ADC"]
pub type JEOC_MST_R = crate::BitReader;
#[doc = "Field `JEOS_MST` reader - End of injected sequence flag of the master ADC"]
pub type JEOS_MST_R = crate::BitReader;
#[doc = "Field `AWD1_MST` reader - Analog watchdog 1 flag of the master ADC"]
pub type AWD1_MST_R = crate::BitReader;
#[doc = "Field `AWD2_MST` reader - Analog watchdog 2 flag of the master ADC"]
pub type AWD2_MST_R = crate::BitReader;
#[doc = "Field `AWD3_MST` reader - Analog watchdog 3 flag of the master ADC"]
pub type AWD3_MST_R = crate::BitReader;
#[doc = "Field `JQOVF_MST` reader - Injected Context Queue Overflow flag of the master ADC"]
pub type JQOVF_MST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - master ADC ready"]
    #[inline(always)]
    pub fn adrdy_mst(&self) -> ADRDY_MST_R {
        ADRDY_MST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Sampling phase flag of the master ADC"]
    #[inline(always)]
    pub fn eosmp_mst(&self) -> EOSMP_MST_R {
        EOSMP_MST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of regular conversion flag of the master ADC"]
    #[inline(always)]
    pub fn eoc_mst(&self) -> EOC_MST_R {
        EOC_MST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of regular sequence flag of the master ADC"]
    #[inline(always)]
    pub fn eos_mst(&self) -> EOS_MST_R {
        EOS_MST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overrun flag of the master ADC"]
    #[inline(always)]
    pub fn ovr_mst(&self) -> OVR_MST_R {
        OVR_MST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of injected conversion flag of the master ADC"]
    #[inline(always)]
    pub fn jeoc_mst(&self) -> JEOC_MST_R {
        JEOC_MST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End of injected sequence flag of the master ADC"]
    #[inline(always)]
    pub fn jeos_mst(&self) -> JEOS_MST_R {
        JEOS_MST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog watchdog 1 flag of the master ADC"]
    #[inline(always)]
    pub fn awd1_mst(&self) -> AWD1_MST_R {
        AWD1_MST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog 2 flag of the master ADC"]
    #[inline(always)]
    pub fn awd2_mst(&self) -> AWD2_MST_R {
        AWD2_MST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog watchdog 3 flag of the master ADC"]
    #[inline(always)]
    pub fn awd3_mst(&self) -> AWD3_MST_R {
        AWD3_MST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Injected Context Queue Overflow flag of the master ADC"]
    #[inline(always)]
    pub fn jqovf_mst(&self) -> JQOVF_MST_R {
        JQOVF_MST_R::new(((self.bits >> 10) & 1) != 0)
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
