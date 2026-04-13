///Register `AHB5LPENSR` writer
pub type W = crate::W<AHB5LPENSRrs>;
///Field `HPDMA1LPENS` writer - HPDMA1 sleep enable
pub type HPDMA1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2DLPENS` writer - DMA2D sleep enable
pub type DMA2DLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JPEGLPENS` writer - JPEG sleep enable
pub type JPEGLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMCLPENS` writer - FMC sleep enable
pub type FMCLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI1LPENS` writer - XSPI1 sleep enable
pub type XSPI1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSSILPENS` writer - PSSI sleep enable
pub type PSSILPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC2LPENS` writer - SDMMC2 sleep enable
pub type SDMMC2LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1LPENS` writer - SDMMC1 sleep enable
pub type SDMMC1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI2LPENS` writer - XSPI2 sleep enable
pub type XSPI2LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIMLPENS` writer - XSPIM sleep enable
pub type XSPIMLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE1LPENS` writer - MCE1 sleep enable
pub type MCE1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE2LPENS` writer - MCE2 sleep enable
pub type MCE2LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE3LPENS` writer - MCE3 sleep enable
pub type MCE3LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI3LPENS` writer - XSPI3 sleep enable
pub type XSPI3LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE4LPENS` writer - MCE4 sleep enable
pub type MCE4LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXMMULPENS` writer - GFXMMU sleep enable
pub type GFXMMULPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPULPENS` writer - GPU sleep enable
pub type GPULPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1MACLPENS` writer - ETH1MAC sleep enable
pub type ETH1MACLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1TXLPENS` writer - ETH1TX sleep enable
pub type ETH1TXLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1RXLPENS` writer - ETH1RX sleep enable
pub type ETH1RXLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1LPENS` writer - ETH1 sleep enable
pub type ETH1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG1LPENS` writer - OTG1 sleep enable
pub type OTG1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGPHY1LPENS` writer - OTGPHY1 sleep enable
pub type OTGPHY1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGPHY2LPENS` writer - OTGPHY2 sleep enable
pub type OTGPHY2LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG2LPENS` writer - OTG2 sleep enable
pub type OTG2LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUCACHELPENS` writer - NPUCACHE sleep enable
pub type NPUCACHELPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPULPENS` writer - NPU sleep enable
pub type NPULPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB5LPENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - HPDMA1 sleep enable
    #[inline(always)]
    pub fn hpdma1lpens(&mut self) -> HPDMA1LPENS_W<'_, AHB5LPENSRrs> {
        HPDMA1LPENS_W::new(self, 0)
    }
    ///Bit 1 - DMA2D sleep enable
    #[inline(always)]
    pub fn dma2dlpens(&mut self) -> DMA2DLPENS_W<'_, AHB5LPENSRrs> {
        DMA2DLPENS_W::new(self, 1)
    }
    ///Bit 3 - JPEG sleep enable
    #[inline(always)]
    pub fn jpeglpens(&mut self) -> JPEGLPENS_W<'_, AHB5LPENSRrs> {
        JPEGLPENS_W::new(self, 3)
    }
    ///Bit 4 - FMC sleep enable
    #[inline(always)]
    pub fn fmclpens(&mut self) -> FMCLPENS_W<'_, AHB5LPENSRrs> {
        FMCLPENS_W::new(self, 4)
    }
    ///Bit 5 - XSPI1 sleep enable
    #[inline(always)]
    pub fn xspi1lpens(&mut self) -> XSPI1LPENS_W<'_, AHB5LPENSRrs> {
        XSPI1LPENS_W::new(self, 5)
    }
    ///Bit 6 - PSSI sleep enable
    #[inline(always)]
    pub fn pssilpens(&mut self) -> PSSILPENS_W<'_, AHB5LPENSRrs> {
        PSSILPENS_W::new(self, 6)
    }
    ///Bit 7 - SDMMC2 sleep enable
    #[inline(always)]
    pub fn sdmmc2lpens(&mut self) -> SDMMC2LPENS_W<'_, AHB5LPENSRrs> {
        SDMMC2LPENS_W::new(self, 7)
    }
    ///Bit 8 - SDMMC1 sleep enable
    #[inline(always)]
    pub fn sdmmc1lpens(&mut self) -> SDMMC1LPENS_W<'_, AHB5LPENSRrs> {
        SDMMC1LPENS_W::new(self, 8)
    }
    ///Bit 12 - XSPI2 sleep enable
    #[inline(always)]
    pub fn xspi2lpens(&mut self) -> XSPI2LPENS_W<'_, AHB5LPENSRrs> {
        XSPI2LPENS_W::new(self, 12)
    }
    ///Bit 13 - XSPIM sleep enable
    #[inline(always)]
    pub fn xspimlpens(&mut self) -> XSPIMLPENS_W<'_, AHB5LPENSRrs> {
        XSPIMLPENS_W::new(self, 13)
    }
    ///Bit 14 - MCE1 sleep enable
    #[inline(always)]
    pub fn mce1lpens(&mut self) -> MCE1LPENS_W<'_, AHB5LPENSRrs> {
        MCE1LPENS_W::new(self, 14)
    }
    ///Bit 15 - MCE2 sleep enable
    #[inline(always)]
    pub fn mce2lpens(&mut self) -> MCE2LPENS_W<'_, AHB5LPENSRrs> {
        MCE2LPENS_W::new(self, 15)
    }
    ///Bit 16 - MCE3 sleep enable
    #[inline(always)]
    pub fn mce3lpens(&mut self) -> MCE3LPENS_W<'_, AHB5LPENSRrs> {
        MCE3LPENS_W::new(self, 16)
    }
    ///Bit 17 - XSPI3 sleep enable
    #[inline(always)]
    pub fn xspi3lpens(&mut self) -> XSPI3LPENS_W<'_, AHB5LPENSRrs> {
        XSPI3LPENS_W::new(self, 17)
    }
    ///Bit 18 - MCE4 sleep enable
    #[inline(always)]
    pub fn mce4lpens(&mut self) -> MCE4LPENS_W<'_, AHB5LPENSRrs> {
        MCE4LPENS_W::new(self, 18)
    }
    ///Bit 19 - GFXMMU sleep enable
    #[inline(always)]
    pub fn gfxmmulpens(&mut self) -> GFXMMULPENS_W<'_, AHB5LPENSRrs> {
        GFXMMULPENS_W::new(self, 19)
    }
    ///Bit 20 - GPU sleep enable
    #[inline(always)]
    pub fn gpulpens(&mut self) -> GPULPENS_W<'_, AHB5LPENSRrs> {
        GPULPENS_W::new(self, 20)
    }
    ///Bit 22 - ETH1MAC sleep enable
    #[inline(always)]
    pub fn eth1maclpens(&mut self) -> ETH1MACLPENS_W<'_, AHB5LPENSRrs> {
        ETH1MACLPENS_W::new(self, 22)
    }
    ///Bit 23 - ETH1TX sleep enable
    #[inline(always)]
    pub fn eth1txlpens(&mut self) -> ETH1TXLPENS_W<'_, AHB5LPENSRrs> {
        ETH1TXLPENS_W::new(self, 23)
    }
    ///Bit 24 - ETH1RX sleep enable
    #[inline(always)]
    pub fn eth1rxlpens(&mut self) -> ETH1RXLPENS_W<'_, AHB5LPENSRrs> {
        ETH1RXLPENS_W::new(self, 24)
    }
    ///Bit 25 - ETH1 sleep enable
    #[inline(always)]
    pub fn eth1lpens(&mut self) -> ETH1LPENS_W<'_, AHB5LPENSRrs> {
        ETH1LPENS_W::new(self, 25)
    }
    ///Bit 26 - OTG1 sleep enable
    #[inline(always)]
    pub fn otg1lpens(&mut self) -> OTG1LPENS_W<'_, AHB5LPENSRrs> {
        OTG1LPENS_W::new(self, 26)
    }
    ///Bit 27 - OTGPHY1 sleep enable
    #[inline(always)]
    pub fn otgphy1lpens(&mut self) -> OTGPHY1LPENS_W<'_, AHB5LPENSRrs> {
        OTGPHY1LPENS_W::new(self, 27)
    }
    ///Bit 28 - OTGPHY2 sleep enable
    #[inline(always)]
    pub fn otgphy2lpens(&mut self) -> OTGPHY2LPENS_W<'_, AHB5LPENSRrs> {
        OTGPHY2LPENS_W::new(self, 28)
    }
    ///Bit 29 - OTG2 sleep enable
    #[inline(always)]
    pub fn otg2lpens(&mut self) -> OTG2LPENS_W<'_, AHB5LPENSRrs> {
        OTG2LPENS_W::new(self, 29)
    }
    ///Bit 30 - NPUCACHE sleep enable
    #[inline(always)]
    pub fn npucachelpens(&mut self) -> NPUCACHELPENS_W<'_, AHB5LPENSRrs> {
        NPUCACHELPENS_W::new(self, 30)
    }
    ///Bit 31 - NPU sleep enable
    #[inline(always)]
    pub fn npulpens(&mut self) -> NPULPENS_W<'_, AHB5LPENSRrs> {
        NPULPENS_W::new(self, 31)
    }
}
/**RCC AHB5 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5lpensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB5LPENSR)*/
pub struct AHB5LPENSRrs;
impl crate::RegisterSpec for AHB5LPENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb5lpensr::W`](W) writer structure
impl crate::Writable for AHB5LPENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB5LPENSR to value 0
impl crate::Resettable for AHB5LPENSRrs {}
