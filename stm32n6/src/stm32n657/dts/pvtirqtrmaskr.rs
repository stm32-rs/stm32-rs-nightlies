///Register `PVTIRQTRMASKR` reader
pub type R = crate::R<PVTIRQTRMASKRrs>;
///Register `PVTIRQTRMASKR` writer
pub type W = crate::W<PVTIRQTRMASKRrs>;
///Field `TMR_IRQ_MASK` reader - Timer IRQ source mask bit
pub type TMR_IRQ_MASK_R = crate::BitReader;
///Field `TMR_IRQ_MASK` writer - Timer IRQ source mask bit
pub type TMR_IRQ_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Timer IRQ source mask bit
    #[inline(always)]
    pub fn tmr_irq_mask(&self) -> TMR_IRQ_MASK_R {
        TMR_IRQ_MASK_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVTIRQTRMASKR")
            .field("tmr_irq_mask", &self.tmr_irq_mask())
            .finish()
    }
}
impl W {
    ///Bit 0 - Timer IRQ source mask bit
    #[inline(always)]
    pub fn tmr_irq_mask(&mut self) -> TMR_IRQ_MASK_W<'_, PVTIRQTRMASKRrs> {
        TMR_IRQ_MASK_W::new(self, 0)
    }
}
/**DTS PVT IRQ timer mask register

You can [`read`](crate::Reg::read) this register and get [`pvtirqtrmaskr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvtirqtrmaskr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DTS:PVTIRQTRMASKR)*/
pub struct PVTIRQTRMASKRrs;
impl crate::RegisterSpec for PVTIRQTRMASKRrs {
    type Ux = u32;
}
///`read()` method returns [`pvtirqtrmaskr::R`](R) reader structure
impl crate::Readable for PVTIRQTRMASKRrs {}
///`write(|w| ..)` method takes [`pvtirqtrmaskr::W`](W) writer structure
impl crate::Writable for PVTIRQTRMASKRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PVTIRQTRMASKR to value 0
impl crate::Resettable for PVTIRQTRMASKRrs {}
