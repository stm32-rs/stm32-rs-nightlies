#[doc = "Register `IER2` reader"]
pub type R = crate::R<IER2rs>;
#[doc = "Register `IER2` writer"]
pub type W = crate::W<IER2rs>;
#[doc = "Field `TIM1IE` reader - illegal access interrupt enable for TIM1"]
pub type TIM1IE_R = crate::BitReader;
#[doc = "Field `TIM1IE` writer - illegal access interrupt enable for TIM1"]
pub type TIM1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1IE` reader - illegal access interrupt enable for SPI1"]
pub type SPI1IE_R = crate::BitReader;
#[doc = "Field `SPI1IE` writer - illegal access interrupt enable for SPI1"]
pub type SPI1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8IE` reader - illegal access interrupt enable for TIM8"]
pub type TIM8IE_R = crate::BitReader;
#[doc = "Field `TIM8IE` writer - illegal access interrupt enable for TIM8"]
pub type TIM8IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1IE` reader - illegal access interrupt enable for USART1"]
pub type USART1IE_R = crate::BitReader;
#[doc = "Field `USART1IE` writer - illegal access interrupt enable for USART1"]
pub type USART1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15IE` reader - illegal access interrupt enable for TIM5"]
pub type TIM15IE_R = crate::BitReader;
#[doc = "Field `TIM15IE` writer - illegal access interrupt enable for TIM5"]
pub type TIM15IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16IE` reader - illegal access interrupt enable for TIM6"]
pub type TIM16IE_R = crate::BitReader;
#[doc = "Field `TIM16IE` writer - illegal access interrupt enable for TIM6"]
pub type TIM16IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17IE` reader - illegal access interrupt enable for TIM7"]
pub type TIM17IE_R = crate::BitReader;
#[doc = "Field `TIM17IE` writer - illegal access interrupt enable for TIM7"]
pub type TIM17IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1IE` reader - illegal access interrupt enable for SAI1"]
pub type SAI1IE_R = crate::BitReader;
#[doc = "Field `SAI1IE` writer - illegal access interrupt enable for SAI1"]
pub type SAI1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI2IE` reader - illegal access interrupt enable for SAI2"]
pub type SAI2IE_R = crate::BitReader;
#[doc = "Field `SAI2IE` writer - illegal access interrupt enable for SAI2"]
pub type SAI2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTDCIE` reader - illegal access interrupt enable for LTDC"]
pub type LTDCIE_R = crate::BitReader;
#[doc = "Field `LTDCIE` writer - illegal access interrupt enable for LTDC"]
pub type LTDCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSIIE` reader - illegal access interrupt enable for DSI"]
pub type DSIIE_R = crate::BitReader;
#[doc = "Field `DSIIE` writer - illegal access interrupt enable for DSI"]
pub type DSIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - illegal access interrupt enable for TIM1"]
    #[inline(always)]
    pub fn tim1ie(&self) -> TIM1IE_R {
        TIM1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - illegal access interrupt enable for SPI1"]
    #[inline(always)]
    pub fn spi1ie(&self) -> SPI1IE_R {
        SPI1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - illegal access interrupt enable for TIM8"]
    #[inline(always)]
    pub fn tim8ie(&self) -> TIM8IE_R {
        TIM8IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - illegal access interrupt enable for USART1"]
    #[inline(always)]
    pub fn usart1ie(&self) -> USART1IE_R {
        USART1IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - illegal access interrupt enable for TIM5"]
    #[inline(always)]
    pub fn tim15ie(&self) -> TIM15IE_R {
        TIM15IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - illegal access interrupt enable for TIM6"]
    #[inline(always)]
    pub fn tim16ie(&self) -> TIM16IE_R {
        TIM16IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - illegal access interrupt enable for TIM7"]
    #[inline(always)]
    pub fn tim17ie(&self) -> TIM17IE_R {
        TIM17IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - illegal access interrupt enable for SAI1"]
    #[inline(always)]
    pub fn sai1ie(&self) -> SAI1IE_R {
        SAI1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - illegal access interrupt enable for SAI2"]
    #[inline(always)]
    pub fn sai2ie(&self) -> SAI2IE_R {
        SAI2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - illegal access interrupt enable for LTDC"]
    #[inline(always)]
    pub fn ltdcie(&self) -> LTDCIE_R {
        LTDCIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - illegal access interrupt enable for DSI"]
    #[inline(always)]
    pub fn dsiie(&self) -> DSIIE_R {
        DSIIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - illegal access interrupt enable for TIM1"]
    #[inline(always)]
    #[must_use]
    pub fn tim1ie(&mut self) -> TIM1IE_W<IER2rs> {
        TIM1IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - illegal access interrupt enable for SPI1"]
    #[inline(always)]
    #[must_use]
    pub fn spi1ie(&mut self) -> SPI1IE_W<IER2rs> {
        SPI1IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - illegal access interrupt enable for TIM8"]
    #[inline(always)]
    #[must_use]
    pub fn tim8ie(&mut self) -> TIM8IE_W<IER2rs> {
        TIM8IE_W::new(self, 2)
    }
    #[doc = "Bit 3 - illegal access interrupt enable for USART1"]
    #[inline(always)]
    #[must_use]
    pub fn usart1ie(&mut self) -> USART1IE_W<IER2rs> {
        USART1IE_W::new(self, 3)
    }
    #[doc = "Bit 4 - illegal access interrupt enable for TIM5"]
    #[inline(always)]
    #[must_use]
    pub fn tim15ie(&mut self) -> TIM15IE_W<IER2rs> {
        TIM15IE_W::new(self, 4)
    }
    #[doc = "Bit 5 - illegal access interrupt enable for TIM6"]
    #[inline(always)]
    #[must_use]
    pub fn tim16ie(&mut self) -> TIM16IE_W<IER2rs> {
        TIM16IE_W::new(self, 5)
    }
    #[doc = "Bit 6 - illegal access interrupt enable for TIM7"]
    #[inline(always)]
    #[must_use]
    pub fn tim17ie(&mut self) -> TIM17IE_W<IER2rs> {
        TIM17IE_W::new(self, 6)
    }
    #[doc = "Bit 7 - illegal access interrupt enable for SAI1"]
    #[inline(always)]
    #[must_use]
    pub fn sai1ie(&mut self) -> SAI1IE_W<IER2rs> {
        SAI1IE_W::new(self, 7)
    }
    #[doc = "Bit 8 - illegal access interrupt enable for SAI2"]
    #[inline(always)]
    #[must_use]
    pub fn sai2ie(&mut self) -> SAI2IE_W<IER2rs> {
        SAI2IE_W::new(self, 8)
    }
    #[doc = "Bit 9 - illegal access interrupt enable for LTDC"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcie(&mut self) -> LTDCIE_W<IER2rs> {
        LTDCIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - illegal access interrupt enable for DSI"]
    #[inline(always)]
    #[must_use]
    pub fn dsiie(&mut self) -> DSIIE_W<IER2rs> {
        DSIIE_W::new(self, 10)
    }
}
#[doc = "TZIC interrupt enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER2rs;
impl crate::RegisterSpec for IER2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier2::R`](R) reader structure"]
impl crate::Readable for IER2rs {}
#[doc = "`write(|w| ..)` method takes [`ier2::W`](W) writer structure"]
impl crate::Writable for IER2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER2 to value 0"]
impl crate::Resettable for IER2rs {
    const RESET_VALUE: u32 = 0;
}
