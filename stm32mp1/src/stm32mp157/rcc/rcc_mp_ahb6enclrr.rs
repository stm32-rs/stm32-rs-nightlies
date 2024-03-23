#[doc = "Register `RCC_MP_AHB6ENCLRR` reader"]
pub type R = crate::R<RCC_MP_AHB6ENCLRRrs>;
#[doc = "Register `RCC_MP_AHB6ENCLRR` writer"]
pub type W = crate::W<RCC_MP_AHB6ENCLRRrs>;
#[doc = "Field `MDMAEN` reader - MDMAEN"]
pub type MDMAEN_R = crate::BitReader;
#[doc = "Field `MDMAEN` writer - MDMAEN"]
pub type MDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPUEN` reader - GPUEN"]
pub type GPUEN_R = crate::BitReader;
#[doc = "Field `GPUEN` writer - GPUEN"]
pub type GPUEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHCKEN` reader - ETHCKEN"]
pub type ETHCKEN_R = crate::BitReader;
#[doc = "Field `ETHCKEN` writer - ETHCKEN"]
pub type ETHCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHTXEN` reader - ETHTXEN"]
pub type ETHTXEN_R = crate::BitReader;
#[doc = "Field `ETHTXEN` writer - ETHTXEN"]
pub type ETHTXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHRXEN` reader - ETHRXEN"]
pub type ETHRXEN_R = crate::BitReader;
#[doc = "Field `ETHRXEN` writer - ETHRXEN"]
pub type ETHRXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHMACEN` reader - ETHMACEN"]
pub type ETHMACEN_R = crate::BitReader;
#[doc = "Field `ETHMACEN` writer - ETHMACEN"]
pub type ETHMACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMCEN` reader - FMCEN"]
pub type FMCEN_R = crate::BitReader;
#[doc = "Field `FMCEN` writer - FMCEN"]
pub type FMCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPIEN` reader - QSPIEN"]
pub type QSPIEN_R = crate::BitReader;
#[doc = "Field `QSPIEN` writer - QSPIEN"]
pub type QSPIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC1EN` reader - SDMMC1EN"]
pub type SDMMC1EN_R = crate::BitReader;
#[doc = "Field `SDMMC1EN` writer - SDMMC1EN"]
pub type SDMMC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC2EN` reader - SDMMC2EN"]
pub type SDMMC2EN_R = crate::BitReader;
#[doc = "Field `SDMMC2EN` writer - SDMMC2EN"]
pub type SDMMC2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC1EN` reader - CRC1EN"]
pub type CRC1EN_R = crate::BitReader;
#[doc = "Field `CRC1EN` writer - CRC1EN"]
pub type CRC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBHEN` reader - USBHEN"]
pub type USBHEN_R = crate::BitReader;
#[doc = "Field `USBHEN` writer - USBHEN"]
pub type USBHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MDMAEN"]
    #[inline(always)]
    pub fn mdmaen(&self) -> MDMAEN_R {
        MDMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - GPUEN"]
    #[inline(always)]
    pub fn gpuen(&self) -> GPUEN_R {
        GPUEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - ETHCKEN"]
    #[inline(always)]
    pub fn ethcken(&self) -> ETHCKEN_R {
        ETHCKEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ETHTXEN"]
    #[inline(always)]
    pub fn ethtxen(&self) -> ETHTXEN_R {
        ETHTXEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ETHRXEN"]
    #[inline(always)]
    pub fn ethrxen(&self) -> ETHRXEN_R {
        ETHRXEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ETHMACEN"]
    #[inline(always)]
    pub fn ethmacen(&self) -> ETHMACEN_R {
        ETHMACEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - FMCEN"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - QSPIEN"]
    #[inline(always)]
    pub fn qspien(&self) -> QSPIEN_R {
        QSPIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1EN"]
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SDMMC2EN"]
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - CRC1EN"]
    #[inline(always)]
    pub fn crc1en(&self) -> CRC1EN_R {
        CRC1EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - USBHEN"]
    #[inline(always)]
    pub fn usbhen(&self) -> USBHEN_R {
        USBHEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn mdmaen(&mut self) -> MDMAEN_W<RCC_MP_AHB6ENCLRRrs> {
        MDMAEN_W::new(self, 0)
    }
    #[doc = "Bit 5 - GPUEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpuen(&mut self) -> GPUEN_W<RCC_MP_AHB6ENCLRRrs> {
        GPUEN_W::new(self, 5)
    }
    #[doc = "Bit 7 - ETHCKEN"]
    #[inline(always)]
    #[must_use]
    pub fn ethcken(&mut self) -> ETHCKEN_W<RCC_MP_AHB6ENCLRRrs> {
        ETHCKEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - ETHTXEN"]
    #[inline(always)]
    #[must_use]
    pub fn ethtxen(&mut self) -> ETHTXEN_W<RCC_MP_AHB6ENCLRRrs> {
        ETHTXEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - ETHRXEN"]
    #[inline(always)]
    #[must_use]
    pub fn ethrxen(&mut self) -> ETHRXEN_W<RCC_MP_AHB6ENCLRRrs> {
        ETHRXEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - ETHMACEN"]
    #[inline(always)]
    #[must_use]
    pub fn ethmacen(&mut self) -> ETHMACEN_W<RCC_MP_AHB6ENCLRRrs> {
        ETHMACEN_W::new(self, 10)
    }
    #[doc = "Bit 12 - FMCEN"]
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FMCEN_W<RCC_MP_AHB6ENCLRRrs> {
        FMCEN_W::new(self, 12)
    }
    #[doc = "Bit 14 - QSPIEN"]
    #[inline(always)]
    #[must_use]
    pub fn qspien(&mut self) -> QSPIEN_W<RCC_MP_AHB6ENCLRRrs> {
        QSPIEN_W::new(self, 14)
    }
    #[doc = "Bit 16 - SDMMC1EN"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<RCC_MP_AHB6ENCLRRrs> {
        SDMMC1EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - SDMMC2EN"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<RCC_MP_AHB6ENCLRRrs> {
        SDMMC2EN_W::new(self, 17)
    }
    #[doc = "Bit 20 - CRC1EN"]
    #[inline(always)]
    #[must_use]
    pub fn crc1en(&mut self) -> CRC1EN_W<RCC_MP_AHB6ENCLRRrs> {
        CRC1EN_W::new(self, 20)
    }
    #[doc = "Bit 24 - USBHEN"]
    #[inline(always)]
    #[must_use]
    pub fn usbhen(&mut self) -> USBHEN_W<RCC_MP_AHB6ENCLRRrs> {
        USBHEN_W::new(self, 24)
    }
}
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb6enclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb6enclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_AHB6ENCLRRrs;
impl crate::RegisterSpec for RCC_MP_AHB6ENCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_ahb6enclrr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_AHB6ENCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mp_ahb6enclrr::W`](W) writer structure"]
impl crate::Writable for RCC_MP_AHB6ENCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MP_AHB6ENCLRR to value 0"]
impl crate::Resettable for RCC_MP_AHB6ENCLRRrs {
    const RESET_VALUE: u32 = 0;
}
