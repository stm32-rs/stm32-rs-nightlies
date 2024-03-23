#[doc = "Register `RCC_MC_APB5ENCLRR` reader"]
pub type R = crate::R<RCC_MC_APB5ENCLRRrs>;
#[doc = "Register `RCC_MC_APB5ENCLRR` writer"]
pub type W = crate::W<RCC_MC_APB5ENCLRRrs>;
#[doc = "Field `SPI6EN` reader - SPI6EN"]
pub type SPI6EN_R = crate::BitReader;
#[doc = "Field `SPI6EN` writer - SPI6EN"]
pub type SPI6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4EN` reader - I2C4EN"]
pub type I2C4EN_R = crate::BitReader;
#[doc = "Field `I2C4EN` writer - I2C4EN"]
pub type I2C4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C6EN` reader - I2C6EN"]
pub type I2C6EN_R = crate::BitReader;
#[doc = "Field `I2C6EN` writer - I2C6EN"]
pub type I2C6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1EN` reader - USART1EN"]
pub type USART1EN_R = crate::BitReader;
#[doc = "Field `USART1EN` writer - USART1EN"]
pub type USART1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAPBEN` reader - RTCAPBEN"]
pub type RTCAPBEN_R = crate::BitReader;
#[doc = "Field `RTCAPBEN` writer - RTCAPBEN"]
pub type RTCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZC1EN` reader - TZC1EN"]
pub type TZC1EN_R = crate::BitReader;
#[doc = "Field `TZC1EN` writer - TZC1EN"]
pub type TZC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZC2EN` reader - TZC2EN"]
pub type TZC2EN_R = crate::BitReader;
#[doc = "Field `TZC2EN` writer - TZC2EN"]
pub type TZC2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZPCEN` reader - TZPCEN"]
pub type TZPCEN_R = crate::BitReader;
#[doc = "Field `TZPCEN` writer - TZPCEN"]
pub type TZPCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSECEN` reader - BSECEN"]
pub type BSECEN_R = crate::BitReader;
#[doc = "Field `BSECEN` writer - BSECEN"]
pub type BSECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STGENEN` reader - STGENEN"]
pub type STGENEN_R = crate::BitReader;
#[doc = "Field `STGENEN` writer - STGENEN"]
pub type STGENEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI6EN"]
    #[inline(always)]
    pub fn spi6en(&self) -> SPI6EN_R {
        SPI6EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - I2C4EN"]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C6EN"]
    #[inline(always)]
    pub fn i2c6en(&self) -> I2C6EN_R {
        I2C6EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1EN"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - RTCAPBEN"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - TZC1EN"]
    #[inline(always)]
    pub fn tzc1en(&self) -> TZC1EN_R {
        TZC1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TZC2EN"]
    #[inline(always)]
    pub fn tzc2en(&self) -> TZC2EN_R {
        TZC2EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TZPCEN"]
    #[inline(always)]
    pub fn tzpcen(&self) -> TZPCEN_R {
        TZPCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - BSECEN"]
    #[inline(always)]
    pub fn bsecen(&self) -> BSECEN_R {
        BSECEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - STGENEN"]
    #[inline(always)]
    pub fn stgenen(&self) -> STGENEN_R {
        STGENEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI6EN"]
    #[inline(always)]
    #[must_use]
    pub fn spi6en(&mut self) -> SPI6EN_W<RCC_MC_APB5ENCLRRrs> {
        SPI6EN_W::new(self, 0)
    }
    #[doc = "Bit 2 - I2C4EN"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4en(&mut self) -> I2C4EN_W<RCC_MC_APB5ENCLRRrs> {
        I2C4EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - I2C6EN"]
    #[inline(always)]
    #[must_use]
    pub fn i2c6en(&mut self) -> I2C6EN_W<RCC_MC_APB5ENCLRRrs> {
        I2C6EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - USART1EN"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<RCC_MC_APB5ENCLRRrs> {
        USART1EN_W::new(self, 4)
    }
    #[doc = "Bit 8 - RTCAPBEN"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<RCC_MC_APB5ENCLRRrs> {
        RTCAPBEN_W::new(self, 8)
    }
    #[doc = "Bit 11 - TZC1EN"]
    #[inline(always)]
    #[must_use]
    pub fn tzc1en(&mut self) -> TZC1EN_W<RCC_MC_APB5ENCLRRrs> {
        TZC1EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - TZC2EN"]
    #[inline(always)]
    #[must_use]
    pub fn tzc2en(&mut self) -> TZC2EN_W<RCC_MC_APB5ENCLRRrs> {
        TZC2EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - TZPCEN"]
    #[inline(always)]
    #[must_use]
    pub fn tzpcen(&mut self) -> TZPCEN_W<RCC_MC_APB5ENCLRRrs> {
        TZPCEN_W::new(self, 13)
    }
    #[doc = "Bit 16 - BSECEN"]
    #[inline(always)]
    #[must_use]
    pub fn bsecen(&mut self) -> BSECEN_W<RCC_MC_APB5ENCLRRrs> {
        BSECEN_W::new(self, 16)
    }
    #[doc = "Bit 20 - STGENEN"]
    #[inline(always)]
    #[must_use]
    pub fn stgenen(&mut self) -> STGENEN_W<RCC_MC_APB5ENCLRRrs> {
        STGENEN_W::new(self, 20)
    }
}
#[doc = "This register is used to clear the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb5enclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb5enclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MC_APB5ENCLRRrs;
impl crate::RegisterSpec for RCC_MC_APB5ENCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mc_apb5enclrr::R`](R) reader structure"]
impl crate::Readable for RCC_MC_APB5ENCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mc_apb5enclrr::W`](W) writer structure"]
impl crate::Writable for RCC_MC_APB5ENCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MC_APB5ENCLRR to value 0"]
impl crate::Resettable for RCC_MC_APB5ENCLRRrs {
    const RESET_VALUE: u32 = 0;
}
