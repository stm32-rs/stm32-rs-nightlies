///Register `APB4LRSTR` reader
pub type R = crate::R<APB4LRSTRrs>;
///Register `APB4LRSTR` writer
pub type W = crate::W<APB4LRSTRrs>;
///Field `HDPRST` reader - HDP reset
pub type HDPRST_R = crate::BitReader;
///Field `HDPRST` writer - HDP reset
pub type HDPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1RST` reader - LPUART1 reset
pub type LPUART1RST_R = crate::BitReader;
///Field `LPUART1RST` writer - LPUART1 reset
pub type LPUART1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI6RST` reader - SPI6 reset
pub type SPI6RST_R = crate::BitReader;
///Field `SPI6RST` writer - SPI6 reset
pub type SPI6RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4RST` reader - I2C4 reset
pub type I2C4RST_R = crate::BitReader;
///Field `I2C4RST` writer - I2C4 reset
pub type I2C4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2RST` reader - LPTIM2 reset
pub type LPTIM2RST_R = crate::BitReader;
///Field `LPTIM2RST` writer - LPTIM2 reset
pub type LPTIM2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3RST` reader - LPTIM3 reset
pub type LPTIM3RST_R = crate::BitReader;
///Field `LPTIM3RST` writer - LPTIM3 reset
pub type LPTIM3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM4RST` reader - LPTIM4 reset
pub type LPTIM4RST_R = crate::BitReader;
///Field `LPTIM4RST` writer - LPTIM4 reset
pub type LPTIM4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM5RST` reader - LPTIM5 reset
pub type LPTIM5RST_R = crate::BitReader;
///Field `LPTIM5RST` writer - LPTIM5 reset
pub type LPTIM5RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFBUFRST` reader - VREFBUF reset
pub type VREFBUFRST_R = crate::BitReader;
///Field `VREFBUFRST` writer - VREFBUF reset
pub type VREFBUFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCRST` reader - RTC reset
pub type RTCRST_R = crate::BitReader;
///Field `RTCRST` writer - RTC reset
pub type RTCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R2GRETRST` reader - R2GRET reset
pub type R2GRETRST_R = crate::BitReader;
///Field `R2GRETRST` writer - R2GRET reset
pub type R2GRETRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R2GNPURST` reader - R2GNPU reset
pub type R2GNPURST_R = crate::BitReader;
///Field `R2GNPURST` writer - R2GNPU reset
pub type R2GNPURST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SERFRST` reader - SERF reset
pub type SERFRST_R = crate::BitReader;
///Field `SERFRST` writer - SERF reset
pub type SERFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - HDP reset
    #[inline(always)]
    pub fn hdprst(&self) -> HDPRST_R {
        HDPRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LPUART1 reset
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - SPI6 reset
    #[inline(always)]
    pub fn spi6rst(&self) -> SPI6RST_R {
        SPI6RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - I2C4 reset
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - LPTIM2 reset
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - LPTIM3 reset
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LPTIM4 reset
    #[inline(always)]
    pub fn lptim4rst(&self) -> LPTIM4RST_R {
        LPTIM4RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM5 reset
    #[inline(always)]
    pub fn lptim5rst(&self) -> LPTIM5RST_R {
        LPTIM5RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - VREFBUF reset
    #[inline(always)]
    pub fn vrefbufrst(&self) -> VREFBUFRST_R {
        VREFBUFRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - RTC reset
    #[inline(always)]
    pub fn rtcrst(&self) -> RTCRST_R {
        RTCRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 22 - R2GRET reset
    #[inline(always)]
    pub fn r2gretrst(&self) -> R2GRETRST_R {
        R2GRETRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - R2GNPU reset
    #[inline(always)]
    pub fn r2gnpurst(&self) -> R2GNPURST_R {
        R2GNPURST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - SERF reset
    #[inline(always)]
    pub fn serfrst(&self) -> SERFRST_R {
        SERFRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB4LRSTR")
            .field("hdprst", &self.hdprst())
            .field("lpuart1rst", &self.lpuart1rst())
            .field("spi6rst", &self.spi6rst())
            .field("i2c4rst", &self.i2c4rst())
            .field("lptim2rst", &self.lptim2rst())
            .field("lptim3rst", &self.lptim3rst())
            .field("lptim4rst", &self.lptim4rst())
            .field("lptim5rst", &self.lptim5rst())
            .field("vrefbufrst", &self.vrefbufrst())
            .field("rtcrst", &self.rtcrst())
            .field("r2gretrst", &self.r2gretrst())
            .field("r2gnpurst", &self.r2gnpurst())
            .field("serfrst", &self.serfrst())
            .finish()
    }
}
impl W {
    ///Bit 2 - HDP reset
    #[inline(always)]
    pub fn hdprst(&mut self) -> HDPRST_W<'_, APB4LRSTRrs> {
        HDPRST_W::new(self, 2)
    }
    ///Bit 3 - LPUART1 reset
    #[inline(always)]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<'_, APB4LRSTRrs> {
        LPUART1RST_W::new(self, 3)
    }
    ///Bit 5 - SPI6 reset
    #[inline(always)]
    pub fn spi6rst(&mut self) -> SPI6RST_W<'_, APB4LRSTRrs> {
        SPI6RST_W::new(self, 5)
    }
    ///Bit 7 - I2C4 reset
    #[inline(always)]
    pub fn i2c4rst(&mut self) -> I2C4RST_W<'_, APB4LRSTRrs> {
        I2C4RST_W::new(self, 7)
    }
    ///Bit 9 - LPTIM2 reset
    #[inline(always)]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<'_, APB4LRSTRrs> {
        LPTIM2RST_W::new(self, 9)
    }
    ///Bit 10 - LPTIM3 reset
    #[inline(always)]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W<'_, APB4LRSTRrs> {
        LPTIM3RST_W::new(self, 10)
    }
    ///Bit 11 - LPTIM4 reset
    #[inline(always)]
    pub fn lptim4rst(&mut self) -> LPTIM4RST_W<'_, APB4LRSTRrs> {
        LPTIM4RST_W::new(self, 11)
    }
    ///Bit 12 - LPTIM5 reset
    #[inline(always)]
    pub fn lptim5rst(&mut self) -> LPTIM5RST_W<'_, APB4LRSTRrs> {
        LPTIM5RST_W::new(self, 12)
    }
    ///Bit 15 - VREFBUF reset
    #[inline(always)]
    pub fn vrefbufrst(&mut self) -> VREFBUFRST_W<'_, APB4LRSTRrs> {
        VREFBUFRST_W::new(self, 15)
    }
    ///Bit 16 - RTC reset
    #[inline(always)]
    pub fn rtcrst(&mut self) -> RTCRST_W<'_, APB4LRSTRrs> {
        RTCRST_W::new(self, 16)
    }
    ///Bit 22 - R2GRET reset
    #[inline(always)]
    pub fn r2gretrst(&mut self) -> R2GRETRST_W<'_, APB4LRSTRrs> {
        R2GRETRST_W::new(self, 22)
    }
    ///Bit 23 - R2GNPU reset
    #[inline(always)]
    pub fn r2gnpurst(&mut self) -> R2GNPURST_W<'_, APB4LRSTRrs> {
        R2GNPURST_W::new(self, 23)
    }
    ///Bit 31 - SERF reset
    #[inline(always)]
    pub fn serfrst(&mut self) -> SERFRST_W<'_, APB4LRSTRrs> {
        SERFRST_W::new(self, 31)
    }
}
/**RCC APB4L reset register

You can [`read`](crate::Reg::read) this register and get [`apb4lrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4lrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:APB4LRSTR)*/
pub struct APB4LRSTRrs;
impl crate::RegisterSpec for APB4LRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb4lrstr::R`](R) reader structure
impl crate::Readable for APB4LRSTRrs {}
///`write(|w| ..)` method takes [`apb4lrstr::W`](W) writer structure
impl crate::Writable for APB4LRSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4LRSTR to value 0
impl crate::Resettable for APB4LRSTRrs {}
