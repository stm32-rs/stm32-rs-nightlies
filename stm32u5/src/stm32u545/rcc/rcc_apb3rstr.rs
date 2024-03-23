#[doc = "Register `RCC_APB3RSTR` reader"]
pub type R = crate::R<RCC_APB3RSTRrs>;
#[doc = "Register `RCC_APB3RSTR` writer"]
pub type W = crate::W<RCC_APB3RSTRrs>;
#[doc = "Field `SYSCFGRST` reader - SYSCFG reset This bit is set and cleared by software."]
pub type SYSCFGRST_R = crate::BitReader;
#[doc = "Field `SYSCFGRST` writer - SYSCFG reset This bit is set and cleared by software."]
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3RST` reader - SPI3 reset This bit is set and cleared by software."]
pub type SPI3RST_R = crate::BitReader;
#[doc = "Field `SPI3RST` writer - SPI3 reset This bit is set and cleared by software."]
pub type SPI3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART1RST` reader - LPUART1 reset This bit is set and cleared by software."]
pub type LPUART1RST_R = crate::BitReader;
#[doc = "Field `LPUART1RST` writer - LPUART1 reset This bit is set and cleared by software."]
pub type LPUART1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3RST` reader - I2C3 reset This bit is set and cleared by software."]
pub type I2C3RST_R = crate::BitReader;
#[doc = "Field `I2C3RST` writer - I2C3 reset This bit is set and cleared by software."]
pub type I2C3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1RST` reader - LPTIM1 reset This bit is set and cleared by software."]
pub type LPTIM1RST_R = crate::BitReader;
#[doc = "Field `LPTIM1RST` writer - LPTIM1 reset This bit is set and cleared by software."]
pub type LPTIM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM3RST` reader - LPTIM3 reset This bit is set and cleared by software."]
pub type LPTIM3RST_R = crate::BitReader;
#[doc = "Field `LPTIM3RST` writer - LPTIM3 reset This bit is set and cleared by software."]
pub type LPTIM3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM4RST` reader - LPTIM4 reset This bit is set and cleared by software."]
pub type LPTIM4RST_R = crate::BitReader;
#[doc = "Field `LPTIM4RST` writer - LPTIM4 reset This bit is set and cleared by software."]
pub type LPTIM4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMPRST` reader - OPAMP reset This bit is set and cleared by software."]
pub type OPAMPRST_R = crate::BitReader;
#[doc = "Field `OPAMPRST` writer - OPAMP reset This bit is set and cleared by software."]
pub type OPAMPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPRST` reader - COMP reset This bit is set and cleared by software."]
pub type COMPRST_R = crate::BitReader;
#[doc = "Field `COMPRST` writer - COMP reset This bit is set and cleared by software."]
pub type COMPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFRST` reader - VREFBUF reset This bit is set and cleared by software."]
pub type VREFRST_R = crate::BitReader;
#[doc = "Field `VREFRST` writer - VREFBUF reset This bit is set and cleared by software."]
pub type VREFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - SYSCFG reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI3 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LPUART1 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C3 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM1 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM3 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LPTIM4 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn lptim4rst(&self) -> LPTIM4RST_R {
        LPTIM4RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OPAMP reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn opamprst(&self) -> OPAMPRST_R {
        OPAMPRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - COMP reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn comprst(&self) -> COMPRST_R {
        COMPRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - VREFBUF reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SYSCFG reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<RCC_APB3RSTRrs> {
        SYSCFGRST_W::new(self, 1)
    }
    #[doc = "Bit 5 - SPI3 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi3rst(&mut self) -> SPI3RST_W<RCC_APB3RSTRrs> {
        SPI3RST_W::new(self, 5)
    }
    #[doc = "Bit 6 - LPUART1 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<RCC_APB3RSTRrs> {
        LPUART1RST_W::new(self, 6)
    }
    #[doc = "Bit 7 - I2C3 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<RCC_APB3RSTRrs> {
        I2C3RST_W::new(self, 7)
    }
    #[doc = "Bit 11 - LPTIM1 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<RCC_APB3RSTRrs> {
        LPTIM1RST_W::new(self, 11)
    }
    #[doc = "Bit 12 - LPTIM3 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W<RCC_APB3RSTRrs> {
        LPTIM3RST_W::new(self, 12)
    }
    #[doc = "Bit 13 - LPTIM4 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim4rst(&mut self) -> LPTIM4RST_W<RCC_APB3RSTRrs> {
        LPTIM4RST_W::new(self, 13)
    }
    #[doc = "Bit 14 - OPAMP reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn opamprst(&mut self) -> OPAMPRST_W<RCC_APB3RSTRrs> {
        OPAMPRST_W::new(self, 14)
    }
    #[doc = "Bit 15 - COMP reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn comprst(&mut self) -> COMPRST_W<RCC_APB3RSTRrs> {
        COMPRST_W::new(self, 15)
    }
    #[doc = "Bit 20 - VREFBUF reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn vrefrst(&mut self) -> VREFRST_W<RCC_APB3RSTRrs> {
        VREFRST_W::new(self, 20)
    }
}
#[doc = "RCC APB3 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb3rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb3rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_APB3RSTRrs;
impl crate::RegisterSpec for RCC_APB3RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb3rstr::R`](R) reader structure"]
impl crate::Readable for RCC_APB3RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb3rstr::W`](W) writer structure"]
impl crate::Writable for RCC_APB3RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB3RSTR to value 0"]
impl crate::Resettable for RCC_APB3RSTRrs {
    const RESET_VALUE: u32 = 0;
}
