///Register `APB1RSTR1` reader
pub type R = crate::R<APB1RSTR1rs>;
///Register `APB1RSTR1` writer
pub type W = crate::W<APB1RSTR1rs>;
///Field `TIM2RST` reader - TIM2 timer reset
pub type TIM2RST_R = crate::BitReader;
///Field `TIM2RST` writer - TIM2 timer reset
pub type TIM2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCDRST` reader - LCD interface reset
pub type LCDRST_R = crate::BitReader;
///Field `LCDRST` writer - LCD interface reset
pub type LCDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2RST` reader - SPI2 reset
pub type SPI2RST_R = crate::BitReader;
///Field `SPI2RST` writer - SPI2 reset
pub type SPI2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1RST` reader - I2C1 reset
pub type I2C1RST_R = crate::BitReader;
///Field `I2C1RST` writer - I2C1 reset
pub type I2C1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3RST` reader - I2C3 reset
pub type I2C3RST_R = crate::BitReader;
///Field `I2C3RST` writer - I2C3 reset
pub type I2C3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRSRST` reader - CRS reset
pub type CRSRST_R = crate::BitReader;
///Field `CRSRST` writer - CRS reset
pub type CRSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBFSRST` reader - USB FS reset
pub type USBFSRST_R = crate::BitReader;
///Field `USBFSRST` writer - USB FS reset
pub type USBFSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1RST` reader - Low Power Timer 1 reset
pub type LPTIM1RST_R = crate::BitReader;
///Field `LPTIM1RST` writer - Low Power Timer 1 reset
pub type LPTIM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2 timer reset
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 9 - LCD interface reset
    #[inline(always)]
    pub fn lcdrst(&self) -> LCDRST_R {
        LCDRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 14 - SPI2 reset
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 21 - I2C1 reset
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - I2C3 reset
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CRS reset
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - USB FS reset
    #[inline(always)]
    pub fn usbfsrst(&self) -> USBFSRST_R {
        USBFSRST_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 31 - Low Power Timer 1 reset
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1RSTR1")
            .field("lptim1rst", &self.lptim1rst())
            .field("usbfsrst", &self.usbfsrst())
            .field("crsrst", &self.crsrst())
            .field("i2c3rst", &self.i2c3rst())
            .field("i2c1rst", &self.i2c1rst())
            .field("spi2rst", &self.spi2rst())
            .field("lcdrst", &self.lcdrst())
            .field("tim2rst", &self.tim2rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 timer reset
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<APB1RSTR1rs> {
        TIM2RST_W::new(self, 0)
    }
    ///Bit 9 - LCD interface reset
    #[inline(always)]
    #[must_use]
    pub fn lcdrst(&mut self) -> LCDRST_W<APB1RSTR1rs> {
        LCDRST_W::new(self, 9)
    }
    ///Bit 14 - SPI2 reset
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<APB1RSTR1rs> {
        SPI2RST_W::new(self, 14)
    }
    ///Bit 21 - I2C1 reset
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<APB1RSTR1rs> {
        I2C1RST_W::new(self, 21)
    }
    ///Bit 23 - I2C3 reset
    #[inline(always)]
    #[must_use]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<APB1RSTR1rs> {
        I2C3RST_W::new(self, 23)
    }
    ///Bit 24 - CRS reset
    #[inline(always)]
    #[must_use]
    pub fn crsrst(&mut self) -> CRSRST_W<APB1RSTR1rs> {
        CRSRST_W::new(self, 24)
    }
    ///Bit 26 - USB FS reset
    #[inline(always)]
    #[must_use]
    pub fn usbfsrst(&mut self) -> USBFSRST_W<APB1RSTR1rs> {
        USBFSRST_W::new(self, 26)
    }
    ///Bit 31 - Low Power Timer 1 reset
    #[inline(always)]
    #[must_use]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<APB1RSTR1rs> {
        LPTIM1RST_W::new(self, 31)
    }
}
/**APB1 peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`apb1rstr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:APB1RSTR1)*/
pub struct APB1RSTR1rs;
impl crate::RegisterSpec for APB1RSTR1rs {
    type Ux = u32;
}
///`read()` method returns [`apb1rstr1::R`](R) reader structure
impl crate::Readable for APB1RSTR1rs {}
///`write(|w| ..)` method takes [`apb1rstr1::W`](W) writer structure
impl crate::Writable for APB1RSTR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APB1RSTR1 to value 0
impl crate::Resettable for APB1RSTR1rs {
    const RESET_VALUE: u32 = 0;
}
