///Register `AHB5ENSR` writer
pub type W = crate::W<AHB5ENSRrs>;
///Field `HPDMA1ENS` writer - HPDMA1 enable
pub type HPDMA1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2DENS` writer - DMA2D enable
pub type DMA2DENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JPEGENS` writer - JPEG enable
pub type JPEGENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMCENS` writer - FMC enable
pub type FMCENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI1ENS` writer - XSPI1 enable
pub type XSPI1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSSIENS` writer - PSSI enable
pub type PSSIENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC2ENS` writer - SDMMC2 enable
pub type SDMMC2ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1ENS` writer - SDMMC1 enable
pub type SDMMC1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI2ENS` writer - XSPI2 enable
pub type XSPI2ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIMENS` writer - XSPIM enable
pub type XSPIMENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE1ENS` writer - MCE1 enable
pub type MCE1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE2ENS` writer - MCE2 enable
pub type MCE2ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE3ENS` writer - MCE3 enable
pub type MCE3ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI3ENS` writer - XSPI3 enable
pub type XSPI3ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE4ENS` writer - MCE4 enable
pub type MCE4ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXMMUENS` writer - GFXMMU enable
pub type GFXMMUENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPUENS` writer - GPU enable
pub type GPUENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1MACENS` writer - ETH1MAC enable
pub type ETH1MACENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1TXENS` writer - ETH1TX enable
pub type ETH1TXENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1RXENS` writer - ETH1RX enable
pub type ETH1RXENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1ENS` writer - ETH1 enable
pub type ETH1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG1ENS` writer - OTG1 enable
pub type OTG1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGPHY1ENS` writer - OTGPHY1 enable
pub type OTGPHY1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGPHY2ENS` writer - OTGPHY2 enable
pub type OTGPHY2ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG2ENS` writer - OTG2 enable
pub type OTG2ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUCACHEENS` writer - NPUCACHE enable
pub type NPUCACHEENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUENS` writer - NPU enable
pub type NPUENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB5ENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - HPDMA1 enable
    #[inline(always)]
    pub fn hpdma1ens(&mut self) -> HPDMA1ENS_W<'_, AHB5ENSRrs> {
        HPDMA1ENS_W::new(self, 0)
    }
    ///Bit 1 - DMA2D enable
    #[inline(always)]
    pub fn dma2dens(&mut self) -> DMA2DENS_W<'_, AHB5ENSRrs> {
        DMA2DENS_W::new(self, 1)
    }
    ///Bit 3 - JPEG enable
    #[inline(always)]
    pub fn jpegens(&mut self) -> JPEGENS_W<'_, AHB5ENSRrs> {
        JPEGENS_W::new(self, 3)
    }
    ///Bit 4 - FMC enable
    #[inline(always)]
    pub fn fmcens(&mut self) -> FMCENS_W<'_, AHB5ENSRrs> {
        FMCENS_W::new(self, 4)
    }
    ///Bit 5 - XSPI1 enable
    #[inline(always)]
    pub fn xspi1ens(&mut self) -> XSPI1ENS_W<'_, AHB5ENSRrs> {
        XSPI1ENS_W::new(self, 5)
    }
    ///Bit 6 - PSSI enable
    #[inline(always)]
    pub fn pssiens(&mut self) -> PSSIENS_W<'_, AHB5ENSRrs> {
        PSSIENS_W::new(self, 6)
    }
    ///Bit 7 - SDMMC2 enable
    #[inline(always)]
    pub fn sdmmc2ens(&mut self) -> SDMMC2ENS_W<'_, AHB5ENSRrs> {
        SDMMC2ENS_W::new(self, 7)
    }
    ///Bit 8 - SDMMC1 enable
    #[inline(always)]
    pub fn sdmmc1ens(&mut self) -> SDMMC1ENS_W<'_, AHB5ENSRrs> {
        SDMMC1ENS_W::new(self, 8)
    }
    ///Bit 12 - XSPI2 enable
    #[inline(always)]
    pub fn xspi2ens(&mut self) -> XSPI2ENS_W<'_, AHB5ENSRrs> {
        XSPI2ENS_W::new(self, 12)
    }
    ///Bit 13 - XSPIM enable
    #[inline(always)]
    pub fn xspimens(&mut self) -> XSPIMENS_W<'_, AHB5ENSRrs> {
        XSPIMENS_W::new(self, 13)
    }
    ///Bit 14 - MCE1 enable
    #[inline(always)]
    pub fn mce1ens(&mut self) -> MCE1ENS_W<'_, AHB5ENSRrs> {
        MCE1ENS_W::new(self, 14)
    }
    ///Bit 15 - MCE2 enable
    #[inline(always)]
    pub fn mce2ens(&mut self) -> MCE2ENS_W<'_, AHB5ENSRrs> {
        MCE2ENS_W::new(self, 15)
    }
    ///Bit 16 - MCE3 enable
    #[inline(always)]
    pub fn mce3ens(&mut self) -> MCE3ENS_W<'_, AHB5ENSRrs> {
        MCE3ENS_W::new(self, 16)
    }
    ///Bit 17 - XSPI3 enable
    #[inline(always)]
    pub fn xspi3ens(&mut self) -> XSPI3ENS_W<'_, AHB5ENSRrs> {
        XSPI3ENS_W::new(self, 17)
    }
    ///Bit 18 - MCE4 enable
    #[inline(always)]
    pub fn mce4ens(&mut self) -> MCE4ENS_W<'_, AHB5ENSRrs> {
        MCE4ENS_W::new(self, 18)
    }
    ///Bit 19 - GFXMMU enable
    #[inline(always)]
    pub fn gfxmmuens(&mut self) -> GFXMMUENS_W<'_, AHB5ENSRrs> {
        GFXMMUENS_W::new(self, 19)
    }
    ///Bit 20 - GPU enable
    #[inline(always)]
    pub fn gpuens(&mut self) -> GPUENS_W<'_, AHB5ENSRrs> {
        GPUENS_W::new(self, 20)
    }
    ///Bit 22 - ETH1MAC enable
    #[inline(always)]
    pub fn eth1macens(&mut self) -> ETH1MACENS_W<'_, AHB5ENSRrs> {
        ETH1MACENS_W::new(self, 22)
    }
    ///Bit 23 - ETH1TX enable
    #[inline(always)]
    pub fn eth1txens(&mut self) -> ETH1TXENS_W<'_, AHB5ENSRrs> {
        ETH1TXENS_W::new(self, 23)
    }
    ///Bit 24 - ETH1RX enable
    #[inline(always)]
    pub fn eth1rxens(&mut self) -> ETH1RXENS_W<'_, AHB5ENSRrs> {
        ETH1RXENS_W::new(self, 24)
    }
    ///Bit 25 - ETH1 enable
    #[inline(always)]
    pub fn eth1ens(&mut self) -> ETH1ENS_W<'_, AHB5ENSRrs> {
        ETH1ENS_W::new(self, 25)
    }
    ///Bit 26 - OTG1 enable
    #[inline(always)]
    pub fn otg1ens(&mut self) -> OTG1ENS_W<'_, AHB5ENSRrs> {
        OTG1ENS_W::new(self, 26)
    }
    ///Bit 27 - OTGPHY1 enable
    #[inline(always)]
    pub fn otgphy1ens(&mut self) -> OTGPHY1ENS_W<'_, AHB5ENSRrs> {
        OTGPHY1ENS_W::new(self, 27)
    }
    ///Bit 28 - OTGPHY2 enable
    #[inline(always)]
    pub fn otgphy2ens(&mut self) -> OTGPHY2ENS_W<'_, AHB5ENSRrs> {
        OTGPHY2ENS_W::new(self, 28)
    }
    ///Bit 29 - OTG2 enable
    #[inline(always)]
    pub fn otg2ens(&mut self) -> OTG2ENS_W<'_, AHB5ENSRrs> {
        OTG2ENS_W::new(self, 29)
    }
    ///Bit 30 - NPUCACHE enable
    #[inline(always)]
    pub fn npucacheens(&mut self) -> NPUCACHEENS_W<'_, AHB5ENSRrs> {
        NPUCACHEENS_W::new(self, 30)
    }
    ///Bit 31 - NPU enable
    #[inline(always)]
    pub fn npuens(&mut self) -> NPUENS_W<'_, AHB5ENSRrs> {
        NPUENS_W::new(self, 31)
    }
}
/**RCC AHB5 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5ensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB5ENSR)*/
pub struct AHB5ENSRrs;
impl crate::RegisterSpec for AHB5ENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb5ensr::W`](W) writer structure
impl crate::Writable for AHB5ENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB5ENSR to value 0
impl crate::Resettable for AHB5ENSRrs {}
