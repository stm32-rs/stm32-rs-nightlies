///Register `MC_APB5LPENCLRR` reader
pub type R = crate::R<MC_APB5LPENCLRRrs>;
///Register `MC_APB5LPENCLRR` writer
pub type W = crate::W<MC_APB5LPENCLRRrs>;
///Field `SPI6LPEN` reader - SPI6LPEN
pub type SPI6LPEN_R = crate::BitReader;
///Field `SPI6LPEN` writer - SPI6LPEN
pub type SPI6LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4LPEN` reader - I2C4LPEN
pub type I2C4LPEN_R = crate::BitReader;
///Field `I2C4LPEN` writer - I2C4LPEN
pub type I2C4LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C6LPEN` reader - I2C6LPEN
pub type I2C6LPEN_R = crate::BitReader;
///Field `I2C6LPEN` writer - I2C6LPEN
pub type I2C6LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1LPEN` reader - USART1LPEN
pub type USART1LPEN_R = crate::BitReader;
///Field `USART1LPEN` writer - USART1LPEN
pub type USART1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCAPBLPEN` reader - RTCAPBLPEN
pub type RTCAPBLPEN_R = crate::BitReader;
///Field `RTCAPBLPEN` writer - RTCAPBLPEN
pub type RTCAPBLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZC1LPEN` reader - TZC1LPEN
pub type TZC1LPEN_R = crate::BitReader;
///Field `TZC1LPEN` writer - TZC1LPEN
pub type TZC1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZC2LPEN` reader - TZC2LPEN
pub type TZC2LPEN_R = crate::BitReader;
///Field `TZC2LPEN` writer - TZC2LPEN
pub type TZC2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZPCLPEN` reader - TZPCLPEN
pub type TZPCLPEN_R = crate::BitReader;
///Field `TZPCLPEN` writer - TZPCLPEN
pub type TZPCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSECLPEN` reader - BSECLPEN
pub type BSECLPEN_R = crate::BitReader;
///Field `BSECLPEN` writer - BSECLPEN
pub type BSECLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STGENLPEN` reader - STGENLPEN
pub type STGENLPEN_R = crate::BitReader;
///Field `STGENLPEN` writer - STGENLPEN
pub type STGENLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STGENSTPEN` reader - STGENSTPEN
pub type STGENSTPEN_R = crate::BitReader;
///Field `STGENSTPEN` writer - STGENSTPEN
pub type STGENSTPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SPI6LPEN
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - I2C4LPEN
    #[inline(always)]
    pub fn i2c4lpen(&self) -> I2C4LPEN_R {
        I2C4LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I2C6LPEN
    #[inline(always)]
    pub fn i2c6lpen(&self) -> I2C6LPEN_R {
        I2C6LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - USART1LPEN
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - RTCAPBLPEN
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - TZC1LPEN
    #[inline(always)]
    pub fn tzc1lpen(&self) -> TZC1LPEN_R {
        TZC1LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TZC2LPEN
    #[inline(always)]
    pub fn tzc2lpen(&self) -> TZC2LPEN_R {
        TZC2LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TZPCLPEN
    #[inline(always)]
    pub fn tzpclpen(&self) -> TZPCLPEN_R {
        TZPCLPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - BSECLPEN
    #[inline(always)]
    pub fn bseclpen(&self) -> BSECLPEN_R {
        BSECLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - STGENLPEN
    #[inline(always)]
    pub fn stgenlpen(&self) -> STGENLPEN_R {
        STGENLPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - STGENSTPEN
    #[inline(always)]
    pub fn stgenstpen(&self) -> STGENSTPEN_R {
        STGENSTPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MC_APB5LPENCLRR")
            .field("spi6lpen", &self.spi6lpen())
            .field("i2c4lpen", &self.i2c4lpen())
            .field("i2c6lpen", &self.i2c6lpen())
            .field("usart1lpen", &self.usart1lpen())
            .field("rtcapblpen", &self.rtcapblpen())
            .field("tzc1lpen", &self.tzc1lpen())
            .field("tzc2lpen", &self.tzc2lpen())
            .field("tzpclpen", &self.tzpclpen())
            .field("bseclpen", &self.bseclpen())
            .field("stgenlpen", &self.stgenlpen())
            .field("stgenstpen", &self.stgenstpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - SPI6LPEN
    #[inline(always)]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W<MC_APB5LPENCLRRrs> {
        SPI6LPEN_W::new(self, 0)
    }
    ///Bit 2 - I2C4LPEN
    #[inline(always)]
    pub fn i2c4lpen(&mut self) -> I2C4LPEN_W<MC_APB5LPENCLRRrs> {
        I2C4LPEN_W::new(self, 2)
    }
    ///Bit 3 - I2C6LPEN
    #[inline(always)]
    pub fn i2c6lpen(&mut self) -> I2C6LPEN_W<MC_APB5LPENCLRRrs> {
        I2C6LPEN_W::new(self, 3)
    }
    ///Bit 4 - USART1LPEN
    #[inline(always)]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<MC_APB5LPENCLRRrs> {
        USART1LPEN_W::new(self, 4)
    }
    ///Bit 8 - RTCAPBLPEN
    #[inline(always)]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W<MC_APB5LPENCLRRrs> {
        RTCAPBLPEN_W::new(self, 8)
    }
    ///Bit 11 - TZC1LPEN
    #[inline(always)]
    pub fn tzc1lpen(&mut self) -> TZC1LPEN_W<MC_APB5LPENCLRRrs> {
        TZC1LPEN_W::new(self, 11)
    }
    ///Bit 12 - TZC2LPEN
    #[inline(always)]
    pub fn tzc2lpen(&mut self) -> TZC2LPEN_W<MC_APB5LPENCLRRrs> {
        TZC2LPEN_W::new(self, 12)
    }
    ///Bit 13 - TZPCLPEN
    #[inline(always)]
    pub fn tzpclpen(&mut self) -> TZPCLPEN_W<MC_APB5LPENCLRRrs> {
        TZPCLPEN_W::new(self, 13)
    }
    ///Bit 16 - BSECLPEN
    #[inline(always)]
    pub fn bseclpen(&mut self) -> BSECLPEN_W<MC_APB5LPENCLRRrs> {
        BSECLPEN_W::new(self, 16)
    }
    ///Bit 20 - STGENLPEN
    #[inline(always)]
    pub fn stgenlpen(&mut self) -> STGENLPEN_W<MC_APB5LPENCLRRrs> {
        STGENLPEN_W::new(self, 20)
    }
    ///Bit 21 - STGENSTPEN
    #[inline(always)]
    pub fn stgenstpen(&mut self) -> STGENSTPEN_W<MC_APB5LPENCLRRrs> {
        STGENSTPEN_W::new(self, 21)
    }
}
/**This register is used by the MCU in order to clear the PERxLPEN bit

You can [`read`](crate::Reg::read) this register and get [`mc_apb5lpenclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb5lpenclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MC_APB5LPENCLRR)*/
pub struct MC_APB5LPENCLRRrs;
impl crate::RegisterSpec for MC_APB5LPENCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`mc_apb5lpenclrr::R`](R) reader structure
impl crate::Readable for MC_APB5LPENCLRRrs {}
///`write(|w| ..)` method takes [`mc_apb5lpenclrr::W`](W) writer structure
impl crate::Writable for MC_APB5LPENCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MC_APB5LPENCLRR to value 0x0011_391d
impl crate::Resettable for MC_APB5LPENCLRRrs {
    const RESET_VALUE: u32 = 0x0011_391d;
}
