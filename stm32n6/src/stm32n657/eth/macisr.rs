///Register `MACISR` reader
pub type R = crate::R<MACISRrs>;
///Register `MACISR` writer
pub type W = crate::W<MACISRrs>;
///Field `RGSMIIIS` reader - RGMII Interrupt Status
pub type RGSMIIIS_R = crate::BitReader;
///Field `PHYIS` reader - PHY Interrupt
pub type PHYIS_R = crate::BitReader;
///Field `PMTIS` reader - PMT Interrupt Status
pub type PMTIS_R = crate::BitReader;
///Field `LPIIS` reader - LPI Interrupt Status
pub type LPIIS_R = crate::BitReader;
///Field `MMCIS` reader - MMC Interrupt Status
pub type MMCIS_R = crate::BitReader;
///Field `MMCRXIS` reader - MMC Receive Interrupt Status
pub type MMCRXIS_R = crate::BitReader;
///Field `MMCTXIS` reader - MMC Transmit Interrupt Status
pub type MMCTXIS_R = crate::BitReader;
/**Field `TSIS` reader - Timestamp Interrupt Status

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type TSIS_R = crate::BitReader;
///Field `TSIS` writer - Timestamp Interrupt Status
pub type TSIS_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `TXSTSIS` reader - Transmit Status Interrupt

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type TXSTSIS_R = crate::BitReader;
///Field `TXSTSIS` writer - Transmit Status Interrupt
pub type TXSTSIS_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `RXSTSIS` reader - Receive Status Interrupt

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type RXSTSIS_R = crate::BitReader;
///Field `RXSTSIS` writer - Receive Status Interrupt
pub type RXSTSIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPEIS` reader - Frame Preemption Interrupt Status
pub type FPEIS_R = crate::BitReader;
/**Field `MDIOIS` reader - MDIO Interrupt Status

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type MDIOIS_R = crate::BitReader;
///Field `MDIOIS` writer - MDIO Interrupt Status
pub type MDIOIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MFTIS` reader - MMC FPE Transmit Interrupt Status
pub type MFTIS_R = crate::BitReader;
///Field `MFRIS` reader - MMC FPE Receive Interrupt Status
pub type MFRIS_R = crate::BitReader;
impl R {
    ///Bit 0 - RGMII Interrupt Status
    #[inline(always)]
    pub fn rgsmiiis(&self) -> RGSMIIIS_R {
        RGSMIIIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - PHY Interrupt
    #[inline(always)]
    pub fn phyis(&self) -> PHYIS_R {
        PHYIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PMT Interrupt Status
    #[inline(always)]
    pub fn pmtis(&self) -> PMTIS_R {
        PMTIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - LPI Interrupt Status
    #[inline(always)]
    pub fn lpiis(&self) -> LPIIS_R {
        LPIIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - MMC Interrupt Status
    #[inline(always)]
    pub fn mmcis(&self) -> MMCIS_R {
        MMCIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - MMC Receive Interrupt Status
    #[inline(always)]
    pub fn mmcrxis(&self) -> MMCRXIS_R {
        MMCRXIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - MMC Transmit Interrupt Status
    #[inline(always)]
    pub fn mmctxis(&self) -> MMCTXIS_R {
        MMCTXIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Timestamp Interrupt Status
    #[inline(always)]
    pub fn tsis(&self) -> TSIS_R {
        TSIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Transmit Status Interrupt
    #[inline(always)]
    pub fn txstsis(&self) -> TXSTSIS_R {
        TXSTSIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Receive Status Interrupt
    #[inline(always)]
    pub fn rxstsis(&self) -> RXSTSIS_R {
        RXSTSIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - Frame Preemption Interrupt Status
    #[inline(always)]
    pub fn fpeis(&self) -> FPEIS_R {
        FPEIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - MDIO Interrupt Status
    #[inline(always)]
    pub fn mdiois(&self) -> MDIOIS_R {
        MDIOIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - MMC FPE Transmit Interrupt Status
    #[inline(always)]
    pub fn mftis(&self) -> MFTIS_R {
        MFTIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - MMC FPE Receive Interrupt Status
    #[inline(always)]
    pub fn mfris(&self) -> MFRIS_R {
        MFRIS_R::new(((self.bits >> 20) & 1) != 0)
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
            .field("fpeis", &self.fpeis())
            .field("mftis", &self.mftis())
            .field("mfris", &self.mfris())
            .finish()
    }
}
impl W {
    ///Bit 12 - Timestamp Interrupt Status
    #[inline(always)]
    pub fn tsis(&mut self) -> TSIS_W<'_, MACISRrs> {
        TSIS_W::new(self, 12)
    }
    ///Bit 13 - Transmit Status Interrupt
    #[inline(always)]
    pub fn txstsis(&mut self) -> TXSTSIS_W<'_, MACISRrs> {
        TXSTSIS_W::new(self, 13)
    }
    ///Bit 14 - Receive Status Interrupt
    #[inline(always)]
    pub fn rxstsis(&mut self) -> RXSTSIS_W<'_, MACISRrs> {
        RXSTSIS_W::new(self, 14)
    }
    ///Bit 18 - MDIO Interrupt Status
    #[inline(always)]
    pub fn mdiois(&mut self) -> MDIOIS_W<'_, MACISRrs> {
        MDIOIS_W::new(self, 18)
    }
}
/**Interrupt status register

You can [`read`](crate::Reg::read) this register and get [`macisr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macisr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:MACISR)*/
pub struct MACISRrs;
impl crate::RegisterSpec for MACISRrs {
    type Ux = u32;
}
///`read()` method returns [`macisr::R`](R) reader structure
impl crate::Readable for MACISRrs {}
///`write(|w| ..)` method takes [`macisr::W`](W) writer structure
impl crate::Writable for MACISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACISR to value 0
impl crate::Resettable for MACISRrs {}
