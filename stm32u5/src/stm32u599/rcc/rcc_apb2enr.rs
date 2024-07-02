///Register `RCC_APB2ENR` reader
pub type R = crate::R<RCC_APB2ENRrs>;
///Register `RCC_APB2ENR` writer
pub type W = crate::W<RCC_APB2ENRrs>;
///Field `TIM1EN` reader - TIM1 clock enable This bit is set and cleared by software.
pub type TIM1EN_R = crate::BitReader;
///Field `TIM1EN` writer - TIM1 clock enable This bit is set and cleared by software.
pub type TIM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1EN` reader - SPI1 clock enable This bit is set and cleared by software.
pub type SPI1EN_R = crate::BitReader;
///Field `SPI1EN` writer - SPI1 clock enable This bit is set and cleared by software.
pub type SPI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM8EN` reader - TIM8 clock enable This bit is set and cleared by software.
pub type TIM8EN_R = crate::BitReader;
///Field `TIM8EN` writer - TIM8 clock enable This bit is set and cleared by software.
pub type TIM8EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1EN` reader - USART1clock enable This bit is set and cleared by software.
pub type USART1EN_R = crate::BitReader;
///Field `USART1EN` writer - USART1clock enable This bit is set and cleared by software.
pub type USART1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15EN` reader - TIM15 clock enable This bit is set and cleared by software.
pub type TIM15EN_R = crate::BitReader;
///Field `TIM15EN` writer - TIM15 clock enable This bit is set and cleared by software.
pub type TIM15EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16EN` reader - TIM16 clock enable This bit is set and cleared by software.
pub type TIM16EN_R = crate::BitReader;
///Field `TIM16EN` writer - TIM16 clock enable This bit is set and cleared by software.
pub type TIM16EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17EN` reader - TIM17 clock enable This bit is set and cleared by software.
pub type TIM17EN_R = crate::BitReader;
///Field `TIM17EN` writer - TIM17 clock enable This bit is set and cleared by software.
pub type TIM17EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI1EN` reader - SAI1 clock enable This bit is set and cleared by software.
pub type SAI1EN_R = crate::BitReader;
///Field `SAI1EN` writer - SAI1 clock enable This bit is set and cleared by software.
pub type SAI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI2EN` reader - SAI2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type SAI2EN_R = crate::BitReader;
///Field `SAI2EN` writer - SAI2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type SAI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBEN` reader - USB clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type USBEN_R = crate::BitReader;
///Field `USBEN` writer - USB clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type USBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXTIMEN` reader - GFXTIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type GFXTIMEN_R = crate::BitReader;
///Field `GFXTIMEN` writer - GFXTIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type GFXTIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LTDCEN` reader - LTDC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type LTDCEN_R = crate::BitReader;
///Field `LTDCEN` writer - LTDC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type LTDCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSIEN` reader - DSI clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type DSIEN_R = crate::BitReader;
///Field `DSIEN` writer - DSI clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type DSIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 11 - TIM1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TIM8 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - USART1clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM15 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - SAI1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SAI2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - USB clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - GFXTIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gfxtimen(&self) -> GFXTIMEN_R {
        GFXTIMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - LTDC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - DSI clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB2ENR")
            .field("tim1en", &self.tim1en())
            .field("spi1en", &self.spi1en())
            .field("tim8en", &self.tim8en())
            .field("usart1en", &self.usart1en())
            .field("tim15en", &self.tim15en())
            .field("tim16en", &self.tim16en())
            .field("tim17en", &self.tim17en())
            .field("sai1en", &self.sai1en())
            .field("sai2en", &self.sai2en())
            .field("usben", &self.usben())
            .field("gfxtimen", &self.gfxtimen())
            .field("ltdcen", &self.ltdcen())
            .field("dsien", &self.dsien())
            .finish()
    }
}
impl W {
    ///Bit 11 - TIM1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<RCC_APB2ENRrs> {
        TIM1EN_W::new(self, 11)
    }
    ///Bit 12 - SPI1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<RCC_APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    ///Bit 13 - TIM8 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim8en(&mut self) -> TIM8EN_W<RCC_APB2ENRrs> {
        TIM8EN_W::new(self, 13)
    }
    ///Bit 14 - USART1clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<RCC_APB2ENRrs> {
        USART1EN_W::new(self, 14)
    }
    ///Bit 16 - TIM15 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim15en(&mut self) -> TIM15EN_W<RCC_APB2ENRrs> {
        TIM15EN_W::new(self, 16)
    }
    ///Bit 17 - TIM16 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<RCC_APB2ENRrs> {
        TIM16EN_W::new(self, 17)
    }
    ///Bit 18 - TIM17 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<RCC_APB2ENRrs> {
        TIM17EN_W::new(self, 18)
    }
    ///Bit 21 - SAI1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn sai1en(&mut self) -> SAI1EN_W<RCC_APB2ENRrs> {
        SAI1EN_W::new(self, 21)
    }
    ///Bit 22 - SAI2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn sai2en(&mut self) -> SAI2EN_W<RCC_APB2ENRrs> {
        SAI2EN_W::new(self, 22)
    }
    ///Bit 24 - USB clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> USBEN_W<RCC_APB2ENRrs> {
        USBEN_W::new(self, 24)
    }
    ///Bit 25 - GFXTIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn gfxtimen(&mut self) -> GFXTIMEN_W<RCC_APB2ENRrs> {
        GFXTIMEN_W::new(self, 25)
    }
    ///Bit 26 - LTDC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn ltdcen(&mut self) -> LTDCEN_W<RCC_APB2ENRrs> {
        LTDCEN_W::new(self, 26)
    }
    ///Bit 27 - DSI clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn dsien(&mut self) -> DSIEN_W<RCC_APB2ENRrs> {
        DSIEN_W::new(self, 27)
    }
}
/**RCC APB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#RCC:RCC_APB2ENR)*/
pub struct RCC_APB2ENRrs;
impl crate::RegisterSpec for RCC_APB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_apb2enr::R`](R) reader structure
impl crate::Readable for RCC_APB2ENRrs {}
///`write(|w| ..)` method takes [`rcc_apb2enr::W`](W) writer structure
impl crate::Writable for RCC_APB2ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_APB2ENR to value 0
impl crate::Resettable for RCC_APB2ENRrs {
    const RESET_VALUE: u32 = 0;
}
