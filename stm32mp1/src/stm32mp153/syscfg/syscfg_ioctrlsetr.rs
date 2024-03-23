#[doc = "Register `SYSCFG_IOCTRLSETR` reader"]
pub type R = crate::R<SYSCFG_IOCTRLSETRrs>;
#[doc = "Register `SYSCFG_IOCTRLSETR` writer"]
pub type W = crate::W<SYSCFG_IOCTRLSETRrs>;
#[doc = "Field `HSLVEN_TRACE` reader - HSLVEN_TRACE"]
pub type HSLVEN_TRACE_R = crate::BitReader;
#[doc = "Field `HSLVEN_TRACE` writer - HSLVEN_TRACE"]
pub type HSLVEN_TRACE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSLVEN_QUADSPI` reader - HSLVEN_QUADSPI"]
pub type HSLVEN_QUADSPI_R = crate::BitReader;
#[doc = "Field `HSLVEN_QUADSPI` writer - HSLVEN_QUADSPI"]
pub type HSLVEN_QUADSPI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSLVEN_ETH` reader - HSLVEN_ETH"]
pub type HSLVEN_ETH_R = crate::BitReader;
#[doc = "Field `HSLVEN_ETH` writer - HSLVEN_ETH"]
pub type HSLVEN_ETH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSLVEN_SDMMC` reader - HSLVEN_SDMMC"]
pub type HSLVEN_SDMMC_R = crate::BitReader;
#[doc = "Field `HSLVEN_SDMMC` writer - HSLVEN_SDMMC"]
pub type HSLVEN_SDMMC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSLVEN_SPI` reader - HSLVEN_SPI"]
pub type HSLVEN_SPI_R = crate::BitReader;
#[doc = "Field `HSLVEN_SPI` writer - HSLVEN_SPI"]
pub type HSLVEN_SPI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HSLVEN_TRACE"]
    #[inline(always)]
    pub fn hslven_trace(&self) -> HSLVEN_TRACE_R {
        HSLVEN_TRACE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSLVEN_QUADSPI"]
    #[inline(always)]
    pub fn hslven_quadspi(&self) -> HSLVEN_QUADSPI_R {
        HSLVEN_QUADSPI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSLVEN_ETH"]
    #[inline(always)]
    pub fn hslven_eth(&self) -> HSLVEN_ETH_R {
        HSLVEN_ETH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSLVEN_SDMMC"]
    #[inline(always)]
    pub fn hslven_sdmmc(&self) -> HSLVEN_SDMMC_R {
        HSLVEN_SDMMC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSLVEN_SPI"]
    #[inline(always)]
    pub fn hslven_spi(&self) -> HSLVEN_SPI_R {
        HSLVEN_SPI_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSLVEN_TRACE"]
    #[inline(always)]
    #[must_use]
    pub fn hslven_trace(&mut self) -> HSLVEN_TRACE_W<SYSCFG_IOCTRLSETRrs> {
        HSLVEN_TRACE_W::new(self, 0)
    }
    #[doc = "Bit 1 - HSLVEN_QUADSPI"]
    #[inline(always)]
    #[must_use]
    pub fn hslven_quadspi(&mut self) -> HSLVEN_QUADSPI_W<SYSCFG_IOCTRLSETRrs> {
        HSLVEN_QUADSPI_W::new(self, 1)
    }
    #[doc = "Bit 2 - HSLVEN_ETH"]
    #[inline(always)]
    #[must_use]
    pub fn hslven_eth(&mut self) -> HSLVEN_ETH_W<SYSCFG_IOCTRLSETRrs> {
        HSLVEN_ETH_W::new(self, 2)
    }
    #[doc = "Bit 3 - HSLVEN_SDMMC"]
    #[inline(always)]
    #[must_use]
    pub fn hslven_sdmmc(&mut self) -> HSLVEN_SDMMC_W<SYSCFG_IOCTRLSETRrs> {
        HSLVEN_SDMMC_W::new(self, 3)
    }
    #[doc = "Bit 4 - HSLVEN_SPI"]
    #[inline(always)]
    #[must_use]
    pub fn hslven_spi(&mut self) -> HSLVEN_SPI_W<SYSCFG_IOCTRLSETRrs> {
        HSLVEN_SPI_W::new(self, 4)
    }
}
#[doc = "SYSCFG IO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_ioctrlsetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_ioctrlsetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_IOCTRLSETRrs;
impl crate::RegisterSpec for SYSCFG_IOCTRLSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_ioctrlsetr::R`](R) reader structure"]
impl crate::Readable for SYSCFG_IOCTRLSETRrs {}
#[doc = "`write(|w| ..)` method takes [`syscfg_ioctrlsetr::W`](W) writer structure"]
impl crate::Writable for SYSCFG_IOCTRLSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCFG_IOCTRLSETR to value 0"]
impl crate::Resettable for SYSCFG_IOCTRLSETRrs {
    const RESET_VALUE: u32 = 0;
}
