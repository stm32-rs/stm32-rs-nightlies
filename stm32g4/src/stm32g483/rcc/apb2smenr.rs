#[doc = "Register `APB2SMENR` reader"]
pub type R = crate::R<APB2SMENRrs>;
#[doc = "Register `APB2SMENR` writer"]
pub type W = crate::W<APB2SMENRrs>;
#[doc = "Field `SYSCFGSMEN` reader - SYSCFG clocks enable during Sleep and Stop modes"]
pub type SYSCFGSMEN_R = crate::BitReader;
#[doc = "Field `SYSCFGSMEN` writer - SYSCFG clocks enable during Sleep and Stop modes"]
pub type SYSCFGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1SMEN` reader - TIM1 timer clocks enable during Sleep and Stop modes"]
pub type TIM1SMEN_R = crate::BitReader;
#[doc = "Field `TIM1SMEN` writer - TIM1 timer clocks enable during Sleep and Stop modes"]
pub type TIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1SMEN` reader - SPI1 clocks enable during Sleep and Stop modes"]
pub type SPI1SMEN_R = crate::BitReader;
#[doc = "Field `SPI1SMEN` writer - SPI1 clocks enable during Sleep and Stop modes"]
pub type SPI1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8SMEN` reader - TIM8 timer clocks enable during Sleep and Stop modes"]
pub type TIM8SMEN_R = crate::BitReader;
#[doc = "Field `TIM8SMEN` writer - TIM8 timer clocks enable during Sleep and Stop modes"]
pub type TIM8SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1SMEN` reader - USART1clocks enable during Sleep and Stop modes"]
pub type USART1SMEN_R = crate::BitReader;
#[doc = "Field `USART1SMEN` writer - USART1clocks enable during Sleep and Stop modes"]
pub type USART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4SMEN` reader - SPI4 timer clocks enable during Sleep and Stop modes"]
pub type SPI4SMEN_R = crate::BitReader;
#[doc = "Field `SPI4SMEN` writer - SPI4 timer clocks enable during Sleep and Stop modes"]
pub type SPI4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15SMEN` reader - TIM15 timer clocks enable during Sleep and Stop modes"]
pub type TIM15SMEN_R = crate::BitReader;
#[doc = "Field `TIM15SMEN` writer - TIM15 timer clocks enable during Sleep and Stop modes"]
pub type TIM15SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16SMEN` reader - TIM16 timer clocks enable during Sleep and Stop modes"]
pub type TIM16SMEN_R = crate::BitReader;
#[doc = "Field `TIM16SMEN` writer - TIM16 timer clocks enable during Sleep and Stop modes"]
pub type TIM16SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17SMEN` reader - TIM17 timer clocks enable during Sleep and Stop modes"]
pub type TIM17SMEN_R = crate::BitReader;
#[doc = "Field `TIM17SMEN` writer - TIM17 timer clocks enable during Sleep and Stop modes"]
pub type TIM17SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM20SMEN` reader - Timer 20clock enable during sleep mode"]
pub type TIM20SMEN_R = crate::BitReader;
#[doc = "Field `TIM20SMEN` writer - Timer 20clock enable during sleep mode"]
pub type TIM20SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1SMEN` reader - SAI1 clock enable during sleep mode"]
pub type SAI1SMEN_R = crate::BitReader;
#[doc = "Field `SAI1SMEN` writer - SAI1 clock enable during sleep mode"]
pub type SAI1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRTIM1SMEN` reader - HRTIMER clock enable during sleep mode"]
pub type HRTIM1SMEN_R = crate::BitReader;
#[doc = "Field `HRTIM1SMEN` writer - HRTIMER clock enable during sleep mode"]
pub type HRTIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSCFG clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim8smen(&self) -> TIM8SMEN_R {
        TIM8SMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI4 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn spi4smen(&self) -> SPI4SMEN_R {
        SPI4SMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim15smen(&self) -> TIM15SMEN_R {
        TIM15SMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer 20clock enable during sleep mode"]
    #[inline(always)]
    pub fn tim20smen(&self) -> TIM20SMEN_R {
        TIM20SMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sai1smen(&self) -> SAI1SMEN_R {
        SAI1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - HRTIMER clock enable during sleep mode"]
    #[inline(always)]
    pub fn hrtim1smen(&self) -> HRTIM1SMEN_R {
        HRTIM1SMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W<APB2SMENRrs> {
        SYSCFGSMEN_W::new(self, 0)
    }
    #[doc = "Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W<APB2SMENRrs> {
        TIM1SMEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<APB2SMENRrs> {
        SPI1SMEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim8smen(&mut self) -> TIM8SMEN_W<APB2SMENRrs> {
        TIM8SMEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - USART1clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<APB2SMENRrs> {
        USART1SMEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI4 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn spi4smen(&mut self) -> SPI4SMEN_W<APB2SMENRrs> {
        SPI4SMEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim15smen(&mut self) -> TIM15SMEN_W<APB2SMENRrs> {
        TIM15SMEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W<APB2SMENRrs> {
        TIM16SMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W<APB2SMENRrs> {
        TIM17SMEN_W::new(self, 18)
    }
    #[doc = "Bit 20 - Timer 20clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim20smen(&mut self) -> TIM20SMEN_W<APB2SMENRrs> {
        TIM20SMEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - SAI1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sai1smen(&mut self) -> SAI1SMEN_W<APB2SMENRrs> {
        SAI1SMEN_W::new(self, 21)
    }
    #[doc = "Bit 26 - HRTIMER clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn hrtim1smen(&mut self) -> HRTIM1SMEN_W<APB2SMENRrs> {
        HRTIM1SMEN_W::new(self, 26)
    }
}
#[doc = "APB2SMENR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2SMENRrs;
impl crate::RegisterSpec for APB2SMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2smenr::R`](R) reader structure"]
impl crate::Readable for APB2SMENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb2smenr::W`](W) writer structure"]
impl crate::Writable for APB2SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2SMENR to value 0x0437_f801"]
impl crate::Resettable for APB2SMENRrs {
    const RESET_VALUE: u32 = 0x0437_f801;
}
