///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Field `ADRDY_MST` reader - Master ADC ready
pub type ADRDY_MST_R = crate::BitReader;
///Field `EOSMP_MST` reader - End of Sampling phase flag of the master ADC
pub type EOSMP_MST_R = crate::BitReader;
///Field `EOC_MST` reader - End of regular conversion of the master ADC
pub type EOC_MST_R = crate::BitReader;
///Field `EOS_MST` reader - End of regular sequence flag of the master ADC
pub type EOS_MST_R = crate::BitReader;
///Field `OVR_MST` reader - Overrun flag of the master ADC
pub type OVR_MST_R = crate::BitReader;
///Field `JEOC_MST` reader - End of injected conversion flag of the master ADC
pub type JEOC_MST_R = crate::BitReader;
///Field `JEOS_MST` reader - End of injected sequence flag of the master ADC
pub type JEOS_MST_R = crate::BitReader;
///Field `AWD1_MST` reader - Analog watchdog 1 flag of the master ADC
pub type AWD1_MST_R = crate::BitReader;
///Field `AWD2_MST` reader - Analog watchdog 2 flag of the master ADC
pub type AWD2_MST_R = crate::BitReader;
///Field `AWD3_MST` reader - Analog watchdog 3 flag of the master ADC
pub type AWD3_MST_R = crate::BitReader;
///Field `JQOVF_MST` reader - Injected Context Queue Overflow flag of the master ADC
pub type JQOVF_MST_R = crate::BitReader;
///Field `ADRDY_SLV` reader - Slave ADC ready
pub type ADRDY_SLV_R = crate::BitReader;
///Field `EOSMP_SLV` reader - End of Sampling phase flag of the slave ADC
pub type EOSMP_SLV_R = crate::BitReader;
///Field `EOC_SLV` reader - End of regular conversion of the slave ADC
pub type EOC_SLV_R = crate::BitReader;
///Field `EOS_SLV` reader - End of regular sequence flag of the slave ADC
pub type EOS_SLV_R = crate::BitReader;
///Field `OVR_SLV` reader - Overrun flag of the slave ADC
pub type OVR_SLV_R = crate::BitReader;
///Field `JEOC_SLV` reader - End of injected conversion flag of the slave ADC
pub type JEOC_SLV_R = crate::BitReader;
///Field `JEOS_SLV` reader - End of injected sequence flag of the slave ADC
pub type JEOS_SLV_R = crate::BitReader;
///Field `AWD1_SLV` reader - Analog watchdog 1 flag of the slave ADC
pub type AWD1_SLV_R = crate::BitReader;
///Field `AWD2_SLV` reader - Analog watchdog 2 flag of the slave ADC
pub type AWD2_SLV_R = crate::BitReader;
///Field `AWD3_SLV` reader - Analog watchdog 3 flag of the slave ADC
pub type AWD3_SLV_R = crate::BitReader;
///Field `JQOVF_SLV` reader - Injected Context Queue Overflow flag of the slave ADC
pub type JQOVF_SLV_R = crate::BitReader;
impl R {
    ///Bit 0 - Master ADC ready
    #[inline(always)]
    pub fn adrdy_mst(&self) -> ADRDY_MST_R {
        ADRDY_MST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of Sampling phase flag of the master ADC
    #[inline(always)]
    pub fn eosmp_mst(&self) -> EOSMP_MST_R {
        EOSMP_MST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End of regular conversion of the master ADC
    #[inline(always)]
    pub fn eoc_mst(&self) -> EOC_MST_R {
        EOC_MST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - End of regular sequence flag of the master ADC
    #[inline(always)]
    pub fn eos_mst(&self) -> EOS_MST_R {
        EOS_MST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Overrun flag of the master ADC
    #[inline(always)]
    pub fn ovr_mst(&self) -> OVR_MST_R {
        OVR_MST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - End of injected conversion flag of the master ADC
    #[inline(always)]
    pub fn jeoc_mst(&self) -> JEOC_MST_R {
        JEOC_MST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - End of injected sequence flag of the master ADC
    #[inline(always)]
    pub fn jeos_mst(&self) -> JEOS_MST_R {
        JEOS_MST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog 1 flag of the master ADC
    #[inline(always)]
    pub fn awd1_mst(&self) -> AWD1_MST_R {
        AWD1_MST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog 2 flag of the master ADC
    #[inline(always)]
    pub fn awd2_mst(&self) -> AWD2_MST_R {
        AWD2_MST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog 3 flag of the master ADC
    #[inline(always)]
    pub fn awd3_mst(&self) -> AWD3_MST_R {
        AWD3_MST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Injected Context Queue Overflow flag of the master ADC
    #[inline(always)]
    pub fn jqovf_mst(&self) -> JQOVF_MST_R {
        JQOVF_MST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - Slave ADC ready
    #[inline(always)]
    pub fn adrdy_slv(&self) -> ADRDY_SLV_R {
        ADRDY_SLV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - End of Sampling phase flag of the slave ADC
    #[inline(always)]
    pub fn eosmp_slv(&self) -> EOSMP_SLV_R {
        EOSMP_SLV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - End of regular conversion of the slave ADC
    #[inline(always)]
    pub fn eoc_slv(&self) -> EOC_SLV_R {
        EOC_SLV_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - End of regular sequence flag of the slave ADC
    #[inline(always)]
    pub fn eos_slv(&self) -> EOS_SLV_R {
        EOS_SLV_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Overrun flag of the slave ADC
    #[inline(always)]
    pub fn ovr_slv(&self) -> OVR_SLV_R {
        OVR_SLV_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - End of injected conversion flag of the slave ADC
    #[inline(always)]
    pub fn jeoc_slv(&self) -> JEOC_SLV_R {
        JEOC_SLV_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - End of injected sequence flag of the slave ADC
    #[inline(always)]
    pub fn jeos_slv(&self) -> JEOS_SLV_R {
        JEOS_SLV_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Analog watchdog 1 flag of the slave ADC
    #[inline(always)]
    pub fn awd1_slv(&self) -> AWD1_SLV_R {
        AWD1_SLV_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Analog watchdog 2 flag of the slave ADC
    #[inline(always)]
    pub fn awd2_slv(&self) -> AWD2_SLV_R {
        AWD2_SLV_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Analog watchdog 3 flag of the slave ADC
    #[inline(always)]
    pub fn awd3_slv(&self) -> AWD3_SLV_R {
        AWD3_SLV_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Injected Context Queue Overflow flag of the slave ADC
    #[inline(always)]
    pub fn jqovf_slv(&self) -> JQOVF_SLV_R {
        JQOVF_SLV_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("adrdy_mst", &self.adrdy_mst())
            .field("eosmp_mst", &self.eosmp_mst())
            .field("eoc_mst", &self.eoc_mst())
            .field("eos_mst", &self.eos_mst())
            .field("ovr_mst", &self.ovr_mst())
            .field("jeoc_mst", &self.jeoc_mst())
            .field("jeos_mst", &self.jeos_mst())
            .field("awd1_mst", &self.awd1_mst())
            .field("awd2_mst", &self.awd2_mst())
            .field("awd3_mst", &self.awd3_mst())
            .field("jqovf_mst", &self.jqovf_mst())
            .field("adrdy_slv", &self.adrdy_slv())
            .field("eosmp_slv", &self.eosmp_slv())
            .field("eoc_slv", &self.eoc_slv())
            .field("eos_slv", &self.eos_slv())
            .field("ovr_slv", &self.ovr_slv())
            .field("jeoc_slv", &self.jeoc_slv())
            .field("jeos_slv", &self.jeos_slv())
            .field("awd1_slv", &self.awd1_slv())
            .field("awd2_slv", &self.awd2_slv())
            .field("awd3_slv", &self.awd3_slv())
            .field("jqovf_slv", &self.jqovf_slv())
            .finish()
    }
}
/**ADC Common status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#ADC12_Common:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
