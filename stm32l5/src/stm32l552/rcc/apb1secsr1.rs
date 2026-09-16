///Register `APB1SECSR1` reader
pub type R = crate::R<APB1SECSR1rs>;
///Field `TIM2SECF` reader - TIM2SECF
pub type TIM2SECF_R = crate::BitReader;
///Field `TIM3SECF` reader - TIM3SECF
pub type TIM3SECF_R = crate::BitReader;
///Field `TIM4SECF` reader - TIM4SECF
pub type TIM4SECF_R = crate::BitReader;
///Field `TIM5SECF` reader - TIM5SECF
pub type TIM5SECF_R = crate::BitReader;
///Field `TIM6SECF` reader - TIM6SECF
pub type TIM6SECF_R = crate::BitReader;
///Field `TIM7SECF` reader - TIM7SECF
pub type TIM7SECF_R = crate::BitReader;
///Field `RTCAPBSECF` reader - RTCAPBSECF
pub type RTCAPBSECF_R = crate::BitReader;
///Field `WWDGSECF` reader - WWDGSECF
pub type WWDGSECF_R = crate::BitReader;
///Field `SPI2SECF` reader - SPI2SECF
pub type SPI2SECF_R = crate::BitReader;
///Field `SPI3SECF` reader - SPI3SECF
pub type SPI3SECF_R = crate::BitReader;
///Field `UART2SECF` reader - UART2SECF
pub type UART2SECF_R = crate::BitReader;
///Field `UART3SECF` reader - UART3SECF
pub type UART3SECF_R = crate::BitReader;
///Field `UART4SECF` reader - UART4SECF
pub type UART4SECF_R = crate::BitReader;
///Field `UART5SECF` reader - UART5SECF
pub type UART5SECF_R = crate::BitReader;
///Field `I2C1SECF` reader - I2C1SECF
pub type I2C1SECF_R = crate::BitReader;
///Field `I2C2SECF` reader - I2C2SECF
pub type I2C2SECF_R = crate::BitReader;
///Field `I2C3SECF` reader - I2C3SECF
pub type I2C3SECF_R = crate::BitReader;
///Field `CRSSECF` reader - CRSSECF
pub type CRSSECF_R = crate::BitReader;
///Field `PWRSECF` reader - PWRSECF
pub type PWRSECF_R = crate::BitReader;
///Field `DACSECF` reader - DACSECF
pub type DACSECF_R = crate::BitReader;
///Field `OPAMPSECF` reader - OPAMPSECF
pub type OPAMPSECF_R = crate::BitReader;
///Field `LPTIM1SECF` reader - LPTIM1SECF
pub type LPTIM1SECF_R = crate::BitReader;
impl R {
    ///Bit 0 - TIM2SECF
    #[inline(always)]
    pub fn tim2secf(&self) -> TIM2SECF_R {
        TIM2SECF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3SECF
    #[inline(always)]
    pub fn tim3secf(&self) -> TIM3SECF_R {
        TIM3SECF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4SECF
    #[inline(always)]
    pub fn tim4secf(&self) -> TIM4SECF_R {
        TIM4SECF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5SECF
    #[inline(always)]
    pub fn tim5secf(&self) -> TIM5SECF_R {
        TIM5SECF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6SECF
    #[inline(always)]
    pub fn tim6secf(&self) -> TIM6SECF_R {
        TIM6SECF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7SECF
    #[inline(always)]
    pub fn tim7secf(&self) -> TIM7SECF_R {
        TIM7SECF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 10 - RTCAPBSECF
    #[inline(always)]
    pub fn rtcapbsecf(&self) -> RTCAPBSECF_R {
        RTCAPBSECF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - WWDGSECF
    #[inline(always)]
    pub fn wwdgsecf(&self) -> WWDGSECF_R {
        WWDGSECF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2SECF
    #[inline(always)]
    pub fn spi2secf(&self) -> SPI2SECF_R {
        SPI2SECF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3SECF
    #[inline(always)]
    pub fn spi3secf(&self) -> SPI3SECF_R {
        SPI3SECF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - UART2SECF
    #[inline(always)]
    pub fn uart2secf(&self) -> UART2SECF_R {
        UART2SECF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - UART3SECF
    #[inline(always)]
    pub fn uart3secf(&self) -> UART3SECF_R {
        UART3SECF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4SECF
    #[inline(always)]
    pub fn uart4secf(&self) -> UART4SECF_R {
        UART4SECF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5SECF
    #[inline(always)]
    pub fn uart5secf(&self) -> UART5SECF_R {
        UART5SECF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1SECF
    #[inline(always)]
    pub fn i2c1secf(&self) -> I2C1SECF_R {
        I2C1SECF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2SECF
    #[inline(always)]
    pub fn i2c2secf(&self) -> I2C2SECF_R {
        I2C2SECF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3SECF
    #[inline(always)]
    pub fn i2c3secf(&self) -> I2C3SECF_R {
        I2C3SECF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CRSSECF
    #[inline(always)]
    pub fn crssecf(&self) -> CRSSECF_R {
        CRSSECF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - PWRSECF
    #[inline(always)]
    pub fn pwrsecf(&self) -> PWRSECF_R {
        PWRSECF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DACSECF
    #[inline(always)]
    pub fn dacsecf(&self) -> DACSECF_R {
        DACSECF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - OPAMPSECF
    #[inline(always)]
    pub fn opampsecf(&self) -> OPAMPSECF_R {
        OPAMPSECF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - LPTIM1SECF
    #[inline(always)]
    pub fn lptim1secf(&self) -> LPTIM1SECF_R {
        LPTIM1SECF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1SECSR1")
            .field("lptim1secf", &self.lptim1secf())
            .field("opampsecf", &self.opampsecf())
            .field("dacsecf", &self.dacsecf())
            .field("pwrsecf", &self.pwrsecf())
            .field("crssecf", &self.crssecf())
            .field("i2c3secf", &self.i2c3secf())
            .field("i2c2secf", &self.i2c2secf())
            .field("i2c1secf", &self.i2c1secf())
            .field("uart5secf", &self.uart5secf())
            .field("uart4secf", &self.uart4secf())
            .field("uart3secf", &self.uart3secf())
            .field("uart2secf", &self.uart2secf())
            .field("spi3secf", &self.spi3secf())
            .field("spi2secf", &self.spi2secf())
            .field("wwdgsecf", &self.wwdgsecf())
            .field("rtcapbsecf", &self.rtcapbsecf())
            .field("tim7secf", &self.tim7secf())
            .field("tim6secf", &self.tim6secf())
            .field("tim5secf", &self.tim5secf())
            .field("tim4secf", &self.tim4secf())
            .field("tim3secf", &self.tim3secf())
            .field("tim2secf", &self.tim2secf())
            .finish()
    }
}
/**RCC APB1 security status register 1

You can [`read`](crate::Reg::read) this register and get [`apb1secsr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#RCC:APB1SECSR1)*/
pub struct APB1SECSR1rs;
impl crate::RegisterSpec for APB1SECSR1rs {
    type Ux = u32;
}
///`read()` method returns [`apb1secsr1::R`](R) reader structure
impl crate::Readable for APB1SECSR1rs {}
///`reset()` method sets APB1SECSR1 to value 0x0400
impl crate::Resettable for APB1SECSR1rs {
    const RESET_VALUE: u32 = 0x0400;
}
