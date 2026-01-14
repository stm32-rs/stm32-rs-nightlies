///Register `AHB5ENCR` writer
pub type W = crate::W<AHB5ENCRrs>;
///Field `HPDMA1ENC` writer - HPDMA1 enable
pub type HPDMA1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2DENC` writer - DMA2D enable
pub type DMA2DENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JPEGENC` writer - JPEG enable
pub type JPEGENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMCENC` writer - FMC enable
pub type FMCENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI1ENC` writer - XSPI1 enable
pub type XSPI1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSSIENC` writer - PSSI enable
pub type PSSIENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC2ENC` writer - SDMMC2 enable
pub type SDMMC2ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1ENC` writer - SDMMC1 enable
pub type SDMMC1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI2ENC` writer - XSPI2 enable
pub type XSPI2ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIMENC` writer - XSPIM enable
pub type XSPIMENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE1ENC` writer - MCE1 enable
pub type MCE1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE2ENC` writer - MCE2 enable
pub type MCE2ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE3ENC` writer - MCE3 enable
pub type MCE3ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI3ENC` writer - XSPI3 enable
pub type XSPI3ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE4ENC` writer - MCE4 enable
pub type MCE4ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXMMUENC` writer - GFXMMU enable
pub type GFXMMUENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPUENC` writer - GPU enable
pub type GPUENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1MACENC` writer - ETH1MAC enable
pub type ETH1MACENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1TXENC` writer - ETH1TX enable
pub type ETH1TXENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1RXENC` writer - ETH1RX enable
pub type ETH1RXENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1ENC` writer - ETH1 enable
pub type ETH1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG1ENC` writer - OTG1 enable
pub type OTG1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGPHY1ENC` writer - OTGPHY1 enable
pub type OTGPHY1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGPHY2ENC` writer - OTGPHY2 enable
pub type OTGPHY2ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG2ENC` writer - OTG2 enable
pub type OTG2ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUCACHEENC` writer - NPUCACHE enable
pub type NPUCACHEENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUENC` writer - NPU enable
pub type NPUENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB5ENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - HPDMA1 enable
    #[inline(always)]
    pub fn hpdma1enc(&mut self) -> HPDMA1ENC_W<'_, AHB5ENCRrs> {
        HPDMA1ENC_W::new(self, 0)
    }
    ///Bit 1 - DMA2D enable
    #[inline(always)]
    pub fn dma2denc(&mut self) -> DMA2DENC_W<'_, AHB5ENCRrs> {
        DMA2DENC_W::new(self, 1)
    }
    ///Bit 3 - JPEG enable
    #[inline(always)]
    pub fn jpegenc(&mut self) -> JPEGENC_W<'_, AHB5ENCRrs> {
        JPEGENC_W::new(self, 3)
    }
    ///Bit 4 - FMC enable
    #[inline(always)]
    pub fn fmcenc(&mut self) -> FMCENC_W<'_, AHB5ENCRrs> {
        FMCENC_W::new(self, 4)
    }
    ///Bit 5 - XSPI1 enable
    #[inline(always)]
    pub fn xspi1enc(&mut self) -> XSPI1ENC_W<'_, AHB5ENCRrs> {
        XSPI1ENC_W::new(self, 5)
    }
    ///Bit 6 - PSSI enable
    #[inline(always)]
    pub fn pssienc(&mut self) -> PSSIENC_W<'_, AHB5ENCRrs> {
        PSSIENC_W::new(self, 6)
    }
    ///Bit 7 - SDMMC2 enable
    #[inline(always)]
    pub fn sdmmc2enc(&mut self) -> SDMMC2ENC_W<'_, AHB5ENCRrs> {
        SDMMC2ENC_W::new(self, 7)
    }
    ///Bit 8 - SDMMC1 enable
    #[inline(always)]
    pub fn sdmmc1enc(&mut self) -> SDMMC1ENC_W<'_, AHB5ENCRrs> {
        SDMMC1ENC_W::new(self, 8)
    }
    ///Bit 12 - XSPI2 enable
    #[inline(always)]
    pub fn xspi2enc(&mut self) -> XSPI2ENC_W<'_, AHB5ENCRrs> {
        XSPI2ENC_W::new(self, 12)
    }
    ///Bit 13 - XSPIM enable
    #[inline(always)]
    pub fn xspimenc(&mut self) -> XSPIMENC_W<'_, AHB5ENCRrs> {
        XSPIMENC_W::new(self, 13)
    }
    ///Bit 14 - MCE1 enable
    #[inline(always)]
    pub fn mce1enc(&mut self) -> MCE1ENC_W<'_, AHB5ENCRrs> {
        MCE1ENC_W::new(self, 14)
    }
    ///Bit 15 - MCE2 enable
    #[inline(always)]
    pub fn mce2enc(&mut self) -> MCE2ENC_W<'_, AHB5ENCRrs> {
        MCE2ENC_W::new(self, 15)
    }
    ///Bit 16 - MCE3 enable
    #[inline(always)]
    pub fn mce3enc(&mut self) -> MCE3ENC_W<'_, AHB5ENCRrs> {
        MCE3ENC_W::new(self, 16)
    }
    ///Bit 17 - XSPI3 enable
    #[inline(always)]
    pub fn xspi3enc(&mut self) -> XSPI3ENC_W<'_, AHB5ENCRrs> {
        XSPI3ENC_W::new(self, 17)
    }
    ///Bit 18 - MCE4 enable
    #[inline(always)]
    pub fn mce4enc(&mut self) -> MCE4ENC_W<'_, AHB5ENCRrs> {
        MCE4ENC_W::new(self, 18)
    }
    ///Bit 19 - GFXMMU enable
    #[inline(always)]
    pub fn gfxmmuenc(&mut self) -> GFXMMUENC_W<'_, AHB5ENCRrs> {
        GFXMMUENC_W::new(self, 19)
    }
    ///Bit 20 - GPU enable
    #[inline(always)]
    pub fn gpuenc(&mut self) -> GPUENC_W<'_, AHB5ENCRrs> {
        GPUENC_W::new(self, 20)
    }
    ///Bit 22 - ETH1MAC enable
    #[inline(always)]
    pub fn eth1macenc(&mut self) -> ETH1MACENC_W<'_, AHB5ENCRrs> {
        ETH1MACENC_W::new(self, 22)
    }
    ///Bit 23 - ETH1TX enable
    #[inline(always)]
    pub fn eth1txenc(&mut self) -> ETH1TXENC_W<'_, AHB5ENCRrs> {
        ETH1TXENC_W::new(self, 23)
    }
    ///Bit 24 - ETH1RX enable
    #[inline(always)]
    pub fn eth1rxenc(&mut self) -> ETH1RXENC_W<'_, AHB5ENCRrs> {
        ETH1RXENC_W::new(self, 24)
    }
    ///Bit 25 - ETH1 enable
    #[inline(always)]
    pub fn eth1enc(&mut self) -> ETH1ENC_W<'_, AHB5ENCRrs> {
        ETH1ENC_W::new(self, 25)
    }
    ///Bit 26 - OTG1 enable
    #[inline(always)]
    pub fn otg1enc(&mut self) -> OTG1ENC_W<'_, AHB5ENCRrs> {
        OTG1ENC_W::new(self, 26)
    }
    ///Bit 27 - OTGPHY1 enable
    #[inline(always)]
    pub fn otgphy1enc(&mut self) -> OTGPHY1ENC_W<'_, AHB5ENCRrs> {
        OTGPHY1ENC_W::new(self, 27)
    }
    ///Bit 28 - OTGPHY2 enable
    #[inline(always)]
    pub fn otgphy2enc(&mut self) -> OTGPHY2ENC_W<'_, AHB5ENCRrs> {
        OTGPHY2ENC_W::new(self, 28)
    }
    ///Bit 29 - OTG2 enable
    #[inline(always)]
    pub fn otg2enc(&mut self) -> OTG2ENC_W<'_, AHB5ENCRrs> {
        OTG2ENC_W::new(self, 29)
    }
    ///Bit 30 - NPUCACHE enable
    #[inline(always)]
    pub fn npucacheenc(&mut self) -> NPUCACHEENC_W<'_, AHB5ENCRrs> {
        NPUCACHEENC_W::new(self, 30)
    }
    ///Bit 31 - NPU enable
    #[inline(always)]
    pub fn npuenc(&mut self) -> NPUENC_W<'_, AHB5ENCRrs> {
        NPUENC_W::new(self, 31)
    }
}
/**RCC AHB5 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5encr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:AHB5ENCR)*/
pub struct AHB5ENCRrs;
impl crate::RegisterSpec for AHB5ENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb5encr::W`](W) writer structure
impl crate::Writable for AHB5ENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB5ENCR to value 0
impl crate::Resettable for AHB5ENCRrs {}
