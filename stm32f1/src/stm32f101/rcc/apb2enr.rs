#[doc = "Register `APB2ENR` reader"]
pub type R = crate::R<APB2ENRrs>;
#[doc = "Register `APB2ENR` writer"]
pub type W = crate::W<APB2ENRrs>;
#[doc = "Alternate function I/O clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFIOEN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<AFIOEN> for bool {
    #[inline(always)]
    fn from(variant: AFIOEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AFIOEN` reader - Alternate function I/O clock enable"]
pub type AFIOEN_R = crate::BitReader<AFIOEN>;
impl AFIOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFIOEN {
        match self.bits {
            false => AFIOEN::Disabled,
            true => AFIOEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AFIOEN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AFIOEN::Enabled
    }
}
#[doc = "Field `AFIOEN` writer - Alternate function I/O clock enable"]
pub type AFIOEN_W<'a, REG> = crate::BitWriter<'a, REG, AFIOEN>;
impl<'a, REG> AFIOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AFIOEN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AFIOEN::Enabled)
    }
}
#[doc = "Field `IOPAEN` reader - I/O port A clock enable"]
pub use AFIOEN_R as IOPAEN_R;
#[doc = "Field `IOPBEN` reader - I/O port B clock enable"]
pub use AFIOEN_R as IOPBEN_R;
#[doc = "Field `IOPCEN` reader - I/O port C clock enable"]
pub use AFIOEN_R as IOPCEN_R;
#[doc = "Field `IOPDEN` reader - I/O port D clock enable"]
pub use AFIOEN_R as IOPDEN_R;
#[doc = "Field `IOPEEN` reader - I/O port E clock enable"]
pub use AFIOEN_R as IOPEEN_R;
#[doc = "Field `IOPFEN` reader - I/O port F clock enable"]
pub use AFIOEN_R as IOPFEN_R;
#[doc = "Field `IOPGEN` reader - I/O port G clock enable"]
pub use AFIOEN_R as IOPGEN_R;
#[doc = "Field `ADC1EN` reader - ADC 1 interface clock enable"]
pub use AFIOEN_R as ADC1EN_R;
#[doc = "Field `SPI1EN` reader - SPI 1 clock enable"]
pub use AFIOEN_R as SPI1EN_R;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub use AFIOEN_R as USART1EN_R;
#[doc = "Field `TIM9EN` reader - TIM9 Timer clock enable"]
pub use AFIOEN_R as TIM9EN_R;
#[doc = "Field `TIM10EN` reader - TIM10 Timer clock enable"]
pub use AFIOEN_R as TIM10EN_R;
#[doc = "Field `TIM11EN` reader - TIM11 Timer clock enable"]
pub use AFIOEN_R as TIM11EN_R;
#[doc = "Field `IOPAEN` writer - I/O port A clock enable"]
pub use AFIOEN_W as IOPAEN_W;
#[doc = "Field `IOPBEN` writer - I/O port B clock enable"]
pub use AFIOEN_W as IOPBEN_W;
#[doc = "Field `IOPCEN` writer - I/O port C clock enable"]
pub use AFIOEN_W as IOPCEN_W;
#[doc = "Field `IOPDEN` writer - I/O port D clock enable"]
pub use AFIOEN_W as IOPDEN_W;
#[doc = "Field `IOPEEN` writer - I/O port E clock enable"]
pub use AFIOEN_W as IOPEEN_W;
#[doc = "Field `IOPFEN` writer - I/O port F clock enable"]
pub use AFIOEN_W as IOPFEN_W;
#[doc = "Field `IOPGEN` writer - I/O port G clock enable"]
pub use AFIOEN_W as IOPGEN_W;
#[doc = "Field `ADC1EN` writer - ADC 1 interface clock enable"]
pub use AFIOEN_W as ADC1EN_W;
#[doc = "Field `SPI1EN` writer - SPI 1 clock enable"]
pub use AFIOEN_W as SPI1EN_W;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub use AFIOEN_W as USART1EN_W;
#[doc = "Field `TIM9EN` writer - TIM9 Timer clock enable"]
pub use AFIOEN_W as TIM9EN_W;
#[doc = "Field `TIM10EN` writer - TIM10 Timer clock enable"]
pub use AFIOEN_W as TIM10EN_W;
#[doc = "Field `TIM11EN` writer - TIM11 Timer clock enable"]
pub use AFIOEN_W as TIM11EN_W;
impl R {
    #[doc = "Bit 0 - Alternate function I/O clock enable"]
    #[inline(always)]
    pub fn afioen(&self) -> AFIOEN_R {
        AFIOEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port A clock enable"]
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port B clock enable"]
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O port C clock enable"]
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port D clock enable"]
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I/O port E clock enable"]
    #[inline(always)]
    pub fn iopeen(&self) -> IOPEEN_R {
        IOPEEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I/O port F clock enable"]
    #[inline(always)]
    pub fn iopfen(&self) -> IOPFEN_R {
        IOPFEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I/O port G clock enable"]
    #[inline(always)]
    pub fn iopgen(&self) -> IOPGEN_R {
        IOPGEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC 1 interface clock enable"]
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 19 - TIM9 Timer clock enable"]
    #[inline(always)]
    pub fn tim9en(&self) -> TIM9EN_R {
        TIM9EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TIM10 Timer clock enable"]
    #[inline(always)]
    pub fn tim10en(&self) -> TIM10EN_R {
        TIM10EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TIM11 Timer clock enable"]
    #[inline(always)]
    pub fn tim11en(&self) -> TIM11EN_R {
        TIM11EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alternate function I/O clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn afioen(&mut self) -> AFIOEN_W<APB2ENRrs> {
        AFIOEN_W::new(self, 0)
    }
    #[doc = "Bit 2 - I/O port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopaen(&mut self) -> IOPAEN_W<APB2ENRrs> {
        IOPAEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - I/O port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopben(&mut self) -> IOPBEN_W<APB2ENRrs> {
        IOPBEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - I/O port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopcen(&mut self) -> IOPCEN_W<APB2ENRrs> {
        IOPCEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - I/O port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopden(&mut self) -> IOPDEN_W<APB2ENRrs> {
        IOPDEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - I/O port E clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopeen(&mut self) -> IOPEEN_W<APB2ENRrs> {
        IOPEEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - I/O port F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopfen(&mut self) -> IOPFEN_W<APB2ENRrs> {
        IOPFEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - I/O port G clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopgen(&mut self) -> IOPGEN_W<APB2ENRrs> {
        IOPGEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - ADC 1 interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1en(&mut self) -> ADC1EN_W<APB2ENRrs> {
        ADC1EN_W::new(self, 9)
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<APB2ENRrs> {
        USART1EN_W::new(self, 14)
    }
    #[doc = "Bit 19 - TIM9 Timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim9en(&mut self) -> TIM9EN_W<APB2ENRrs> {
        TIM9EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - TIM10 Timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim10en(&mut self) -> TIM10EN_W<APB2ENRrs> {
        TIM10EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - TIM11 Timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim11en(&mut self) -> TIM11EN_W<APB2ENRrs> {
        TIM11EN_W::new(self, 21)
    }
}
#[doc = "APB2 peripheral clock enable register (RCC_APB2ENR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2ENRrs;
impl crate::RegisterSpec for APB2ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2enr::R`](R) reader structure"]
impl crate::Readable for APB2ENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb2enr::W`](W) writer structure"]
impl crate::Writable for APB2ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2ENR to value 0"]
impl crate::Resettable for APB2ENRrs {
    const RESET_VALUE: u32 = 0;
}
