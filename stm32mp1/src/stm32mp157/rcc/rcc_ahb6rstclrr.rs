#[doc = "Register `RCC_AHB6RSTCLRR` reader"]
pub type R = crate::R<RCC_AHB6RSTCLRRrs>;
#[doc = "Register `RCC_AHB6RSTCLRR` writer"]
pub type W = crate::W<RCC_AHB6RSTCLRRrs>;
#[doc = "Field `ETHMACRST` reader - ETHMACRST"]
pub type ETHMACRST_R = crate::BitReader;
#[doc = "Field `ETHMACRST` writer - ETHMACRST"]
pub type ETHMACRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMCRST` reader - FMCRST"]
pub type FMCRST_R = crate::BitReader;
#[doc = "Field `FMCRST` writer - FMCRST"]
pub type FMCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPIRST` reader - QSPIRST"]
pub type QSPIRST_R = crate::BitReader;
#[doc = "Field `QSPIRST` writer - QSPIRST"]
pub type QSPIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC1RST` reader - SDMMC1RST"]
pub type SDMMC1RST_R = crate::BitReader;
#[doc = "Field `SDMMC1RST` writer - SDMMC1RST"]
pub type SDMMC1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC2RST` reader - SDMMC2RST"]
pub type SDMMC2RST_R = crate::BitReader;
#[doc = "Field `SDMMC2RST` writer - SDMMC2RST"]
pub type SDMMC2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC1RST` reader - CRC1RST"]
pub type CRC1RST_R = crate::BitReader;
#[doc = "Field `CRC1RST` writer - CRC1RST"]
pub type CRC1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBHRST` reader - USBHRST"]
pub type USBHRST_R = crate::BitReader;
#[doc = "Field `USBHRST` writer - USBHRST"]
pub type USBHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 10 - ETHMACRST"]
    #[inline(always)]
    pub fn ethmacrst(&self) -> ETHMACRST_R {
        ETHMACRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - FMCRST"]
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - QSPIRST"]
    #[inline(always)]
    pub fn qspirst(&self) -> QSPIRST_R {
        QSPIRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1RST"]
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SDMMC2RST"]
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - CRC1RST"]
    #[inline(always)]
    pub fn crc1rst(&self) -> CRC1RST_R {
        CRC1RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - USBHRST"]
    #[inline(always)]
    pub fn usbhrst(&self) -> USBHRST_R {
        USBHRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - ETHMACRST"]
    #[inline(always)]
    #[must_use]
    pub fn ethmacrst(&mut self) -> ETHMACRST_W<RCC_AHB6RSTCLRRrs> {
        ETHMACRST_W::new(self, 10)
    }
    #[doc = "Bit 12 - FMCRST"]
    #[inline(always)]
    #[must_use]
    pub fn fmcrst(&mut self) -> FMCRST_W<RCC_AHB6RSTCLRRrs> {
        FMCRST_W::new(self, 12)
    }
    #[doc = "Bit 14 - QSPIRST"]
    #[inline(always)]
    #[must_use]
    pub fn qspirst(&mut self) -> QSPIRST_W<RCC_AHB6RSTCLRRrs> {
        QSPIRST_W::new(self, 14)
    }
    #[doc = "Bit 16 - SDMMC1RST"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<RCC_AHB6RSTCLRRrs> {
        SDMMC1RST_W::new(self, 16)
    }
    #[doc = "Bit 17 - SDMMC2RST"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W<RCC_AHB6RSTCLRRrs> {
        SDMMC2RST_W::new(self, 17)
    }
    #[doc = "Bit 20 - CRC1RST"]
    #[inline(always)]
    #[must_use]
    pub fn crc1rst(&mut self) -> CRC1RST_W<RCC_AHB6RSTCLRRrs> {
        CRC1RST_W::new(self, 20)
    }
    #[doc = "Bit 24 - USBHRST"]
    #[inline(always)]
    #[must_use]
    pub fn usbhrst(&mut self) -> USBHRST_W<RCC_AHB6RSTCLRRrs> {
        USBHRST_W::new(self, 24)
    }
}
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb6rstclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb6rstclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_AHB6RSTCLRRrs;
impl crate::RegisterSpec for RCC_AHB6RSTCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb6rstclrr::R`](R) reader structure"]
impl crate::Readable for RCC_AHB6RSTCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb6rstclrr::W`](W) writer structure"]
impl crate::Writable for RCC_AHB6RSTCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHB6RSTCLRR to value 0"]
impl crate::Resettable for RCC_AHB6RSTCLRRrs {
    const RESET_VALUE: u32 = 0;
}
