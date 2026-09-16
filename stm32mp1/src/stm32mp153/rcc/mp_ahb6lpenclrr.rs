///Register `MP_AHB6LPENCLRR` reader
pub type R = crate::R<MP_AHB6LPENCLRRrs>;
///Register `MP_AHB6LPENCLRR` writer
pub type W = crate::W<MP_AHB6LPENCLRRrs>;
///Field `MDMALPEN` reader - MDMALPEN
pub type MDMALPEN_R = crate::BitReader;
///Field `MDMALPEN` writer - MDMALPEN
pub type MDMALPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPULPEN` reader - GPULPEN
pub type GPULPEN_R = crate::BitReader;
///Field `GPULPEN` writer - GPULPEN
pub type GPULPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHCKLPEN` reader - ETHCKLPEN
pub type ETHCKLPEN_R = crate::BitReader;
///Field `ETHCKLPEN` writer - ETHCKLPEN
pub type ETHCKLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHTXLPEN` reader - ETHTXLPEN
pub type ETHTXLPEN_R = crate::BitReader;
///Field `ETHTXLPEN` writer - ETHTXLPEN
pub type ETHTXLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHRXLPEN` reader - ETHRXLPEN
pub type ETHRXLPEN_R = crate::BitReader;
///Field `ETHRXLPEN` writer - ETHRXLPEN
pub type ETHRXLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHMACLPEN` reader - ETHMACLPEN
pub type ETHMACLPEN_R = crate::BitReader;
///Field `ETHMACLPEN` writer - ETHMACLPEN
pub type ETHMACLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHSTPEN` reader - ETHSTPEN
pub type ETHSTPEN_R = crate::BitReader;
///Field `ETHSTPEN` writer - ETHSTPEN
pub type ETHSTPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMCLPEN` reader - FMCLPEN
pub type FMCLPEN_R = crate::BitReader;
///Field `FMCLPEN` writer - FMCLPEN
pub type FMCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `QSPILPEN` reader - QSPILPEN
pub type QSPILPEN_R = crate::BitReader;
///Field `QSPILPEN` writer - QSPILPEN
pub type QSPILPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1LPEN` reader - SDMMC1LPEN
pub type SDMMC1LPEN_R = crate::BitReader;
///Field `SDMMC1LPEN` writer - SDMMC1LPEN
pub type SDMMC1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC2LPEN` reader - SDMMC2LPEN
pub type SDMMC2LPEN_R = crate::BitReader;
///Field `SDMMC2LPEN` writer - SDMMC2LPEN
pub type SDMMC2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRC1LPEN` reader - CRC1LPEN
pub type CRC1LPEN_R = crate::BitReader;
///Field `CRC1LPEN` writer - CRC1LPEN
pub type CRC1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBHLPEN` reader - USBHLPEN
pub type USBHLPEN_R = crate::BitReader;
///Field `USBHLPEN` writer - USBHLPEN
pub type USBHLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MDMALPEN
    #[inline(always)]
    pub fn mdmalpen(&self) -> MDMALPEN_R {
        MDMALPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 5 - GPULPEN
    #[inline(always)]
    pub fn gpulpen(&self) -> GPULPEN_R {
        GPULPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - ETHCKLPEN
    #[inline(always)]
    pub fn ethcklpen(&self) -> ETHCKLPEN_R {
        ETHCKLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ETHTXLPEN
    #[inline(always)]
    pub fn ethtxlpen(&self) -> ETHTXLPEN_R {
        ETHTXLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ETHRXLPEN
    #[inline(always)]
    pub fn ethrxlpen(&self) -> ETHRXLPEN_R {
        ETHRXLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ETHMACLPEN
    #[inline(always)]
    pub fn ethmaclpen(&self) -> ETHMACLPEN_R {
        ETHMACLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - ETHSTPEN
    #[inline(always)]
    pub fn ethstpen(&self) -> ETHSTPEN_R {
        ETHSTPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - FMCLPEN
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - QSPILPEN
    #[inline(always)]
    pub fn qspilpen(&self) -> QSPILPEN_R {
        QSPILPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - SDMMC1LPEN
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SDMMC2LPEN
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> SDMMC2LPEN_R {
        SDMMC2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - CRC1LPEN
    #[inline(always)]
    pub fn crc1lpen(&self) -> CRC1LPEN_R {
        CRC1LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - USBHLPEN
    #[inline(always)]
    pub fn usbhlpen(&self) -> USBHLPEN_R {
        USBHLPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_AHB6LPENCLRR")
            .field("mdmalpen", &self.mdmalpen())
            .field("gpulpen", &self.gpulpen())
            .field("ethcklpen", &self.ethcklpen())
            .field("ethtxlpen", &self.ethtxlpen())
            .field("ethrxlpen", &self.ethrxlpen())
            .field("ethmaclpen", &self.ethmaclpen())
            .field("ethstpen", &self.ethstpen())
            .field("fmclpen", &self.fmclpen())
            .field("qspilpen", &self.qspilpen())
            .field("sdmmc1lpen", &self.sdmmc1lpen())
            .field("sdmmc2lpen", &self.sdmmc2lpen())
            .field("crc1lpen", &self.crc1lpen())
            .field("usbhlpen", &self.usbhlpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - MDMALPEN
    #[inline(always)]
    pub fn mdmalpen(&mut self) -> MDMALPEN_W<'_, MP_AHB6LPENCLRRrs> {
        MDMALPEN_W::new(self, 0)
    }
    ///Bit 5 - GPULPEN
    #[inline(always)]
    pub fn gpulpen(&mut self) -> GPULPEN_W<'_, MP_AHB6LPENCLRRrs> {
        GPULPEN_W::new(self, 5)
    }
    ///Bit 7 - ETHCKLPEN
    #[inline(always)]
    pub fn ethcklpen(&mut self) -> ETHCKLPEN_W<'_, MP_AHB6LPENCLRRrs> {
        ETHCKLPEN_W::new(self, 7)
    }
    ///Bit 8 - ETHTXLPEN
    #[inline(always)]
    pub fn ethtxlpen(&mut self) -> ETHTXLPEN_W<'_, MP_AHB6LPENCLRRrs> {
        ETHTXLPEN_W::new(self, 8)
    }
    ///Bit 9 - ETHRXLPEN
    #[inline(always)]
    pub fn ethrxlpen(&mut self) -> ETHRXLPEN_W<'_, MP_AHB6LPENCLRRrs> {
        ETHRXLPEN_W::new(self, 9)
    }
    ///Bit 10 - ETHMACLPEN
    #[inline(always)]
    pub fn ethmaclpen(&mut self) -> ETHMACLPEN_W<'_, MP_AHB6LPENCLRRrs> {
        ETHMACLPEN_W::new(self, 10)
    }
    ///Bit 11 - ETHSTPEN
    #[inline(always)]
    pub fn ethstpen(&mut self) -> ETHSTPEN_W<'_, MP_AHB6LPENCLRRrs> {
        ETHSTPEN_W::new(self, 11)
    }
    ///Bit 12 - FMCLPEN
    #[inline(always)]
    pub fn fmclpen(&mut self) -> FMCLPEN_W<'_, MP_AHB6LPENCLRRrs> {
        FMCLPEN_W::new(self, 12)
    }
    ///Bit 14 - QSPILPEN
    #[inline(always)]
    pub fn qspilpen(&mut self) -> QSPILPEN_W<'_, MP_AHB6LPENCLRRrs> {
        QSPILPEN_W::new(self, 14)
    }
    ///Bit 16 - SDMMC1LPEN
    #[inline(always)]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W<'_, MP_AHB6LPENCLRRrs> {
        SDMMC1LPEN_W::new(self, 16)
    }
    ///Bit 17 - SDMMC2LPEN
    #[inline(always)]
    pub fn sdmmc2lpen(&mut self) -> SDMMC2LPEN_W<'_, MP_AHB6LPENCLRRrs> {
        SDMMC2LPEN_W::new(self, 17)
    }
    ///Bit 20 - CRC1LPEN
    #[inline(always)]
    pub fn crc1lpen(&mut self) -> CRC1LPEN_W<'_, MP_AHB6LPENCLRRrs> {
        CRC1LPEN_W::new(self, 20)
    }
    ///Bit 24 - USBHLPEN
    #[inline(always)]
    pub fn usbhlpen(&mut self) -> USBHLPEN_W<'_, MP_AHB6LPENCLRRrs> {
        USBHLPEN_W::new(self, 24)
    }
}
/**This register is used by the MCU in order to clear the PERxLPEN bits

You can [`read`](crate::Reg::read) this register and get [`mp_ahb6lpenclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb6lpenclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB6LPENCLRR)*/
pub struct MP_AHB6LPENCLRRrs;
impl crate::RegisterSpec for MP_AHB6LPENCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_ahb6lpenclrr::R`](R) reader structure
impl crate::Readable for MP_AHB6LPENCLRRrs {}
///`write(|w| ..)` method takes [`mp_ahb6lpenclrr::W`](W) writer structure
impl crate::Writable for MP_AHB6LPENCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_AHB6LPENCLRR to value 0x0113_57a1
impl crate::Resettable for MP_AHB6LPENCLRRrs {
    const RESET_VALUE: u32 = 0x0113_57a1;
}
