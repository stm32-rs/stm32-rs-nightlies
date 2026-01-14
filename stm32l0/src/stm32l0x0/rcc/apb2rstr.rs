///Register `APB2RSTR` reader
pub type R = crate::R<APB2RSTRrs>;
///Register `APB2RSTR` writer
pub type W = crate::W<APB2RSTRrs>;
/**System configuration controller reset

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
///Field `SYSCFGRST` reader - System configuration controller reset
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
///Field `SYSCFGRST` writer - System configuration controller reset
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
///Field `TIM21RST` reader - TIM21 timer reset
pub use SYSCFGRST_R as TIM21RST_R;
///Field `TIM22RST` reader - TIM22 timer reset
pub use SYSCFGRST_R as TIM22RST_R;
///Field `ADCRST` reader - ADC interface reset
pub use SYSCFGRST_R as ADCRST_R;
///Field `SPI1RST` reader - SPI 1 reset
pub use SYSCFGRST_R as SPI1RST_R;
///Field `USART1RST` reader - USART1 reset
pub use SYSCFGRST_R as USART1RST_R;
///Field `DBGRST` reader - DBG reset
pub use SYSCFGRST_R as DBGRST_R;
///Field `TIM21RST` writer - TIM21 timer reset
pub use SYSCFGRST_W as TIM21RST_W;
///Field `TIM22RST` writer - TIM22 timer reset
pub use SYSCFGRST_W as TIM22RST_W;
///Field `ADCRST` writer - ADC interface reset
pub use SYSCFGRST_W as ADCRST_W;
///Field `SPI1RST` writer - SPI 1 reset
pub use SYSCFGRST_W as SPI1RST_W;
///Field `USART1RST` writer - USART1 reset
pub use SYSCFGRST_W as USART1RST_W;
///Field `DBGRST` writer - DBG reset
pub use SYSCFGRST_W as DBGRST_W;
impl R {
    ///Bit 0 - System configuration controller reset
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - TIM21 timer reset
    #[inline(always)]
    pub fn tim21rst(&self) -> TIM21RST_R {
        TIM21RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - TIM22 timer reset
    #[inline(always)]
    pub fn tim22rst(&self) -> TIM22RST_R {
        TIM22RST_R::new(((self.bits >> 5) & 1) != 0)
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
    ///Bit 22 - DBG reset
    #[inline(always)]
    pub fn dbgrst(&self) -> DBGRST_R {
        DBGRST_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RSTR")
            .field("syscfgrst", &self.syscfgrst())
            .field("dbgrst", &self.dbgrst())
            .field("usart1rst", &self.usart1rst())
            .field("spi1rst", &self.spi1rst())
            .field("adcrst", &self.adcrst())
            .field("tim22rst", &self.tim22rst())
            .field("tim21rst", &self.tim21rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - System configuration controller reset
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<'_, APB2RSTRrs> {
        SYSCFGRST_W::new(self, 0)
    }
    ///Bit 2 - TIM21 timer reset
    #[inline(always)]
    pub fn tim21rst(&mut self) -> TIM21RST_W<'_, APB2RSTRrs> {
        TIM21RST_W::new(self, 2)
    }
    ///Bit 5 - TIM22 timer reset
    #[inline(always)]
    pub fn tim22rst(&mut self) -> TIM22RST_W<'_, APB2RSTRrs> {
        TIM22RST_W::new(self, 5)
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
    ///Bit 22 - DBG reset
    #[inline(always)]
    pub fn dbgrst(&mut self) -> DBGRST_W<'_, APB2RSTRrs> {
        DBGRST_W::new(self, 22)
    }
}
/**APB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:APB2RSTR)*/
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
