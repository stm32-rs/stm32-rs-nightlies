///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Field `AWD1` reader - Analog watchdog flag of the ADC
pub type AWD1_R = crate::BitReader;
///Field `EOC1` reader - End of conversion of the ADC
pub type EOC1_R = crate::BitReader;
///Field `JEOC1` reader - Injected channel end of conversion of the ADC
pub type JEOC1_R = crate::BitReader;
///Field `JSTRT1` reader - Injected channel Start flag of the ADC
pub type JSTRT1_R = crate::BitReader;
///Field `STRT1` reader - Regular channel Start flag of the ADC
pub type STRT1_R = crate::BitReader;
///Field `OVR1` reader - Overrun flag of the ADC
pub type OVR1_R = crate::BitReader;
///Field `ADONS1` reader - ADON Status of ADC1
pub type ADONS1_R = crate::BitReader;
impl R {
    ///Bit 0 - Analog watchdog flag of the ADC
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of conversion of the ADC
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Injected channel end of conversion of the ADC
    #[inline(always)]
    pub fn jeoc1(&self) -> JEOC1_R {
        JEOC1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Injected channel Start flag of the ADC
    #[inline(always)]
    pub fn jstrt1(&self) -> JSTRT1_R {
        JSTRT1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Regular channel Start flag of the ADC
    #[inline(always)]
    pub fn strt1(&self) -> STRT1_R {
        STRT1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Overrun flag of the ADC
    #[inline(always)]
    pub fn ovr1(&self) -> OVR1_R {
        OVR1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ADON Status of ADC1
    #[inline(always)]
    pub fn adons1(&self) -> ADONS1_R {
        ADONS1_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("awd1", &self.awd1())
            .field("eoc1", &self.eoc1())
            .field("jeoc1", &self.jeoc1())
            .field("jstrt1", &self.jstrt1())
            .field("strt1", &self.strt1())
            .field("ovr1", &self.ovr1())
            .field("adons1", &self.adons1())
            .finish()
    }
}
/**ADC common status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#ADC:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
