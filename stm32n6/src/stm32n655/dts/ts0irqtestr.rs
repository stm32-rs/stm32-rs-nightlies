///Register `TS0IRQTESTR` reader
pub type R = crate::R<TS0IRQTESTRrs>;
///Register `TS0IRQTESTR` writer
pub type W = crate::W<TS0IRQTESTRrs>;
///Field `IRQ_TEST_FAULT` reader - Fault IRQ test bit
pub type IRQ_TEST_FAULT_R = crate::BitReader;
///Field `IRQ_TEST_FAULT` writer - Fault IRQ test bit
pub type IRQ_TEST_FAULT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IRQ_TEST_DONE` reader - Sample done IRQ test bit
pub type IRQ_TEST_DONE_R = crate::BitReader;
///Field `IRQ_TEST_DONE` writer - Sample done IRQ test bit
pub type IRQ_TEST_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IRQ_TEST_ALARMA` reader - Alarm A IRQ test bit
pub type IRQ_TEST_ALARMA_R = crate::BitReader;
///Field `IRQ_TEST_ALARMA` writer - Alarm A IRQ test bit
pub type IRQ_TEST_ALARMA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IRQ_TEST_ALARMB` reader - Alarm B IRQ test bit
pub type IRQ_TEST_ALARMB_R = crate::BitReader;
///Field `IRQ_TEST_ALARMB` writer - Alarm B IRQ test bit
pub type IRQ_TEST_ALARMB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Fault IRQ test bit
    #[inline(always)]
    pub fn irq_test_fault(&self) -> IRQ_TEST_FAULT_R {
        IRQ_TEST_FAULT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Sample done IRQ test bit
    #[inline(always)]
    pub fn irq_test_done(&self) -> IRQ_TEST_DONE_R {
        IRQ_TEST_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Alarm A IRQ test bit
    #[inline(always)]
    pub fn irq_test_alarma(&self) -> IRQ_TEST_ALARMA_R {
        IRQ_TEST_ALARMA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Alarm B IRQ test bit
    #[inline(always)]
    pub fn irq_test_alarmb(&self) -> IRQ_TEST_ALARMB_R {
        IRQ_TEST_ALARMB_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TS0IRQTESTR")
            .field("irq_test_fault", &self.irq_test_fault())
            .field("irq_test_done", &self.irq_test_done())
            .field("irq_test_alarma", &self.irq_test_alarma())
            .field("irq_test_alarmb", &self.irq_test_alarmb())
            .finish()
    }
}
impl W {
    ///Bit 0 - Fault IRQ test bit
    #[inline(always)]
    pub fn irq_test_fault(&mut self) -> IRQ_TEST_FAULT_W<'_, TS0IRQTESTRrs> {
        IRQ_TEST_FAULT_W::new(self, 0)
    }
    ///Bit 1 - Sample done IRQ test bit
    #[inline(always)]
    pub fn irq_test_done(&mut self) -> IRQ_TEST_DONE_W<'_, TS0IRQTESTRrs> {
        IRQ_TEST_DONE_W::new(self, 1)
    }
    ///Bit 3 - Alarm A IRQ test bit
    #[inline(always)]
    pub fn irq_test_alarma(&mut self) -> IRQ_TEST_ALARMA_W<'_, TS0IRQTESTRrs> {
        IRQ_TEST_ALARMA_W::new(self, 3)
    }
    ///Bit 4 - Alarm B IRQ test bit
    #[inline(always)]
    pub fn irq_test_alarmb(&mut self) -> IRQ_TEST_ALARMB_W<'_, TS0IRQTESTRrs> {
        IRQ_TEST_ALARMB_W::new(self, 4)
    }
}
/**DTS TS0 IRQ test register

You can [`read`](crate::Reg::read) this register and get [`ts0irqtestr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts0irqtestr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DTS:TS0IRQTESTR)*/
pub struct TS0IRQTESTRrs;
impl crate::RegisterSpec for TS0IRQTESTRrs {
    type Ux = u32;
}
///`read()` method returns [`ts0irqtestr::R`](R) reader structure
impl crate::Readable for TS0IRQTESTRrs {}
///`write(|w| ..)` method takes [`ts0irqtestr::W`](W) writer structure
impl crate::Writable for TS0IRQTESTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TS0IRQTESTR to value 0
impl crate::Resettable for TS0IRQTESTRrs {}
