///Register `APB2RSTR` reader
pub type R = crate::R<APB2RSTRrs>;
///Register `APB2RSTR` writer
pub type W = crate::W<APB2RSTRrs>;
/**TIM1 block reset

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
///Field `TIM1RST` reader - TIM1 block reset
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
///Field `TIM1RST` writer - TIM1 block reset
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
///Field `TIM8RST` reader - TIM8 block reset
pub use TIM1RST_R as TIM8RST_R;
///Field `USART1RST` reader - USART1 block reset
pub use TIM1RST_R as USART1RST_R;
///Field `USART6RST` reader - USART6 block reset
pub use TIM1RST_R as USART6RST_R;
///Field `SPI1RST` reader - SPI1 block reset
pub use TIM1RST_R as SPI1RST_R;
///Field `SPI4RST` reader - SPI4 block reset
pub use TIM1RST_R as SPI4RST_R;
///Field `TIM15RST` reader - TIM15 block reset
pub use TIM1RST_R as TIM15RST_R;
///Field `TIM16RST` reader - TIM16 block reset
pub use TIM1RST_R as TIM16RST_R;
///Field `TIM17RST` reader - TIM17 block reset
pub use TIM1RST_R as TIM17RST_R;
///Field `SPI5RST` reader - SPI5 block reset
pub use TIM1RST_R as SPI5RST_R;
///Field `SAI1RST` reader - SAI1 block reset
pub use TIM1RST_R as SAI1RST_R;
///Field `SAI2RST` reader - SAI2 block reset
pub use TIM1RST_R as SAI2RST_R;
///Field `SAI3RST` reader - SAI3 block reset
pub use TIM1RST_R as SAI3RST_R;
///Field `DFSDM1RST` reader - DFSDM1 block reset
pub use TIM1RST_R as DFSDM1RST_R;
///Field `HRTIMRST` reader - HRTIM block reset
pub use TIM1RST_R as HRTIMRST_R;
///Field `TIM8RST` writer - TIM8 block reset
pub use TIM1RST_W as TIM8RST_W;
///Field `USART1RST` writer - USART1 block reset
pub use TIM1RST_W as USART1RST_W;
///Field `USART6RST` writer - USART6 block reset
pub use TIM1RST_W as USART6RST_W;
///Field `SPI1RST` writer - SPI1 block reset
pub use TIM1RST_W as SPI1RST_W;
///Field `SPI4RST` writer - SPI4 block reset
pub use TIM1RST_W as SPI4RST_W;
///Field `TIM15RST` writer - TIM15 block reset
pub use TIM1RST_W as TIM15RST_W;
///Field `TIM16RST` writer - TIM16 block reset
pub use TIM1RST_W as TIM16RST_W;
///Field `TIM17RST` writer - TIM17 block reset
pub use TIM1RST_W as TIM17RST_W;
///Field `SPI5RST` writer - SPI5 block reset
pub use TIM1RST_W as SPI5RST_W;
///Field `SAI1RST` writer - SAI1 block reset
pub use TIM1RST_W as SAI1RST_W;
///Field `SAI2RST` writer - SAI2 block reset
pub use TIM1RST_W as SAI2RST_W;
///Field `SAI3RST` writer - SAI3 block reset
pub use TIM1RST_W as SAI3RST_W;
///Field `DFSDM1RST` writer - DFSDM1 block reset
pub use TIM1RST_W as DFSDM1RST_W;
///Field `HRTIMRST` writer - HRTIM block reset
pub use TIM1RST_W as HRTIMRST_W;
impl R {
    ///Bit 0 - TIM1 block reset
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM8 block reset
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - USART1 block reset
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - USART6 block reset
    #[inline(always)]
    pub fn usart6rst(&self) -> USART6RST_R {
        USART6RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 12 - SPI1 block reset
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SPI4 block reset
    #[inline(always)]
    pub fn spi4rst(&self) -> SPI4RST_R {
        SPI4RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - TIM15 block reset
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 block reset
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 block reset
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - SPI5 block reset
    #[inline(always)]
    pub fn spi5rst(&self) -> SPI5RST_R {
        SPI5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - SAI1 block reset
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SAI2 block reset
    #[inline(always)]
    pub fn sai2rst(&self) -> SAI2RST_R {
        SAI2RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SAI3 block reset
    #[inline(always)]
    pub fn sai3rst(&self) -> SAI3RST_R {
        SAI3RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - DFSDM1 block reset
    #[inline(always)]
    pub fn dfsdm1rst(&self) -> DFSDM1RST_R {
        DFSDM1RST_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - HRTIM block reset
    #[inline(always)]
    pub fn hrtimrst(&self) -> HRTIMRST_R {
        HRTIMRST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RSTR")
            .field("tim1rst", &self.tim1rst())
            .field("tim8rst", &self.tim8rst())
            .field("usart1rst", &self.usart1rst())
            .field("usart6rst", &self.usart6rst())
            .field("spi1rst", &self.spi1rst())
            .field("spi4rst", &self.spi4rst())
            .field("tim15rst", &self.tim15rst())
            .field("tim16rst", &self.tim16rst())
            .field("tim17rst", &self.tim17rst())
            .field("spi5rst", &self.spi5rst())
            .field("sai1rst", &self.sai1rst())
            .field("sai2rst", &self.sai2rst())
            .field("sai3rst", &self.sai3rst())
            .field("dfsdm1rst", &self.dfsdm1rst())
            .field("hrtimrst", &self.hrtimrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM1 block reset
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W<'_, APB2RSTRrs> {
        TIM1RST_W::new(self, 0)
    }
    ///Bit 1 - TIM8 block reset
    #[inline(always)]
    pub fn tim8rst(&mut self) -> TIM8RST_W<'_, APB2RSTRrs> {
        TIM8RST_W::new(self, 1)
    }
    ///Bit 4 - USART1 block reset
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W<'_, APB2RSTRrs> {
        USART1RST_W::new(self, 4)
    }
    ///Bit 5 - USART6 block reset
    #[inline(always)]
    pub fn usart6rst(&mut self) -> USART6RST_W<'_, APB2RSTRrs> {
        USART6RST_W::new(self, 5)
    }
    ///Bit 12 - SPI1 block reset
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W<'_, APB2RSTRrs> {
        SPI1RST_W::new(self, 12)
    }
    ///Bit 13 - SPI4 block reset
    #[inline(always)]
    pub fn spi4rst(&mut self) -> SPI4RST_W<'_, APB2RSTRrs> {
        SPI4RST_W::new(self, 13)
    }
    ///Bit 16 - TIM15 block reset
    #[inline(always)]
    pub fn tim15rst(&mut self) -> TIM15RST_W<'_, APB2RSTRrs> {
        TIM15RST_W::new(self, 16)
    }
    ///Bit 17 - TIM16 block reset
    #[inline(always)]
    pub fn tim16rst(&mut self) -> TIM16RST_W<'_, APB2RSTRrs> {
        TIM16RST_W::new(self, 17)
    }
    ///Bit 18 - TIM17 block reset
    #[inline(always)]
    pub fn tim17rst(&mut self) -> TIM17RST_W<'_, APB2RSTRrs> {
        TIM17RST_W::new(self, 18)
    }
    ///Bit 20 - SPI5 block reset
    #[inline(always)]
    pub fn spi5rst(&mut self) -> SPI5RST_W<'_, APB2RSTRrs> {
        SPI5RST_W::new(self, 20)
    }
    ///Bit 22 - SAI1 block reset
    #[inline(always)]
    pub fn sai1rst(&mut self) -> SAI1RST_W<'_, APB2RSTRrs> {
        SAI1RST_W::new(self, 22)
    }
    ///Bit 23 - SAI2 block reset
    #[inline(always)]
    pub fn sai2rst(&mut self) -> SAI2RST_W<'_, APB2RSTRrs> {
        SAI2RST_W::new(self, 23)
    }
    ///Bit 24 - SAI3 block reset
    #[inline(always)]
    pub fn sai3rst(&mut self) -> SAI3RST_W<'_, APB2RSTRrs> {
        SAI3RST_W::new(self, 24)
    }
    ///Bit 28 - DFSDM1 block reset
    #[inline(always)]
    pub fn dfsdm1rst(&mut self) -> DFSDM1RST_W<'_, APB2RSTRrs> {
        DFSDM1RST_W::new(self, 28)
    }
    ///Bit 29 - HRTIM block reset
    #[inline(always)]
    pub fn hrtimrst(&mut self) -> HRTIMRST_W<'_, APB2RSTRrs> {
        HRTIMRST_W::new(self, 29)
    }
}
/**RCC APB2 Peripheral Reset Register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RCC:APB2RSTR)*/
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
