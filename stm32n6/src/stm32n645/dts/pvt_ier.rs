///Register `PVT_IER` reader
pub type R = crate::R<PVT_IERrs>;
///Register `PVT_IER` writer
pub type W = crate::W<PVT_IERrs>;
///Field `TMR_IRQ_ENABLE` reader - Timer IRQ source enable bit
pub type TMR_IRQ_ENABLE_R = crate::BitReader;
///Field `TMR_IRQ_ENABLE` writer - Timer IRQ source enable bit
pub type TMR_IRQ_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS_IRQ_ENABLE` reader - TS IRQ source enable bit
pub type TS_IRQ_ENABLE_R = crate::BitReader;
///Field `TS_IRQ_ENABLE` writer - TS IRQ source enable bit
pub type TS_IRQ_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Timer IRQ source enable bit
    #[inline(always)]
    pub fn tmr_irq_enable(&self) -> TMR_IRQ_ENABLE_R {
        TMR_IRQ_ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TS IRQ source enable bit
    #[inline(always)]
    pub fn ts_irq_enable(&self) -> TS_IRQ_ENABLE_R {
        TS_IRQ_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT_IER")
            .field("tmr_irq_enable", &self.tmr_irq_enable())
            .field("ts_irq_enable", &self.ts_irq_enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - Timer IRQ source enable bit
    #[inline(always)]
    pub fn tmr_irq_enable(&mut self) -> TMR_IRQ_ENABLE_W<'_, PVT_IERrs> {
        TMR_IRQ_ENABLE_W::new(self, 0)
    }
    ///Bit 1 - TS IRQ source enable bit
    #[inline(always)]
    pub fn ts_irq_enable(&mut self) -> TS_IRQ_ENABLE_W<'_, PVT_IERrs> {
        TS_IRQ_ENABLE_W::new(self, 1)
    }
}
/**DTS PVT IRQ enable register

You can [`read`](crate::Reg::read) this register and get [`pvt_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DTS:PVT_IER)*/
pub struct PVT_IERrs;
impl crate::RegisterSpec for PVT_IERrs {
    type Ux = u32;
}
///`read()` method returns [`pvt_ier::R`](R) reader structure
impl crate::Readable for PVT_IERrs {}
///`write(|w| ..)` method takes [`pvt_ier::W`](W) writer structure
impl crate::Writable for PVT_IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PVT_IER to value 0
impl crate::Resettable for PVT_IERrs {}
