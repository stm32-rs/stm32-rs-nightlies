///Register `APB2RSTR` reader
pub type R = crate::R<APB2RSTRrs>;
///Register `APB2RSTR` writer
pub type W = crate::W<APB2RSTRrs>;
/**SYSCFGRST

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGRSTW {
    ///1: Reset the module
    Reset = 1,
}
impl From<SYSCFGRSTW> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGRSTW) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGRST` reader - SYSCFGRST
pub type SYSCFGRST_R = crate::BitReader<SYSCFGRSTW>;
impl SYSCFGRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCFGRSTW> {
        match self.bits {
            true => Some(SYSCFGRSTW::Reset),
            _ => None,
        }
    }
    ///Reset the module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYSCFGRSTW::Reset
    }
}
///Field `SYSCFGRST` writer - SYSCFGRST
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGRSTW>;
impl<'a, REG> SYSCFGRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGRSTW::Reset)
    }
}
///Field `TIM9RST` reader - TIM9RST
pub use SYSCFGRST_R as TIM9RST_R;
///Field `TM10RST` reader - TM10RST
pub use SYSCFGRST_R as TM10RST_R;
///Field `TM11RST` reader - TM11RST
pub use SYSCFGRST_R as TM11RST_R;
///Field `ADC1RST` reader - ADC1RST
pub use SYSCFGRST_R as ADC1RST_R;
///Field `SDIORST` reader - SDIORST
pub use SYSCFGRST_R as SDIORST_R;
///Field `SPI1RST` reader - SPI1RST
pub use SYSCFGRST_R as SPI1RST_R;
///Field `USART1RST` reader - USART1RST
pub use SYSCFGRST_R as USART1RST_R;
///Field `TIM9RST` writer - TIM9RST
pub use SYSCFGRST_W as TIM9RST_W;
///Field `TM10RST` writer - TM10RST
pub use SYSCFGRST_W as TM10RST_W;
///Field `TM11RST` writer - TM11RST
pub use SYSCFGRST_W as TM11RST_W;
///Field `ADC1RST` writer - ADC1RST
pub use SYSCFGRST_W as ADC1RST_W;
///Field `SDIORST` writer - SDIORST
pub use SYSCFGRST_W as SDIORST_W;
///Field `SPI1RST` writer - SPI1RST
pub use SYSCFGRST_W as SPI1RST_W;
///Field `USART1RST` writer - USART1RST
pub use SYSCFGRST_W as USART1RST_W;
impl R {
    ///Bit 0 - SYSCFGRST
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - TIM9RST
    #[inline(always)]
    pub fn tim9rst(&self) -> TIM9RST_R {
        TIM9RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TM10RST
    #[inline(always)]
    pub fn tm10rst(&self) -> TM10RST_R {
        TM10RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TM11RST
    #[inline(always)]
    pub fn tm11rst(&self) -> TM11RST_R {
        TM11RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 9 - ADC1RST
    #[inline(always)]
    pub fn adc1rst(&self) -> ADC1RST_R {
        ADC1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - SDIORST
    #[inline(always)]
    pub fn sdiorst(&self) -> SDIORST_R {
        SDIORST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1RST
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1RST
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RSTR")
            .field("syscfgrst", &self.syscfgrst())
            .field("usart1rst", &self.usart1rst())
            .field("spi1rst", &self.spi1rst())
            .field("sdiorst", &self.sdiorst())
            .field("adc1rst", &self.adc1rst())
            .field("tm11rst", &self.tm11rst())
            .field("tm10rst", &self.tm10rst())
            .field("tim9rst", &self.tim9rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYSCFGRST
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<'_, APB2RSTRrs> {
        SYSCFGRST_W::new(self, 0)
    }
    ///Bit 2 - TIM9RST
    #[inline(always)]
    pub fn tim9rst(&mut self) -> TIM9RST_W<'_, APB2RSTRrs> {
        TIM9RST_W::new(self, 2)
    }
    ///Bit 3 - TM10RST
    #[inline(always)]
    pub fn tm10rst(&mut self) -> TM10RST_W<'_, APB2RSTRrs> {
        TM10RST_W::new(self, 3)
    }
    ///Bit 4 - TM11RST
    #[inline(always)]
    pub fn tm11rst(&mut self) -> TM11RST_W<'_, APB2RSTRrs> {
        TM11RST_W::new(self, 4)
    }
    ///Bit 9 - ADC1RST
    #[inline(always)]
    pub fn adc1rst(&mut self) -> ADC1RST_W<'_, APB2RSTRrs> {
        ADC1RST_W::new(self, 9)
    }
    ///Bit 11 - SDIORST
    #[inline(always)]
    pub fn sdiorst(&mut self) -> SDIORST_W<'_, APB2RSTRrs> {
        SDIORST_W::new(self, 11)
    }
    ///Bit 12 - SPI1RST
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W<'_, APB2RSTRrs> {
        SPI1RST_W::new(self, 12)
    }
    ///Bit 14 - USART1RST
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W<'_, APB2RSTRrs> {
        USART1RST_W::new(self, 14)
    }
}
/**APB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RCC:APB2RSTR)*/
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
