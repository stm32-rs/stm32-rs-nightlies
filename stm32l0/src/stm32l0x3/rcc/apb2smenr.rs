#[doc = "Register `APB2SMENR` reader"]
pub type R = crate::R<APB2SMENRrs>;
#[doc = "Register `APB2SMENR` writer"]
pub type W = crate::W<APB2SMENRrs>;
#[doc = "System configuration controller clock enable during sleep mode bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGSMEN {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<SYSCFGSMEN> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGSMEN` reader - System configuration controller clock enable during sleep mode bit"]
pub type SYSCFGSMEN_R = crate::BitReader<SYSCFGSMEN>;
impl SYSCFGSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGSMEN {
        match self.bits {
            false => SYSCFGSMEN::Disabled,
            true => SYSCFGSMEN::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGSMEN::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGSMEN::Enabled
    }
}
#[doc = "Field `SYSCFGSMEN` writer - System configuration controller clock enable during sleep mode bit"]
pub type SYSCFGSMEN_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGSMEN>;
impl<'a, REG> SYSCFGSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGSMEN::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGSMEN::Enabled)
    }
}
#[doc = "Field `TIM21SMEN` reader - TIM21 timer clock enable during sleep mode bit"]
pub use SYSCFGSMEN_R as TIM21SMEN_R;
#[doc = "Field `TIM22SMEN` reader - TIM22 timer clock enable during sleep mode bit"]
pub use SYSCFGSMEN_R as TIM22SMEN_R;
#[doc = "Field `ADCSMEN` reader - ADC clock enable during sleep mode bit"]
pub use SYSCFGSMEN_R as ADCSMEN_R;
#[doc = "Field `SPI1SMEN` reader - SPI1 clock enable during sleep mode bit"]
pub use SYSCFGSMEN_R as SPI1SMEN_R;
#[doc = "Field `USART1SMEN` reader - USART1 clock enable during sleep mode bit"]
pub use SYSCFGSMEN_R as USART1SMEN_R;
#[doc = "Field `DBGSMEN` reader - DBG clock enable during sleep mode bit"]
pub use SYSCFGSMEN_R as DBGSMEN_R;
#[doc = "Field `TIM21SMEN` writer - TIM21 timer clock enable during sleep mode bit"]
pub use SYSCFGSMEN_W as TIM21SMEN_W;
#[doc = "Field `TIM22SMEN` writer - TIM22 timer clock enable during sleep mode bit"]
pub use SYSCFGSMEN_W as TIM22SMEN_W;
#[doc = "Field `ADCSMEN` writer - ADC clock enable during sleep mode bit"]
pub use SYSCFGSMEN_W as ADCSMEN_W;
#[doc = "Field `SPI1SMEN` writer - SPI1 clock enable during sleep mode bit"]
pub use SYSCFGSMEN_W as SPI1SMEN_W;
#[doc = "Field `USART1SMEN` writer - USART1 clock enable during sleep mode bit"]
pub use SYSCFGSMEN_W as USART1SMEN_W;
#[doc = "Field `DBGSMEN` writer - DBG clock enable during sleep mode bit"]
pub use SYSCFGSMEN_W as DBGSMEN_W;
impl R {
    #[doc = "Bit 0 - System configuration controller clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - TIM21 timer clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn tim21smen(&self) -> TIM21SMEN_R {
        TIM21SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM22 timer clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn tim22smen(&self) -> TIM22SMEN_R {
        TIM22SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn adcsmen(&self) -> ADCSMEN_R {
        ADCSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 22 - DBG clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn dbgsmen(&self) -> DBGSMEN_R {
        DBGSMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System configuration controller clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W<APB2SMENRrs> {
        SYSCFGSMEN_W::new(self, 0)
    }
    #[doc = "Bit 2 - TIM21 timer clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim21smen(&mut self) -> TIM21SMEN_W<APB2SMENRrs> {
        TIM21SMEN_W::new(self, 2)
    }
    #[doc = "Bit 5 - TIM22 timer clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim22smen(&mut self) -> TIM22SMEN_W<APB2SMENRrs> {
        TIM22SMEN_W::new(self, 5)
    }
    #[doc = "Bit 9 - ADC clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn adcsmen(&mut self) -> ADCSMEN_W<APB2SMENRrs> {
        ADCSMEN_W::new(self, 9)
    }
    #[doc = "Bit 12 - SPI1 clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<APB2SMENRrs> {
        SPI1SMEN_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<APB2SMENRrs> {
        USART1SMEN_W::new(self, 14)
    }
    #[doc = "Bit 22 - DBG clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn dbgsmen(&mut self) -> DBGSMEN_W<APB2SMENRrs> {
        DBGSMEN_W::new(self, 22)
    }
}
#[doc = "APB2 peripheral clock enable in sleep mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets APB2SMENR to value 0x0040_5225"]
impl crate::Resettable for APB2SMENRrs {
    const RESET_VALUE: u32 = 0x0040_5225;
}
