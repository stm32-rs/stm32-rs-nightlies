///Register `TS0_IER` reader
pub type R = crate::R<TS0_IERrs>;
///Register `TS0_IER` writer
pub type W = crate::W<TS0_IERrs>;
///Field `IRQ_EN_FAULT` reader - Fault IRQ enable bit
pub type IRQ_EN_FAULT_R = crate::BitReader;
///Field `IRQ_EN_FAULT` writer - Fault IRQ enable bit
pub type IRQ_EN_FAULT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IRQ_EN_DONE` reader - Sample done IRQ enable bit
pub type IRQ_EN_DONE_R = crate::BitReader;
///Field `IRQ_EN_DONE` writer - Sample done IRQ enable bit
pub type IRQ_EN_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IRQ_EN_ALARMA` reader - Alarm A IRQ enable bit
pub type IRQ_EN_ALARMA_R = crate::BitReader;
///Field `IRQ_EN_ALARMA` writer - Alarm A IRQ enable bit
pub type IRQ_EN_ALARMA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IRQ_EN_ALARMB` reader - Alarm B IRQ enable bit
pub type IRQ_EN_ALARMB_R = crate::BitReader;
///Field `IRQ_EN_ALARMB` writer - Alarm B IRQ enable bit
pub type IRQ_EN_ALARMB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Fault IRQ enable bit
    #[inline(always)]
    pub fn irq_en_fault(&self) -> IRQ_EN_FAULT_R {
        IRQ_EN_FAULT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Sample done IRQ enable bit
    #[inline(always)]
    pub fn irq_en_done(&self) -> IRQ_EN_DONE_R {
        IRQ_EN_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Alarm A IRQ enable bit
    #[inline(always)]
    pub fn irq_en_alarma(&self) -> IRQ_EN_ALARMA_R {
        IRQ_EN_ALARMA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Alarm B IRQ enable bit
    #[inline(always)]
    pub fn irq_en_alarmb(&self) -> IRQ_EN_ALARMB_R {
        IRQ_EN_ALARMB_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TS0_IER")
            .field("irq_en_fault", &self.irq_en_fault())
            .field("irq_en_done", &self.irq_en_done())
            .field("irq_en_alarma", &self.irq_en_alarma())
            .field("irq_en_alarmb", &self.irq_en_alarmb())
            .finish()
    }
}
impl W {
    ///Bit 0 - Fault IRQ enable bit
    #[inline(always)]
    pub fn irq_en_fault(&mut self) -> IRQ_EN_FAULT_W<'_, TS0_IERrs> {
        IRQ_EN_FAULT_W::new(self, 0)
    }
    ///Bit 1 - Sample done IRQ enable bit
    #[inline(always)]
    pub fn irq_en_done(&mut self) -> IRQ_EN_DONE_W<'_, TS0_IERrs> {
        IRQ_EN_DONE_W::new(self, 1)
    }
    ///Bit 3 - Alarm A IRQ enable bit
    #[inline(always)]
    pub fn irq_en_alarma(&mut self) -> IRQ_EN_ALARMA_W<'_, TS0_IERrs> {
        IRQ_EN_ALARMA_W::new(self, 3)
    }
    ///Bit 4 - Alarm B IRQ enable bit
    #[inline(always)]
    pub fn irq_en_alarmb(&mut self) -> IRQ_EN_ALARMB_W<'_, TS0_IERrs> {
        IRQ_EN_ALARMB_W::new(self, 4)
    }
}
/**DTS TS0 IRQ enable register

You can [`read`](crate::Reg::read) this register and get [`ts0_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts0_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DTS:TS0_IER)*/
pub struct TS0_IERrs;
impl crate::RegisterSpec for TS0_IERrs {
    type Ux = u32;
}
///`read()` method returns [`ts0_ier::R`](R) reader structure
impl crate::Readable for TS0_IERrs {}
///`write(|w| ..)` method takes [`ts0_ier::W`](W) writer structure
impl crate::Writable for TS0_IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TS0_IER to value 0
impl crate::Resettable for TS0_IERrs {}
