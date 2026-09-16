///Register `APB4LLPENR` reader
pub type R = crate::R<APB4LLPENRrs>;
///Register `APB4LLPENR` writer
pub type W = crate::W<APB4LLPENRrs>;
///Field `HDPLPEN` reader - HDP sleep enable
pub type HDPLPEN_R = crate::BitReader;
///Field `HDPLPEN` writer - HDP sleep enable
pub type HDPLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1LPEN` reader - LPUART1 sleep enable
pub type LPUART1LPEN_R = crate::BitReader;
///Field `LPUART1LPEN` writer - LPUART1 sleep enable
pub type LPUART1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI6LPEN` reader - SPI6 sleep enable
pub type SPI6LPEN_R = crate::BitReader;
///Field `SPI6LPEN` writer - SPI6 sleep enable
pub type SPI6LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4LPEN` reader - I2C4 sleep enable
pub type I2C4LPEN_R = crate::BitReader;
///Field `I2C4LPEN` writer - I2C4 sleep enable
pub type I2C4LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2LPEN` reader - LPTIM2 sleep enable
pub type LPTIM2LPEN_R = crate::BitReader;
///Field `LPTIM2LPEN` writer - LPTIM2 sleep enable
pub type LPTIM2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3LPEN` reader - LPTIM3 sleep enable
pub type LPTIM3LPEN_R = crate::BitReader;
///Field `LPTIM3LPEN` writer - LPTIM3 sleep enable
pub type LPTIM3LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM4LPEN` reader - LPTIM4 sleep enable
pub type LPTIM4LPEN_R = crate::BitReader;
///Field `LPTIM4LPEN` writer - LPTIM4 sleep enable
pub type LPTIM4LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM5LPEN` reader - LPTIM5 sleep enable
pub type LPTIM5LPEN_R = crate::BitReader;
///Field `LPTIM5LPEN` writer - LPTIM5 sleep enable
pub type LPTIM5LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFBUFLPEN` reader - VREFBUF sleep enable
pub type VREFBUFLPEN_R = crate::BitReader;
///Field `VREFBUFLPEN` writer - VREFBUF sleep enable
pub type VREFBUFLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCLPEN` reader - RTC sleep enable
pub type RTCLPEN_R = crate::BitReader;
///Field `RTCLPEN` writer - RTC sleep enable
pub type RTCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCAPBLPEN` reader - RTCAPB sleep enable
pub type RTCAPBLPEN_R = crate::BitReader;
///Field `RTCAPBLPEN` writer - RTCAPB sleep enable
pub type RTCAPBLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R2GRETLPEN` reader - R2GRET sleep enable
pub type R2GRETLPEN_R = crate::BitReader;
///Field `R2GRETLPEN` writer - R2GRET sleep enable
pub type R2GRETLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R2GNPULPEN` reader - R2GNPU sleep enable
pub type R2GNPULPEN_R = crate::BitReader;
///Field `R2GNPULPEN` writer - R2GNPU sleep enable
pub type R2GNPULPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SERFLPEN` reader - SERF sleep enable
pub type SERFLPEN_R = crate::BitReader;
///Field `SERFLPEN` writer - SERF sleep enable
pub type SERFLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - HDP sleep enable
    #[inline(always)]
    pub fn hdplpen(&self) -> HDPLPEN_R {
        HDPLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LPUART1 sleep enable
    #[inline(always)]
    pub fn lpuart1lpen(&self) -> LPUART1LPEN_R {
        LPUART1LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - SPI6 sleep enable
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - I2C4 sleep enable
    #[inline(always)]
    pub fn i2c4lpen(&self) -> I2C4LPEN_R {
        I2C4LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - LPTIM2 sleep enable
    #[inline(always)]
    pub fn lptim2lpen(&self) -> LPTIM2LPEN_R {
        LPTIM2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - LPTIM3 sleep enable
    #[inline(always)]
    pub fn lptim3lpen(&self) -> LPTIM3LPEN_R {
        LPTIM3LPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LPTIM4 sleep enable
    #[inline(always)]
    pub fn lptim4lpen(&self) -> LPTIM4LPEN_R {
        LPTIM4LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM5 sleep enable
    #[inline(always)]
    pub fn lptim5lpen(&self) -> LPTIM5LPEN_R {
        LPTIM5LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - VREFBUF sleep enable
    #[inline(always)]
    pub fn vrefbuflpen(&self) -> VREFBUFLPEN_R {
        VREFBUFLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - RTC sleep enable
    #[inline(always)]
    pub fn rtclpen(&self) -> RTCLPEN_R {
        RTCLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - RTCAPB sleep enable
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 22 - R2GRET sleep enable
    #[inline(always)]
    pub fn r2gretlpen(&self) -> R2GRETLPEN_R {
        R2GRETLPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - R2GNPU sleep enable
    #[inline(always)]
    pub fn r2gnpulpen(&self) -> R2GNPULPEN_R {
        R2GNPULPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - SERF sleep enable
    #[inline(always)]
    pub fn serflpen(&self) -> SERFLPEN_R {
        SERFLPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB4LLPENR")
            .field("hdplpen", &self.hdplpen())
            .field("lpuart1lpen", &self.lpuart1lpen())
            .field("spi6lpen", &self.spi6lpen())
            .field("i2c4lpen", &self.i2c4lpen())
            .field("lptim2lpen", &self.lptim2lpen())
            .field("lptim3lpen", &self.lptim3lpen())
            .field("lptim4lpen", &self.lptim4lpen())
            .field("lptim5lpen", &self.lptim5lpen())
            .field("vrefbuflpen", &self.vrefbuflpen())
            .field("rtclpen", &self.rtclpen())
            .field("rtcapblpen", &self.rtcapblpen())
            .field("r2gretlpen", &self.r2gretlpen())
            .field("r2gnpulpen", &self.r2gnpulpen())
            .field("serflpen", &self.serflpen())
            .finish()
    }
}
impl W {
    ///Bit 2 - HDP sleep enable
    #[inline(always)]
    pub fn hdplpen(&mut self) -> HDPLPEN_W<'_, APB4LLPENRrs> {
        HDPLPEN_W::new(self, 2)
    }
    ///Bit 3 - LPUART1 sleep enable
    #[inline(always)]
    pub fn lpuart1lpen(&mut self) -> LPUART1LPEN_W<'_, APB4LLPENRrs> {
        LPUART1LPEN_W::new(self, 3)
    }
    ///Bit 5 - SPI6 sleep enable
    #[inline(always)]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W<'_, APB4LLPENRrs> {
        SPI6LPEN_W::new(self, 5)
    }
    ///Bit 7 - I2C4 sleep enable
    #[inline(always)]
    pub fn i2c4lpen(&mut self) -> I2C4LPEN_W<'_, APB4LLPENRrs> {
        I2C4LPEN_W::new(self, 7)
    }
    ///Bit 9 - LPTIM2 sleep enable
    #[inline(always)]
    pub fn lptim2lpen(&mut self) -> LPTIM2LPEN_W<'_, APB4LLPENRrs> {
        LPTIM2LPEN_W::new(self, 9)
    }
    ///Bit 10 - LPTIM3 sleep enable
    #[inline(always)]
    pub fn lptim3lpen(&mut self) -> LPTIM3LPEN_W<'_, APB4LLPENRrs> {
        LPTIM3LPEN_W::new(self, 10)
    }
    ///Bit 11 - LPTIM4 sleep enable
    #[inline(always)]
    pub fn lptim4lpen(&mut self) -> LPTIM4LPEN_W<'_, APB4LLPENRrs> {
        LPTIM4LPEN_W::new(self, 11)
    }
    ///Bit 12 - LPTIM5 sleep enable
    #[inline(always)]
    pub fn lptim5lpen(&mut self) -> LPTIM5LPEN_W<'_, APB4LLPENRrs> {
        LPTIM5LPEN_W::new(self, 12)
    }
    ///Bit 15 - VREFBUF sleep enable
    #[inline(always)]
    pub fn vrefbuflpen(&mut self) -> VREFBUFLPEN_W<'_, APB4LLPENRrs> {
        VREFBUFLPEN_W::new(self, 15)
    }
    ///Bit 16 - RTC sleep enable
    #[inline(always)]
    pub fn rtclpen(&mut self) -> RTCLPEN_W<'_, APB4LLPENRrs> {
        RTCLPEN_W::new(self, 16)
    }
    ///Bit 17 - RTCAPB sleep enable
    #[inline(always)]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W<'_, APB4LLPENRrs> {
        RTCAPBLPEN_W::new(self, 17)
    }
    ///Bit 22 - R2GRET sleep enable
    #[inline(always)]
    pub fn r2gretlpen(&mut self) -> R2GRETLPEN_W<'_, APB4LLPENRrs> {
        R2GRETLPEN_W::new(self, 22)
    }
    ///Bit 23 - R2GNPU sleep enable
    #[inline(always)]
    pub fn r2gnpulpen(&mut self) -> R2GNPULPEN_W<'_, APB4LLPENRrs> {
        R2GNPULPEN_W::new(self, 23)
    }
    ///Bit 31 - SERF sleep enable
    #[inline(always)]
    pub fn serflpen(&mut self) -> SERFLPEN_W<'_, APB4LLPENRrs> {
        SERFLPEN_W::new(self, 31)
    }
}
/**RCC APB4L Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`apb4llpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4llpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4LLPENR)*/
pub struct APB4LLPENRrs;
impl crate::RegisterSpec for APB4LLPENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb4llpenr::R`](R) reader structure
impl crate::Readable for APB4LLPENRrs {}
///`write(|w| ..)` method takes [`apb4llpenr::W`](W) writer structure
impl crate::Writable for APB4LLPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4LLPENR to value 0
impl crate::Resettable for APB4LLPENRrs {}
