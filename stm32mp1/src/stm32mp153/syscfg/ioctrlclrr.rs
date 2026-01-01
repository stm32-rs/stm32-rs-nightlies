///Register `IOCTRLCLRR` reader
pub type R = crate::R<IOCTRLCLRRrs>;
///Register `IOCTRLCLRR` writer
pub type W = crate::W<IOCTRLCLRRrs>;
///Field `HSLVEN_TRACE` reader - HSLVEN_TRACE
pub type HSLVEN_TRACE_R = crate::BitReader;
///Field `HSLVEN_TRACE` writer - HSLVEN_TRACE
pub type HSLVEN_TRACE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLVEN_QUADSPI` reader - HSLVEN_QUADSPI
pub type HSLVEN_QUADSPI_R = crate::BitReader;
///Field `HSLVEN_QUADSPI` writer - HSLVEN_QUADSPI
pub type HSLVEN_QUADSPI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLVEN_ETH` reader - HSLVEN_ETH
pub type HSLVEN_ETH_R = crate::BitReader;
///Field `HSLVEN_ETH` writer - HSLVEN_ETH
pub type HSLVEN_ETH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLVEN_SDMMC` reader - HSLVEN_SDMMC
pub type HSLVEN_SDMMC_R = crate::BitReader;
///Field `HSLVEN_SDMMC` writer - HSLVEN_SDMMC
pub type HSLVEN_SDMMC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLVEN_SPI` reader - HSLVEN_SPI
pub type HSLVEN_SPI_R = crate::BitReader;
///Field `HSLVEN_SPI` writer - HSLVEN_SPI
pub type HSLVEN_SPI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - HSLVEN_TRACE
    #[inline(always)]
    pub fn hslven_trace(&self) -> HSLVEN_TRACE_R {
        HSLVEN_TRACE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HSLVEN_QUADSPI
    #[inline(always)]
    pub fn hslven_quadspi(&self) -> HSLVEN_QUADSPI_R {
        HSLVEN_QUADSPI_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSLVEN_ETH
    #[inline(always)]
    pub fn hslven_eth(&self) -> HSLVEN_ETH_R {
        HSLVEN_ETH_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSLVEN_SDMMC
    #[inline(always)]
    pub fn hslven_sdmmc(&self) -> HSLVEN_SDMMC_R {
        HSLVEN_SDMMC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSLVEN_SPI
    #[inline(always)]
    pub fn hslven_spi(&self) -> HSLVEN_SPI_R {
        HSLVEN_SPI_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCTRLCLRR")
            .field("hslven_trace", &self.hslven_trace())
            .field("hslven_quadspi", &self.hslven_quadspi())
            .field("hslven_eth", &self.hslven_eth())
            .field("hslven_sdmmc", &self.hslven_sdmmc())
            .field("hslven_spi", &self.hslven_spi())
            .finish()
    }
}
impl W {
    ///Bit 0 - HSLVEN_TRACE
    #[inline(always)]
    pub fn hslven_trace(&mut self) -> HSLVEN_TRACE_W<'_, IOCTRLCLRRrs> {
        HSLVEN_TRACE_W::new(self, 0)
    }
    ///Bit 1 - HSLVEN_QUADSPI
    #[inline(always)]
    pub fn hslven_quadspi(&mut self) -> HSLVEN_QUADSPI_W<'_, IOCTRLCLRRrs> {
        HSLVEN_QUADSPI_W::new(self, 1)
    }
    ///Bit 2 - HSLVEN_ETH
    #[inline(always)]
    pub fn hslven_eth(&mut self) -> HSLVEN_ETH_W<'_, IOCTRLCLRRrs> {
        HSLVEN_ETH_W::new(self, 2)
    }
    ///Bit 3 - HSLVEN_SDMMC
    #[inline(always)]
    pub fn hslven_sdmmc(&mut self) -> HSLVEN_SDMMC_W<'_, IOCTRLCLRRrs> {
        HSLVEN_SDMMC_W::new(self, 3)
    }
    ///Bit 4 - HSLVEN_SPI
    #[inline(always)]
    pub fn hslven_spi(&mut self) -> HSLVEN_SPI_W<'_, IOCTRLCLRRrs> {
        HSLVEN_SPI_W::new(self, 4)
    }
}
/**SYSCFG IO control register

You can [`read`](crate::Reg::read) this register and get [`ioctrlclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioctrlclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SYSCFG:IOCTRLCLRR)*/
pub struct IOCTRLCLRRrs;
impl crate::RegisterSpec for IOCTRLCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`ioctrlclrr::R`](R) reader structure
impl crate::Readable for IOCTRLCLRRrs {}
///`write(|w| ..)` method takes [`ioctrlclrr::W`](W) writer structure
impl crate::Writable for IOCTRLCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOCTRLCLRR to value 0
impl crate::Resettable for IOCTRLCLRRrs {}
