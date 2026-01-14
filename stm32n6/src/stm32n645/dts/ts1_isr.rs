///Register `TS1_ISR` reader
pub type R = crate::R<TS1_ISRrs>;
///Field `IRQ_STATUS_FAULT` reader - Fault IRQ status bit
pub type IRQ_STATUS_FAULT_R = crate::BitReader;
///Field `IRQ_STATUS_DONE` reader - Sample done IRQ status bit
pub type IRQ_STATUS_DONE_R = crate::BitReader;
///Field `IRQ_STATUS_ALARMA` reader - Alarm A IRQ status bit
pub type IRQ_STATUS_ALARMA_R = crate::BitReader;
///Field `IRQ_STATUS_ALARMB` reader - Alarm B IRQ status bit
pub type IRQ_STATUS_ALARMB_R = crate::BitReader;
impl R {
    ///Bit 0 - Fault IRQ status bit
    #[inline(always)]
    pub fn irq_status_fault(&self) -> IRQ_STATUS_FAULT_R {
        IRQ_STATUS_FAULT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Sample done IRQ status bit
    #[inline(always)]
    pub fn irq_status_done(&self) -> IRQ_STATUS_DONE_R {
        IRQ_STATUS_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Alarm A IRQ status bit
    #[inline(always)]
    pub fn irq_status_alarma(&self) -> IRQ_STATUS_ALARMA_R {
        IRQ_STATUS_ALARMA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Alarm B IRQ status bit
    #[inline(always)]
    pub fn irq_status_alarmb(&self) -> IRQ_STATUS_ALARMB_R {
        IRQ_STATUS_ALARMB_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TS1_ISR")
            .field("irq_status_fault", &self.irq_status_fault())
            .field("irq_status_done", &self.irq_status_done())
            .field("irq_status_alarma", &self.irq_status_alarma())
            .field("irq_status_alarmb", &self.irq_status_alarmb())
            .finish()
    }
}
/**DTS TS1 IRQ status register

You can [`read`](crate::Reg::read) this register and get [`ts1_isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DTS:TS1_ISR)*/
pub struct TS1_ISRrs;
impl crate::RegisterSpec for TS1_ISRrs {
    type Ux = u32;
}
///`read()` method returns [`ts1_isr::R`](R) reader structure
impl crate::Readable for TS1_ISRrs {}
///`reset()` method sets TS1_ISR to value 0
impl crate::Resettable for TS1_ISRrs {}
