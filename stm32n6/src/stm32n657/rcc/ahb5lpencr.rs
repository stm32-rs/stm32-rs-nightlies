///Register `AHB5LPENCR` writer
pub type W = crate::W<AHB5LPENCRrs>;
///Field `HPDMA1LPENC` writer - HPDMA1 sleep enable
pub type HPDMA1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2DLPENC` writer - DMA2D sleep enable
pub type DMA2DLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JPEGLPENC` writer - JPEG sleep enable
pub type JPEGLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMCLPENC` writer - FMC sleep enable
pub type FMCLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI1LPENC` writer - XSPI1 sleep enable
pub type XSPI1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSSILPENC` writer - PSSI sleep enable
pub type PSSILPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC2LPENC` writer - SDMMC2 sleep enable
pub type SDMMC2LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1LPENC` writer - SDMMC1 sleep enable
pub type SDMMC1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI2LPENC` writer - XSPI2 sleep enable
pub type XSPI2LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIMLPENC` writer - XSPIM sleep enable
pub type XSPIMLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE1LPENC` writer - MCE1 sleep enable
pub type MCE1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE2LPENC` writer - MCE2 sleep enable
pub type MCE2LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE3LPENC` writer - MCE3 sleep enable
pub type MCE3LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI3LPENC` writer - XSPI3 sleep enable
pub type XSPI3LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE4LPENC` writer - MCE4 sleep enable
pub type MCE4LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXMMULPENC` writer - GFXMMU sleep enable
pub type GFXMMULPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPULPENC` writer - GPU sleep enable
pub type GPULPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1MACLPENC` writer - ETH1MAC sleep enable
pub type ETH1MACLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1TXLPENC` writer - ETH1TX sleep enable
pub type ETH1TXLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1RXLPENC` writer - ETH1RX sleep enable
pub type ETH1RXLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1LPENC` writer - ETH1 sleep enable
pub type ETH1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG1LPENC` writer - OTG1 sleep enable
pub type OTG1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGPHY1LPENC` writer - OTGPHY1 sleep enable
pub type OTGPHY1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGPHY2LPENC` writer - OTGPHY2 sleep enable
pub type OTGPHY2LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG2LPENC` writer - OTG2 sleep enable
pub type OTG2LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUCACHELPENC` writer - NPUCACHE sleep enable
pub type NPUCACHELPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPULPENC` writer - NPU sleep enable
pub type NPULPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB5LPENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - HPDMA1 sleep enable
    #[inline(always)]
    pub fn hpdma1lpenc(&mut self) -> HPDMA1LPENC_W<'_, AHB5LPENCRrs> {
        HPDMA1LPENC_W::new(self, 0)
    }
    ///Bit 1 - DMA2D sleep enable
    #[inline(always)]
    pub fn dma2dlpenc(&mut self) -> DMA2DLPENC_W<'_, AHB5LPENCRrs> {
        DMA2DLPENC_W::new(self, 1)
    }
    ///Bit 3 - JPEG sleep enable
    #[inline(always)]
    pub fn jpeglpenc(&mut self) -> JPEGLPENC_W<'_, AHB5LPENCRrs> {
        JPEGLPENC_W::new(self, 3)
    }
    ///Bit 4 - FMC sleep enable
    #[inline(always)]
    pub fn fmclpenc(&mut self) -> FMCLPENC_W<'_, AHB5LPENCRrs> {
        FMCLPENC_W::new(self, 4)
    }
    ///Bit 5 - XSPI1 sleep enable
    #[inline(always)]
    pub fn xspi1lpenc(&mut self) -> XSPI1LPENC_W<'_, AHB5LPENCRrs> {
        XSPI1LPENC_W::new(self, 5)
    }
    ///Bit 6 - PSSI sleep enable
    #[inline(always)]
    pub fn pssilpenc(&mut self) -> PSSILPENC_W<'_, AHB5LPENCRrs> {
        PSSILPENC_W::new(self, 6)
    }
    ///Bit 7 - SDMMC2 sleep enable
    #[inline(always)]
    pub fn sdmmc2lpenc(&mut self) -> SDMMC2LPENC_W<'_, AHB5LPENCRrs> {
        SDMMC2LPENC_W::new(self, 7)
    }
    ///Bit 8 - SDMMC1 sleep enable
    #[inline(always)]
    pub fn sdmmc1lpenc(&mut self) -> SDMMC1LPENC_W<'_, AHB5LPENCRrs> {
        SDMMC1LPENC_W::new(self, 8)
    }
    ///Bit 12 - XSPI2 sleep enable
    #[inline(always)]
    pub fn xspi2lpenc(&mut self) -> XSPI2LPENC_W<'_, AHB5LPENCRrs> {
        XSPI2LPENC_W::new(self, 12)
    }
    ///Bit 13 - XSPIM sleep enable
    #[inline(always)]
    pub fn xspimlpenc(&mut self) -> XSPIMLPENC_W<'_, AHB5LPENCRrs> {
        XSPIMLPENC_W::new(self, 13)
    }
    ///Bit 14 - MCE1 sleep enable
    #[inline(always)]
    pub fn mce1lpenc(&mut self) -> MCE1LPENC_W<'_, AHB5LPENCRrs> {
        MCE1LPENC_W::new(self, 14)
    }
    ///Bit 15 - MCE2 sleep enable
    #[inline(always)]
    pub fn mce2lpenc(&mut self) -> MCE2LPENC_W<'_, AHB5LPENCRrs> {
        MCE2LPENC_W::new(self, 15)
    }
    ///Bit 16 - MCE3 sleep enable
    #[inline(always)]
    pub fn mce3lpenc(&mut self) -> MCE3LPENC_W<'_, AHB5LPENCRrs> {
        MCE3LPENC_W::new(self, 16)
    }
    ///Bit 17 - XSPI3 sleep enable
    #[inline(always)]
    pub fn xspi3lpenc(&mut self) -> XSPI3LPENC_W<'_, AHB5LPENCRrs> {
        XSPI3LPENC_W::new(self, 17)
    }
    ///Bit 18 - MCE4 sleep enable
    #[inline(always)]
    pub fn mce4lpenc(&mut self) -> MCE4LPENC_W<'_, AHB5LPENCRrs> {
        MCE4LPENC_W::new(self, 18)
    }
    ///Bit 19 - GFXMMU sleep enable
    #[inline(always)]
    pub fn gfxmmulpenc(&mut self) -> GFXMMULPENC_W<'_, AHB5LPENCRrs> {
        GFXMMULPENC_W::new(self, 19)
    }
    ///Bit 20 - GPU sleep enable
    #[inline(always)]
    pub fn gpulpenc(&mut self) -> GPULPENC_W<'_, AHB5LPENCRrs> {
        GPULPENC_W::new(self, 20)
    }
    ///Bit 22 - ETH1MAC sleep enable
    #[inline(always)]
    pub fn eth1maclpenc(&mut self) -> ETH1MACLPENC_W<'_, AHB5LPENCRrs> {
        ETH1MACLPENC_W::new(self, 22)
    }
    ///Bit 23 - ETH1TX sleep enable
    #[inline(always)]
    pub fn eth1txlpenc(&mut self) -> ETH1TXLPENC_W<'_, AHB5LPENCRrs> {
        ETH1TXLPENC_W::new(self, 23)
    }
    ///Bit 24 - ETH1RX sleep enable
    #[inline(always)]
    pub fn eth1rxlpenc(&mut self) -> ETH1RXLPENC_W<'_, AHB5LPENCRrs> {
        ETH1RXLPENC_W::new(self, 24)
    }
    ///Bit 25 - ETH1 sleep enable
    #[inline(always)]
    pub fn eth1lpenc(&mut self) -> ETH1LPENC_W<'_, AHB5LPENCRrs> {
        ETH1LPENC_W::new(self, 25)
    }
    ///Bit 26 - OTG1 sleep enable
    #[inline(always)]
    pub fn otg1lpenc(&mut self) -> OTG1LPENC_W<'_, AHB5LPENCRrs> {
        OTG1LPENC_W::new(self, 26)
    }
    ///Bit 27 - OTGPHY1 sleep enable
    #[inline(always)]
    pub fn otgphy1lpenc(&mut self) -> OTGPHY1LPENC_W<'_, AHB5LPENCRrs> {
        OTGPHY1LPENC_W::new(self, 27)
    }
    ///Bit 28 - OTGPHY2 sleep enable
    #[inline(always)]
    pub fn otgphy2lpenc(&mut self) -> OTGPHY2LPENC_W<'_, AHB5LPENCRrs> {
        OTGPHY2LPENC_W::new(self, 28)
    }
    ///Bit 29 - OTG2 sleep enable
    #[inline(always)]
    pub fn otg2lpenc(&mut self) -> OTG2LPENC_W<'_, AHB5LPENCRrs> {
        OTG2LPENC_W::new(self, 29)
    }
    ///Bit 30 - NPUCACHE sleep enable
    #[inline(always)]
    pub fn npucachelpenc(&mut self) -> NPUCACHELPENC_W<'_, AHB5LPENCRrs> {
        NPUCACHELPENC_W::new(self, 30)
    }
    ///Bit 31 - NPU sleep enable
    #[inline(always)]
    pub fn npulpenc(&mut self) -> NPULPENC_W<'_, AHB5LPENCRrs> {
        NPULPENC_W::new(self, 31)
    }
}
/**RCC AHB5 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5lpencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:AHB5LPENCR)*/
pub struct AHB5LPENCRrs;
impl crate::RegisterSpec for AHB5LPENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb5lpencr::W`](W) writer structure
impl crate::Writable for AHB5LPENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB5LPENCR to value 0
impl crate::Resettable for AHB5LPENCRrs {}
