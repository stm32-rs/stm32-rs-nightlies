///Register `AHB5RSTSR` writer
pub type W = crate::W<AHB5RSTSRrs>;
///Field `HPDMA1RSTS` writer - HPDMA1 reset
pub type HPDMA1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2DRSTS` writer - DMA2D reset
pub type DMA2DRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JPEGRSTS` writer - JPEG reset
pub type JPEGRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMCRSTS` writer - FMC reset
pub type FMCRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI1RSTS` writer - XSPI1 reset
pub type XSPI1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSSIRSTS` writer - PSSI reset
pub type PSSIRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC2RSTS` writer - SDMMC2 reset
pub type SDMMC2RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1RSTS` writer - SDMMC1 reset
pub type SDMMC1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI2RSTS` writer - XSPI2 reset
pub type XSPI2RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIMRSTS` writer - XSPIM reset
pub type XSPIMRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI3RSTS` writer - XSPI3 reset
pub type XSPI3RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE4RSTS` writer - MCE4 reset
pub type MCE4RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXMMURSTS` writer - GFXMMU reset
pub type GFXMMURSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPURSTS` writer - GPU reset
pub type GPURSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCFGOTGHSPHY1RSTS` writer - SYSCFGOTGHSPHY1 reset
pub type SYSCFGOTGHSPHY1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCFGOTGHSPHY2RSTS` writer - SYSCFGOTGHSPHY2 reset
pub type SYSCFGOTGHSPHY2RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1RSTS` writer - ETH1 reset
pub type ETH1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG1RSTS` writer - OTG1 reset
pub type OTG1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGPHY1RSTS` writer - OTGPHY1 reset
pub type OTGPHY1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGPHY2RSTS` writer - OTGPHY2 reset
pub type OTGPHY2RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG2RSTS` writer - OTG2 reset
pub type OTG2RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUCACHERSTS` writer - NPUCACHE reset
pub type NPUCACHERSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPURSTS` writer - NPU reset
pub type NPURSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB5RSTSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - HPDMA1 reset
    #[inline(always)]
    pub fn hpdma1rsts(&mut self) -> HPDMA1RSTS_W<'_, AHB5RSTSRrs> {
        HPDMA1RSTS_W::new(self, 0)
    }
    ///Bit 1 - DMA2D reset
    #[inline(always)]
    pub fn dma2drsts(&mut self) -> DMA2DRSTS_W<'_, AHB5RSTSRrs> {
        DMA2DRSTS_W::new(self, 1)
    }
    ///Bit 3 - JPEG reset
    #[inline(always)]
    pub fn jpegrsts(&mut self) -> JPEGRSTS_W<'_, AHB5RSTSRrs> {
        JPEGRSTS_W::new(self, 3)
    }
    ///Bit 4 - FMC reset
    #[inline(always)]
    pub fn fmcrsts(&mut self) -> FMCRSTS_W<'_, AHB5RSTSRrs> {
        FMCRSTS_W::new(self, 4)
    }
    ///Bit 5 - XSPI1 reset
    #[inline(always)]
    pub fn xspi1rsts(&mut self) -> XSPI1RSTS_W<'_, AHB5RSTSRrs> {
        XSPI1RSTS_W::new(self, 5)
    }
    ///Bit 6 - PSSI reset
    #[inline(always)]
    pub fn pssirsts(&mut self) -> PSSIRSTS_W<'_, AHB5RSTSRrs> {
        PSSIRSTS_W::new(self, 6)
    }
    ///Bit 7 - SDMMC2 reset
    #[inline(always)]
    pub fn sdmmc2rsts(&mut self) -> SDMMC2RSTS_W<'_, AHB5RSTSRrs> {
        SDMMC2RSTS_W::new(self, 7)
    }
    ///Bit 8 - SDMMC1 reset
    #[inline(always)]
    pub fn sdmmc1rsts(&mut self) -> SDMMC1RSTS_W<'_, AHB5RSTSRrs> {
        SDMMC1RSTS_W::new(self, 8)
    }
    ///Bit 12 - XSPI2 reset
    #[inline(always)]
    pub fn xspi2rsts(&mut self) -> XSPI2RSTS_W<'_, AHB5RSTSRrs> {
        XSPI2RSTS_W::new(self, 12)
    }
    ///Bit 13 - XSPIM reset
    #[inline(always)]
    pub fn xspimrsts(&mut self) -> XSPIMRSTS_W<'_, AHB5RSTSRrs> {
        XSPIMRSTS_W::new(self, 13)
    }
    ///Bit 17 - XSPI3 reset
    #[inline(always)]
    pub fn xspi3rsts(&mut self) -> XSPI3RSTS_W<'_, AHB5RSTSRrs> {
        XSPI3RSTS_W::new(self, 17)
    }
    ///Bit 18 - MCE4 reset
    #[inline(always)]
    pub fn mce4rsts(&mut self) -> MCE4RSTS_W<'_, AHB5RSTSRrs> {
        MCE4RSTS_W::new(self, 18)
    }
    ///Bit 19 - GFXMMU reset
    #[inline(always)]
    pub fn gfxmmursts(&mut self) -> GFXMMURSTS_W<'_, AHB5RSTSRrs> {
        GFXMMURSTS_W::new(self, 19)
    }
    ///Bit 20 - GPU reset
    #[inline(always)]
    pub fn gpursts(&mut self) -> GPURSTS_W<'_, AHB5RSTSRrs> {
        GPURSTS_W::new(self, 20)
    }
    ///Bit 23 - SYSCFGOTGHSPHY1 reset
    #[inline(always)]
    pub fn syscfgotghsphy1rsts(&mut self) -> SYSCFGOTGHSPHY1RSTS_W<'_, AHB5RSTSRrs> {
        SYSCFGOTGHSPHY1RSTS_W::new(self, 23)
    }
    ///Bit 24 - SYSCFGOTGHSPHY2 reset
    #[inline(always)]
    pub fn syscfgotghsphy2rsts(&mut self) -> SYSCFGOTGHSPHY2RSTS_W<'_, AHB5RSTSRrs> {
        SYSCFGOTGHSPHY2RSTS_W::new(self, 24)
    }
    ///Bit 25 - ETH1 reset
    #[inline(always)]
    pub fn eth1rsts(&mut self) -> ETH1RSTS_W<'_, AHB5RSTSRrs> {
        ETH1RSTS_W::new(self, 25)
    }
    ///Bit 26 - OTG1 reset
    #[inline(always)]
    pub fn otg1rsts(&mut self) -> OTG1RSTS_W<'_, AHB5RSTSRrs> {
        OTG1RSTS_W::new(self, 26)
    }
    ///Bit 27 - OTGPHY1 reset
    #[inline(always)]
    pub fn otgphy1rsts(&mut self) -> OTGPHY1RSTS_W<'_, AHB5RSTSRrs> {
        OTGPHY1RSTS_W::new(self, 27)
    }
    ///Bit 28 - OTGPHY2 reset
    #[inline(always)]
    pub fn otgphy2rsts(&mut self) -> OTGPHY2RSTS_W<'_, AHB5RSTSRrs> {
        OTGPHY2RSTS_W::new(self, 28)
    }
    ///Bit 29 - OTG2 reset
    #[inline(always)]
    pub fn otg2rsts(&mut self) -> OTG2RSTS_W<'_, AHB5RSTSRrs> {
        OTG2RSTS_W::new(self, 29)
    }
    ///Bit 30 - NPUCACHE reset
    #[inline(always)]
    pub fn npucachersts(&mut self) -> NPUCACHERSTS_W<'_, AHB5RSTSRrs> {
        NPUCACHERSTS_W::new(self, 30)
    }
    ///Bit 31 - NPU reset
    #[inline(always)]
    pub fn npursts(&mut self) -> NPURSTS_W<'_, AHB5RSTSRrs> {
        NPURSTS_W::new(self, 31)
    }
}
/**RCC AHB5 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5rstsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:AHB5RSTSR)*/
pub struct AHB5RSTSRrs;
impl crate::RegisterSpec for AHB5RSTSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb5rstsr::W`](W) writer structure
impl crate::Writable for AHB5RSTSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB5RSTSR to value 0
impl crate::Resettable for AHB5RSTSRrs {}
