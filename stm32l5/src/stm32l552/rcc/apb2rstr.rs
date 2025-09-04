///Register `APB2RSTR` reader
pub type R = crate::R<APB2RSTRrs>;
///Register `APB2RSTR` writer
pub type W = crate::W<APB2RSTRrs>;
/**System configuration (SYSCFG) reset

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
///Field `SYSCFGRST` reader - System configuration (SYSCFG) reset
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
///Field `SYSCFGRST` writer - System configuration (SYSCFG) reset
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
///Field `TIM1RST` reader - TIM1 timer reset
pub use SYSCFGRST_R as TIM1RST_R;
///Field `SPI1RST` reader - SPI1 reset
pub use SYSCFGRST_R as SPI1RST_R;
///Field `TIM8RST` reader - TIM8 timer reset
pub use SYSCFGRST_R as TIM8RST_R;
///Field `USART1RST` reader - USART1 reset
pub use SYSCFGRST_R as USART1RST_R;
///Field `TIM15RST` reader - TIM15 timer reset
pub use SYSCFGRST_R as TIM15RST_R;
///Field `TIM16RST` reader - TIM16 timer reset
pub use SYSCFGRST_R as TIM16RST_R;
///Field `TIM17RST` reader - TIM17 timer reset
pub use SYSCFGRST_R as TIM17RST_R;
///Field `SAI1RST` reader - Serial audio interface 1 (SAI1) reset
pub use SYSCFGRST_R as SAI1RST_R;
///Field `SAI2RST` reader - Serial audio interface 2 (SAI2) reset
pub use SYSCFGRST_R as SAI2RST_R;
///Field `DFSDM1RST` reader - Digital filters for sigma-delata modulators (DFSDM) reset
pub use SYSCFGRST_R as DFSDM1RST_R;
///Field `TIM1RST` writer - TIM1 timer reset
pub use SYSCFGRST_W as TIM1RST_W;
///Field `SPI1RST` writer - SPI1 reset
pub use SYSCFGRST_W as SPI1RST_W;
///Field `TIM8RST` writer - TIM8 timer reset
pub use SYSCFGRST_W as TIM8RST_W;
///Field `USART1RST` writer - USART1 reset
pub use SYSCFGRST_W as USART1RST_W;
///Field `TIM15RST` writer - TIM15 timer reset
pub use SYSCFGRST_W as TIM15RST_W;
///Field `TIM16RST` writer - TIM16 timer reset
pub use SYSCFGRST_W as TIM16RST_W;
///Field `TIM17RST` writer - TIM17 timer reset
pub use SYSCFGRST_W as TIM17RST_W;
///Field `SAI1RST` writer - Serial audio interface 1 (SAI1) reset
pub use SYSCFGRST_W as SAI1RST_W;
///Field `SAI2RST` writer - Serial audio interface 2 (SAI2) reset
pub use SYSCFGRST_W as SAI2RST_W;
///Field `DFSDM1RST` writer - Digital filters for sigma-delata modulators (DFSDM) reset
pub use SYSCFGRST_W as DFSDM1RST_W;
impl R {
    ///Bit 0 - System configuration (SYSCFG) reset
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 11 - TIM1 timer reset
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 reset
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
    ///Bit 21 - Serial audio interface 1 (SAI1) reset
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Serial audio interface 2 (SAI2) reset
    #[inline(always)]
    pub fn sai2rst(&self) -> SAI2RST_R {
        SAI2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Digital filters for sigma-delata modulators (DFSDM) reset
    #[inline(always)]
    pub fn dfsdm1rst(&self) -> DFSDM1RST_R {
        DFSDM1RST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RSTR")
            .field("syscfgrst", &self.syscfgrst())
            .field("tim1rst", &self.tim1rst())
            .field("spi1rst", &self.spi1rst())
            .field("tim8rst", &self.tim8rst())
            .field("usart1rst", &self.usart1rst())
            .field("tim15rst", &self.tim15rst())
            .field("tim16rst", &self.tim16rst())
            .field("tim17rst", &self.tim17rst())
            .field("sai1rst", &self.sai1rst())
            .field("sai2rst", &self.sai2rst())
            .field("dfsdm1rst", &self.dfsdm1rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - System configuration (SYSCFG) reset
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<APB2RSTRrs> {
        SYSCFGRST_W::new(self, 0)
    }
    ///Bit 11 - TIM1 timer reset
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W<APB2RSTRrs> {
        TIM1RST_W::new(self, 11)
    }
    ///Bit 12 - SPI1 reset
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W<APB2RSTRrs> {
        SPI1RST_W::new(self, 12)
    }
    ///Bit 13 - TIM8 timer reset
    #[inline(always)]
    pub fn tim8rst(&mut self) -> TIM8RST_W<APB2RSTRrs> {
        TIM8RST_W::new(self, 13)
    }
    ///Bit 14 - USART1 reset
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W<APB2RSTRrs> {
        USART1RST_W::new(self, 14)
    }
    ///Bit 16 - TIM15 timer reset
    #[inline(always)]
    pub fn tim15rst(&mut self) -> TIM15RST_W<APB2RSTRrs> {
        TIM15RST_W::new(self, 16)
    }
    ///Bit 17 - TIM16 timer reset
    #[inline(always)]
    pub fn tim16rst(&mut self) -> TIM16RST_W<APB2RSTRrs> {
        TIM16RST_W::new(self, 17)
    }
    ///Bit 18 - TIM17 timer reset
    #[inline(always)]
    pub fn tim17rst(&mut self) -> TIM17RST_W<APB2RSTRrs> {
        TIM17RST_W::new(self, 18)
    }
    ///Bit 21 - Serial audio interface 1 (SAI1) reset
    #[inline(always)]
    pub fn sai1rst(&mut self) -> SAI1RST_W<APB2RSTRrs> {
        SAI1RST_W::new(self, 21)
    }
    ///Bit 22 - Serial audio interface 2 (SAI2) reset
    #[inline(always)]
    pub fn sai2rst(&mut self) -> SAI2RST_W<APB2RSTRrs> {
        SAI2RST_W::new(self, 22)
    }
    ///Bit 24 - Digital filters for sigma-delata modulators (DFSDM) reset
    #[inline(always)]
    pub fn dfsdm1rst(&mut self) -> DFSDM1RST_W<APB2RSTRrs> {
        DFSDM1RST_W::new(self, 24)
    }
}
/**APB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#RCC:APB2RSTR)*/
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
