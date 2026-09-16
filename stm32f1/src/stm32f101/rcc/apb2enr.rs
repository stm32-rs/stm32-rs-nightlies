///Register `APB2ENR` reader
pub type R = crate::R<APB2ENRrs>;
///Register `APB2ENR` writer
pub type W = crate::W<APB2ENRrs>;
/**Alternate function I/O clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFIOEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<AFIOEN> for bool {
    #[inline(always)]
    fn from(variant: AFIOEN) -> Self {
        variant as u8 != 0
    }
}
///Field `AFIOEN` reader - Alternate function I/O clock enable
pub type AFIOEN_R = crate::BitReader<AFIOEN>;
impl AFIOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFIOEN {
        match self.bits {
            false => AFIOEN::Disabled,
            true => AFIOEN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AFIOEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AFIOEN::Enabled
    }
}
///Field `AFIOEN` writer - Alternate function I/O clock enable
pub type AFIOEN_W<'a, REG> = crate::BitWriter<'a, REG, AFIOEN>;
impl<'a, REG> AFIOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AFIOEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AFIOEN::Enabled)
    }
}
///Field `IOPAEN` reader - I/O port A clock enable
pub use AFIOEN_R as IOPAEN_R;
///Field `IOPBEN` reader - I/O port B clock enable
pub use AFIOEN_R as IOPBEN_R;
///Field `IOPCEN` reader - I/O port C clock enable
pub use AFIOEN_R as IOPCEN_R;
///Field `IOPDEN` reader - I/O port D clock enable
pub use AFIOEN_R as IOPDEN_R;
///Field `IOPEEN` reader - I/O port E clock enable
pub use AFIOEN_R as IOPEEN_R;
///Field `IOPFEN` reader - I/O port F clock enable
pub use AFIOEN_R as IOPFEN_R;
///Field `IOPGEN` reader - I/O port G clock enable
pub use AFIOEN_R as IOPGEN_R;
///Field `ADC1EN` reader - ADC 1 interface clock enable
pub use AFIOEN_R as ADC1EN_R;
///Field `SPI1EN` reader - SPI 1 clock enable
pub use AFIOEN_R as SPI1EN_R;
///Field `USART1EN` reader - USART1 clock enable
pub use AFIOEN_R as USART1EN_R;
///Field `TIM9EN` reader - TIM9 Timer clock enable
pub use AFIOEN_R as TIM9EN_R;
///Field `TIM10EN` reader - TIM10 Timer clock enable
pub use AFIOEN_R as TIM10EN_R;
///Field `TIM11EN` reader - TIM11 Timer clock enable
pub use AFIOEN_R as TIM11EN_R;
///Field `IOPAEN` writer - I/O port A clock enable
pub use AFIOEN_W as IOPAEN_W;
///Field `IOPBEN` writer - I/O port B clock enable
pub use AFIOEN_W as IOPBEN_W;
///Field `IOPCEN` writer - I/O port C clock enable
pub use AFIOEN_W as IOPCEN_W;
///Field `IOPDEN` writer - I/O port D clock enable
pub use AFIOEN_W as IOPDEN_W;
///Field `IOPEEN` writer - I/O port E clock enable
pub use AFIOEN_W as IOPEEN_W;
///Field `IOPFEN` writer - I/O port F clock enable
pub use AFIOEN_W as IOPFEN_W;
///Field `IOPGEN` writer - I/O port G clock enable
pub use AFIOEN_W as IOPGEN_W;
///Field `ADC1EN` writer - ADC 1 interface clock enable
pub use AFIOEN_W as ADC1EN_W;
///Field `SPI1EN` writer - SPI 1 clock enable
pub use AFIOEN_W as SPI1EN_W;
///Field `USART1EN` writer - USART1 clock enable
pub use AFIOEN_W as USART1EN_W;
///Field `TIM9EN` writer - TIM9 Timer clock enable
pub use AFIOEN_W as TIM9EN_W;
///Field `TIM10EN` writer - TIM10 Timer clock enable
pub use AFIOEN_W as TIM10EN_W;
///Field `TIM11EN` writer - TIM11 Timer clock enable
pub use AFIOEN_W as TIM11EN_W;
impl R {
    ///Bit 0 - Alternate function I/O clock enable
    #[inline(always)]
    pub fn afioen(&self) -> AFIOEN_R {
        AFIOEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - I/O port A clock enable
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port B clock enable
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - I/O port C clock enable
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I/O port D clock enable
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - I/O port E clock enable
    #[inline(always)]
    pub fn iopeen(&self) -> IOPEEN_R {
        IOPEEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I/O port F clock enable
    #[inline(always)]
    pub fn iopfen(&self) -> IOPFEN_R {
        IOPFEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - I/O port G clock enable
    #[inline(always)]
    pub fn iopgen(&self) -> IOPGEN_R {
        IOPGEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ADC 1 interface clock enable
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - SPI 1 clock enable
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1 clock enable
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 19 - TIM9 Timer clock enable
    #[inline(always)]
    pub fn tim9en(&self) -> TIM9EN_R {
        TIM9EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - TIM10 Timer clock enable
    #[inline(always)]
    pub fn tim10en(&self) -> TIM10EN_R {
        TIM10EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TIM11 Timer clock enable
    #[inline(always)]
    pub fn tim11en(&self) -> TIM11EN_R {
        TIM11EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2ENR")
            .field("afioen", &self.afioen())
            .field("iopaen", &self.iopaen())
            .field("iopben", &self.iopben())
            .field("iopcen", &self.iopcen())
            .field("iopden", &self.iopden())
            .field("iopeen", &self.iopeen())
            .field("iopfen", &self.iopfen())
            .field("iopgen", &self.iopgen())
            .field("adc1en", &self.adc1en())
            .field("spi1en", &self.spi1en())
            .field("usart1en", &self.usart1en())
            .field("tim9en", &self.tim9en())
            .field("tim10en", &self.tim10en())
            .field("tim11en", &self.tim11en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Alternate function I/O clock enable
    #[inline(always)]
    pub fn afioen(&mut self) -> AFIOEN_W<'_, APB2ENRrs> {
        AFIOEN_W::new(self, 0)
    }
    ///Bit 2 - I/O port A clock enable
    #[inline(always)]
    pub fn iopaen(&mut self) -> IOPAEN_W<'_, APB2ENRrs> {
        IOPAEN_W::new(self, 2)
    }
    ///Bit 3 - I/O port B clock enable
    #[inline(always)]
    pub fn iopben(&mut self) -> IOPBEN_W<'_, APB2ENRrs> {
        IOPBEN_W::new(self, 3)
    }
    ///Bit 4 - I/O port C clock enable
    #[inline(always)]
    pub fn iopcen(&mut self) -> IOPCEN_W<'_, APB2ENRrs> {
        IOPCEN_W::new(self, 4)
    }
    ///Bit 5 - I/O port D clock enable
    #[inline(always)]
    pub fn iopden(&mut self) -> IOPDEN_W<'_, APB2ENRrs> {
        IOPDEN_W::new(self, 5)
    }
    ///Bit 6 - I/O port E clock enable
    #[inline(always)]
    pub fn iopeen(&mut self) -> IOPEEN_W<'_, APB2ENRrs> {
        IOPEEN_W::new(self, 6)
    }
    ///Bit 7 - I/O port F clock enable
    #[inline(always)]
    pub fn iopfen(&mut self) -> IOPFEN_W<'_, APB2ENRrs> {
        IOPFEN_W::new(self, 7)
    }
    ///Bit 8 - I/O port G clock enable
    #[inline(always)]
    pub fn iopgen(&mut self) -> IOPGEN_W<'_, APB2ENRrs> {
        IOPGEN_W::new(self, 8)
    }
    ///Bit 9 - ADC 1 interface clock enable
    #[inline(always)]
    pub fn adc1en(&mut self) -> ADC1EN_W<'_, APB2ENRrs> {
        ADC1EN_W::new(self, 9)
    }
    ///Bit 12 - SPI 1 clock enable
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W<'_, APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    ///Bit 14 - USART1 clock enable
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<'_, APB2ENRrs> {
        USART1EN_W::new(self, 14)
    }
    ///Bit 19 - TIM9 Timer clock enable
    #[inline(always)]
    pub fn tim9en(&mut self) -> TIM9EN_W<'_, APB2ENRrs> {
        TIM9EN_W::new(self, 19)
    }
    ///Bit 20 - TIM10 Timer clock enable
    #[inline(always)]
    pub fn tim10en(&mut self) -> TIM10EN_W<'_, APB2ENRrs> {
        TIM10EN_W::new(self, 20)
    }
    ///Bit 21 - TIM11 Timer clock enable
    #[inline(always)]
    pub fn tim11en(&mut self) -> TIM11EN_W<'_, APB2ENRrs> {
        TIM11EN_W::new(self, 21)
    }
}
/**APB2 peripheral clock enable register (RCC_APB2ENR)

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#RCC:APB2ENR)*/
pub struct APB2ENRrs;
impl crate::RegisterSpec for APB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2enr::R`](R) reader structure
impl crate::Readable for APB2ENRrs {}
///`write(|w| ..)` method takes [`apb2enr::W`](W) writer structure
impl crate::Writable for APB2ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2ENR to value 0
impl crate::Resettable for APB2ENRrs {}
