///Register `AHB5ENR` reader
pub type R = crate::R<AHB5ENRrs>;
///Register `AHB5ENR` writer
pub type W = crate::W<AHB5ENRrs>;
///Field `HPDMA1EN` reader - HPDMA1 enable
pub type HPDMA1EN_R = crate::BitReader;
///Field `HPDMA1EN` writer - HPDMA1 enable
pub type HPDMA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2DEN` reader - DMA2D enable
pub type DMA2DEN_R = crate::BitReader;
///Field `DMA2DEN` writer - DMA2D enable
pub type DMA2DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JPEGEN` reader - JPEG enable
pub type JPEGEN_R = crate::BitReader;
///Field `JPEGEN` writer - JPEG enable
pub type JPEGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMCEN` reader - FMC enable
pub type FMCEN_R = crate::BitReader;
///Field `FMCEN` writer - FMC enable
pub type FMCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI1EN` reader - XSPI1 enable
pub type XSPI1EN_R = crate::BitReader;
///Field `XSPI1EN` writer - XSPI1 enable
pub type XSPI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSSIEN` reader - PSSI enable
pub type PSSIEN_R = crate::BitReader;
///Field `PSSIEN` writer - PSSI enable
pub type PSSIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC2EN` reader - SDMMC2 enable
pub type SDMMC2EN_R = crate::BitReader;
///Field `SDMMC2EN` writer - SDMMC2 enable
pub type SDMMC2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1EN` reader - SDMMC1 enable
pub type SDMMC1EN_R = crate::BitReader;
///Field `SDMMC1EN` writer - SDMMC1 enable
pub type SDMMC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI2EN` reader - XSPI2 enable
pub type XSPI2EN_R = crate::BitReader;
///Field `XSPI2EN` writer - XSPI2 enable
pub type XSPI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIMEN` reader - XSPIM enable
pub type XSPIMEN_R = crate::BitReader;
///Field `XSPIMEN` writer - XSPIM enable
pub type XSPIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE1EN` reader - MCE1 enable
pub type MCE1EN_R = crate::BitReader;
///Field `MCE1EN` writer - MCE1 enable
pub type MCE1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE2EN` reader - MCE2 enable
pub type MCE2EN_R = crate::BitReader;
///Field `MCE2EN` writer - MCE2 enable
pub type MCE2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE3EN` reader - MCE3 enable
pub type MCE3EN_R = crate::BitReader;
///Field `MCE3EN` writer - MCE3 enable
pub type MCE3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI3EN` reader - XSPI3 enable
pub type XSPI3EN_R = crate::BitReader;
///Field `XSPI3EN` writer - XSPI3 enable
pub type XSPI3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE4EN` reader - MCE4 enable
pub type MCE4EN_R = crate::BitReader;
///Field `MCE4EN` writer - MCE4 enable
pub type MCE4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXMMUEN` reader - GFXMMU enable
pub type GFXMMUEN_R = crate::BitReader;
///Field `GFXMMUEN` writer - GFXMMU enable
pub type GFXMMUEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPUEN` reader - GPU enable
pub type GPUEN_R = crate::BitReader;
///Field `GPUEN` writer - GPU enable
pub type GPUEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1MACEN` reader - ETH1MAC enable
pub type ETH1MACEN_R = crate::BitReader;
///Field `ETH1MACEN` writer - ETH1MAC enable
pub type ETH1MACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1TXEN` reader - ETH1TX enable
pub type ETH1TXEN_R = crate::BitReader;
///Field `ETH1TXEN` writer - ETH1TX enable
pub type ETH1TXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1RXEN` reader - ETH1RX enable
pub type ETH1RXEN_R = crate::BitReader;
///Field `ETH1RXEN` writer - ETH1RX enable
pub type ETH1RXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1EN` reader - ETH1 enable
pub type ETH1EN_R = crate::BitReader;
///Field `ETH1EN` writer - ETH1 enable
pub type ETH1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG1EN` reader - OTG1 enable
pub type OTG1EN_R = crate::BitReader;
///Field `OTG1EN` writer - OTG1 enable
pub type OTG1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGPHY1EN` reader - OTGPHY1 enable
pub type OTGPHY1EN_R = crate::BitReader;
///Field `OTGPHY1EN` writer - OTGPHY1 enable
pub type OTGPHY1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGPHY2EN` reader - OTGPHY2 enable
pub type OTGPHY2EN_R = crate::BitReader;
///Field `OTGPHY2EN` writer - OTGPHY2 enable
pub type OTGPHY2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG2EN` reader - OTG2 enable
pub type OTG2EN_R = crate::BitReader;
///Field `OTG2EN` writer - OTG2 enable
pub type OTG2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUCACHEEN` reader - NPUCACHE enable
pub type NPUCACHEEN_R = crate::BitReader;
///Field `NPUCACHEEN` writer - NPUCACHE enable
pub type NPUCACHEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUEN` reader - NPU enable
pub type NPUEN_R = crate::BitReader;
///Field `NPUEN` writer - NPU enable
pub type NPUEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - HPDMA1 enable
    #[inline(always)]
    pub fn hpdma1en(&self) -> HPDMA1EN_R {
        HPDMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2D enable
    #[inline(always)]
    pub fn dma2den(&self) -> DMA2DEN_R {
        DMA2DEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - JPEG enable
    #[inline(always)]
    pub fn jpegen(&self) -> JPEGEN_R {
        JPEGEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FMC enable
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - XSPI1 enable
    #[inline(always)]
    pub fn xspi1en(&self) -> XSPI1EN_R {
        XSPI1EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PSSI enable
    #[inline(always)]
    pub fn pssien(&self) -> PSSIEN_R {
        PSSIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SDMMC2 enable
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SDMMC1 enable
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - XSPI2 enable
    #[inline(always)]
    pub fn xspi2en(&self) -> XSPI2EN_R {
        XSPI2EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - XSPIM enable
    #[inline(always)]
    pub fn xspimen(&self) -> XSPIMEN_R {
        XSPIMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - MCE1 enable
    #[inline(always)]
    pub fn mce1en(&self) -> MCE1EN_R {
        MCE1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - MCE2 enable
    #[inline(always)]
    pub fn mce2en(&self) -> MCE2EN_R {
        MCE2EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - MCE3 enable
    #[inline(always)]
    pub fn mce3en(&self) -> MCE3EN_R {
        MCE3EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - XSPI3 enable
    #[inline(always)]
    pub fn xspi3en(&self) -> XSPI3EN_R {
        XSPI3EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - MCE4 enable
    #[inline(always)]
    pub fn mce4en(&self) -> MCE4EN_R {
        MCE4EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - GFXMMU enable
    #[inline(always)]
    pub fn gfxmmuen(&self) -> GFXMMUEN_R {
        GFXMMUEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - GPU enable
    #[inline(always)]
    pub fn gpuen(&self) -> GPUEN_R {
        GPUEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - ETH1MAC enable
    #[inline(always)]
    pub fn eth1macen(&self) -> ETH1MACEN_R {
        ETH1MACEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ETH1TX enable
    #[inline(always)]
    pub fn eth1txen(&self) -> ETH1TXEN_R {
        ETH1TXEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ETH1RX enable
    #[inline(always)]
    pub fn eth1rxen(&self) -> ETH1RXEN_R {
        ETH1RXEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - ETH1 enable
    #[inline(always)]
    pub fn eth1en(&self) -> ETH1EN_R {
        ETH1EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - OTG1 enable
    #[inline(always)]
    pub fn otg1en(&self) -> OTG1EN_R {
        OTG1EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - OTGPHY1 enable
    #[inline(always)]
    pub fn otgphy1en(&self) -> OTGPHY1EN_R {
        OTGPHY1EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - OTGPHY2 enable
    #[inline(always)]
    pub fn otgphy2en(&self) -> OTGPHY2EN_R {
        OTGPHY2EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - OTG2 enable
    #[inline(always)]
    pub fn otg2en(&self) -> OTG2EN_R {
        OTG2EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - NPUCACHE enable
    #[inline(always)]
    pub fn npucacheen(&self) -> NPUCACHEEN_R {
        NPUCACHEEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - NPU enable
    #[inline(always)]
    pub fn npuen(&self) -> NPUEN_R {
        NPUEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB5ENR")
            .field("hpdma1en", &self.hpdma1en())
            .field("dma2den", &self.dma2den())
            .field("jpegen", &self.jpegen())
            .field("fmcen", &self.fmcen())
            .field("xspi1en", &self.xspi1en())
            .field("pssien", &self.pssien())
            .field("sdmmc2en", &self.sdmmc2en())
            .field("sdmmc1en", &self.sdmmc1en())
            .field("xspi2en", &self.xspi2en())
            .field("xspimen", &self.xspimen())
            .field("mce1en", &self.mce1en())
            .field("mce2en", &self.mce2en())
            .field("mce3en", &self.mce3en())
            .field("xspi3en", &self.xspi3en())
            .field("mce4en", &self.mce4en())
            .field("gfxmmuen", &self.gfxmmuen())
            .field("gpuen", &self.gpuen())
            .field("eth1macen", &self.eth1macen())
            .field("eth1txen", &self.eth1txen())
            .field("eth1rxen", &self.eth1rxen())
            .field("eth1en", &self.eth1en())
            .field("otg1en", &self.otg1en())
            .field("otgphy1en", &self.otgphy1en())
            .field("otgphy2en", &self.otgphy2en())
            .field("otg2en", &self.otg2en())
            .field("npucacheen", &self.npucacheen())
            .field("npuen", &self.npuen())
            .finish()
    }
}
impl W {
    ///Bit 0 - HPDMA1 enable
    #[inline(always)]
    pub fn hpdma1en(&mut self) -> HPDMA1EN_W<'_, AHB5ENRrs> {
        HPDMA1EN_W::new(self, 0)
    }
    ///Bit 1 - DMA2D enable
    #[inline(always)]
    pub fn dma2den(&mut self) -> DMA2DEN_W<'_, AHB5ENRrs> {
        DMA2DEN_W::new(self, 1)
    }
    ///Bit 3 - JPEG enable
    #[inline(always)]
    pub fn jpegen(&mut self) -> JPEGEN_W<'_, AHB5ENRrs> {
        JPEGEN_W::new(self, 3)
    }
    ///Bit 4 - FMC enable
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W<'_, AHB5ENRrs> {
        FMCEN_W::new(self, 4)
    }
    ///Bit 5 - XSPI1 enable
    #[inline(always)]
    pub fn xspi1en(&mut self) -> XSPI1EN_W<'_, AHB5ENRrs> {
        XSPI1EN_W::new(self, 5)
    }
    ///Bit 6 - PSSI enable
    #[inline(always)]
    pub fn pssien(&mut self) -> PSSIEN_W<'_, AHB5ENRrs> {
        PSSIEN_W::new(self, 6)
    }
    ///Bit 7 - SDMMC2 enable
    #[inline(always)]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<'_, AHB5ENRrs> {
        SDMMC2EN_W::new(self, 7)
    }
    ///Bit 8 - SDMMC1 enable
    #[inline(always)]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<'_, AHB5ENRrs> {
        SDMMC1EN_W::new(self, 8)
    }
    ///Bit 12 - XSPI2 enable
    #[inline(always)]
    pub fn xspi2en(&mut self) -> XSPI2EN_W<'_, AHB5ENRrs> {
        XSPI2EN_W::new(self, 12)
    }
    ///Bit 13 - XSPIM enable
    #[inline(always)]
    pub fn xspimen(&mut self) -> XSPIMEN_W<'_, AHB5ENRrs> {
        XSPIMEN_W::new(self, 13)
    }
    ///Bit 14 - MCE1 enable
    #[inline(always)]
    pub fn mce1en(&mut self) -> MCE1EN_W<'_, AHB5ENRrs> {
        MCE1EN_W::new(self, 14)
    }
    ///Bit 15 - MCE2 enable
    #[inline(always)]
    pub fn mce2en(&mut self) -> MCE2EN_W<'_, AHB5ENRrs> {
        MCE2EN_W::new(self, 15)
    }
    ///Bit 16 - MCE3 enable
    #[inline(always)]
    pub fn mce3en(&mut self) -> MCE3EN_W<'_, AHB5ENRrs> {
        MCE3EN_W::new(self, 16)
    }
    ///Bit 17 - XSPI3 enable
    #[inline(always)]
    pub fn xspi3en(&mut self) -> XSPI3EN_W<'_, AHB5ENRrs> {
        XSPI3EN_W::new(self, 17)
    }
    ///Bit 18 - MCE4 enable
    #[inline(always)]
    pub fn mce4en(&mut self) -> MCE4EN_W<'_, AHB5ENRrs> {
        MCE4EN_W::new(self, 18)
    }
    ///Bit 19 - GFXMMU enable
    #[inline(always)]
    pub fn gfxmmuen(&mut self) -> GFXMMUEN_W<'_, AHB5ENRrs> {
        GFXMMUEN_W::new(self, 19)
    }
    ///Bit 20 - GPU enable
    #[inline(always)]
    pub fn gpuen(&mut self) -> GPUEN_W<'_, AHB5ENRrs> {
        GPUEN_W::new(self, 20)
    }
    ///Bit 22 - ETH1MAC enable
    #[inline(always)]
    pub fn eth1macen(&mut self) -> ETH1MACEN_W<'_, AHB5ENRrs> {
        ETH1MACEN_W::new(self, 22)
    }
    ///Bit 23 - ETH1TX enable
    #[inline(always)]
    pub fn eth1txen(&mut self) -> ETH1TXEN_W<'_, AHB5ENRrs> {
        ETH1TXEN_W::new(self, 23)
    }
    ///Bit 24 - ETH1RX enable
    #[inline(always)]
    pub fn eth1rxen(&mut self) -> ETH1RXEN_W<'_, AHB5ENRrs> {
        ETH1RXEN_W::new(self, 24)
    }
    ///Bit 25 - ETH1 enable
    #[inline(always)]
    pub fn eth1en(&mut self) -> ETH1EN_W<'_, AHB5ENRrs> {
        ETH1EN_W::new(self, 25)
    }
    ///Bit 26 - OTG1 enable
    #[inline(always)]
    pub fn otg1en(&mut self) -> OTG1EN_W<'_, AHB5ENRrs> {
        OTG1EN_W::new(self, 26)
    }
    ///Bit 27 - OTGPHY1 enable
    #[inline(always)]
    pub fn otgphy1en(&mut self) -> OTGPHY1EN_W<'_, AHB5ENRrs> {
        OTGPHY1EN_W::new(self, 27)
    }
    ///Bit 28 - OTGPHY2 enable
    #[inline(always)]
    pub fn otgphy2en(&mut self) -> OTGPHY2EN_W<'_, AHB5ENRrs> {
        OTGPHY2EN_W::new(self, 28)
    }
    ///Bit 29 - OTG2 enable
    #[inline(always)]
    pub fn otg2en(&mut self) -> OTG2EN_W<'_, AHB5ENRrs> {
        OTG2EN_W::new(self, 29)
    }
    ///Bit 30 - NPUCACHE enable
    #[inline(always)]
    pub fn npucacheen(&mut self) -> NPUCACHEEN_W<'_, AHB5ENRrs> {
        NPUCACHEEN_W::new(self, 30)
    }
    ///Bit 31 - NPU enable
    #[inline(always)]
    pub fn npuen(&mut self) -> NPUEN_W<'_, AHB5ENRrs> {
        NPUEN_W::new(self, 31)
    }
}
/**RCC AHB5 enable register

You can [`read`](crate::Reg::read) this register and get [`ahb5enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB5ENR)*/
pub struct AHB5ENRrs;
impl crate::RegisterSpec for AHB5ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb5enr::R`](R) reader structure
impl crate::Readable for AHB5ENRrs {}
///`write(|w| ..)` method takes [`ahb5enr::W`](W) writer structure
impl crate::Writable for AHB5ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB5ENR to value 0
impl crate::Resettable for AHB5ENRrs {}
