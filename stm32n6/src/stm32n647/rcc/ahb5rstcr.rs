///Register `AHB5RSTCR` writer
pub type W = crate::W<AHB5RSTCRrs>;
///Field `HPDMA1RSTC` writer - HPDMA1 reset
pub type HPDMA1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2DRSTC` writer - DMA2D reset
pub type DMA2DRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JPEGRSTC` writer - JPEG reset
pub type JPEGRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMCRSTC` writer - FMC reset
pub type FMCRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI1RSTC` writer - XSPI1 reset
pub type XSPI1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSSIRSTC` writer - PSSI reset
pub type PSSIRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC2RSTC` writer - SDMMC2 reset
pub type SDMMC2RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1RSTC` writer - SDMMC1 reset
pub type SDMMC1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI2RSTC` writer - XSPI2 reset
pub type XSPI2RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIMRSTC` writer - XSPIM reset
pub type XSPIMRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI3RSTC` writer - XSPI3 reset
pub type XSPI3RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE4RSTC` writer - MCE4 reset
pub type MCE4RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXMMURSTC` writer - GFXMMU reset
pub type GFXMMURSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPURSTC` writer - GPU reset
pub type GPURSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCFGOTGHSPHY1RSTC` writer - SYSCFGOTGHSPHY1 reset
pub type SYSCFGOTGHSPHY1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCFGOTGHSPHY2RSTC` writer - SYSCFGOTGHSPHY2 reset
pub type SYSCFGOTGHSPHY2RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1RSTC` writer - ETH1 reset
pub type ETH1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG1RSTC` writer - OTG1 reset
pub type OTG1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGPHY1RSTC` writer - OTGPHY1 reset
pub type OTGPHY1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGPHY2RSTC` writer - OTGPHY2 reset
pub type OTGPHY2RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG2RSTC` writer - OTG2 reset
pub type OTG2RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUCACHERSTC` writer - NPUCACHE reset
pub type NPUCACHERSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPURSTC` writer - NPU reset
pub type NPURSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB5RSTCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - HPDMA1 reset
    #[inline(always)]
    pub fn hpdma1rstc(&mut self) -> HPDMA1RSTC_W<'_, AHB5RSTCRrs> {
        HPDMA1RSTC_W::new(self, 0)
    }
    ///Bit 1 - DMA2D reset
    #[inline(always)]
    pub fn dma2drstc(&mut self) -> DMA2DRSTC_W<'_, AHB5RSTCRrs> {
        DMA2DRSTC_W::new(self, 1)
    }
    ///Bit 3 - JPEG reset
    #[inline(always)]
    pub fn jpegrstc(&mut self) -> JPEGRSTC_W<'_, AHB5RSTCRrs> {
        JPEGRSTC_W::new(self, 3)
    }
    ///Bit 4 - FMC reset
    #[inline(always)]
    pub fn fmcrstc(&mut self) -> FMCRSTC_W<'_, AHB5RSTCRrs> {
        FMCRSTC_W::new(self, 4)
    }
    ///Bit 5 - XSPI1 reset
    #[inline(always)]
    pub fn xspi1rstc(&mut self) -> XSPI1RSTC_W<'_, AHB5RSTCRrs> {
        XSPI1RSTC_W::new(self, 5)
    }
    ///Bit 6 - PSSI reset
    #[inline(always)]
    pub fn pssirstc(&mut self) -> PSSIRSTC_W<'_, AHB5RSTCRrs> {
        PSSIRSTC_W::new(self, 6)
    }
    ///Bit 7 - SDMMC2 reset
    #[inline(always)]
    pub fn sdmmc2rstc(&mut self) -> SDMMC2RSTC_W<'_, AHB5RSTCRrs> {
        SDMMC2RSTC_W::new(self, 7)
    }
    ///Bit 8 - SDMMC1 reset
    #[inline(always)]
    pub fn sdmmc1rstc(&mut self) -> SDMMC1RSTC_W<'_, AHB5RSTCRrs> {
        SDMMC1RSTC_W::new(self, 8)
    }
    ///Bit 12 - XSPI2 reset
    #[inline(always)]
    pub fn xspi2rstc(&mut self) -> XSPI2RSTC_W<'_, AHB5RSTCRrs> {
        XSPI2RSTC_W::new(self, 12)
    }
    ///Bit 13 - XSPIM reset
    #[inline(always)]
    pub fn xspimrstc(&mut self) -> XSPIMRSTC_W<'_, AHB5RSTCRrs> {
        XSPIMRSTC_W::new(self, 13)
    }
    ///Bit 17 - XSPI3 reset
    #[inline(always)]
    pub fn xspi3rstc(&mut self) -> XSPI3RSTC_W<'_, AHB5RSTCRrs> {
        XSPI3RSTC_W::new(self, 17)
    }
    ///Bit 18 - MCE4 reset
    #[inline(always)]
    pub fn mce4rstc(&mut self) -> MCE4RSTC_W<'_, AHB5RSTCRrs> {
        MCE4RSTC_W::new(self, 18)
    }
    ///Bit 19 - GFXMMU reset
    #[inline(always)]
    pub fn gfxmmurstc(&mut self) -> GFXMMURSTC_W<'_, AHB5RSTCRrs> {
        GFXMMURSTC_W::new(self, 19)
    }
    ///Bit 20 - GPU reset
    #[inline(always)]
    pub fn gpurstc(&mut self) -> GPURSTC_W<'_, AHB5RSTCRrs> {
        GPURSTC_W::new(self, 20)
    }
    ///Bit 23 - SYSCFGOTGHSPHY1 reset
    #[inline(always)]
    pub fn syscfgotghsphy1rstc(&mut self) -> SYSCFGOTGHSPHY1RSTC_W<'_, AHB5RSTCRrs> {
        SYSCFGOTGHSPHY1RSTC_W::new(self, 23)
    }
    ///Bit 24 - SYSCFGOTGHSPHY2 reset
    #[inline(always)]
    pub fn syscfgotghsphy2rstc(&mut self) -> SYSCFGOTGHSPHY2RSTC_W<'_, AHB5RSTCRrs> {
        SYSCFGOTGHSPHY2RSTC_W::new(self, 24)
    }
    ///Bit 25 - ETH1 reset
    #[inline(always)]
    pub fn eth1rstc(&mut self) -> ETH1RSTC_W<'_, AHB5RSTCRrs> {
        ETH1RSTC_W::new(self, 25)
    }
    ///Bit 26 - OTG1 reset
    #[inline(always)]
    pub fn otg1rstc(&mut self) -> OTG1RSTC_W<'_, AHB5RSTCRrs> {
        OTG1RSTC_W::new(self, 26)
    }
    ///Bit 27 - OTGPHY1 reset
    #[inline(always)]
    pub fn otgphy1rstc(&mut self) -> OTGPHY1RSTC_W<'_, AHB5RSTCRrs> {
        OTGPHY1RSTC_W::new(self, 27)
    }
    ///Bit 28 - OTGPHY2 reset
    #[inline(always)]
    pub fn otgphy2rstc(&mut self) -> OTGPHY2RSTC_W<'_, AHB5RSTCRrs> {
        OTGPHY2RSTC_W::new(self, 28)
    }
    ///Bit 29 - OTG2 reset
    #[inline(always)]
    pub fn otg2rstc(&mut self) -> OTG2RSTC_W<'_, AHB5RSTCRrs> {
        OTG2RSTC_W::new(self, 29)
    }
    ///Bit 30 - NPUCACHE reset
    #[inline(always)]
    pub fn npucacherstc(&mut self) -> NPUCACHERSTC_W<'_, AHB5RSTCRrs> {
        NPUCACHERSTC_W::new(self, 30)
    }
    ///Bit 31 - NPU reset
    #[inline(always)]
    pub fn npurstc(&mut self) -> NPURSTC_W<'_, AHB5RSTCRrs> {
        NPURSTC_W::new(self, 31)
    }
}
/**RCC AHB5 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5rstcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB5RSTCR)*/
pub struct AHB5RSTCRrs;
impl crate::RegisterSpec for AHB5RSTCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb5rstcr::W`](W) writer structure
impl crate::Writable for AHB5RSTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB5RSTCR to value 0
impl crate::Resettable for AHB5RSTCRrs {}
