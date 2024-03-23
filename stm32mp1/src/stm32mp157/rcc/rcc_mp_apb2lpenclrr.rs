#[doc = "Register `RCC_MP_APB2LPENCLRR` reader"]
pub type R = crate::R<RCC_MP_APB2LPENCLRRrs>;
#[doc = "Register `RCC_MP_APB2LPENCLRR` writer"]
pub type W = crate::W<RCC_MP_APB2LPENCLRRrs>;
#[doc = "Field `TIM1LPEN` reader - TIM1LPEN"]
pub type TIM1LPEN_R = crate::BitReader;
#[doc = "Field `TIM1LPEN` writer - TIM1LPEN"]
pub type TIM1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8LPEN` reader - TIM8LPEN"]
pub type TIM8LPEN_R = crate::BitReader;
#[doc = "Field `TIM8LPEN` writer - TIM8LPEN"]
pub type TIM8LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15LPEN` reader - TIM15LPEN"]
pub type TIM15LPEN_R = crate::BitReader;
#[doc = "Field `TIM15LPEN` writer - TIM15LPEN"]
pub type TIM15LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16LPEN` reader - TIM16LPEN"]
pub type TIM16LPEN_R = crate::BitReader;
#[doc = "Field `TIM16LPEN` writer - TIM16LPEN"]
pub type TIM16LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17LPEN` reader - TIM17LPEN"]
pub type TIM17LPEN_R = crate::BitReader;
#[doc = "Field `TIM17LPEN` writer - TIM17LPEN"]
pub type TIM17LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1LPEN` reader - SPI1LPEN"]
pub type SPI1LPEN_R = crate::BitReader;
#[doc = "Field `SPI1LPEN` writer - SPI1LPEN"]
pub type SPI1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4LPEN` reader - SPI4LPEN"]
pub type SPI4LPEN_R = crate::BitReader;
#[doc = "Field `SPI4LPEN` writer - SPI4LPEN"]
pub type SPI4LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI5LPEN` reader - SPI5LPEN"]
pub type SPI5LPEN_R = crate::BitReader;
#[doc = "Field `SPI5LPEN` writer - SPI5LPEN"]
pub type SPI5LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6LPEN` reader - USART6LPEN"]
pub type USART6LPEN_R = crate::BitReader;
#[doc = "Field `USART6LPEN` writer - USART6LPEN"]
pub type USART6LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1LPEN` reader - SAI1LPEN"]
pub type SAI1LPEN_R = crate::BitReader;
#[doc = "Field `SAI1LPEN` writer - SAI1LPEN"]
pub type SAI1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI2LPEN` reader - SAI2LPEN"]
pub type SAI2LPEN_R = crate::BitReader;
#[doc = "Field `SAI2LPEN` writer - SAI2LPEN"]
pub type SAI2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI3LPEN` reader - SAI3LPEN"]
pub type SAI3LPEN_R = crate::BitReader;
#[doc = "Field `SAI3LPEN` writer - SAI3LPEN"]
pub type SAI3LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFSDMLPEN` reader - DFSDMLPEN"]
pub type DFSDMLPEN_R = crate::BitReader;
#[doc = "Field `DFSDMLPEN` writer - DFSDMLPEN"]
pub type DFSDMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADFSDMLPEN` reader - ADFSDMLPEN"]
pub type ADFSDMLPEN_R = crate::BitReader;
#[doc = "Field `ADFSDMLPEN` writer - ADFSDMLPEN"]
pub type ADFSDMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDCANLPEN` reader - FDCANLPEN"]
pub type FDCANLPEN_R = crate::BitReader;
#[doc = "Field `FDCANLPEN` writer - FDCANLPEN"]
pub type FDCANLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM1LPEN"]
    #[inline(always)]
    pub fn tim1lpen(&self) -> TIM1LPEN_R {
        TIM1LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8LPEN"]
    #[inline(always)]
    pub fn tim8lpen(&self) -> TIM8LPEN_R {
        TIM8LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM15LPEN"]
    #[inline(always)]
    pub fn tim15lpen(&self) -> TIM15LPEN_R {
        TIM15LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM16LPEN"]
    #[inline(always)]
    pub fn tim16lpen(&self) -> TIM16LPEN_R {
        TIM16LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM17LPEN"]
    #[inline(always)]
    pub fn tim17lpen(&self) -> TIM17LPEN_R {
        TIM17LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI1LPEN"]
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI4LPEN"]
    #[inline(always)]
    pub fn spi4lpen(&self) -> SPI4LPEN_R {
        SPI4LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI5LPEN"]
    #[inline(always)]
    pub fn spi5lpen(&self) -> SPI5LPEN_R {
        SPI5LPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - USART6LPEN"]
    #[inline(always)]
    pub fn usart6lpen(&self) -> USART6LPEN_R {
        USART6LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - SAI1LPEN"]
    #[inline(always)]
    pub fn sai1lpen(&self) -> SAI1LPEN_R {
        SAI1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SAI2LPEN"]
    #[inline(always)]
    pub fn sai2lpen(&self) -> SAI2LPEN_R {
        SAI2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SAI3LPEN"]
    #[inline(always)]
    pub fn sai3lpen(&self) -> SAI3LPEN_R {
        SAI3LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - DFSDMLPEN"]
    #[inline(always)]
    pub fn dfsdmlpen(&self) -> DFSDMLPEN_R {
        DFSDMLPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ADFSDMLPEN"]
    #[inline(always)]
    pub fn adfsdmlpen(&self) -> ADFSDMLPEN_R {
        ADFSDMLPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - FDCANLPEN"]
    #[inline(always)]
    pub fn fdcanlpen(&self) -> FDCANLPEN_R {
        FDCANLPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn tim1lpen(&mut self) -> TIM1LPEN_W<RCC_MP_APB2LPENCLRRrs> {
        TIM1LPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM8LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn tim8lpen(&mut self) -> TIM8LPEN_W<RCC_MP_APB2LPENCLRRrs> {
        TIM8LPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM15LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn tim15lpen(&mut self) -> TIM15LPEN_W<RCC_MP_APB2LPENCLRRrs> {
        TIM15LPEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM16LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn tim16lpen(&mut self) -> TIM16LPEN_W<RCC_MP_APB2LPENCLRRrs> {
        TIM16LPEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM17LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn tim17lpen(&mut self) -> TIM17LPEN_W<RCC_MP_APB2LPENCLRRrs> {
        TIM17LPEN_W::new(self, 4)
    }
    #[doc = "Bit 8 - SPI1LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<RCC_MP_APB2LPENCLRRrs> {
        SPI1LPEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - SPI4LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn spi4lpen(&mut self) -> SPI4LPEN_W<RCC_MP_APB2LPENCLRRrs> {
        SPI4LPEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - SPI5LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn spi5lpen(&mut self) -> SPI5LPEN_W<RCC_MP_APB2LPENCLRRrs> {
        SPI5LPEN_W::new(self, 10)
    }
    #[doc = "Bit 13 - USART6LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W<RCC_MP_APB2LPENCLRRrs> {
        USART6LPEN_W::new(self, 13)
    }
    #[doc = "Bit 16 - SAI1LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn sai1lpen(&mut self) -> SAI1LPEN_W<RCC_MP_APB2LPENCLRRrs> {
        SAI1LPEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - SAI2LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn sai2lpen(&mut self) -> SAI2LPEN_W<RCC_MP_APB2LPENCLRRrs> {
        SAI2LPEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - SAI3LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn sai3lpen(&mut self) -> SAI3LPEN_W<RCC_MP_APB2LPENCLRRrs> {
        SAI3LPEN_W::new(self, 18)
    }
    #[doc = "Bit 20 - DFSDMLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdmlpen(&mut self) -> DFSDMLPEN_W<RCC_MP_APB2LPENCLRRrs> {
        DFSDMLPEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - ADFSDMLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn adfsdmlpen(&mut self) -> ADFSDMLPEN_W<RCC_MP_APB2LPENCLRRrs> {
        ADFSDMLPEN_W::new(self, 21)
    }
    #[doc = "Bit 24 - FDCANLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn fdcanlpen(&mut self) -> FDCANLPEN_W<RCC_MP_APB2LPENCLRRrs> {
        FDCANLPEN_W::new(self, 24)
    }
}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb2lpenclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb2lpenclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_APB2LPENCLRRrs;
impl crate::RegisterSpec for RCC_MP_APB2LPENCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_apb2lpenclrr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_APB2LPENCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mp_apb2lpenclrr::W`](W) writer structure"]
impl crate::Writable for RCC_MP_APB2LPENCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MP_APB2LPENCLRR to value 0x0137_271f"]
impl crate::Resettable for RCC_MP_APB2LPENCLRRrs {
    const RESET_VALUE: u32 = 0x0137_271f;
}
