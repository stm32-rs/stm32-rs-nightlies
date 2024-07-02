///Register `RCC_APB3RSTR` reader
pub type R = crate::R<RCC_APB3RSTRrs>;
///Register `RCC_APB3RSTR` writer
pub type W = crate::W<RCC_APB3RSTRrs>;
///Field `SYSCFGRST` reader - SYSCFG reset This bit is set and cleared by software.
pub type SYSCFGRST_R = crate::BitReader;
///Field `SYSCFGRST` writer - SYSCFG reset This bit is set and cleared by software.
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3RST` reader - SPI3 reset This bit is set and cleared by software.
pub type SPI3RST_R = crate::BitReader;
///Field `SPI3RST` writer - SPI3 reset This bit is set and cleared by software.
pub type SPI3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1RST` reader - LPUART1 reset This bit is set and cleared by software.
pub type LPUART1RST_R = crate::BitReader;
///Field `LPUART1RST` writer - LPUART1 reset This bit is set and cleared by software.
pub type LPUART1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3RST` reader - I2C3 reset This bit is set and cleared by software.
pub type I2C3RST_R = crate::BitReader;
///Field `I2C3RST` writer - I2C3 reset This bit is set and cleared by software.
pub type I2C3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1RST` reader - LPTIM1 reset This bit is set and cleared by software.
pub type LPTIM1RST_R = crate::BitReader;
///Field `LPTIM1RST` writer - LPTIM1 reset This bit is set and cleared by software.
pub type LPTIM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3RST` reader - LPTIM3 reset This bit is set and cleared by software.
pub type LPTIM3RST_R = crate::BitReader;
///Field `LPTIM3RST` writer - LPTIM3 reset This bit is set and cleared by software.
pub type LPTIM3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM4RST` reader - LPTIM4 reset This bit is set and cleared by software.
pub type LPTIM4RST_R = crate::BitReader;
///Field `LPTIM4RST` writer - LPTIM4 reset This bit is set and cleared by software.
pub type LPTIM4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPAMPRST` reader - OPAMP reset This bit is set and cleared by software.
pub type OPAMPRST_R = crate::BitReader;
///Field `OPAMPRST` writer - OPAMP reset This bit is set and cleared by software.
pub type OPAMPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMPRST` reader - COMP reset This bit is set and cleared by software.
pub type COMPRST_R = crate::BitReader;
///Field `COMPRST` writer - COMP reset This bit is set and cleared by software.
pub type COMPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFRST` reader - VREFBUF reset This bit is set and cleared by software.
pub type VREFRST_R = crate::BitReader;
///Field `VREFRST` writer - VREFBUF reset This bit is set and cleared by software.
pub type VREFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - SYSCFG reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - SPI3 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LPUART1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C3 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - LPTIM1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM3 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - LPTIM4 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim4rst(&self) -> LPTIM4RST_R {
        LPTIM4RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - OPAMP reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn opamprst(&self) -> OPAMPRST_R {
        OPAMPRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - COMP reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn comprst(&self) -> COMPRST_R {
        COMPRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - VREFBUF reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB3RSTR")
            .field("syscfgrst", &self.syscfgrst())
            .field("spi3rst", &self.spi3rst())
            .field("lpuart1rst", &self.lpuart1rst())
            .field("i2c3rst", &self.i2c3rst())
            .field("lptim1rst", &self.lptim1rst())
            .field("lptim3rst", &self.lptim3rst())
            .field("lptim4rst", &self.lptim4rst())
            .field("opamprst", &self.opamprst())
            .field("comprst", &self.comprst())
            .field("vrefrst", &self.vrefrst())
            .finish()
    }
}
impl W {
    ///Bit 1 - SYSCFG reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<RCC_APB3RSTRrs> {
        SYSCFGRST_W::new(self, 1)
    }
    ///Bit 5 - SPI3 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn spi3rst(&mut self) -> SPI3RST_W<RCC_APB3RSTRrs> {
        SPI3RST_W::new(self, 5)
    }
    ///Bit 6 - LPUART1 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<RCC_APB3RSTRrs> {
        LPUART1RST_W::new(self, 6)
    }
    ///Bit 7 - I2C3 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<RCC_APB3RSTRrs> {
        I2C3RST_W::new(self, 7)
    }
    ///Bit 11 - LPTIM1 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<RCC_APB3RSTRrs> {
        LPTIM1RST_W::new(self, 11)
    }
    ///Bit 12 - LPTIM3 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W<RCC_APB3RSTRrs> {
        LPTIM3RST_W::new(self, 12)
    }
    ///Bit 13 - LPTIM4 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lptim4rst(&mut self) -> LPTIM4RST_W<RCC_APB3RSTRrs> {
        LPTIM4RST_W::new(self, 13)
    }
    ///Bit 14 - OPAMP reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn opamprst(&mut self) -> OPAMPRST_W<RCC_APB3RSTRrs> {
        OPAMPRST_W::new(self, 14)
    }
    ///Bit 15 - COMP reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn comprst(&mut self) -> COMPRST_W<RCC_APB3RSTRrs> {
        COMPRST_W::new(self, 15)
    }
    ///Bit 20 - VREFBUF reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn vrefrst(&mut self) -> VREFRST_W<RCC_APB3RSTRrs> {
        VREFRST_W::new(self, 20)
    }
}
/**RCC APB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`rcc_apb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RCC:RCC_APB3RSTR)*/
pub struct RCC_APB3RSTRrs;
impl crate::RegisterSpec for RCC_APB3RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_apb3rstr::R`](R) reader structure
impl crate::Readable for RCC_APB3RSTRrs {}
///`write(|w| ..)` method takes [`rcc_apb3rstr::W`](W) writer structure
impl crate::Writable for RCC_APB3RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_APB3RSTR to value 0
impl crate::Resettable for RCC_APB3RSTRrs {
    const RESET_VALUE: u32 = 0;
}
