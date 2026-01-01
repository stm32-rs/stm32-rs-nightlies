///Register `TS_ISR` reader
pub type R = crate::R<TS_ISRrs>;
///Field `TS0_IRQ_STATUS` reader - TS0 IRQ status bit after masking
pub type TS0_IRQ_STATUS_R = crate::BitReader;
///Field `TS1_IRQ_STATUS` reader - TS1 IRQ status bit after masking
pub type TS1_IRQ_STATUS_R = crate::BitReader;
impl R {
    ///Bit 0 - TS0 IRQ status bit after masking
    #[inline(always)]
    pub fn ts0_irq_status(&self) -> TS0_IRQ_STATUS_R {
        TS0_IRQ_STATUS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TS1 IRQ status bit after masking
    #[inline(always)]
    pub fn ts1_irq_status(&self) -> TS1_IRQ_STATUS_R {
        TS1_IRQ_STATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TS_ISR")
            .field("ts0_irq_status", &self.ts0_irq_status())
            .field("ts1_irq_status", &self.ts1_irq_status())
            .finish()
    }
}
/**DTS PVT IRQ TS status register

You can [`read`](crate::Reg::read) this register and get [`ts_isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DTS:TS_ISR)*/
pub struct TS_ISRrs;
impl crate::RegisterSpec for TS_ISRrs {
    type Ux = u32;
}
///`read()` method returns [`ts_isr::R`](R) reader structure
impl crate::Readable for TS_ISRrs {}
///`reset()` method sets TS_ISR to value 0
impl crate::Resettable for TS_ISRrs {}
