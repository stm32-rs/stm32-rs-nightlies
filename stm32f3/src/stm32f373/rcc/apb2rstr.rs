///Register `APB2RSTR` reader
pub type R = crate::R<APB2RSTRrs>;
///Register `APB2RSTR` writer
pub type W = crate::W<APB2RSTRrs>;
/**SYSCFG and COMP reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGRST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<SYSCFGRST> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGRST) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGRST` reader - SYSCFG and COMP reset
pub type SYSCFGRST_R = crate::BitReader<SYSCFGRST>;
impl SYSCFGRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCFGRST> {
        match self.bits {
            true => Some(SYSCFGRST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYSCFGRST::Reset
    }
}
///Field `SYSCFGRST` writer - SYSCFG and COMP reset
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGRST>;
impl<'a, REG> SYSCFGRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGRST::Reset)
    }
}
///Field `ADCRST` reader - ADC interface reset
pub use SYSCFGRST_R as ADCRST_R;
///Field `SPI1RST` reader - SPI 1 reset
pub use SYSCFGRST_R as SPI1RST_R;
///Field `USART1RST` reader - USART1 reset
pub use SYSCFGRST_R as USART1RST_R;
///Field `TIM15RST` reader - TIM15 timer reset
pub use SYSCFGRST_R as TIM15RST_R;
///Field `TIM16RST` reader - TIM16 timer reset
pub use SYSCFGRST_R as TIM16RST_R;
///Field `TIM17RST` reader - TIM17 timer reset
pub use SYSCFGRST_R as TIM17RST_R;
///Field `TIM19RST` reader - TIM19 timer reset
pub use SYSCFGRST_R as TIM19RST_R;
///Field `SDADC1RST` reader - SDADC1 (Sigma delta ADC 1) reset
pub use SYSCFGRST_R as SDADC1RST_R;
///Field `SDADC2RST` reader - SDADC2 (Sigma delta ADC 2) reset
pub use SYSCFGRST_R as SDADC2RST_R;
///Field `SDADC3RST` reader - SDADC3 (Sigma delta ADC 3) reset
pub use SYSCFGRST_R as SDADC3RST_R;
///Field `ADCRST` writer - ADC interface reset
pub use SYSCFGRST_W as ADCRST_W;
///Field `SPI1RST` writer - SPI 1 reset
pub use SYSCFGRST_W as SPI1RST_W;
///Field `USART1RST` writer - USART1 reset
pub use SYSCFGRST_W as USART1RST_W;
///Field `TIM15RST` writer - TIM15 timer reset
pub use SYSCFGRST_W as TIM15RST_W;
///Field `TIM16RST` writer - TIM16 timer reset
pub use SYSCFGRST_W as TIM16RST_W;
///Field `TIM17RST` writer - TIM17 timer reset
pub use SYSCFGRST_W as TIM17RST_W;
///Field `TIM19RST` writer - TIM19 timer reset
pub use SYSCFGRST_W as TIM19RST_W;
///Field `SDADC1RST` writer - SDADC1 (Sigma delta ADC 1) reset
pub use SYSCFGRST_W as SDADC1RST_W;
///Field `SDADC2RST` writer - SDADC2 (Sigma delta ADC 2) reset
pub use SYSCFGRST_W as SDADC2RST_W;
///Field `SDADC3RST` writer - SDADC3 (Sigma delta ADC 3) reset
pub use SYSCFGRST_W as SDADC3RST_W;
impl R {
    ///Bit 0 - SYSCFG and COMP reset
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 9 - ADC interface reset
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - SPI 1 reset
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1 reset
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM15 timer reset
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 timer reset
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 timer reset
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TIM19 timer reset
    #[inline(always)]
    pub fn tim19rst(&self) -> TIM19RST_R {
        TIM19RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - SDADC1 (Sigma delta ADC 1) reset
    #[inline(always)]
    pub fn sdadc1rst(&self) -> SDADC1RST_R {
        SDADC1RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SDADC2 (Sigma delta ADC 2) reset
    #[inline(always)]
    pub fn sdadc2rst(&self) -> SDADC2RST_R {
        SDADC2RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SDADC3 (Sigma delta ADC 3) reset
    #[inline(always)]
    pub fn sdadc3rst(&self) -> SDADC3RST_R {
        SDADC3RST_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RSTR")
            .field("syscfgrst", &self.syscfgrst())
            .field("adcrst", &self.adcrst())
            .field("spi1rst", &self.spi1rst())
            .field("usart1rst", &self.usart1rst())
            .field("tim15rst", &self.tim15rst())
            .field("tim16rst", &self.tim16rst())
            .field("tim17rst", &self.tim17rst())
            .field("tim19rst", &self.tim19rst())
            .field("sdadc1rst", &self.sdadc1rst())
            .field("sdadc2rst", &self.sdadc2rst())
            .field("sdadc3rst", &self.sdadc3rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYSCFG and COMP reset
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<'_, APB2RSTRrs> {
        SYSCFGRST_W::new(self, 0)
    }
    ///Bit 9 - ADC interface reset
    #[inline(always)]
    pub fn adcrst(&mut self) -> ADCRST_W<'_, APB2RSTRrs> {
        ADCRST_W::new(self, 9)
    }
    ///Bit 12 - SPI 1 reset
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W<'_, APB2RSTRrs> {
        SPI1RST_W::new(self, 12)
    }
    ///Bit 14 - USART1 reset
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W<'_, APB2RSTRrs> {
        USART1RST_W::new(self, 14)
    }
    ///Bit 16 - TIM15 timer reset
    #[inline(always)]
    pub fn tim15rst(&mut self) -> TIM15RST_W<'_, APB2RSTRrs> {
        TIM15RST_W::new(self, 16)
    }
    ///Bit 17 - TIM16 timer reset
    #[inline(always)]
    pub fn tim16rst(&mut self) -> TIM16RST_W<'_, APB2RSTRrs> {
        TIM16RST_W::new(self, 17)
    }
    ///Bit 18 - TIM17 timer reset
    #[inline(always)]
    pub fn tim17rst(&mut self) -> TIM17RST_W<'_, APB2RSTRrs> {
        TIM17RST_W::new(self, 18)
    }
    ///Bit 19 - TIM19 timer reset
    #[inline(always)]
    pub fn tim19rst(&mut self) -> TIM19RST_W<'_, APB2RSTRrs> {
        TIM19RST_W::new(self, 19)
    }
    ///Bit 24 - SDADC1 (Sigma delta ADC 1) reset
    #[inline(always)]
    pub fn sdadc1rst(&mut self) -> SDADC1RST_W<'_, APB2RSTRrs> {
        SDADC1RST_W::new(self, 24)
    }
    ///Bit 25 - SDADC2 (Sigma delta ADC 2) reset
    #[inline(always)]
    pub fn sdadc2rst(&mut self) -> SDADC2RST_W<'_, APB2RSTRrs> {
        SDADC2RST_W::new(self, 25)
    }
    ///Bit 26 - SDADC3 (Sigma delta ADC 3) reset
    #[inline(always)]
    pub fn sdadc3rst(&mut self) -> SDADC3RST_W<'_, APB2RSTRrs> {
        SDADC3RST_W::new(self, 26)
    }
}
/**APB2 peripheral reset register (RCC_APB2RSTR)

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#RCC:APB2RSTR)*/
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
