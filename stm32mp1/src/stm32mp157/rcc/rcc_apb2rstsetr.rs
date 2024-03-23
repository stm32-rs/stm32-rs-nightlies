#[doc = "Register `RCC_APB2RSTSETR` reader"]
pub type R = crate::R<RCC_APB2RSTSETRrs>;
#[doc = "Register `RCC_APB2RSTSETR` writer"]
pub type W = crate::W<RCC_APB2RSTSETRrs>;
#[doc = "Field `TIM1RST` reader - TIM1RST"]
pub type TIM1RST_R = crate::BitReader;
#[doc = "Field `TIM1RST` writer - TIM1RST"]
pub type TIM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8RST` reader - TIM8RST"]
pub type TIM8RST_R = crate::BitReader;
#[doc = "Field `TIM8RST` writer - TIM8RST"]
pub type TIM8RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15RST` reader - TIM15RST"]
pub type TIM15RST_R = crate::BitReader;
#[doc = "Field `TIM15RST` writer - TIM15RST"]
pub type TIM15RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16RST` reader - TIM16RST"]
pub type TIM16RST_R = crate::BitReader;
#[doc = "Field `TIM16RST` writer - TIM16RST"]
pub type TIM16RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17RST` reader - TIM17RST"]
pub type TIM17RST_R = crate::BitReader;
#[doc = "Field `TIM17RST` writer - TIM17RST"]
pub type TIM17RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1RST` reader - SPI1RST"]
pub type SPI1RST_R = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1RST"]
pub type SPI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4RST` reader - SPI4RST"]
pub type SPI4RST_R = crate::BitReader;
#[doc = "Field `SPI4RST` writer - SPI4RST"]
pub type SPI4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI5RST` reader - SPI5RST"]
pub type SPI5RST_R = crate::BitReader;
#[doc = "Field `SPI5RST` writer - SPI5RST"]
pub type SPI5RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6RST` reader - USART6RST"]
pub type USART6RST_R = crate::BitReader;
#[doc = "Field `USART6RST` writer - USART6RST"]
pub type USART6RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1RST` reader - SAI1RST"]
pub type SAI1RST_R = crate::BitReader;
#[doc = "Field `SAI1RST` writer - SAI1RST"]
pub type SAI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI2RST` reader - SAI2RST"]
pub type SAI2RST_R = crate::BitReader;
#[doc = "Field `SAI2RST` writer - SAI2RST"]
pub type SAI2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI3RST` reader - SAI3RST"]
pub type SAI3RST_R = crate::BitReader;
#[doc = "Field `SAI3RST` writer - SAI3RST"]
pub type SAI3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFSDMRST` reader - DFSDMRST"]
pub type DFSDMRST_R = crate::BitReader;
#[doc = "Field `DFSDMRST` writer - DFSDMRST"]
pub type DFSDMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDCANRST` reader - FDCANRST"]
pub type FDCANRST_R = crate::BitReader;
#[doc = "Field `FDCANRST` writer - FDCANRST"]
pub type FDCANRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM1RST"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8RST"]
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM15RST"]
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM16RST"]
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM17RST"]
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI4RST"]
    #[inline(always)]
    pub fn spi4rst(&self) -> SPI4RST_R {
        SPI4RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI5RST"]
    #[inline(always)]
    pub fn spi5rst(&self) -> SPI5RST_R {
        SPI5RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - USART6RST"]
    #[inline(always)]
    pub fn usart6rst(&self) -> USART6RST_R {
        USART6RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - SAI1RST"]
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SAI2RST"]
    #[inline(always)]
    pub fn sai2rst(&self) -> SAI2RST_R {
        SAI2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SAI3RST"]
    #[inline(always)]
    pub fn sai3rst(&self) -> SAI3RST_R {
        SAI3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - DFSDMRST"]
    #[inline(always)]
    pub fn dfsdmrst(&self) -> DFSDMRST_R {
        DFSDMRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - FDCANRST"]
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<RCC_APB2RSTSETRrs> {
        TIM1RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM8RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim8rst(&mut self) -> TIM8RST_W<RCC_APB2RSTSETRrs> {
        TIM8RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM15RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim15rst(&mut self) -> TIM15RST_W<RCC_APB2RSTSETRrs> {
        TIM15RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM16RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim16rst(&mut self) -> TIM16RST_W<RCC_APB2RSTSETRrs> {
        TIM16RST_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM17RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim17rst(&mut self) -> TIM17RST_W<RCC_APB2RSTSETRrs> {
        TIM17RST_W::new(self, 4)
    }
    #[doc = "Bit 8 - SPI1RST"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<RCC_APB2RSTSETRrs> {
        SPI1RST_W::new(self, 8)
    }
    #[doc = "Bit 9 - SPI4RST"]
    #[inline(always)]
    #[must_use]
    pub fn spi4rst(&mut self) -> SPI4RST_W<RCC_APB2RSTSETRrs> {
        SPI4RST_W::new(self, 9)
    }
    #[doc = "Bit 10 - SPI5RST"]
    #[inline(always)]
    #[must_use]
    pub fn spi5rst(&mut self) -> SPI5RST_W<RCC_APB2RSTSETRrs> {
        SPI5RST_W::new(self, 10)
    }
    #[doc = "Bit 13 - USART6RST"]
    #[inline(always)]
    #[must_use]
    pub fn usart6rst(&mut self) -> USART6RST_W<RCC_APB2RSTSETRrs> {
        USART6RST_W::new(self, 13)
    }
    #[doc = "Bit 16 - SAI1RST"]
    #[inline(always)]
    #[must_use]
    pub fn sai1rst(&mut self) -> SAI1RST_W<RCC_APB2RSTSETRrs> {
        SAI1RST_W::new(self, 16)
    }
    #[doc = "Bit 17 - SAI2RST"]
    #[inline(always)]
    #[must_use]
    pub fn sai2rst(&mut self) -> SAI2RST_W<RCC_APB2RSTSETRrs> {
        SAI2RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - SAI3RST"]
    #[inline(always)]
    #[must_use]
    pub fn sai3rst(&mut self) -> SAI3RST_W<RCC_APB2RSTSETRrs> {
        SAI3RST_W::new(self, 18)
    }
    #[doc = "Bit 20 - DFSDMRST"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdmrst(&mut self) -> DFSDMRST_W<RCC_APB2RSTSETRrs> {
        DFSDMRST_W::new(self, 20)
    }
    #[doc = "Bit 24 - FDCANRST"]
    #[inline(always)]
    #[must_use]
    pub fn fdcanrst(&mut self) -> FDCANRST_W<RCC_APB2RSTSETRrs> {
        FDCANRST_W::new(self, 24)
    }
}
#[doc = "This register is used to activate the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb2rstsetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb2rstsetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_APB2RSTSETRrs;
impl crate::RegisterSpec for RCC_APB2RSTSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb2rstsetr::R`](R) reader structure"]
impl crate::Readable for RCC_APB2RSTSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb2rstsetr::W`](W) writer structure"]
impl crate::Writable for RCC_APB2RSTSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB2RSTSETR to value 0"]
impl crate::Resettable for RCC_APB2RSTSETRrs {
    const RESET_VALUE: u32 = 0;
}
