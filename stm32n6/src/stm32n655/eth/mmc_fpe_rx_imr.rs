///Register `MMC_FPE_RX_IMR` reader
pub type R = crate::R<MMC_FPE_RX_IMRrs>;
///Register `MMC_FPE_RX_IMR` writer
pub type W = crate::W<MMC_FPE_RX_IMRrs>;
///Field `PAECIM` reader - MMC Rx Packet Assembly Error Counter Interrupt Mask
pub type PAECIM_R = crate::BitReader;
///Field `PAECIM` writer - MMC Rx Packet Assembly Error Counter Interrupt Mask
pub type PAECIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSECIM` reader - MMC Rx Packet SMD Error Counter Interrupt Mask
pub type PSECIM_R = crate::BitReader;
///Field `PSECIM` writer - MMC Rx Packet SMD Error Counter Interrupt Mask
pub type PSECIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PAOCIM` reader - MMC Rx Packet Assembly OK Counter Interrupt Mask
pub type PAOCIM_R = crate::BitReader;
///Field `PAOCIM` writer - MMC Rx Packet Assembly OK Counter Interrupt Mask
pub type PAOCIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FCIM` reader - MMC Rx FPE Fragment Counter Interrupt Mask
pub type FCIM_R = crate::BitReader;
///Field `FCIM` writer - MMC Rx FPE Fragment Counter Interrupt Mask
pub type FCIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MMC Rx Packet Assembly Error Counter Interrupt Mask
    #[inline(always)]
    pub fn paecim(&self) -> PAECIM_R {
        PAECIM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MMC Rx Packet SMD Error Counter Interrupt Mask
    #[inline(always)]
    pub fn psecim(&self) -> PSECIM_R {
        PSECIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MMC Rx Packet Assembly OK Counter Interrupt Mask
    #[inline(always)]
    pub fn paocim(&self) -> PAOCIM_R {
        PAOCIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MMC Rx FPE Fragment Counter Interrupt Mask
    #[inline(always)]
    pub fn fcim(&self) -> FCIM_R {
        FCIM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMC_FPE_RX_IMR")
            .field("paecim", &self.paecim())
            .field("psecim", &self.psecim())
            .field("paocim", &self.paocim())
            .field("fcim", &self.fcim())
            .finish()
    }
}
impl W {
    ///Bit 0 - MMC Rx Packet Assembly Error Counter Interrupt Mask
    #[inline(always)]
    pub fn paecim(&mut self) -> PAECIM_W<'_, MMC_FPE_RX_IMRrs> {
        PAECIM_W::new(self, 0)
    }
    ///Bit 1 - MMC Rx Packet SMD Error Counter Interrupt Mask
    #[inline(always)]
    pub fn psecim(&mut self) -> PSECIM_W<'_, MMC_FPE_RX_IMRrs> {
        PSECIM_W::new(self, 1)
    }
    ///Bit 2 - MMC Rx Packet Assembly OK Counter Interrupt Mask
    #[inline(always)]
    pub fn paocim(&mut self) -> PAOCIM_W<'_, MMC_FPE_RX_IMRrs> {
        PAOCIM_W::new(self, 2)
    }
    ///Bit 3 - MMC Rx FPE Fragment Counter Interrupt Mask
    #[inline(always)]
    pub fn fcim(&mut self) -> FCIM_W<'_, MMC_FPE_RX_IMRrs> {
        FCIM_W::new(self, 3)
    }
}
/**MMC FPE Rx interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`mmc_fpe_rx_imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_fpe_rx_imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MMC_FPE_RX_IMR)*/
pub struct MMC_FPE_RX_IMRrs;
impl crate::RegisterSpec for MMC_FPE_RX_IMRrs {
    type Ux = u32;
}
///`read()` method returns [`mmc_fpe_rx_imr::R`](R) reader structure
impl crate::Readable for MMC_FPE_RX_IMRrs {}
///`write(|w| ..)` method takes [`mmc_fpe_rx_imr::W`](W) writer structure
impl crate::Writable for MMC_FPE_RX_IMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMC_FPE_RX_IMR to value 0
impl crate::Resettable for MMC_FPE_RX_IMRrs {}
