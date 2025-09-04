///Register `MP_AHB6ENCLRR` reader
pub type R = crate::R<MP_AHB6ENCLRRrs>;
///Register `MP_AHB6ENCLRR` writer
pub type W = crate::W<MP_AHB6ENCLRRrs>;
///Field `MDMAEN` reader - MDMAEN
pub type MDMAEN_R = crate::BitReader;
///Field `MDMAEN` writer - MDMAEN
pub type MDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPUEN` reader - GPUEN
pub type GPUEN_R = crate::BitReader;
///Field `GPUEN` writer - GPUEN
pub type GPUEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHCKEN` reader - ETHCKEN
pub type ETHCKEN_R = crate::BitReader;
///Field `ETHCKEN` writer - ETHCKEN
pub type ETHCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHTXEN` reader - ETHTXEN
pub type ETHTXEN_R = crate::BitReader;
///Field `ETHTXEN` writer - ETHTXEN
pub type ETHTXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHRXEN` reader - ETHRXEN
pub type ETHRXEN_R = crate::BitReader;
///Field `ETHRXEN` writer - ETHRXEN
pub type ETHRXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHMACEN` reader - ETHMACEN
pub type ETHMACEN_R = crate::BitReader;
///Field `ETHMACEN` writer - ETHMACEN
pub type ETHMACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMCEN` reader - FMCEN
pub type FMCEN_R = crate::BitReader;
///Field `FMCEN` writer - FMCEN
pub type FMCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `QSPIEN` reader - QSPIEN
pub type QSPIEN_R = crate::BitReader;
///Field `QSPIEN` writer - QSPIEN
pub type QSPIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1EN` reader - SDMMC1EN
pub type SDMMC1EN_R = crate::BitReader;
///Field `SDMMC1EN` writer - SDMMC1EN
pub type SDMMC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC2EN` reader - SDMMC2EN
pub type SDMMC2EN_R = crate::BitReader;
///Field `SDMMC2EN` writer - SDMMC2EN
pub type SDMMC2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRC1EN` reader - CRC1EN
pub type CRC1EN_R = crate::BitReader;
///Field `CRC1EN` writer - CRC1EN
pub type CRC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBHEN` reader - USBHEN
pub type USBHEN_R = crate::BitReader;
///Field `USBHEN` writer - USBHEN
pub type USBHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MDMAEN
    #[inline(always)]
    pub fn mdmaen(&self) -> MDMAEN_R {
        MDMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 5 - GPUEN
    #[inline(always)]
    pub fn gpuen(&self) -> GPUEN_R {
        GPUEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - ETHCKEN
    #[inline(always)]
    pub fn ethcken(&self) -> ETHCKEN_R {
        ETHCKEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ETHTXEN
    #[inline(always)]
    pub fn ethtxen(&self) -> ETHTXEN_R {
        ETHTXEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ETHRXEN
    #[inline(always)]
    pub fn ethrxen(&self) -> ETHRXEN_R {
        ETHRXEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ETHMACEN
    #[inline(always)]
    pub fn ethmacen(&self) -> ETHMACEN_R {
        ETHMACEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - FMCEN
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - QSPIEN
    #[inline(always)]
    pub fn qspien(&self) -> QSPIEN_R {
        QSPIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - SDMMC1EN
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SDMMC2EN
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - CRC1EN
    #[inline(always)]
    pub fn crc1en(&self) -> CRC1EN_R {
        CRC1EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - USBHEN
    #[inline(always)]
    pub fn usbhen(&self) -> USBHEN_R {
        USBHEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_AHB6ENCLRR")
            .field("mdmaen", &self.mdmaen())
            .field("gpuen", &self.gpuen())
            .field("ethcken", &self.ethcken())
            .field("ethtxen", &self.ethtxen())
            .field("ethrxen", &self.ethrxen())
            .field("ethmacen", &self.ethmacen())
            .field("fmcen", &self.fmcen())
            .field("qspien", &self.qspien())
            .field("sdmmc1en", &self.sdmmc1en())
            .field("sdmmc2en", &self.sdmmc2en())
            .field("crc1en", &self.crc1en())
            .field("usbhen", &self.usbhen())
            .finish()
    }
}
impl W {
    ///Bit 0 - MDMAEN
    #[inline(always)]
    pub fn mdmaen(&mut self) -> MDMAEN_W<MP_AHB6ENCLRRrs> {
        MDMAEN_W::new(self, 0)
    }
    ///Bit 5 - GPUEN
    #[inline(always)]
    pub fn gpuen(&mut self) -> GPUEN_W<MP_AHB6ENCLRRrs> {
        GPUEN_W::new(self, 5)
    }
    ///Bit 7 - ETHCKEN
    #[inline(always)]
    pub fn ethcken(&mut self) -> ETHCKEN_W<MP_AHB6ENCLRRrs> {
        ETHCKEN_W::new(self, 7)
    }
    ///Bit 8 - ETHTXEN
    #[inline(always)]
    pub fn ethtxen(&mut self) -> ETHTXEN_W<MP_AHB6ENCLRRrs> {
        ETHTXEN_W::new(self, 8)
    }
    ///Bit 9 - ETHRXEN
    #[inline(always)]
    pub fn ethrxen(&mut self) -> ETHRXEN_W<MP_AHB6ENCLRRrs> {
        ETHRXEN_W::new(self, 9)
    }
    ///Bit 10 - ETHMACEN
    #[inline(always)]
    pub fn ethmacen(&mut self) -> ETHMACEN_W<MP_AHB6ENCLRRrs> {
        ETHMACEN_W::new(self, 10)
    }
    ///Bit 12 - FMCEN
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W<MP_AHB6ENCLRRrs> {
        FMCEN_W::new(self, 12)
    }
    ///Bit 14 - QSPIEN
    #[inline(always)]
    pub fn qspien(&mut self) -> QSPIEN_W<MP_AHB6ENCLRRrs> {
        QSPIEN_W::new(self, 14)
    }
    ///Bit 16 - SDMMC1EN
    #[inline(always)]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<MP_AHB6ENCLRRrs> {
        SDMMC1EN_W::new(self, 16)
    }
    ///Bit 17 - SDMMC2EN
    #[inline(always)]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<MP_AHB6ENCLRRrs> {
        SDMMC2EN_W::new(self, 17)
    }
    ///Bit 20 - CRC1EN
    #[inline(always)]
    pub fn crc1en(&mut self) -> CRC1EN_W<MP_AHB6ENCLRRrs> {
        CRC1EN_W::new(self, 20)
    }
    ///Bit 24 - USBHEN
    #[inline(always)]
    pub fn usbhen(&mut self) -> USBHEN_W<MP_AHB6ENCLRRrs> {
        USBHEN_W::new(self, 24)
    }
}
/**This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .

You can [`read`](crate::Reg::read) this register and get [`mp_ahb6enclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb6enclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB6ENCLRR)*/
pub struct MP_AHB6ENCLRRrs;
impl crate::RegisterSpec for MP_AHB6ENCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_ahb6enclrr::R`](R) reader structure
impl crate::Readable for MP_AHB6ENCLRRrs {}
///`write(|w| ..)` method takes [`mp_ahb6enclrr::W`](W) writer structure
impl crate::Writable for MP_AHB6ENCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_AHB6ENCLRR to value 0
impl crate::Resettable for MP_AHB6ENCLRRrs {}
