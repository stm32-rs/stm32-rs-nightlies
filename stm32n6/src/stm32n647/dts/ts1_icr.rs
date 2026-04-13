///Register `TS1_ICR` writer
pub type W = crate::W<TS1_ICRrs>;
///Field `IRQ_CLEAR_FAULT` writer - Fault IRQ clear bit
pub type IRQ_CLEAR_FAULT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IRQ_CLEAR_DONE` writer - Sample done IRQ clear bit
pub type IRQ_CLEAR_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IRQ_CLEAR_ALARMA` writer - Alarm A IRQ clear bit
pub type IRQ_CLEAR_ALARMA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IRQ_CLEAR_ALARMB` writer - Alarm B IRQ clear bit
pub type IRQ_CLEAR_ALARMB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<TS1_ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Fault IRQ clear bit
    #[inline(always)]
    pub fn irq_clear_fault(&mut self) -> IRQ_CLEAR_FAULT_W<'_, TS1_ICRrs> {
        IRQ_CLEAR_FAULT_W::new(self, 0)
    }
    ///Bit 1 - Sample done IRQ clear bit
    #[inline(always)]
    pub fn irq_clear_done(&mut self) -> IRQ_CLEAR_DONE_W<'_, TS1_ICRrs> {
        IRQ_CLEAR_DONE_W::new(self, 1)
    }
    ///Bit 3 - Alarm A IRQ clear bit
    #[inline(always)]
    pub fn irq_clear_alarma(&mut self) -> IRQ_CLEAR_ALARMA_W<'_, TS1_ICRrs> {
        IRQ_CLEAR_ALARMA_W::new(self, 3)
    }
    ///Bit 4 - Alarm B IRQ clear bit
    #[inline(always)]
    pub fn irq_clear_alarmb(&mut self) -> IRQ_CLEAR_ALARMB_W<'_, TS1_ICRrs> {
        IRQ_CLEAR_ALARMB_W::new(self, 4)
    }
}
/**DTS TS1 IRQ clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts1_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS1_ICR)*/
pub struct TS1_ICRrs;
impl crate::RegisterSpec for TS1_ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ts1_icr::W`](W) writer structure
impl crate::Writable for TS1_ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TS1_ICR to value 0
impl crate::Resettable for TS1_ICRrs {}
