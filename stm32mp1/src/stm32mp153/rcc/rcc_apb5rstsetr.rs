#[doc = "Register `RCC_APB5RSTSETR` reader"]
pub type R = crate::R<RCC_APB5RSTSETRrs>;
#[doc = "Register `RCC_APB5RSTSETR` writer"]
pub type W = crate::W<RCC_APB5RSTSETRrs>;
#[doc = "Field `SPI6RST` reader - SPI6RST"]
pub type SPI6RST_R = crate::BitReader;
#[doc = "Field `SPI6RST` writer - SPI6RST"]
pub type SPI6RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4RST` reader - I2C4RST"]
pub type I2C4RST_R = crate::BitReader;
#[doc = "Field `I2C4RST` writer - I2C4RST"]
pub type I2C4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C6RST` reader - I2C6RST"]
pub type I2C6RST_R = crate::BitReader;
#[doc = "Field `I2C6RST` writer - I2C6RST"]
pub type I2C6RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1RST` reader - USART1RST"]
pub type USART1RST_R = crate::BitReader;
#[doc = "Field `USART1RST` writer - USART1RST"]
pub type USART1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STGENRST` reader - STGENRST"]
pub type STGENRST_R = crate::BitReader;
#[doc = "Field `STGENRST` writer - STGENRST"]
pub type STGENRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI6RST"]
    #[inline(always)]
    pub fn spi6rst(&self) -> SPI6RST_R {
        SPI6RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - I2C4RST"]
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C6RST"]
    #[inline(always)]
    pub fn i2c6rst(&self) -> I2C6RST_R {
        I2C6RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1RST"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 20 - STGENRST"]
    #[inline(always)]
    pub fn stgenrst(&self) -> STGENRST_R {
        STGENRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI6RST"]
    #[inline(always)]
    #[must_use]
    pub fn spi6rst(&mut self) -> SPI6RST_W<RCC_APB5RSTSETRrs> {
        SPI6RST_W::new(self, 0)
    }
    #[doc = "Bit 2 - I2C4RST"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4rst(&mut self) -> I2C4RST_W<RCC_APB5RSTSETRrs> {
        I2C4RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - I2C6RST"]
    #[inline(always)]
    #[must_use]
    pub fn i2c6rst(&mut self) -> I2C6RST_W<RCC_APB5RSTSETRrs> {
        I2C6RST_W::new(self, 3)
    }
    #[doc = "Bit 4 - USART1RST"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<RCC_APB5RSTSETRrs> {
        USART1RST_W::new(self, 4)
    }
    #[doc = "Bit 20 - STGENRST"]
    #[inline(always)]
    #[must_use]
    pub fn stgenrst(&mut self) -> STGENRST_W<RCC_APB5RSTSETRrs> {
        STGENRST_W::new(self, 20)
    }
}
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb5rstsetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb5rstsetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_APB5RSTSETRrs;
impl crate::RegisterSpec for RCC_APB5RSTSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb5rstsetr::R`](R) reader structure"]
impl crate::Readable for RCC_APB5RSTSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb5rstsetr::W`](W) writer structure"]
impl crate::Writable for RCC_APB5RSTSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB5RSTSETR to value 0"]
impl crate::Resettable for RCC_APB5RSTSETRrs {
    const RESET_VALUE: u32 = 0;
}
