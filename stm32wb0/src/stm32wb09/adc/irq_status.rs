///Register `IRQ_STATUS` reader
pub type R = crate::R<IRQ_STATUSrs>;
///Register `IRQ_STATUS` writer
pub type W = crate::W<IRQ_STATUSrs>;
///Field `EOC_IRQ` reader - EOC_IRQ (Used in test mode only): set when the ADC conversion is completed. When read, provide the status of the interrupt: 0: ADC conversion is not completed 1: ADC conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
pub type EOC_IRQ_R = crate::BitReader;
///Field `EOC_IRQ` writer - EOC_IRQ (Used in test mode only): set when the ADC conversion is completed. When read, provide the status of the interrupt: 0: ADC conversion is not completed 1: ADC conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
pub type EOC_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EODS_IRQ` reader - EODS_IRQ: set when the Down Sampler conversion is completed. When read, provide the status of the interrupt: 0: Down Sampler conversion is not completed 1: Down Sampler conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
pub type EODS_IRQ_R = crate::BitReader;
///Field `EODS_IRQ` writer - EODS_IRQ: set when the Down Sampler conversion is completed. When read, provide the status of the interrupt: 0: Down Sampler conversion is not completed 1: Down Sampler conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
pub type EODS_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOS_IRQ` reader - EOS_IRQ: set when a sequence of conversion is completed. When read, provide the status of the interrupt: 0: sequence of conversion is not completed 1: sequence of conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
pub type EOS_IRQ_R = crate::BitReader;
///Field `EOS_IRQ` writer - EOS_IRQ: set when a sequence of conversion is completed. When read, provide the status of the interrupt: 0: sequence of conversion is not completed 1: sequence of conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
pub type EOS_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD_IRQ` reader - AWD_IRQ: set when an analog watchdog event occurs. When read, provide the status of the interrupt: 0: no analog watchdog event occurred 1: analog watchdog event has occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
pub type AWD_IRQ_R = crate::BitReader;
///Field `AWD_IRQ` writer - AWD_IRQ: set when an analog watchdog event occurs. When read, provide the status of the interrupt: 0: no analog watchdog event occurred 1: analog watchdog event has occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
pub type AWD_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVR_DS_IRQ` reader - OVR_DS_IRQ: set to indicate a Down Sampler overrun (at least one data is lost) When read, provide the status of the interrupt: 0: no overrun occurred 1: overrun occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
pub type OVR_DS_IRQ_R = crate::BitReader;
///Field `OVR_DS_IRQ` writer - OVR_DS_IRQ: set to indicate a Down Sampler overrun (at least one data is lost) When read, provide the status of the interrupt: 0: no overrun occurred 1: overrun occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
pub type OVR_DS_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - EOC_IRQ (Used in test mode only): set when the ADC conversion is completed. When read, provide the status of the interrupt: 0: ADC conversion is not completed 1: ADC conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
    #[inline(always)]
    pub fn eoc_irq(&self) -> EOC_IRQ_R {
        EOC_IRQ_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EODS_IRQ: set when the Down Sampler conversion is completed. When read, provide the status of the interrupt: 0: Down Sampler conversion is not completed 1: Down Sampler conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
    #[inline(always)]
    pub fn eods_irq(&self) -> EODS_IRQ_R {
        EODS_IRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - EOS_IRQ: set when a sequence of conversion is completed. When read, provide the status of the interrupt: 0: sequence of conversion is not completed 1: sequence of conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
    #[inline(always)]
    pub fn eos_irq(&self) -> EOS_IRQ_R {
        EOS_IRQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AWD_IRQ: set when an analog watchdog event occurs. When read, provide the status of the interrupt: 0: no analog watchdog event occurred 1: analog watchdog event has occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
    #[inline(always)]
    pub fn awd_irq(&self) -> AWD_IRQ_R {
        AWD_IRQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - OVR_DS_IRQ: set to indicate a Down Sampler overrun (at least one data is lost) When read, provide the status of the interrupt: 0: no overrun occurred 1: overrun occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
    #[inline(always)]
    pub fn ovr_ds_irq(&self) -> OVR_DS_IRQ_R {
        OVR_DS_IRQ_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ_STATUS")
            .field("eoc_irq", &self.eoc_irq())
            .field("eods_irq", &self.eods_irq())
            .field("eos_irq", &self.eos_irq())
            .field("awd_irq", &self.awd_irq())
            .field("ovr_ds_irq", &self.ovr_ds_irq())
            .finish()
    }
}
impl W {
    ///Bit 0 - EOC_IRQ (Used in test mode only): set when the ADC conversion is completed. When read, provide the status of the interrupt: 0: ADC conversion is not completed 1: ADC conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
    #[inline(always)]
    pub fn eoc_irq(&mut self) -> EOC_IRQ_W<'_, IRQ_STATUSrs> {
        EOC_IRQ_W::new(self, 0)
    }
    ///Bit 1 - EODS_IRQ: set when the Down Sampler conversion is completed. When read, provide the status of the interrupt: 0: Down Sampler conversion is not completed 1: Down Sampler conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
    #[inline(always)]
    pub fn eods_irq(&mut self) -> EODS_IRQ_W<'_, IRQ_STATUSrs> {
        EODS_IRQ_W::new(self, 1)
    }
    ///Bit 3 - EOS_IRQ: set when a sequence of conversion is completed. When read, provide the status of the interrupt: 0: sequence of conversion is not completed 1: sequence of conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
    #[inline(always)]
    pub fn eos_irq(&mut self) -> EOS_IRQ_W<'_, IRQ_STATUSrs> {
        EOS_IRQ_W::new(self, 3)
    }
    ///Bit 4 - AWD_IRQ: set when an analog watchdog event occurs. When read, provide the status of the interrupt: 0: no analog watchdog event occurred 1: analog watchdog event has occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
    #[inline(always)]
    pub fn awd_irq(&mut self) -> AWD_IRQ_W<'_, IRQ_STATUSrs> {
        AWD_IRQ_W::new(self, 4)
    }
    ///Bit 5 - OVR_DS_IRQ: set to indicate a Down Sampler overrun (at least one data is lost) When read, provide the status of the interrupt: 0: no overrun occurred 1: overrun occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt
    #[inline(always)]
    pub fn ovr_ds_irq(&mut self) -> OVR_DS_IRQ_W<'_, IRQ_STATUSrs> {
        OVR_DS_IRQ_W::new(self, 5)
    }
}
/**IRQ_STATUS register

You can [`read`](crate::Reg::read) this register and get [`irq_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#ADC:IRQ_STATUS)*/
pub struct IRQ_STATUSrs;
impl crate::RegisterSpec for IRQ_STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`irq_status::R`](R) reader structure
impl crate::Readable for IRQ_STATUSrs {}
///`write(|w| ..)` method takes [`irq_status::W`](W) writer structure
impl crate::Writable for IRQ_STATUSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IRQ_STATUS to value 0
impl crate::Resettable for IRQ_STATUSrs {}
