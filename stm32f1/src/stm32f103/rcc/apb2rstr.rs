///Register `APB2RSTR` reader
pub type R = crate::R<APB2RSTRrs>;
///Register `APB2RSTR` writer
pub type W = crate::W<APB2RSTRrs>;
/**Alternate function I/O reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFIORST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<AFIORST> for bool {
    #[inline(always)]
    fn from(variant: AFIORST) -> Self {
        variant as u8 != 0
    }
}
///Field `AFIORST` reader - Alternate function I/O reset
pub type AFIORST_R = crate::BitReader<AFIORST>;
impl AFIORST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<AFIORST> {
        match self.bits {
            true => Some(AFIORST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == AFIORST::Reset
    }
}
///Field `AFIORST` writer - Alternate function I/O reset
pub type AFIORST_W<'a, REG> = crate::BitWriter<'a, REG, AFIORST>;
impl<'a, REG> AFIORST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(AFIORST::Reset)
    }
}
///Field `IOPARST` reader - IO port A reset
pub use AFIORST_R as IOPARST_R;
///Field `IOPBRST` reader - IO port B reset
pub use AFIORST_R as IOPBRST_R;
///Field `IOPCRST` reader - IO port C reset
pub use AFIORST_R as IOPCRST_R;
///Field `IOPDRST` reader - IO port D reset
pub use AFIORST_R as IOPDRST_R;
///Field `IOPERST` reader - IO port E reset
pub use AFIORST_R as IOPERST_R;
///Field `IOPFRST` reader - IO port F reset
pub use AFIORST_R as IOPFRST_R;
///Field `IOPGRST` reader - IO port G reset
pub use AFIORST_R as IOPGRST_R;
///Field `ADC1RST` reader - ADC 1 interface reset
pub use AFIORST_R as ADC1RST_R;
///Field `ADC2RST` reader - ADC 2 interface reset
pub use AFIORST_R as ADC2RST_R;
///Field `TIM1RST` reader - TIM1 timer reset
pub use AFIORST_R as TIM1RST_R;
///Field `SPI1RST` reader - SPI 1 reset
pub use AFIORST_R as SPI1RST_R;
///Field `TIM8RST` reader - TIM8 timer reset
pub use AFIORST_R as TIM8RST_R;
///Field `USART1RST` reader - USART1 reset
pub use AFIORST_R as USART1RST_R;
///Field `ADC3RST` reader - ADC 3 interface reset
pub use AFIORST_R as ADC3RST_R;
///Field `TIM9RST` reader - TIM9 timer reset
pub use AFIORST_R as TIM9RST_R;
///Field `TIM10RST` reader - TIM10 timer reset
pub use AFIORST_R as TIM10RST_R;
///Field `TIM11RST` reader - TIM11 timer reset
pub use AFIORST_R as TIM11RST_R;
///Field `IOPARST` writer - IO port A reset
pub use AFIORST_W as IOPARST_W;
///Field `IOPBRST` writer - IO port B reset
pub use AFIORST_W as IOPBRST_W;
///Field `IOPCRST` writer - IO port C reset
pub use AFIORST_W as IOPCRST_W;
///Field `IOPDRST` writer - IO port D reset
pub use AFIORST_W as IOPDRST_W;
///Field `IOPERST` writer - IO port E reset
pub use AFIORST_W as IOPERST_W;
///Field `IOPFRST` writer - IO port F reset
pub use AFIORST_W as IOPFRST_W;
///Field `IOPGRST` writer - IO port G reset
pub use AFIORST_W as IOPGRST_W;
///Field `ADC1RST` writer - ADC 1 interface reset
pub use AFIORST_W as ADC1RST_W;
///Field `ADC2RST` writer - ADC 2 interface reset
pub use AFIORST_W as ADC2RST_W;
///Field `TIM1RST` writer - TIM1 timer reset
pub use AFIORST_W as TIM1RST_W;
///Field `SPI1RST` writer - SPI 1 reset
pub use AFIORST_W as SPI1RST_W;
///Field `TIM8RST` writer - TIM8 timer reset
pub use AFIORST_W as TIM8RST_W;
///Field `USART1RST` writer - USART1 reset
pub use AFIORST_W as USART1RST_W;
///Field `ADC3RST` writer - ADC 3 interface reset
pub use AFIORST_W as ADC3RST_W;
///Field `TIM9RST` writer - TIM9 timer reset
pub use AFIORST_W as TIM9RST_W;
///Field `TIM10RST` writer - TIM10 timer reset
pub use AFIORST_W as TIM10RST_W;
///Field `TIM11RST` writer - TIM11 timer reset
pub use AFIORST_W as TIM11RST_W;
impl R {
    ///Bit 0 - Alternate function I/O reset
    #[inline(always)]
    pub fn afiorst(&self) -> AFIORST_R {
        AFIORST_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - IO port A reset
    #[inline(always)]
    pub fn ioparst(&self) -> IOPARST_R {
        IOPARST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port B reset
    #[inline(always)]
    pub fn iopbrst(&self) -> IOPBRST_R {
        IOPBRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port C reset
    #[inline(always)]
    pub fn iopcrst(&self) -> IOPCRST_R {
        IOPCRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port D reset
    #[inline(always)]
    pub fn iopdrst(&self) -> IOPDRST_R {
        IOPDRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port E reset
    #[inline(always)]
    pub fn ioperst(&self) -> IOPERST_R {
        IOPERST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port F reset
    #[inline(always)]
    pub fn iopfrst(&self) -> IOPFRST_R {
        IOPFRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IO port G reset
    #[inline(always)]
    pub fn iopgrst(&self) -> IOPGRST_R {
        IOPGRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ADC 1 interface reset
    #[inline(always)]
    pub fn adc1rst(&self) -> ADC1RST_R {
        ADC1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADC 2 interface reset
    #[inline(always)]
    pub fn adc2rst(&self) -> ADC2RST_R {
        ADC2RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TIM1 timer reset
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI 1 reset
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TIM8 timer reset
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - USART1 reset
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ADC 3 interface reset
    #[inline(always)]
    pub fn adc3rst(&self) -> ADC3RST_R {
        ADC3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 19 - TIM9 timer reset
    #[inline(always)]
    pub fn tim9rst(&self) -> TIM9RST_R {
        TIM9RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - TIM10 timer reset
    #[inline(always)]
    pub fn tim10rst(&self) -> TIM10RST_R {
        TIM10RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TIM11 timer reset
    #[inline(always)]
    pub fn tim11rst(&self) -> TIM11RST_R {
        TIM11RST_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RSTR")
            .field("afiorst", &self.afiorst())
            .field("ioparst", &self.ioparst())
            .field("iopbrst", &self.iopbrst())
            .field("iopcrst", &self.iopcrst())
            .field("iopdrst", &self.iopdrst())
            .field("ioperst", &self.ioperst())
            .field("iopfrst", &self.iopfrst())
            .field("iopgrst", &self.iopgrst())
            .field("adc1rst", &self.adc1rst())
            .field("adc2rst", &self.adc2rst())
            .field("tim1rst", &self.tim1rst())
            .field("spi1rst", &self.spi1rst())
            .field("tim8rst", &self.tim8rst())
            .field("usart1rst", &self.usart1rst())
            .field("adc3rst", &self.adc3rst())
            .field("tim9rst", &self.tim9rst())
            .field("tim10rst", &self.tim10rst())
            .field("tim11rst", &self.tim11rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Alternate function I/O reset
    #[inline(always)]
    pub fn afiorst(&mut self) -> AFIORST_W<'_, APB2RSTRrs> {
        AFIORST_W::new(self, 0)
    }
    ///Bit 2 - IO port A reset
    #[inline(always)]
    pub fn ioparst(&mut self) -> IOPARST_W<'_, APB2RSTRrs> {
        IOPARST_W::new(self, 2)
    }
    ///Bit 3 - IO port B reset
    #[inline(always)]
    pub fn iopbrst(&mut self) -> IOPBRST_W<'_, APB2RSTRrs> {
        IOPBRST_W::new(self, 3)
    }
    ///Bit 4 - IO port C reset
    #[inline(always)]
    pub fn iopcrst(&mut self) -> IOPCRST_W<'_, APB2RSTRrs> {
        IOPCRST_W::new(self, 4)
    }
    ///Bit 5 - IO port D reset
    #[inline(always)]
    pub fn iopdrst(&mut self) -> IOPDRST_W<'_, APB2RSTRrs> {
        IOPDRST_W::new(self, 5)
    }
    ///Bit 6 - IO port E reset
    #[inline(always)]
    pub fn ioperst(&mut self) -> IOPERST_W<'_, APB2RSTRrs> {
        IOPERST_W::new(self, 6)
    }
    ///Bit 7 - IO port F reset
    #[inline(always)]
    pub fn iopfrst(&mut self) -> IOPFRST_W<'_, APB2RSTRrs> {
        IOPFRST_W::new(self, 7)
    }
    ///Bit 8 - IO port G reset
    #[inline(always)]
    pub fn iopgrst(&mut self) -> IOPGRST_W<'_, APB2RSTRrs> {
        IOPGRST_W::new(self, 8)
    }
    ///Bit 9 - ADC 1 interface reset
    #[inline(always)]
    pub fn adc1rst(&mut self) -> ADC1RST_W<'_, APB2RSTRrs> {
        ADC1RST_W::new(self, 9)
    }
    ///Bit 10 - ADC 2 interface reset
    #[inline(always)]
    pub fn adc2rst(&mut self) -> ADC2RST_W<'_, APB2RSTRrs> {
        ADC2RST_W::new(self, 10)
    }
    ///Bit 11 - TIM1 timer reset
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W<'_, APB2RSTRrs> {
        TIM1RST_W::new(self, 11)
    }
    ///Bit 12 - SPI 1 reset
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W<'_, APB2RSTRrs> {
        SPI1RST_W::new(self, 12)
    }
    ///Bit 13 - TIM8 timer reset
    #[inline(always)]
    pub fn tim8rst(&mut self) -> TIM8RST_W<'_, APB2RSTRrs> {
        TIM8RST_W::new(self, 13)
    }
    ///Bit 14 - USART1 reset
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W<'_, APB2RSTRrs> {
        USART1RST_W::new(self, 14)
    }
    ///Bit 15 - ADC 3 interface reset
    #[inline(always)]
    pub fn adc3rst(&mut self) -> ADC3RST_W<'_, APB2RSTRrs> {
        ADC3RST_W::new(self, 15)
    }
    ///Bit 19 - TIM9 timer reset
    #[inline(always)]
    pub fn tim9rst(&mut self) -> TIM9RST_W<'_, APB2RSTRrs> {
        TIM9RST_W::new(self, 19)
    }
    ///Bit 20 - TIM10 timer reset
    #[inline(always)]
    pub fn tim10rst(&mut self) -> TIM10RST_W<'_, APB2RSTRrs> {
        TIM10RST_W::new(self, 20)
    }
    ///Bit 21 - TIM11 timer reset
    #[inline(always)]
    pub fn tim11rst(&mut self) -> TIM11RST_W<'_, APB2RSTRrs> {
        TIM11RST_W::new(self, 21)
    }
}
/**APB2 peripheral reset register (RCC_APB2RSTR)

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#RCC:APB2RSTR)*/
pub struct APB2RSTRrs;
impl crate::RegisterSpec for APB2RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2rstr::R`](R) reader structure
impl crate::Readable for APB2RSTRrs {}
///`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure
impl crate::Writable for APB2RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2RSTR to value 0
impl crate::Resettable for APB2RSTRrs {}
