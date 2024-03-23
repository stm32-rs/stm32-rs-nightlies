#[doc = "Register `RCC_APB2RSTR` reader"]
pub type R = crate::R<RCC_APB2RSTRrs>;
#[doc = "Register `RCC_APB2RSTR` writer"]
pub type W = crate::W<RCC_APB2RSTRrs>;
#[doc = "Field `TIM1RST` reader - TIM1 reset This bit is set and cleared by software."]
pub type TIM1RST_R = crate::BitReader;
#[doc = "Field `TIM1RST` writer - TIM1 reset This bit is set and cleared by software."]
pub type TIM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1RST` reader - SPI1 reset This bit is set and cleared by software."]
pub type SPI1RST_R = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1 reset This bit is set and cleared by software."]
pub type SPI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8RST` reader - TIM8 reset This bit is set and cleared by software."]
pub type TIM8RST_R = crate::BitReader;
#[doc = "Field `TIM8RST` writer - TIM8 reset This bit is set and cleared by software."]
pub type TIM8RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1RST` reader - USART1 reset This bit is set and cleared by software."]
pub type USART1RST_R = crate::BitReader;
#[doc = "Field `USART1RST` writer - USART1 reset This bit is set and cleared by software."]
pub type USART1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15RST` reader - TIM15 reset This bit is set and cleared by software."]
pub type TIM15RST_R = crate::BitReader;
#[doc = "Field `TIM15RST` writer - TIM15 reset This bit is set and cleared by software."]
pub type TIM15RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16RST` reader - TIM16 reset This bit is set and cleared by software."]
pub type TIM16RST_R = crate::BitReader;
#[doc = "Field `TIM16RST` writer - TIM16 reset This bit is set and cleared by software."]
pub type TIM16RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17RST` reader - TIM17 reset This bit is set and cleared by software."]
pub type TIM17RST_R = crate::BitReader;
#[doc = "Field `TIM17RST` writer - TIM17 reset This bit is set and cleared by software."]
pub type TIM17RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1RST` reader - SAI1 reset This bit is set and cleared by software."]
pub type SAI1RST_R = crate::BitReader;
#[doc = "Field `SAI1RST` writer - SAI1 reset This bit is set and cleared by software."]
pub type SAI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI2RST` reader - SAI2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SAI2RST_R = crate::BitReader;
#[doc = "Field `SAI2RST` writer - SAI2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SAI2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRST` reader - USB reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type USBRST_R = crate::BitReader;
#[doc = "Field `USBRST` writer - USB reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFXTIMRST` reader - GFXTIM reset This bit is set and cleared by software. Note: .This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GFXTIMRST_R = crate::BitReader;
#[doc = "Field `GFXTIMRST` writer - GFXTIM reset This bit is set and cleared by software. Note: .This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GFXTIMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTDCRST` reader - LTDC reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type LTDCRST_R = crate::BitReader;
#[doc = "Field `LTDCRST` writer - LTDC reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type LTDCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSIRST` reader - DSI reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type DSIRST_R = crate::BitReader;
#[doc = "Field `DSIRST` writer - DSI reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type DSIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 11 - TIM1 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn sai2rst(&self) -> SAI2RST_R {
        SAI2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - USB reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GFXTIM reset This bit is set and cleared by software. Note: .This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn gfxtimrst(&self) -> GFXTIMRST_R {
        GFXTIMRST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LTDC reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn ltdcrst(&self) -> LTDCRST_R {
        LTDCRST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DSI reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn dsirst(&self) -> DSIRST_R {
        DSIRST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<RCC_APB2RSTRrs> {
        TIM1RST_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<RCC_APB2RSTRrs> {
        SPI1RST_W::new(self, 12)
    }
    #[doc = "Bit 13 - TIM8 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim8rst(&mut self) -> TIM8RST_W<RCC_APB2RSTRrs> {
        TIM8RST_W::new(self, 13)
    }
    #[doc = "Bit 14 - USART1 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<RCC_APB2RSTRrs> {
        USART1RST_W::new(self, 14)
    }
    #[doc = "Bit 16 - TIM15 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim15rst(&mut self) -> TIM15RST_W<RCC_APB2RSTRrs> {
        TIM15RST_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim16rst(&mut self) -> TIM16RST_W<RCC_APB2RSTRrs> {
        TIM16RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim17rst(&mut self) -> TIM17RST_W<RCC_APB2RSTRrs> {
        TIM17RST_W::new(self, 18)
    }
    #[doc = "Bit 21 - SAI1 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sai1rst(&mut self) -> SAI1RST_W<RCC_APB2RSTRrs> {
        SAI1RST_W::new(self, 21)
    }
    #[doc = "Bit 22 - SAI2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sai2rst(&mut self) -> SAI2RST_W<RCC_APB2RSTRrs> {
        SAI2RST_W::new(self, 22)
    }
    #[doc = "Bit 24 - USB reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<RCC_APB2RSTRrs> {
        USBRST_W::new(self, 24)
    }
    #[doc = "Bit 25 - GFXTIM reset This bit is set and cleared by software. Note: .This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn gfxtimrst(&mut self) -> GFXTIMRST_W<RCC_APB2RSTRrs> {
        GFXTIMRST_W::new(self, 25)
    }
    #[doc = "Bit 26 - LTDC reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn ltdcrst(&mut self) -> LTDCRST_W<RCC_APB2RSTRrs> {
        LTDCRST_W::new(self, 26)
    }
    #[doc = "Bit 27 - DSI reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn dsirst(&mut self) -> DSIRST_W<RCC_APB2RSTRrs> {
        DSIRST_W::new(self, 27)
    }
}
#[doc = "RCC APB2 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb2rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb2rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_APB2RSTRrs;
impl crate::RegisterSpec for RCC_APB2RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb2rstr::R`](R) reader structure"]
impl crate::Readable for RCC_APB2RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb2rstr::W`](W) writer structure"]
impl crate::Writable for RCC_APB2RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB2RSTR to value 0"]
impl crate::Resettable for RCC_APB2RSTRrs {
    const RESET_VALUE: u32 = 0;
}
