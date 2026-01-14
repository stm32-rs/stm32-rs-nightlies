///Register `MMC_FPE_TX_IMR` reader
pub type R = crate::R<MMC_FPE_TX_IMRrs>;
///Register `MMC_FPE_TX_IMR` writer
pub type W = crate::W<MMC_FPE_TX_IMRrs>;
///Field `FCIM` reader - MMC Transmit Fragment Counter Interrupt Mask
pub type FCIM_R = crate::BitReader;
///Field `FCIM` writer - MMC Transmit Fragment Counter Interrupt Mask
pub type FCIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HRCIM` reader - MMC Transmit Hold Request Counter Interrupt Mask
pub type HRCIM_R = crate::BitReader;
///Field `HRCIM` writer - MMC Transmit Hold Request Counter Interrupt Mask
pub type HRCIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MMC Transmit Fragment Counter Interrupt Mask
    #[inline(always)]
    pub fn fcim(&self) -> FCIM_R {
        FCIM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MMC Transmit Hold Request Counter Interrupt Mask
    #[inline(always)]
    pub fn hrcim(&self) -> HRCIM_R {
        HRCIM_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMC_FPE_TX_IMR")
            .field("fcim", &self.fcim())
            .field("hrcim", &self.hrcim())
            .finish()
    }
}
impl W {
    ///Bit 0 - MMC Transmit Fragment Counter Interrupt Mask
    #[inline(always)]
    pub fn fcim(&mut self) -> FCIM_W<'_, MMC_FPE_TX_IMRrs> {
        FCIM_W::new(self, 0)
    }
    ///Bit 1 - MMC Transmit Hold Request Counter Interrupt Mask
    #[inline(always)]
    pub fn hrcim(&mut self) -> HRCIM_W<'_, MMC_FPE_TX_IMRrs> {
        HRCIM_W::new(self, 1)
    }
}
/**MMC FPE Tx interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`mmc_fpe_tx_imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_fpe_tx_imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:MMC_FPE_TX_IMR)*/
pub struct MMC_FPE_TX_IMRrs;
impl crate::RegisterSpec for MMC_FPE_TX_IMRrs {
    type Ux = u32;
}
///`read()` method returns [`mmc_fpe_tx_imr::R`](R) reader structure
impl crate::Readable for MMC_FPE_TX_IMRrs {}
///`write(|w| ..)` method takes [`mmc_fpe_tx_imr::W`](W) writer structure
impl crate::Writable for MMC_FPE_TX_IMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMC_FPE_TX_IMR to value 0
impl crate::Resettable for MMC_FPE_TX_IMRrs {}
