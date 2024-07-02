///Register `RCC_APB1SMENR2` reader
pub type R = crate::R<RCC_APB1SMENR2rs>;
///Register `RCC_APB1SMENR2` writer
pub type W = crate::W<RCC_APB1SMENR2rs>;
///Field `I2C4SMEN` reader - I2C4 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C4SMEN_R = crate::BitReader;
///Field `I2C4SMEN` writer - I2C4 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2SMEN` reader - LPTIM2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPTIM2SMEN_R = crate::BitReader;
///Field `LPTIM2SMEN` writer - LPTIM2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPTIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C5SMEN` reader - I2C5 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type I2C5SMEN_R = crate::BitReader;
///Field `I2C5SMEN` writer - I2C5 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type I2C5SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C6SMEN` reader - I2C6 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type I2C6SMEN_R = crate::BitReader;
///Field `I2C6SMEN` writer - I2C6 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type I2C6SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCAN1SMEN` reader - FDCAN1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type FDCAN1SMEN_R = crate::BitReader;
///Field `FDCAN1SMEN` writer - FDCAN1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type FDCAN1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD1SMEN` reader - UCPD1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type UCPD1SMEN_R = crate::BitReader;
///Field `UCPD1SMEN` writer - UCPD1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type UCPD1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - I2C4 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn i2c4smen(&self) -> I2C4SMEN_R {
        I2C4SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - LPTIM2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - I2C5 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn i2c5smen(&self) -> I2C5SMEN_R {
        I2C5SMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C6 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn i2c6smen(&self) -> I2C6SMEN_R {
        I2C6SMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - FDCAN1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn fdcan1smen(&self) -> FDCAN1SMEN_R {
        FDCAN1SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 23 - UCPD1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn ucpd1smen(&self) -> UCPD1SMEN_R {
        UCPD1SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB1SMENR2")
            .field("i2c4smen", &self.i2c4smen())
            .field("lptim2smen", &self.lptim2smen())
            .field("i2c5smen", &self.i2c5smen())
            .field("i2c6smen", &self.i2c6smen())
            .field("fdcan1smen", &self.fdcan1smen())
            .field("ucpd1smen", &self.ucpd1smen())
            .finish()
    }
}
impl W {
    ///Bit 1 - I2C4 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn i2c4smen(&mut self) -> I2C4SMEN_W<RCC_APB1SMENR2rs> {
        I2C4SMEN_W::new(self, 1)
    }
    ///Bit 5 - LPTIM2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W<RCC_APB1SMENR2rs> {
        LPTIM2SMEN_W::new(self, 5)
    }
    ///Bit 6 - I2C5 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn i2c5smen(&mut self) -> I2C5SMEN_W<RCC_APB1SMENR2rs> {
        I2C5SMEN_W::new(self, 6)
    }
    ///Bit 7 - I2C6 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn i2c6smen(&mut self) -> I2C6SMEN_W<RCC_APB1SMENR2rs> {
        I2C6SMEN_W::new(self, 7)
    }
    ///Bit 9 - FDCAN1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn fdcan1smen(&mut self) -> FDCAN1SMEN_W<RCC_APB1SMENR2rs> {
        FDCAN1SMEN_W::new(self, 9)
    }
    ///Bit 23 - UCPD1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn ucpd1smen(&mut self) -> UCPD1SMEN_W<RCC_APB1SMENR2rs> {
        UCPD1SMEN_W::new(self, 23)
    }
}
/**RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_apb1smenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1smenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:RCC_APB1SMENR2)*/
pub struct RCC_APB1SMENR2rs;
impl crate::RegisterSpec for RCC_APB1SMENR2rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_apb1smenr2::R`](R) reader structure
impl crate::Readable for RCC_APB1SMENR2rs {}
///`write(|w| ..)` method takes [`rcc_apb1smenr2::W`](W) writer structure
impl crate::Writable for RCC_APB1SMENR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_APB1SMENR2 to value 0xffff_ffff
impl crate::Resettable for RCC_APB1SMENR2rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
