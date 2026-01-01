///Register `APB5RSTCLRR` reader
pub type R = crate::R<APB5RSTCLRRrs>;
///Register `APB5RSTCLRR` writer
pub type W = crate::W<APB5RSTCLRRrs>;
///Field `SPI6RST` reader - SPI6RST
pub type SPI6RST_R = crate::BitReader;
///Field `SPI6RST` writer - SPI6RST
pub type SPI6RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4RST` reader - I2C4RST
pub type I2C4RST_R = crate::BitReader;
///Field `I2C4RST` writer - I2C4RST
pub type I2C4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C6RST` reader - I2C6RST
pub type I2C6RST_R = crate::BitReader;
///Field `I2C6RST` writer - I2C6RST
pub type I2C6RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1RST` reader - USART1RST
pub type USART1RST_R = crate::BitReader;
///Field `USART1RST` writer - USART1RST
pub type USART1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STGENRST` reader - STGENRST
pub type STGENRST_R = crate::BitReader;
///Field `STGENRST` writer - STGENRST
pub type STGENRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SPI6RST
    #[inline(always)]
    pub fn spi6rst(&self) -> SPI6RST_R {
        SPI6RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - I2C4RST
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I2C6RST
    #[inline(always)]
    pub fn i2c6rst(&self) -> I2C6RST_R {
        I2C6RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - USART1RST
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 20 - STGENRST
    #[inline(always)]
    pub fn stgenrst(&self) -> STGENRST_R {
        STGENRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB5RSTCLRR")
            .field("spi6rst", &self.spi6rst())
            .field("i2c4rst", &self.i2c4rst())
            .field("i2c6rst", &self.i2c6rst())
            .field("usart1rst", &self.usart1rst())
            .field("stgenrst", &self.stgenrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - SPI6RST
    #[inline(always)]
    pub fn spi6rst(&mut self) -> SPI6RST_W<'_, APB5RSTCLRRrs> {
        SPI6RST_W::new(self, 0)
    }
    ///Bit 2 - I2C4RST
    #[inline(always)]
    pub fn i2c4rst(&mut self) -> I2C4RST_W<'_, APB5RSTCLRRrs> {
        I2C4RST_W::new(self, 2)
    }
    ///Bit 3 - I2C6RST
    #[inline(always)]
    pub fn i2c6rst(&mut self) -> I2C6RST_W<'_, APB5RSTCLRRrs> {
        I2C6RST_W::new(self, 3)
    }
    ///Bit 4 - USART1RST
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W<'_, APB5RSTCLRRrs> {
        USART1RST_W::new(self, 4)
    }
    ///Bit 20 - STGENRST
    #[inline(always)]
    pub fn stgenrst(&mut self) -> STGENRST_W<'_, APB5RSTCLRRrs> {
        STGENRST_W::new(self, 20)
    }
}
/**This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`apb5rstclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5rstclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB5RSTCLRR)*/
pub struct APB5RSTCLRRrs;
impl crate::RegisterSpec for APB5RSTCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`apb5rstclrr::R`](R) reader structure
impl crate::Readable for APB5RSTCLRRrs {}
///`write(|w| ..)` method takes [`apb5rstclrr::W`](W) writer structure
impl crate::Writable for APB5RSTCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB5RSTCLRR to value 0
impl crate::Resettable for APB5RSTCLRRrs {}
