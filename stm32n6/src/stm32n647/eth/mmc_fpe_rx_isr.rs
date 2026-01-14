///Register `MMC_FPE_RX_ISR` reader
pub type R = crate::R<MMC_FPE_RX_ISRrs>;
///Field `PAECIS` reader - MMC Rx Packet Assembly Error Counter Interrupt Status
pub type PAECIS_R = crate::BitReader;
///Field `PSECIS` reader - MMC Rx Packet SMD Error Counter Interrupt Status
pub type PSECIS_R = crate::BitReader;
///Field `PAOCIS` reader - MMC Rx Packet Assembly OK Counter Interrupt Status
pub type PAOCIS_R = crate::BitReader;
///Field `FCIS` reader - MMC Rx FPE Fragment Counter Interrupt Status
pub type FCIS_R = crate::BitReader;
impl R {
    ///Bit 0 - MMC Rx Packet Assembly Error Counter Interrupt Status
    #[inline(always)]
    pub fn paecis(&self) -> PAECIS_R {
        PAECIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MMC Rx Packet SMD Error Counter Interrupt Status
    #[inline(always)]
    pub fn psecis(&self) -> PSECIS_R {
        PSECIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MMC Rx Packet Assembly OK Counter Interrupt Status
    #[inline(always)]
    pub fn paocis(&self) -> PAOCIS_R {
        PAOCIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MMC Rx FPE Fragment Counter Interrupt Status
    #[inline(always)]
    pub fn fcis(&self) -> FCIS_R {
        FCIS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMC_FPE_RX_ISR")
            .field("paecis", &self.paecis())
            .field("psecis", &self.psecis())
            .field("paocis", &self.paocis())
            .field("fcis", &self.fcis())
            .finish()
    }
}
/**MMC FPE Rx interrupt status register

You can [`read`](crate::Reg::read) this register and get [`mmc_fpe_rx_isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ETH:MMC_FPE_RX_ISR)*/
pub struct MMC_FPE_RX_ISRrs;
impl crate::RegisterSpec for MMC_FPE_RX_ISRrs {
    type Ux = u32;
}
///`read()` method returns [`mmc_fpe_rx_isr::R`](R) reader structure
impl crate::Readable for MMC_FPE_RX_ISRrs {}
///`reset()` method sets MMC_FPE_RX_ISR to value 0
impl crate::Resettable for MMC_FPE_RX_ISRrs {}
