///Register `MMC_RX_INTERRUPT_MASK` reader
pub type R = crate::R<MMC_RX_INTERRUPT_MASKrs>;
///Register `MMC_RX_INTERRUPT_MASK` writer
pub type W = crate::W<MMC_RX_INTERRUPT_MASKrs>;
///Field `RXCRCERPIM` reader - RXCRCERPIM
pub type RXCRCERPIM_R = crate::BitReader;
///Field `RXCRCERPIM` writer - RXCRCERPIM
pub type RXCRCERPIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXALGNERPIM` reader - RXALGNERPIM
pub type RXALGNERPIM_R = crate::BitReader;
///Field `RXALGNERPIM` writer - RXALGNERPIM
pub type RXALGNERPIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXUCGPIM` reader - RXUCGPIM
pub type RXUCGPIM_R = crate::BitReader;
///Field `RXUCGPIM` writer - RXUCGPIM
pub type RXUCGPIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXLPIUSCIM` reader - RXLPIUSCIM
pub type RXLPIUSCIM_R = crate::BitReader;
///Field `RXLPIUSCIM` writer - RXLPIUSCIM
pub type RXLPIUSCIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXLPITRCIM` reader - RXLPITRCIM
pub type RXLPITRCIM_R = crate::BitReader;
impl R {
    ///Bit 5 - RXCRCERPIM
    #[inline(always)]
    pub fn rxcrcerpim(&self) -> RXCRCERPIM_R {
        RXCRCERPIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RXALGNERPIM
    #[inline(always)]
    pub fn rxalgnerpim(&self) -> RXALGNERPIM_R {
        RXALGNERPIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 17 - RXUCGPIM
    #[inline(always)]
    pub fn rxucgpim(&self) -> RXUCGPIM_R {
        RXUCGPIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 26 - RXLPIUSCIM
    #[inline(always)]
    pub fn rxlpiuscim(&self) -> RXLPIUSCIM_R {
        RXLPIUSCIM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - RXLPITRCIM
    #[inline(always)]
    pub fn rxlpitrcim(&self) -> RXLPITRCIM_R {
        RXLPITRCIM_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMC_RX_INTERRUPT_MASK")
            .field("rxcrcerpim", &self.rxcrcerpim())
            .field("rxalgnerpim", &self.rxalgnerpim())
            .field("rxucgpim", &self.rxucgpim())
            .field("rxlpiuscim", &self.rxlpiuscim())
            .field("rxlpitrcim", &self.rxlpitrcim())
            .finish()
    }
}
impl W {
    ///Bit 5 - RXCRCERPIM
    #[inline(always)]
    pub fn rxcrcerpim(&mut self) -> RXCRCERPIM_W<'_, MMC_RX_INTERRUPT_MASKrs> {
        RXCRCERPIM_W::new(self, 5)
    }
    ///Bit 6 - RXALGNERPIM
    #[inline(always)]
    pub fn rxalgnerpim(&mut self) -> RXALGNERPIM_W<'_, MMC_RX_INTERRUPT_MASKrs> {
        RXALGNERPIM_W::new(self, 6)
    }
    ///Bit 17 - RXUCGPIM
    #[inline(always)]
    pub fn rxucgpim(&mut self) -> RXUCGPIM_W<'_, MMC_RX_INTERRUPT_MASKrs> {
        RXUCGPIM_W::new(self, 17)
    }
    ///Bit 26 - RXLPIUSCIM
    #[inline(always)]
    pub fn rxlpiuscim(&mut self) -> RXLPIUSCIM_W<'_, MMC_RX_INTERRUPT_MASKrs> {
        RXLPIUSCIM_W::new(self, 26)
    }
}
/**The MMC Receive Interrupt Mask register maintains the masks for the interrupts generated when receive statistic counters reach half of their maximum value or the maximum values.

You can [`read`](crate::Reg::read) this register and get [`mmc_rx_interrupt_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_rx_interrupt_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:MMC_RX_INTERRUPT_MASK)*/
pub struct MMC_RX_INTERRUPT_MASKrs;
impl crate::RegisterSpec for MMC_RX_INTERRUPT_MASKrs {
    type Ux = u32;
}
///`read()` method returns [`mmc_rx_interrupt_mask::R`](R) reader structure
impl crate::Readable for MMC_RX_INTERRUPT_MASKrs {}
///`write(|w| ..)` method takes [`mmc_rx_interrupt_mask::W`](W) writer structure
impl crate::Writable for MMC_RX_INTERRUPT_MASKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMC_RX_INTERRUPT_MASK to value 0
impl crate::Resettable for MMC_RX_INTERRUPT_MASKrs {}
