///Register `MMC_FPE_TX_ISR` reader
pub type R = crate::R<MMC_FPE_TX_ISRrs>;
///Register `MMC_FPE_TX_ISR` writer
pub type W = crate::W<MMC_FPE_TX_ISRrs>;
/**Field `FCIS` reader - MMC Tx FPE Fragment Counter Interrupt status

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type FCIS_R = crate::BitReader;
///Field `FCIS` writer - MMC Tx FPE Fragment Counter Interrupt status
pub type FCIS_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `HRCIS` reader - MMC Tx Hold Request Counter Interrupt Status

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type HRCIS_R = crate::BitReader;
///Field `HRCIS` writer - MMC Tx Hold Request Counter Interrupt Status
pub type HRCIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MMC Tx FPE Fragment Counter Interrupt status
    #[inline(always)]
    pub fn fcis(&self) -> FCIS_R {
        FCIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MMC Tx Hold Request Counter Interrupt Status
    #[inline(always)]
    pub fn hrcis(&self) -> HRCIS_R {
        HRCIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMC_FPE_TX_ISR").finish()
    }
}
impl W {
    ///Bit 0 - MMC Tx FPE Fragment Counter Interrupt status
    #[inline(always)]
    pub fn fcis(&mut self) -> FCIS_W<'_, MMC_FPE_TX_ISRrs> {
        FCIS_W::new(self, 0)
    }
    ///Bit 1 - MMC Tx Hold Request Counter Interrupt Status
    #[inline(always)]
    pub fn hrcis(&mut self) -> HRCIS_W<'_, MMC_FPE_TX_ISRrs> {
        HRCIS_W::new(self, 1)
    }
}
/**MMC FPE Tx interrupt status register

You can [`read`](crate::Reg::read) this register and get [`mmc_fpe_tx_isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_fpe_tx_isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MMC_FPE_TX_ISR)*/
pub struct MMC_FPE_TX_ISRrs;
impl crate::RegisterSpec for MMC_FPE_TX_ISRrs {
    type Ux = u32;
}
///`read()` method returns [`mmc_fpe_tx_isr::R`](R) reader structure
impl crate::Readable for MMC_FPE_TX_ISRrs {}
///`write(|w| ..)` method takes [`mmc_fpe_tx_isr::W`](W) writer structure
impl crate::Writable for MMC_FPE_TX_ISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMC_FPE_TX_ISR to value 0
impl crate::Resettable for MMC_FPE_TX_ISRrs {}
