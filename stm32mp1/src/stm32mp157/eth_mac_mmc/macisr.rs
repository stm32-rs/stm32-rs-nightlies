///Register `MACISR` reader
pub type R = crate::R<MACISRrs>;
///Field `RGSMIIIS` reader - RGSMIIIS
pub type RGSMIIIS_R = crate::BitReader;
///Field `PHYIS` reader - PHYIS
pub type PHYIS_R = crate::BitReader;
///Field `PMTIS` reader - PMTIS
pub type PMTIS_R = crate::BitReader;
///Field `LPIIS` reader - LPIIS
pub type LPIIS_R = crate::BitReader;
///Field `MMCIS` reader - MMCIS
pub type MMCIS_R = crate::BitReader;
///Field `MMCRXIS` reader - MMCRXIS
pub type MMCRXIS_R = crate::BitReader;
///Field `MMCTXIS` reader - MMCTXIS
pub type MMCTXIS_R = crate::BitReader;
///Field `TSIS` reader - TSIS
pub type TSIS_R = crate::BitReader;
///Field `TXSTSIS` reader - TXSTSIS
pub type TXSTSIS_R = crate::BitReader;
///Field `RXSTSIS` reader - RXSTSIS
pub type RXSTSIS_R = crate::BitReader;
impl R {
    ///Bit 0 - RGSMIIIS
    #[inline(always)]
    pub fn rgsmiiis(&self) -> RGSMIIIS_R {
        RGSMIIIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - PHYIS
    #[inline(always)]
    pub fn phyis(&self) -> PHYIS_R {
        PHYIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PMTIS
    #[inline(always)]
    pub fn pmtis(&self) -> PMTIS_R {
        PMTIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - LPIIS
    #[inline(always)]
    pub fn lpiis(&self) -> LPIIS_R {
        LPIIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - MMCIS
    #[inline(always)]
    pub fn mmcis(&self) -> MMCIS_R {
        MMCIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - MMCRXIS
    #[inline(always)]
    pub fn mmcrxis(&self) -> MMCRXIS_R {
        MMCRXIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - MMCTXIS
    #[inline(always)]
    pub fn mmctxis(&self) -> MMCTXIS_R {
        MMCTXIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - TSIS
    #[inline(always)]
    pub fn tsis(&self) -> TSIS_R {
        TSIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TXSTSIS
    #[inline(always)]
    pub fn txstsis(&self) -> TXSTSIS_R {
        TXSTSIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - RXSTSIS
    #[inline(always)]
    pub fn rxstsis(&self) -> RXSTSIS_R {
        RXSTSIS_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACISR")
            .field("rgsmiiis", &self.rgsmiiis())
            .field("phyis", &self.phyis())
            .field("pmtis", &self.pmtis())
            .field("lpiis", &self.lpiis())
            .field("mmcis", &self.mmcis())
            .field("mmcrxis", &self.mmcrxis())
            .field("mmctxis", &self.mmctxis())
            .field("tsis", &self.tsis())
            .field("txstsis", &self.txstsis())
            .field("rxstsis", &self.rxstsis())
            .finish()
    }
}
/**The Interrupt Status register contains the status of interrupts.

You can [`read`](crate::Reg::read) this register and get [`macisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:MACISR)*/
pub struct MACISRrs;
impl crate::RegisterSpec for MACISRrs {
    type Ux = u32;
}
///`read()` method returns [`macisr::R`](R) reader structure
impl crate::Readable for MACISRrs {}
///`reset()` method sets MACISR to value 0
impl crate::Resettable for MACISRrs {}
