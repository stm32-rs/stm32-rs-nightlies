#[doc = "Register `RCC_MP_AHB6LPENSETR` reader"]
pub type R = crate::R<RCC_MP_AHB6LPENSETRrs>;
#[doc = "Register `RCC_MP_AHB6LPENSETR` writer"]
pub type W = crate::W<RCC_MP_AHB6LPENSETRrs>;
#[doc = "Field `MDMALPEN` reader - MDMALPEN"]
pub type MDMALPEN_R = crate::BitReader;
#[doc = "Field `MDMALPEN` writer - MDMALPEN"]
pub type MDMALPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPULPEN` reader - GPULPEN"]
pub type GPULPEN_R = crate::BitReader;
#[doc = "Field `GPULPEN` writer - GPULPEN"]
pub type GPULPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHCKLPEN` reader - ETHCKLPEN"]
pub type ETHCKLPEN_R = crate::BitReader;
#[doc = "Field `ETHCKLPEN` writer - ETHCKLPEN"]
pub type ETHCKLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHTXLPEN` reader - ETHTXLPEN"]
pub type ETHTXLPEN_R = crate::BitReader;
#[doc = "Field `ETHTXLPEN` writer - ETHTXLPEN"]
pub type ETHTXLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHRXLPEN` reader - ETHRXLPEN"]
pub type ETHRXLPEN_R = crate::BitReader;
#[doc = "Field `ETHRXLPEN` writer - ETHRXLPEN"]
pub type ETHRXLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHMACLPEN` reader - ETHMACLPEN"]
pub type ETHMACLPEN_R = crate::BitReader;
#[doc = "Field `ETHMACLPEN` writer - ETHMACLPEN"]
pub type ETHMACLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHSTPEN` reader - ETHSTPEN"]
pub type ETHSTPEN_R = crate::BitReader;
#[doc = "Field `ETHSTPEN` writer - ETHSTPEN"]
pub type ETHSTPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMCLPEN` reader - FMCLPEN"]
pub type FMCLPEN_R = crate::BitReader;
#[doc = "Field `FMCLPEN` writer - FMCLPEN"]
pub type FMCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPILPEN` reader - QSPILPEN"]
pub type QSPILPEN_R = crate::BitReader;
#[doc = "Field `QSPILPEN` writer - QSPILPEN"]
pub type QSPILPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC1LPEN` reader - SDMMC1LPEN"]
pub type SDMMC1LPEN_R = crate::BitReader;
#[doc = "Field `SDMMC1LPEN` writer - SDMMC1LPEN"]
pub type SDMMC1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC2LPEN` reader - SDMMC2LPEN"]
pub type SDMMC2LPEN_R = crate::BitReader;
#[doc = "Field `SDMMC2LPEN` writer - SDMMC2LPEN"]
pub type SDMMC2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC1LPEN` reader - CRC1LPEN"]
pub type CRC1LPEN_R = crate::BitReader;
#[doc = "Field `CRC1LPEN` writer - CRC1LPEN"]
pub type CRC1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBHLPEN` reader - USBHLPEN"]
pub type USBHLPEN_R = crate::BitReader;
#[doc = "Field `USBHLPEN` writer - USBHLPEN"]
pub type USBHLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MDMALPEN"]
    #[inline(always)]
    pub fn mdmalpen(&self) -> MDMALPEN_R {
        MDMALPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - GPULPEN"]
    #[inline(always)]
    pub fn gpulpen(&self) -> GPULPEN_R {
        GPULPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - ETHCKLPEN"]
    #[inline(always)]
    pub fn ethcklpen(&self) -> ETHCKLPEN_R {
        ETHCKLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ETHTXLPEN"]
    #[inline(always)]
    pub fn ethtxlpen(&self) -> ETHTXLPEN_R {
        ETHTXLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ETHRXLPEN"]
    #[inline(always)]
    pub fn ethrxlpen(&self) -> ETHRXLPEN_R {
        ETHRXLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ETHMACLPEN"]
    #[inline(always)]
    pub fn ethmaclpen(&self) -> ETHMACLPEN_R {
        ETHMACLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ETHSTPEN"]
    #[inline(always)]
    pub fn ethstpen(&self) -> ETHSTPEN_R {
        ETHSTPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FMCLPEN"]
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - QSPILPEN"]
    #[inline(always)]
    pub fn qspilpen(&self) -> QSPILPEN_R {
        QSPILPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1LPEN"]
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SDMMC2LPEN"]
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> SDMMC2LPEN_R {
        SDMMC2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - CRC1LPEN"]
    #[inline(always)]
    pub fn crc1lpen(&self) -> CRC1LPEN_R {
        CRC1LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - USBHLPEN"]
    #[inline(always)]
    pub fn usbhlpen(&self) -> USBHLPEN_R {
        USBHLPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMALPEN"]
    #[inline(always)]
    #[must_use]
    pub fn mdmalpen(&mut self) -> MDMALPEN_W<RCC_MP_AHB6LPENSETRrs> {
        MDMALPEN_W::new(self, 0)
    }
    #[doc = "Bit 5 - GPULPEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpulpen(&mut self) -> GPULPEN_W<RCC_MP_AHB6LPENSETRrs> {
        GPULPEN_W::new(self, 5)
    }
    #[doc = "Bit 7 - ETHCKLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ethcklpen(&mut self) -> ETHCKLPEN_W<RCC_MP_AHB6LPENSETRrs> {
        ETHCKLPEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - ETHTXLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ethtxlpen(&mut self) -> ETHTXLPEN_W<RCC_MP_AHB6LPENSETRrs> {
        ETHTXLPEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - ETHRXLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ethrxlpen(&mut self) -> ETHRXLPEN_W<RCC_MP_AHB6LPENSETRrs> {
        ETHRXLPEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - ETHMACLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ethmaclpen(&mut self) -> ETHMACLPEN_W<RCC_MP_AHB6LPENSETRrs> {
        ETHMACLPEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - ETHSTPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ethstpen(&mut self) -> ETHSTPEN_W<RCC_MP_AHB6LPENSETRrs> {
        ETHSTPEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - FMCLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn fmclpen(&mut self) -> FMCLPEN_W<RCC_MP_AHB6LPENSETRrs> {
        FMCLPEN_W::new(self, 12)
    }
    #[doc = "Bit 14 - QSPILPEN"]
    #[inline(always)]
    #[must_use]
    pub fn qspilpen(&mut self) -> QSPILPEN_W<RCC_MP_AHB6LPENSETRrs> {
        QSPILPEN_W::new(self, 14)
    }
    #[doc = "Bit 16 - SDMMC1LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W<RCC_MP_AHB6LPENSETRrs> {
        SDMMC1LPEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - SDMMC2LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2lpen(&mut self) -> SDMMC2LPEN_W<RCC_MP_AHB6LPENSETRrs> {
        SDMMC2LPEN_W::new(self, 17)
    }
    #[doc = "Bit 20 - CRC1LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn crc1lpen(&mut self) -> CRC1LPEN_W<RCC_MP_AHB6LPENSETRrs> {
        CRC1LPEN_W::new(self, 20)
    }
    #[doc = "Bit 24 - USBHLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn usbhlpen(&mut self) -> USBHLPEN_W<RCC_MP_AHB6LPENSETRrs> {
        USBHLPEN_W::new(self, 24)
    }
}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb6lpensetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb6lpensetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_AHB6LPENSETRrs;
impl crate::RegisterSpec for RCC_MP_AHB6LPENSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_ahb6lpensetr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_AHB6LPENSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mp_ahb6lpensetr::W`](W) writer structure"]
impl crate::Writable for RCC_MP_AHB6LPENSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MP_AHB6LPENSETR to value 0x0113_57a1"]
impl crate::Resettable for RCC_MP_AHB6LPENSETRrs {
    const RESET_VALUE: u32 = 0x0113_57a1;
}
