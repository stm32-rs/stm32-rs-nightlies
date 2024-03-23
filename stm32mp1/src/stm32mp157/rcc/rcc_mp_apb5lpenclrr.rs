#[doc = "Register `RCC_MP_APB5LPENCLRR` reader"]
pub type R = crate::R<RCC_MP_APB5LPENCLRRrs>;
#[doc = "Register `RCC_MP_APB5LPENCLRR` writer"]
pub type W = crate::W<RCC_MP_APB5LPENCLRRrs>;
#[doc = "Field `SPI6LPEN` reader - SPI6LPEN"]
pub type SPI6LPEN_R = crate::BitReader;
#[doc = "Field `SPI6LPEN` writer - SPI6LPEN"]
pub type SPI6LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4LPEN` reader - I2C4LPEN"]
pub type I2C4LPEN_R = crate::BitReader;
#[doc = "Field `I2C4LPEN` writer - I2C4LPEN"]
pub type I2C4LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C6LPEN` reader - I2C6LPEN"]
pub type I2C6LPEN_R = crate::BitReader;
#[doc = "Field `I2C6LPEN` writer - I2C6LPEN"]
pub type I2C6LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1LPEN` reader - USART1LPEN"]
pub type USART1LPEN_R = crate::BitReader;
#[doc = "Field `USART1LPEN` writer - USART1LPEN"]
pub type USART1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAPBLPEN` reader - RTCAPBLPEN"]
pub type RTCAPBLPEN_R = crate::BitReader;
#[doc = "Field `RTCAPBLPEN` writer - RTCAPBLPEN"]
pub type RTCAPBLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZC1LPEN` reader - TZC1LPEN"]
pub type TZC1LPEN_R = crate::BitReader;
#[doc = "Field `TZC1LPEN` writer - TZC1LPEN"]
pub type TZC1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZC2LPEN` reader - TZC2LPEN"]
pub type TZC2LPEN_R = crate::BitReader;
#[doc = "Field `TZC2LPEN` writer - TZC2LPEN"]
pub type TZC2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZPCLPEN` reader - TZPCLPEN"]
pub type TZPCLPEN_R = crate::BitReader;
#[doc = "Field `TZPCLPEN` writer - TZPCLPEN"]
pub type TZPCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG1APBLPEN` reader - IWDG1APBLPEN"]
pub type IWDG1APBLPEN_R = crate::BitReader;
#[doc = "Field `IWDG1APBLPEN` writer - IWDG1APBLPEN"]
pub type IWDG1APBLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSECLPEN` reader - BSECLPEN"]
pub type BSECLPEN_R = crate::BitReader;
#[doc = "Field `BSECLPEN` writer - BSECLPEN"]
pub type BSECLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STGENLPEN` reader - STGENLPEN"]
pub type STGENLPEN_R = crate::BitReader;
#[doc = "Field `STGENLPEN` writer - STGENLPEN"]
pub type STGENLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STGENSTPEN` reader - STGENSTPEN"]
pub type STGENSTPEN_R = crate::BitReader;
#[doc = "Field `STGENSTPEN` writer - STGENSTPEN"]
pub type STGENSTPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI6LPEN"]
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - I2C4LPEN"]
    #[inline(always)]
    pub fn i2c4lpen(&self) -> I2C4LPEN_R {
        I2C4LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C6LPEN"]
    #[inline(always)]
    pub fn i2c6lpen(&self) -> I2C6LPEN_R {
        I2C6LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1LPEN"]
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - RTCAPBLPEN"]
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - TZC1LPEN"]
    #[inline(always)]
    pub fn tzc1lpen(&self) -> TZC1LPEN_R {
        TZC1LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TZC2LPEN"]
    #[inline(always)]
    pub fn tzc2lpen(&self) -> TZC2LPEN_R {
        TZC2LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TZPCLPEN"]
    #[inline(always)]
    pub fn tzpclpen(&self) -> TZPCLPEN_R {
        TZPCLPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - IWDG1APBLPEN"]
    #[inline(always)]
    pub fn iwdg1apblpen(&self) -> IWDG1APBLPEN_R {
        IWDG1APBLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - BSECLPEN"]
    #[inline(always)]
    pub fn bseclpen(&self) -> BSECLPEN_R {
        BSECLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - STGENLPEN"]
    #[inline(always)]
    pub fn stgenlpen(&self) -> STGENLPEN_R {
        STGENLPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STGENSTPEN"]
    #[inline(always)]
    pub fn stgenstpen(&self) -> STGENSTPEN_R {
        STGENSTPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI6LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W<RCC_MP_APB5LPENCLRRrs> {
        SPI6LPEN_W::new(self, 0)
    }
    #[doc = "Bit 2 - I2C4LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4lpen(&mut self) -> I2C4LPEN_W<RCC_MP_APB5LPENCLRRrs> {
        I2C4LPEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - I2C6LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn i2c6lpen(&mut self) -> I2C6LPEN_W<RCC_MP_APB5LPENCLRRrs> {
        I2C6LPEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - USART1LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<RCC_MP_APB5LPENCLRRrs> {
        USART1LPEN_W::new(self, 4)
    }
    #[doc = "Bit 8 - RTCAPBLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W<RCC_MP_APB5LPENCLRRrs> {
        RTCAPBLPEN_W::new(self, 8)
    }
    #[doc = "Bit 11 - TZC1LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn tzc1lpen(&mut self) -> TZC1LPEN_W<RCC_MP_APB5LPENCLRRrs> {
        TZC1LPEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - TZC2LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn tzc2lpen(&mut self) -> TZC2LPEN_W<RCC_MP_APB5LPENCLRRrs> {
        TZC2LPEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - TZPCLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn tzpclpen(&mut self) -> TZPCLPEN_W<RCC_MP_APB5LPENCLRRrs> {
        TZPCLPEN_W::new(self, 13)
    }
    #[doc = "Bit 15 - IWDG1APBLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg1apblpen(&mut self) -> IWDG1APBLPEN_W<RCC_MP_APB5LPENCLRRrs> {
        IWDG1APBLPEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - BSECLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn bseclpen(&mut self) -> BSECLPEN_W<RCC_MP_APB5LPENCLRRrs> {
        BSECLPEN_W::new(self, 16)
    }
    #[doc = "Bit 20 - STGENLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn stgenlpen(&mut self) -> STGENLPEN_W<RCC_MP_APB5LPENCLRRrs> {
        STGENLPEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - STGENSTPEN"]
    #[inline(always)]
    #[must_use]
    pub fn stgenstpen(&mut self) -> STGENSTPEN_W<RCC_MP_APB5LPENCLRRrs> {
        STGENSTPEN_W::new(self, 21)
    }
}
#[doc = "This register is used by the Mpu.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb5lpenclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb5lpenclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_APB5LPENCLRRrs;
impl crate::RegisterSpec for RCC_MP_APB5LPENCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_apb5lpenclrr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_APB5LPENCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mp_apb5lpenclrr::W`](W) writer structure"]
impl crate::Writable for RCC_MP_APB5LPENCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MP_APB5LPENCLRR to value 0x0011_391d"]
impl crate::Resettable for RCC_MP_APB5LPENCLRRrs {
    const RESET_VALUE: u32 = 0x0011_391d;
}
