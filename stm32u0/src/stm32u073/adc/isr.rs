///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
///Field `ADRDY` reader - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN+1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.
pub type ADRDY_R = crate::BitReader;
///Field `ADRDY` writer - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN+1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.
pub type ADRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOSMP` reader - End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to 1.
pub type EOSMP_R = crate::BitReader;
///Field `EOSMP` writer - End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to 1.
pub type EOSMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOC` reader - End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register.
pub type EOC_R = crate::BitReader;
///Field `EOC` writer - End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register.
pub type EOC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOS` reader - End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it.
pub type EOS_R = crate::BitReader;
///Field `EOS` writer - End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it.
pub type EOS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVR` reader - ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it.
pub type OVR_R = crate::BitReader;
///Field `OVR` writer - ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it.
pub type OVR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1` reader - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1.
pub type AWD1_R = crate::BitReader;
///Field `AWD1` writer - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1.
pub type AWD1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2` reader - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it.
pub type AWD2_R = crate::BitReader;
///Field `AWD2` writer - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it.
pub type AWD2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD3` reader - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1.
pub type AWD3_R = crate::BitReader;
///Field `AWD3` writer - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1.
pub type AWD3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOCAL` reader - End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it.
pub type EOCAL_R = crate::BitReader;
///Field `EOCAL` writer - End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it.
pub type EOCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCRDY` reader - Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration.
pub type CCRDY_R = crate::BitReader;
///Field `CCRDY` writer - Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration.
pub type CCRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN+1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to 1.
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register.
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1.
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it.
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1.
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn eocal(&self) -> EOCAL_R {
        EOCAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration.
    #[inline(always)]
    pub fn ccrdy(&self) -> CCRDY_R {
        CCRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("adrdy", &self.adrdy())
            .field("eosmp", &self.eosmp())
            .field("eoc", &self.eoc())
            .field("eos", &self.eos())
            .field("ovr", &self.ovr())
            .field("awd1", &self.awd1())
            .field("awd2", &self.awd2())
            .field("awd3", &self.awd3())
            .field("eocal", &self.eocal())
            .field("ccrdy", &self.ccrdy())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN+1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn adrdy(&mut self) -> ADRDY_W<ISRrs> {
        ADRDY_W::new(self, 0)
    }
    ///Bit 1 - End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to 1.
    #[inline(always)]
    pub fn eosmp(&mut self) -> EOSMP_W<ISRrs> {
        EOSMP_W::new(self, 1)
    }
    ///Bit 2 - End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register.
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W<ISRrs> {
        EOC_W::new(self, 2)
    }
    ///Bit 3 - End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn eos(&mut self) -> EOS_W<ISRrs> {
        EOS_W::new(self, 3)
    }
    ///Bit 4 - ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W<ISRrs> {
        OVR_W::new(self, 4)
    }
    ///Bit 7 - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1.
    #[inline(always)]
    pub fn awd1(&mut self) -> AWD1_W<ISRrs> {
        AWD1_W::new(self, 7)
    }
    ///Bit 8 - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it.
    #[inline(always)]
    pub fn awd2(&mut self) -> AWD2_W<ISRrs> {
        AWD2_W::new(self, 8)
    }
    ///Bit 9 - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1.
    #[inline(always)]
    pub fn awd3(&mut self) -> AWD3_W<ISRrs> {
        AWD3_W::new(self, 9)
    }
    ///Bit 11 - End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it.
    #[inline(always)]
    pub fn eocal(&mut self) -> EOCAL_W<ISRrs> {
        EOCAL_W::new(self, 11)
    }
    ///Bit 13 - Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration.
    #[inline(always)]
    pub fn ccrdy(&mut self) -> CCRDY_W<ISRrs> {
        CCRDY_W::new(self, 13)
    }
}
/**ADC interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#ADC:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
