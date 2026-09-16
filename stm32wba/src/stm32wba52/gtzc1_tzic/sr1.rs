///Register `SR1` reader
pub type R = crate::R<SR1rs>;
///Field `TIM2F` reader - illegal access flag for TIM2
pub type TIM2F_R = crate::BitReader;
///Field `TIM3F` reader - illegal access flag for TIM3
pub type TIM3F_R = crate::BitReader;
///Field `WWDGF` reader - illegal access flag for WWDG
pub type WWDGF_R = crate::BitReader;
///Field `IWDGF` reader - illegal access flag for IWDG
pub type IWDGF_R = crate::BitReader;
///Field `USART2F` reader - illegal access flag for USART2
pub type USART2F_R = crate::BitReader;
///Field `I2C1F` reader - illegal access flag for I2C1
pub type I2C1F_R = crate::BitReader;
///Field `LPTIM2F` reader - illegal access flag for LPTIM2
pub type LPTIM2F_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access flag for TIM2
    #[inline(always)]
    pub fn tim2f(&self) -> TIM2F_R {
        TIM2F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access flag for TIM3
    #[inline(always)]
    pub fn tim3f(&self) -> TIM3F_R {
        TIM3F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 6 - illegal access flag for WWDG
    #[inline(always)]
    pub fn wwdgf(&self) -> WWDGF_R {
        WWDGF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access flag for IWDG
    #[inline(always)]
    pub fn iwdgf(&self) -> IWDGF_R {
        IWDGF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - illegal access flag for USART2
    #[inline(always)]
    pub fn usart2f(&self) -> USART2F_R {
        USART2F_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - illegal access flag for I2C1
    #[inline(always)]
    pub fn i2c1f(&self) -> I2C1F_R {
        I2C1F_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 17 - illegal access flag for LPTIM2
    #[inline(always)]
    pub fn lptim2f(&self) -> LPTIM2F_R {
        LPTIM2F_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR1")
            .field("tim2f", &self.tim2f())
            .field("tim3f", &self.tim3f())
            .field("wwdgf", &self.wwdgf())
            .field("iwdgf", &self.iwdgf())
            .field("usart2f", &self.usart2f())
            .field("i2c1f", &self.i2c1f())
            .field("lptim2f", &self.lptim2f())
            .finish()
    }
}
/**TZIC status register 1

You can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GTZC1_TZIC:SR1)*/
pub struct SR1rs;
impl crate::RegisterSpec for SR1rs {
    type Ux = u32;
}
///`read()` method returns [`sr1::R`](R) reader structure
impl crate::Readable for SR1rs {}
///`reset()` method sets SR1 to value 0
impl crate::Resettable for SR1rs {}
