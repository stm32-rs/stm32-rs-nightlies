///Register `RCC_APB1ENR2` reader
pub type R = crate::R<RCC_APB1ENR2rs>;
///Register `RCC_APB1ENR2` writer
pub type W = crate::W<RCC_APB1ENR2rs>;
///Field `I2C4EN` reader - I2C4 clock enable This bit is set and cleared by software
pub type I2C4EN_R = crate::BitReader;
///Field `I2C4EN` writer - I2C4 clock enable This bit is set and cleared by software
pub type I2C4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2EN` reader - LPTIM2 clock enable This bit is set and cleared by software.
pub type LPTIM2EN_R = crate::BitReader;
///Field `LPTIM2EN` writer - LPTIM2 clock enable This bit is set and cleared by software.
pub type LPTIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C5EN` reader - I2C5 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type I2C5EN_R = crate::BitReader;
///Field `I2C5EN` writer - I2C5 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type I2C5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C6EN` reader - I2C6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type I2C6EN_R = crate::BitReader;
///Field `I2C6EN` writer - I2C6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type I2C6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCAN1EN` reader - FDCAN1 clock enable This bit is set and cleared by software.
pub type FDCAN1EN_R = crate::BitReader;
///Field `FDCAN1EN` writer - FDCAN1 clock enable This bit is set and cleared by software.
pub type FDCAN1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD1EN` reader - UCPD1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type UCPD1EN_R = crate::BitReader;
///Field `UCPD1EN` writer - UCPD1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type UCPD1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - I2C4 clock enable This bit is set and cleared by software
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - LPTIM2 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - I2C5 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn i2c5en(&self) -> I2C5EN_R {
        I2C5EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn i2c6en(&self) -> I2C6EN_R {
        I2C6EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - FDCAN1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn fdcan1en(&self) -> FDCAN1EN_R {
        FDCAN1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 23 - UCPD1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn ucpd1en(&self) -> UCPD1EN_R {
        UCPD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB1ENR2")
            .field("i2c4en", &self.i2c4en())
            .field("lptim2en", &self.lptim2en())
            .field("i2c5en", &self.i2c5en())
            .field("i2c6en", &self.i2c6en())
            .field("fdcan1en", &self.fdcan1en())
            .field("ucpd1en", &self.ucpd1en())
            .finish()
    }
}
impl W {
    ///Bit 1 - I2C4 clock enable This bit is set and cleared by software
    #[inline(always)]
    #[must_use]
    pub fn i2c4en(&mut self) -> I2C4EN_W<RCC_APB1ENR2rs> {
        I2C4EN_W::new(self, 1)
    }
    ///Bit 5 - LPTIM2 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<RCC_APB1ENR2rs> {
        LPTIM2EN_W::new(self, 5)
    }
    ///Bit 6 - I2C5 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn i2c5en(&mut self) -> I2C5EN_W<RCC_APB1ENR2rs> {
        I2C5EN_W::new(self, 6)
    }
    ///Bit 7 - I2C6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn i2c6en(&mut self) -> I2C6EN_W<RCC_APB1ENR2rs> {
        I2C6EN_W::new(self, 7)
    }
    ///Bit 9 - FDCAN1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn fdcan1en(&mut self) -> FDCAN1EN_W<RCC_APB1ENR2rs> {
        FDCAN1EN_W::new(self, 9)
    }
    ///Bit 23 - UCPD1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn ucpd1en(&mut self) -> UCPD1EN_W<RCC_APB1ENR2rs> {
        UCPD1EN_W::new(self, 23)
    }
}
/**RCC APB1 peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_apb1enr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1enr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#RCC:RCC_APB1ENR2)*/
pub struct RCC_APB1ENR2rs;
impl crate::RegisterSpec for RCC_APB1ENR2rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_apb1enr2::R`](R) reader structure
impl crate::Readable for RCC_APB1ENR2rs {}
///`write(|w| ..)` method takes [`rcc_apb1enr2::W`](W) writer structure
impl crate::Writable for RCC_APB1ENR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_APB1ENR2 to value 0
impl crate::Resettable for RCC_APB1ENR2rs {
    const RESET_VALUE: u32 = 0;
}
