///Register `ADC_IER` reader
pub type R = crate::R<ADC_IERrs>;
///Register `ADC_IER` writer
pub type W = crate::W<ADC_IERrs>;
///Field `ADRDYIE` reader - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type ADRDYIE_R = crate::BitReader;
///Field `ADRDYIE` writer - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type ADRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOSMPIE` reader - End of sampling flag interrupt enable for regular conversions This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt for regular conversions. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type EOSMPIE_R = crate::BitReader;
///Field `EOSMPIE` writer - End of sampling flag interrupt enable for regular conversions This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt for regular conversions. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type EOSMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOCIE` reader - End of regular conversion interrupt enable This bit is set and cleared by software to enable/disable the end of a regular conversion interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type EOCIE_R = crate::BitReader;
///Field `EOCIE` writer - End of regular conversion interrupt enable This bit is set and cleared by software to enable/disable the end of a regular conversion interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOSIE` reader - End of regular sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of regular sequence of conversions interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type EOSIE_R = crate::BitReader;
///Field `EOSIE` writer - End of regular sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of regular sequence of conversions interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type EOSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRIE` reader - Overrun interrupt enable This bit is set and cleared by software to enable/disable the Overrun interrupt of a regular conversion. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type OVRIE_R = crate::BitReader;
///Field `OVRIE` writer - Overrun interrupt enable This bit is set and cleared by software to enable/disable the Overrun interrupt of a regular conversion. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JEOCIE` reader - End of injected conversion interrupt enable This bit is set and cleared by software to enable/disable the end of an injected conversion interrupt. Note: Software is allowed to write this bit only when JADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type JEOCIE_R = crate::BitReader;
///Field `JEOCIE` writer - End of injected conversion interrupt enable This bit is set and cleared by software to enable/disable the end of an injected conversion interrupt. Note: Software is allowed to write this bit only when JADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type JEOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JEOSIE` reader - End of injected sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of injected sequence of conversions interrupt. Note: Software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
pub type JEOSIE_R = crate::BitReader;
///Field `JEOSIE` writer - End of injected sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of injected sequence of conversions interrupt. Note: Software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
pub type JEOSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1IE` reader - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 1 interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD1IE_R = crate::BitReader;
///Field `AWD1IE` writer - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 1 interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2IE` reader - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2IE_R = crate::BitReader;
///Field `AWD2IE` writer - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD3IE` reader - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD3IE_R = crate::BitReader;
///Field `AWD3IE` writer - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of sampling flag interrupt enable for regular conversions This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt for regular conversions. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End of regular conversion interrupt enable This bit is set and cleared by software to enable/disable the end of a regular conversion interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - End of regular sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of regular sequence of conversions interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn eosie(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Overrun interrupt enable This bit is set and cleared by software to enable/disable the Overrun interrupt of a regular conversion. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - End of injected conversion interrupt enable This bit is set and cleared by software to enable/disable the end of an injected conversion interrupt. Note: Software is allowed to write this bit only when JADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - End of injected sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of injected sequence of conversions interrupt. Note: Software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
    #[inline(always)]
    pub fn jeosie(&self) -> JEOSIE_R {
        JEOSIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 1 interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd1ie(&self) -> AWD1IE_R {
        AWD1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ie(&self) -> AWD2IE_R {
        AWD2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ie(&self) -> AWD3IE_R {
        AWD3IE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_IER")
            .field("adrdyie", &self.adrdyie())
            .field("eosmpie", &self.eosmpie())
            .field("eocie", &self.eocie())
            .field("eosie", &self.eosie())
            .field("ovrie", &self.ovrie())
            .field("jeocie", &self.jeocie())
            .field("jeosie", &self.jeosie())
            .field("awd1ie", &self.awd1ie())
            .field("awd2ie", &self.awd2ie())
            .field("awd3ie", &self.awd3ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn adrdyie(&mut self) -> ADRDYIE_W<ADC_IERrs> {
        ADRDYIE_W::new(self, 0)
    }
    ///Bit 1 - End of sampling flag interrupt enable for regular conversions This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt for regular conversions. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<ADC_IERrs> {
        EOSMPIE_W::new(self, 1)
    }
    ///Bit 2 - End of regular conversion interrupt enable This bit is set and cleared by software to enable/disable the end of a regular conversion interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<ADC_IERrs> {
        EOCIE_W::new(self, 2)
    }
    ///Bit 3 - End of regular sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of regular sequence of conversions interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn eosie(&mut self) -> EOSIE_W<ADC_IERrs> {
        EOSIE_W::new(self, 3)
    }
    ///Bit 4 - Overrun interrupt enable This bit is set and cleared by software to enable/disable the Overrun interrupt of a regular conversion. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<ADC_IERrs> {
        OVRIE_W::new(self, 4)
    }
    ///Bit 5 - End of injected conversion interrupt enable This bit is set and cleared by software to enable/disable the end of an injected conversion interrupt. Note: Software is allowed to write this bit only when JADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn jeocie(&mut self) -> JEOCIE_W<ADC_IERrs> {
        JEOCIE_W::new(self, 5)
    }
    ///Bit 6 - End of injected sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of injected sequence of conversions interrupt. Note: Software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn jeosie(&mut self) -> JEOSIE_W<ADC_IERrs> {
        JEOSIE_W::new(self, 6)
    }
    ///Bit 7 - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 1 interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn awd1ie(&mut self) -> AWD1IE_W<ADC_IERrs> {
        AWD1IE_W::new(self, 7)
    }
    ///Bit 8 - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn awd2ie(&mut self) -> AWD2IE_W<ADC_IERrs> {
        AWD2IE_W::new(self, 8)
    }
    ///Bit 9 - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn awd3ie(&mut self) -> AWD3IE_W<ADC_IERrs> {
        AWD3IE_W::new(self, 9)
    }
}
/**ADC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`adc_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#ADC1:ADC_IER)*/
pub struct ADC_IERrs;
impl crate::RegisterSpec for ADC_IERrs {
    type Ux = u32;
}
///`read()` method returns [`adc_ier::R`](R) reader structure
impl crate::Readable for ADC_IERrs {}
///`write(|w| ..)` method takes [`adc_ier::W`](W) writer structure
impl crate::Writable for ADC_IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_IER to value 0
impl crate::Resettable for ADC_IERrs {
    const RESET_VALUE: u32 = 0;
}
