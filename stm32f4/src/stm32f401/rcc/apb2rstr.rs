///Register `APB2RSTR` reader
pub type R = crate::R<APB2RSTRrs>;
///Register `APB2RSTR` writer
pub type W = crate::W<APB2RSTRrs>;
/**TIM1 reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1RST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<TIM1RST> for bool {
    #[inline(always)]
    fn from(variant: TIM1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM1RST` reader - TIM1 reset
pub type TIM1RST_R = crate::BitReader<TIM1RST>;
impl TIM1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIM1RST> {
        match self.bits {
            true => Some(TIM1RST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM1RST::Reset
    }
}
///Field `TIM1RST` writer - TIM1 reset
pub type TIM1RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM1RST>;
impl<'a, REG> TIM1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1RST::Reset)
    }
}
///Field `USART1RST` reader - USART1 reset
pub use TIM1RST_R as USART1RST_R;
///Field `USART6RST` reader - USART6 reset
pub use TIM1RST_R as USART6RST_R;
///Field `ADCRST` reader - ADC interface reset (common to all ADCs)
pub use TIM1RST_R as ADCRST_R;
///Field `SDIORST` reader - SDIO reset
pub use TIM1RST_R as SDIORST_R;
///Field `SPI1RST` reader - SPI 1 reset
pub use TIM1RST_R as SPI1RST_R;
///Field `SPI4RST` reader - SPI4 reset
pub use TIM1RST_R as SPI4RST_R;
///Field `SYSCFGRST` reader - System configuration controller reset
pub use TIM1RST_R as SYSCFGRST_R;
///Field `TIM9RST` reader - TIM9 reset
pub use TIM1RST_R as TIM9RST_R;
///Field `TIM10RST` reader - TIM10 reset
pub use TIM1RST_R as TIM10RST_R;
///Field `TIM11RST` reader - TIM11 reset
pub use TIM1RST_R as TIM11RST_R;
///Field `USART1RST` writer - USART1 reset
pub use TIM1RST_W as USART1RST_W;
///Field `USART6RST` writer - USART6 reset
pub use TIM1RST_W as USART6RST_W;
///Field `ADCRST` writer - ADC interface reset (common to all ADCs)
pub use TIM1RST_W as ADCRST_W;
///Field `SDIORST` writer - SDIO reset
pub use TIM1RST_W as SDIORST_W;
///Field `SPI1RST` writer - SPI 1 reset
pub use TIM1RST_W as SPI1RST_W;
///Field `SPI4RST` writer - SPI4 reset
pub use TIM1RST_W as SPI4RST_W;
///Field `SYSCFGRST` writer - System configuration controller reset
pub use TIM1RST_W as SYSCFGRST_W;
///Field `TIM9RST` writer - TIM9 reset
pub use TIM1RST_W as TIM9RST_W;
///Field `TIM10RST` writer - TIM10 reset
pub use TIM1RST_W as TIM10RST_W;
///Field `TIM11RST` writer - TIM11 reset
pub use TIM1RST_W as TIM11RST_W;
impl R {
    ///Bit 0 - TIM1 reset
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - USART1 reset
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - USART6 reset
    #[inline(always)]
    pub fn usart6rst(&self) -> USART6RST_R {
        USART6RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - ADC interface reset (common to all ADCs)
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - SDIO reset
    #[inline(always)]
    pub fn sdiorst(&self) -> SDIORST_R {
        SDIORST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI 1 reset
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SPI4 reset
    #[inline(always)]
    pub fn spi4rst(&self) -> SPI4RST_R {
        SPI4RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - System configuration controller reset
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM9 reset
    #[inline(always)]
    pub fn tim9rst(&self) -> TIM9RST_R {
        TIM9RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM10 reset
    #[inline(always)]
    pub fn tim10rst(&self) -> TIM10RST_R {
        TIM10RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM11 reset
    #[inline(always)]
    pub fn tim11rst(&self) -> TIM11RST_R {
        TIM11RST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RSTR")
            .field("tim1rst", &self.tim1rst())
            .field("tim11rst", &self.tim11rst())
            .field("tim10rst", &self.tim10rst())
            .field("tim9rst", &self.tim9rst())
            .field("syscfgrst", &self.syscfgrst())
            .field("spi1rst", &self.spi1rst())
            .field("sdiorst", &self.sdiorst())
            .field("adcrst", &self.adcrst())
            .field("usart6rst", &self.usart6rst())
            .field("usart1rst", &self.usart1rst())
            .field("spi4rst", &self.spi4rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM1 reset
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W<'_, APB2RSTRrs> {
        TIM1RST_W::new(self, 0)
    }
    ///Bit 4 - USART1 reset
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W<'_, APB2RSTRrs> {
        USART1RST_W::new(self, 4)
    }
    ///Bit 5 - USART6 reset
    #[inline(always)]
    pub fn usart6rst(&mut self) -> USART6RST_W<'_, APB2RSTRrs> {
        USART6RST_W::new(self, 5)
    }
    ///Bit 8 - ADC interface reset (common to all ADCs)
    #[inline(always)]
    pub fn adcrst(&mut self) -> ADCRST_W<'_, APB2RSTRrs> {
        ADCRST_W::new(self, 8)
    }
    ///Bit 11 - SDIO reset
    #[inline(always)]
    pub fn sdiorst(&mut self) -> SDIORST_W<'_, APB2RSTRrs> {
        SDIORST_W::new(self, 11)
    }
    ///Bit 12 - SPI 1 reset
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W<'_, APB2RSTRrs> {
        SPI1RST_W::new(self, 12)
    }
    ///Bit 13 - SPI4 reset
    #[inline(always)]
    pub fn spi4rst(&mut self) -> SPI4RST_W<'_, APB2RSTRrs> {
        SPI4RST_W::new(self, 13)
    }
    ///Bit 14 - System configuration controller reset
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<'_, APB2RSTRrs> {
        SYSCFGRST_W::new(self, 14)
    }
    ///Bit 16 - TIM9 reset
    #[inline(always)]
    pub fn tim9rst(&mut self) -> TIM9RST_W<'_, APB2RSTRrs> {
        TIM9RST_W::new(self, 16)
    }
    ///Bit 17 - TIM10 reset
    #[inline(always)]
    pub fn tim10rst(&mut self) -> TIM10RST_W<'_, APB2RSTRrs> {
        TIM10RST_W::new(self, 17)
    }
    ///Bit 18 - TIM11 reset
    #[inline(always)]
    pub fn tim11rst(&mut self) -> TIM11RST_W<'_, APB2RSTRrs> {
        TIM11RST_W::new(self, 18)
    }
}
/**APB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F401.html#RCC:APB2RSTR)*/
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
