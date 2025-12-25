///Register `TS_MR` reader
pub type R = crate::R<TS_MRrs>;
///Register `TS_MR` writer
pub type W = crate::W<TS_MRrs>;
///Field `TS0_IRQ_MASK` reader - TS0 IRQ source mask bit
pub type TS0_IRQ_MASK_R = crate::BitReader;
///Field `TS0_IRQ_MASK` writer - TS0 IRQ source mask bit
pub type TS0_IRQ_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS1_IRQ_MASK` reader - TS1 IRQ source mask bit
pub type TS1_IRQ_MASK_R = crate::BitReader;
///Field `TS1_IRQ_MASK` writer - TS1 IRQ source mask bit
pub type TS1_IRQ_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TS0 IRQ source mask bit
    #[inline(always)]
    pub fn ts0_irq_mask(&self) -> TS0_IRQ_MASK_R {
        TS0_IRQ_MASK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TS1 IRQ source mask bit
    #[inline(always)]
    pub fn ts1_irq_mask(&self) -> TS1_IRQ_MASK_R {
        TS1_IRQ_MASK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TS_MR")
            .field("ts0_irq_mask", &self.ts0_irq_mask())
            .field("ts1_irq_mask", &self.ts1_irq_mask())
            .finish()
    }
}
impl W {
    ///Bit 0 - TS0 IRQ source mask bit
    #[inline(always)]
    pub fn ts0_irq_mask(&mut self) -> TS0_IRQ_MASK_W<'_, TS_MRrs> {
        TS0_IRQ_MASK_W::new(self, 0)
    }
    ///Bit 1 - TS1 IRQ source mask bit
    #[inline(always)]
    pub fn ts1_irq_mask(&mut self) -> TS1_IRQ_MASK_W<'_, TS_MRrs> {
        TS1_IRQ_MASK_W::new(self, 1)
    }
}
/**DTS PVT IRQ TS mask register

You can [`read`](crate::Reg::read) this register and get [`ts_mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DTS:TS_MR)*/
pub struct TS_MRrs;
impl crate::RegisterSpec for TS_MRrs {
    type Ux = u32;
}
///`read()` method returns [`ts_mr::R`](R) reader structure
impl crate::Readable for TS_MRrs {}
///`write(|w| ..)` method takes [`ts_mr::W`](W) writer structure
impl crate::Writable for TS_MRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TS_MR to value 0
impl crate::Resettable for TS_MRrs {}
