///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Field `AWD1` reader - Analog watchdog flag of ADC 1
pub type AWD1_R = crate::BitReader;
///Field `EOC1` reader - End of conversion of ADC 1
pub type EOC1_R = crate::BitReader;
///Field `JEOC1` reader - Injected channel end of conversion of ADC 1
pub type JEOC1_R = crate::BitReader;
///Field `JSTRT1` reader - Injected channel Start flag of ADC 1
pub type JSTRT1_R = crate::BitReader;
///Field `STRT1` reader - Regular channel Start flag of ADC 1
pub type STRT1_R = crate::BitReader;
///Field `OVR1` reader - Overrun flag of ADC 1
pub type OVR1_R = crate::BitReader;
///Field `AWD2` reader - Analog watchdog flag of ADC 2
pub type AWD2_R = crate::BitReader;
///Field `EOC2` reader - End of conversion of ADC 2
pub type EOC2_R = crate::BitReader;
///Field `JEOC2` reader - Injected channel end of conversion of ADC 2
pub type JEOC2_R = crate::BitReader;
///Field `JSTRT2` reader - Injected channel Start flag of ADC 2
pub type JSTRT2_R = crate::BitReader;
///Field `STRT2` reader - Regular channel Start flag of ADC 2
pub type STRT2_R = crate::BitReader;
///Field `OVR2` reader - Overrun flag of ADC 2
pub type OVR2_R = crate::BitReader;
///Field `AWD3` reader - Analog watchdog flag of ADC 3
pub type AWD3_R = crate::BitReader;
///Field `EOC3` reader - End of conversion of ADC 3
pub type EOC3_R = crate::BitReader;
///Field `JEOC3` reader - Injected channel end of conversion of ADC 3
pub type JEOC3_R = crate::BitReader;
///Field `JSTRT3` reader - Injected channel Start flag of ADC 3
pub type JSTRT3_R = crate::BitReader;
///Field `STRT3` reader - Regular channel Start flag of ADC 3
pub type STRT3_R = crate::BitReader;
///Field `OVR3` reader - Overrun flag of ADC3
pub type OVR3_R = crate::BitReader;
impl R {
    ///Bit 0 - Analog watchdog flag of ADC 1
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of conversion of ADC 1
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Injected channel end of conversion of ADC 1
    #[inline(always)]
    pub fn jeoc1(&self) -> JEOC1_R {
        JEOC1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Injected channel Start flag of ADC 1
    #[inline(always)]
    pub fn jstrt1(&self) -> JSTRT1_R {
        JSTRT1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Regular channel Start flag of ADC 1
    #[inline(always)]
    pub fn strt1(&self) -> STRT1_R {
        STRT1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Overrun flag of ADC 1
    #[inline(always)]
    pub fn ovr1(&self) -> OVR1_R {
        OVR1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog flag of ADC 2
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - End of conversion of ADC 2
    #[inline(always)]
    pub fn eoc2(&self) -> EOC2_R {
        EOC2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Injected channel end of conversion of ADC 2
    #[inline(always)]
    pub fn jeoc2(&self) -> JEOC2_R {
        JEOC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Injected channel Start flag of ADC 2
    #[inline(always)]
    pub fn jstrt2(&self) -> JSTRT2_R {
        JSTRT2_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Regular channel Start flag of ADC 2
    #[inline(always)]
    pub fn strt2(&self) -> STRT2_R {
        STRT2_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Overrun flag of ADC 2
    #[inline(always)]
    pub fn ovr2(&self) -> OVR2_R {
        OVR2_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Analog watchdog flag of ADC 3
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - End of conversion of ADC 3
    #[inline(always)]
    pub fn eoc3(&self) -> EOC3_R {
        EOC3_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Injected channel end of conversion of ADC 3
    #[inline(always)]
    pub fn jeoc3(&self) -> JEOC3_R {
        JEOC3_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Injected channel Start flag of ADC 3
    #[inline(always)]
    pub fn jstrt3(&self) -> JSTRT3_R {
        JSTRT3_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Regular channel Start flag of ADC 3
    #[inline(always)]
    pub fn strt3(&self) -> STRT3_R {
        STRT3_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Overrun flag of ADC3
    #[inline(always)]
    pub fn ovr3(&self) -> OVR3_R {
        OVR3_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("ovr3", &self.ovr3())
            .field("strt3", &self.strt3())
            .field("jstrt3", &self.jstrt3())
            .field("jeoc3", &self.jeoc3())
            .field("eoc3", &self.eoc3())
            .field("awd3", &self.awd3())
            .field("ovr2", &self.ovr2())
            .field("strt2", &self.strt2())
            .field("jstrt2", &self.jstrt2())
            .field("jeoc2", &self.jeoc2())
            .field("eoc2", &self.eoc2())
            .field("awd2", &self.awd2())
            .field("ovr1", &self.ovr1())
            .field("strt1", &self.strt1())
            .field("jstrt1", &self.jstrt1())
            .field("jeoc1", &self.jeoc1())
            .field("eoc1", &self.eoc1())
            .field("awd1", &self.awd1())
            .finish()
    }
}
/**ADC Common status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#C_ADC:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
