///Register `C1_APB2LPENR` reader
pub type R = crate::R<C1_APB2LPENRrs>;
///Register `C1_APB2LPENR` writer
pub type W = crate::W<C1_APB2LPENRrs>;
/**TIM1 peripheral clock enable during CSleep mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1LPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<TIM1LPEN> for bool {
    #[inline(always)]
    fn from(variant: TIM1LPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM1LPEN` reader - TIM1 peripheral clock enable during CSleep mode
pub type TIM1LPEN_R = crate::BitReader<TIM1LPEN>;
impl TIM1LPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM1LPEN {
        match self.bits {
            false => TIM1LPEN::Disabled,
            true => TIM1LPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM1LPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM1LPEN::Enabled
    }
}
///Field `TIM1LPEN` writer - TIM1 peripheral clock enable during CSleep mode
pub type TIM1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM1LPEN>;
impl<'a, REG> TIM1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1LPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1LPEN::Enabled)
    }
}
///Field `TIM8LPEN` reader - TIM8 peripheral clock enable during CSleep mode
pub use TIM1LPEN_R as TIM8LPEN_R;
///Field `USART1LPEN` reader - USART1 Peripheral Clocks Enable During CSleep Mode
pub use TIM1LPEN_R as USART1LPEN_R;
///Field `USART6LPEN` reader - USART6 Peripheral Clocks Enable During CSleep Mode
pub use TIM1LPEN_R as USART6LPEN_R;
///Field `UART9LPEN` reader - UART9 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the UART9 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock.
pub use TIM1LPEN_R as UART9LPEN_R;
///Field `USART10LPEN` reader - USART10 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART10 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock.
pub use TIM1LPEN_R as USART10LPEN_R;
///Field `SPI1LPEN` reader - SPI1 Peripheral Clocks Enable During CSleep Mode
pub use TIM1LPEN_R as SPI1LPEN_R;
///Field `SPI4LPEN` reader - SPI4 Peripheral Clocks Enable During CSleep Mode
pub use TIM1LPEN_R as SPI4LPEN_R;
///Field `TIM15LPEN` reader - TIM15 peripheral clock enable during CSleep mode
pub use TIM1LPEN_R as TIM15LPEN_R;
///Field `TIM16LPEN` reader - TIM16 peripheral clock enable during CSleep mode
pub use TIM1LPEN_R as TIM16LPEN_R;
///Field `TIM17LPEN` reader - TIM17 peripheral clock enable during CSleep mode
pub use TIM1LPEN_R as TIM17LPEN_R;
///Field `SPI5LPEN` reader - SPI5 Peripheral Clocks Enable During CSleep Mode
pub use TIM1LPEN_R as SPI5LPEN_R;
///Field `SAI1LPEN` reader - SAI1 Peripheral Clocks Enable During CSleep Mode
pub use TIM1LPEN_R as SAI1LPEN_R;
///Field `DFSDM1LPEN` reader - DFSDM1 peripheral clocks enable during CSleep mode Set and reset by software. DFSDM1 peripheral clocks are the kernel clocks selected by SAI1SEL and DFSDM1SEL and provided to Aclk and clk inputs respectively, and the rcc_pclk2 bus interface clock.
pub use TIM1LPEN_R as DFSDM1LPEN_R;
///Field `TIM8LPEN` writer - TIM8 peripheral clock enable during CSleep mode
pub use TIM1LPEN_W as TIM8LPEN_W;
///Field `USART1LPEN` writer - USART1 Peripheral Clocks Enable During CSleep Mode
pub use TIM1LPEN_W as USART1LPEN_W;
///Field `USART6LPEN` writer - USART6 Peripheral Clocks Enable During CSleep Mode
pub use TIM1LPEN_W as USART6LPEN_W;
///Field `UART9LPEN` writer - UART9 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the UART9 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock.
pub use TIM1LPEN_W as UART9LPEN_W;
///Field `USART10LPEN` writer - USART10 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART10 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock.
pub use TIM1LPEN_W as USART10LPEN_W;
///Field `SPI1LPEN` writer - SPI1 Peripheral Clocks Enable During CSleep Mode
pub use TIM1LPEN_W as SPI1LPEN_W;
///Field `SPI4LPEN` writer - SPI4 Peripheral Clocks Enable During CSleep Mode
pub use TIM1LPEN_W as SPI4LPEN_W;
///Field `TIM15LPEN` writer - TIM15 peripheral clock enable during CSleep mode
pub use TIM1LPEN_W as TIM15LPEN_W;
///Field `TIM16LPEN` writer - TIM16 peripheral clock enable during CSleep mode
pub use TIM1LPEN_W as TIM16LPEN_W;
///Field `TIM17LPEN` writer - TIM17 peripheral clock enable during CSleep mode
pub use TIM1LPEN_W as TIM17LPEN_W;
///Field `SPI5LPEN` writer - SPI5 Peripheral Clocks Enable During CSleep Mode
pub use TIM1LPEN_W as SPI5LPEN_W;
///Field `SAI1LPEN` writer - SAI1 Peripheral Clocks Enable During CSleep Mode
pub use TIM1LPEN_W as SAI1LPEN_W;
///Field `DFSDM1LPEN` writer - DFSDM1 peripheral clocks enable during CSleep mode Set and reset by software. DFSDM1 peripheral clocks are the kernel clocks selected by SAI1SEL and DFSDM1SEL and provided to Aclk and clk inputs respectively, and the rcc_pclk2 bus interface clock.
pub use TIM1LPEN_W as DFSDM1LPEN_W;
impl R {
    ///Bit 0 - TIM1 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim1lpen(&self) -> TIM1LPEN_R {
        TIM1LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM8 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim8lpen(&self) -> TIM8LPEN_R {
        TIM8LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - USART1 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - USART6 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn usart6lpen(&self) -> USART6LPEN_R {
        USART6LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - UART9 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the UART9 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock.
    #[inline(always)]
    pub fn uart9lpen(&self) -> UART9LPEN_R {
        UART9LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - USART10 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART10 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock.
    #[inline(always)]
    pub fn usart10lpen(&self) -> USART10LPEN_R {
        USART10LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 12 - SPI1 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SPI4 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn spi4lpen(&self) -> SPI4LPEN_R {
        SPI4LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - TIM15 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim15lpen(&self) -> TIM15LPEN_R {
        TIM15LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim16lpen(&self) -> TIM16LPEN_R {
        TIM16LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim17lpen(&self) -> TIM17LPEN_R {
        TIM17LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - SPI5 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn spi5lpen(&self) -> SPI5LPEN_R {
        SPI5LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - SAI1 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn sai1lpen(&self) -> SAI1LPEN_R {
        SAI1LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 30 - DFSDM1 peripheral clocks enable during CSleep mode Set and reset by software. DFSDM1 peripheral clocks are the kernel clocks selected by SAI1SEL and DFSDM1SEL and provided to Aclk and clk inputs respectively, and the rcc_pclk2 bus interface clock.
    #[inline(always)]
    pub fn dfsdm1lpen(&self) -> DFSDM1LPEN_R {
        DFSDM1LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1_APB2LPENR")
            .field("tim1lpen", &self.tim1lpen())
            .field("tim8lpen", &self.tim8lpen())
            .field("usart1lpen", &self.usart1lpen())
            .field("usart6lpen", &self.usart6lpen())
            .field("uart9lpen", &self.uart9lpen())
            .field("usart10lpen", &self.usart10lpen())
            .field("spi1lpen", &self.spi1lpen())
            .field("spi4lpen", &self.spi4lpen())
            .field("tim15lpen", &self.tim15lpen())
            .field("tim16lpen", &self.tim16lpen())
            .field("tim17lpen", &self.tim17lpen())
            .field("spi5lpen", &self.spi5lpen())
            .field("sai1lpen", &self.sai1lpen())
            .field("dfsdm1lpen", &self.dfsdm1lpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM1 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim1lpen(&mut self) -> TIM1LPEN_W<'_, C1_APB2LPENRrs> {
        TIM1LPEN_W::new(self, 0)
    }
    ///Bit 1 - TIM8 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim8lpen(&mut self) -> TIM8LPEN_W<'_, C1_APB2LPENRrs> {
        TIM8LPEN_W::new(self, 1)
    }
    ///Bit 4 - USART1 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<'_, C1_APB2LPENRrs> {
        USART1LPEN_W::new(self, 4)
    }
    ///Bit 5 - USART6 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W<'_, C1_APB2LPENRrs> {
        USART6LPEN_W::new(self, 5)
    }
    ///Bit 6 - UART9 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the UART9 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock.
    #[inline(always)]
    pub fn uart9lpen(&mut self) -> UART9LPEN_W<'_, C1_APB2LPENRrs> {
        UART9LPEN_W::new(self, 6)
    }
    ///Bit 7 - USART10 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART10 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock.
    #[inline(always)]
    pub fn usart10lpen(&mut self) -> USART10LPEN_W<'_, C1_APB2LPENRrs> {
        USART10LPEN_W::new(self, 7)
    }
    ///Bit 12 - SPI1 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<'_, C1_APB2LPENRrs> {
        SPI1LPEN_W::new(self, 12)
    }
    ///Bit 13 - SPI4 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn spi4lpen(&mut self) -> SPI4LPEN_W<'_, C1_APB2LPENRrs> {
        SPI4LPEN_W::new(self, 13)
    }
    ///Bit 16 - TIM15 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim15lpen(&mut self) -> TIM15LPEN_W<'_, C1_APB2LPENRrs> {
        TIM15LPEN_W::new(self, 16)
    }
    ///Bit 17 - TIM16 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim16lpen(&mut self) -> TIM16LPEN_W<'_, C1_APB2LPENRrs> {
        TIM16LPEN_W::new(self, 17)
    }
    ///Bit 18 - TIM17 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim17lpen(&mut self) -> TIM17LPEN_W<'_, C1_APB2LPENRrs> {
        TIM17LPEN_W::new(self, 18)
    }
    ///Bit 20 - SPI5 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn spi5lpen(&mut self) -> SPI5LPEN_W<'_, C1_APB2LPENRrs> {
        SPI5LPEN_W::new(self, 20)
    }
    ///Bit 22 - SAI1 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn sai1lpen(&mut self) -> SAI1LPEN_W<'_, C1_APB2LPENRrs> {
        SAI1LPEN_W::new(self, 22)
    }
    ///Bit 30 - DFSDM1 peripheral clocks enable during CSleep mode Set and reset by software. DFSDM1 peripheral clocks are the kernel clocks selected by SAI1SEL and DFSDM1SEL and provided to Aclk and clk inputs respectively, and the rcc_pclk2 bus interface clock.
    #[inline(always)]
    pub fn dfsdm1lpen(&mut self) -> DFSDM1LPEN_W<'_, C1_APB2LPENRrs> {
        DFSDM1LPEN_W::new(self, 30)
    }
}
/**RCC APB2 Sleep Clock Register

You can [`read`](crate::Reg::read) this register and get [`c1_apb2lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_apb2lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#RCC:C1_APB2LPENR)*/
pub struct C1_APB2LPENRrs;
impl crate::RegisterSpec for C1_APB2LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`c1_apb2lpenr::R`](R) reader structure
impl crate::Readable for C1_APB2LPENRrs {}
///`write(|w| ..)` method takes [`c1_apb2lpenr::W`](W) writer structure
impl crate::Writable for C1_APB2LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C1_APB2LPENR to value 0
impl crate::Resettable for C1_APB2LPENRrs {}
