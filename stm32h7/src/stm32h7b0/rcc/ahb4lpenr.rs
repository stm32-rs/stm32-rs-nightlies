///Register `AHB4LPENR` reader
pub type R = crate::R<AHB4LPENRrs>;
///Register `AHB4LPENR` writer
pub type W = crate::W<AHB4LPENRrs>;
/**GPIOA peripheral clock enable during CSleep mode Set and reset by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOALPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<GPIOALPEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOALPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOALPEN` reader - GPIOA peripheral clock enable during CSleep mode Set and reset by software.
pub type GPIOALPEN_R = crate::BitReader<GPIOALPEN>;
impl GPIOALPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOALPEN {
        match self.bits {
            false => GPIOALPEN::Disabled,
            true => GPIOALPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOALPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOALPEN::Enabled
    }
}
///Field `GPIOALPEN` writer - GPIOA peripheral clock enable during CSleep mode Set and reset by software.
pub type GPIOALPEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOALPEN>;
impl<'a, REG> GPIOALPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOALPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOALPEN::Enabled)
    }
}
///Field `GPIOBLPEN` reader - GPIOB peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_R as GPIOBLPEN_R;
///Field `GPIOCLPEN` reader - GPIOC peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_R as GPIOCLPEN_R;
///Field `GPIODLPEN` reader - GPIOD peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_R as GPIODLPEN_R;
///Field `GPIOELPEN` reader - GPIOE peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_R as GPIOELPEN_R;
///Field `GPIOFLPEN` reader - GPIOF peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_R as GPIOFLPEN_R;
///Field `GPIOGLPEN` reader - GPIOG peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_R as GPIOGLPEN_R;
///Field `GPIOHLPEN` reader - GPIOH peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_R as GPIOHLPEN_R;
///Field `GPIOILPEN` reader - GPIOI peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_R as GPIOILPEN_R;
///Field `GPIOJLPEN` reader - GPIOJ peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_R as GPIOJLPEN_R;
///Field `GPIOKLPEN` reader - GPIOK peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_R as GPIOKLPEN_R;
///Field `BDMA2LPEN` reader - SmartRun domain DMA and DMAMUX clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_R as BDMA2LPEN_R;
///Field `BKPRAMLPEN` reader - Backup RAM clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_R as BKPRAMLPEN_R;
///Field `SRDSRAMLPEN` reader - SmartRun domain SRAM clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_R as SRDSRAMLPEN_R;
///Field `GPIOBLPEN` writer - GPIOB peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_W as GPIOBLPEN_W;
///Field `GPIOCLPEN` writer - GPIOC peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_W as GPIOCLPEN_W;
///Field `GPIODLPEN` writer - GPIOD peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_W as GPIODLPEN_W;
///Field `GPIOELPEN` writer - GPIOE peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_W as GPIOELPEN_W;
///Field `GPIOFLPEN` writer - GPIOF peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_W as GPIOFLPEN_W;
///Field `GPIOGLPEN` writer - GPIOG peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_W as GPIOGLPEN_W;
///Field `GPIOHLPEN` writer - GPIOH peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_W as GPIOHLPEN_W;
///Field `GPIOILPEN` writer - GPIOI peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_W as GPIOILPEN_W;
///Field `GPIOJLPEN` writer - GPIOJ peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_W as GPIOJLPEN_W;
///Field `GPIOKLPEN` writer - GPIOK peripheral clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_W as GPIOKLPEN_W;
///Field `BDMA2LPEN` writer - SmartRun domain DMA and DMAMUX clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_W as BDMA2LPEN_W;
///Field `BKPRAMLPEN` writer - Backup RAM clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_W as BKPRAMLPEN_W;
///Field `SRDSRAMLPEN` writer - SmartRun domain SRAM clock enable during CSleep mode Set and reset by software.
pub use GPIOALPEN_W as SRDSRAMLPEN_W;
impl R {
    ///Bit 0 - GPIOA peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIOB peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIOC peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIOD peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPIOE peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIOF peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIOG peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIOH peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GPIOI peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpioilpen(&self) -> GPIOILPEN_R {
        GPIOILPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GPIOJ peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpiojlpen(&self) -> GPIOJLPEN_R {
        GPIOJLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GPIOK peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpioklpen(&self) -> GPIOKLPEN_R {
        GPIOKLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 21 - SmartRun domain DMA and DMAMUX clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn bdma2lpen(&self) -> BDMA2LPEN_R {
        BDMA2LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 28 - Backup RAM clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn bkpramlpen(&self) -> BKPRAMLPEN_R {
        BKPRAMLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SmartRun domain SRAM clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn srdsramlpen(&self) -> SRDSRAMLPEN_R {
        SRDSRAMLPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB4LPENR")
            .field("gpioalpen", &self.gpioalpen())
            .field("gpioblpen", &self.gpioblpen())
            .field("gpioclpen", &self.gpioclpen())
            .field("gpiodlpen", &self.gpiodlpen())
            .field("gpioelpen", &self.gpioelpen())
            .field("gpioflpen", &self.gpioflpen())
            .field("gpioglpen", &self.gpioglpen())
            .field("gpiohlpen", &self.gpiohlpen())
            .field("gpioilpen", &self.gpioilpen())
            .field("gpiojlpen", &self.gpiojlpen())
            .field("gpioklpen", &self.gpioklpen())
            .field("bdma2lpen", &self.bdma2lpen())
            .field("bkpramlpen", &self.bkpramlpen())
            .field("srdsramlpen", &self.srdsramlpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPIOA peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<'_, AHB4LPENRrs> {
        GPIOALPEN_W::new(self, 0)
    }
    ///Bit 1 - GPIOB peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<'_, AHB4LPENRrs> {
        GPIOBLPEN_W::new(self, 1)
    }
    ///Bit 2 - GPIOC peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<'_, AHB4LPENRrs> {
        GPIOCLPEN_W::new(self, 2)
    }
    ///Bit 3 - GPIOD peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<'_, AHB4LPENRrs> {
        GPIODLPEN_W::new(self, 3)
    }
    ///Bit 4 - GPIOE peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W<'_, AHB4LPENRrs> {
        GPIOELPEN_W::new(self, 4)
    }
    ///Bit 5 - GPIOF peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W<'_, AHB4LPENRrs> {
        GPIOFLPEN_W::new(self, 5)
    }
    ///Bit 6 - GPIOG peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W<'_, AHB4LPENRrs> {
        GPIOGLPEN_W::new(self, 6)
    }
    ///Bit 7 - GPIOH peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<'_, AHB4LPENRrs> {
        GPIOHLPEN_W::new(self, 7)
    }
    ///Bit 8 - GPIOI peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpioilpen(&mut self) -> GPIOILPEN_W<'_, AHB4LPENRrs> {
        GPIOILPEN_W::new(self, 8)
    }
    ///Bit 9 - GPIOJ peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpiojlpen(&mut self) -> GPIOJLPEN_W<'_, AHB4LPENRrs> {
        GPIOJLPEN_W::new(self, 9)
    }
    ///Bit 10 - GPIOK peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpioklpen(&mut self) -> GPIOKLPEN_W<'_, AHB4LPENRrs> {
        GPIOKLPEN_W::new(self, 10)
    }
    ///Bit 21 - SmartRun domain DMA and DMAMUX clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn bdma2lpen(&mut self) -> BDMA2LPEN_W<'_, AHB4LPENRrs> {
        BDMA2LPEN_W::new(self, 21)
    }
    ///Bit 28 - Backup RAM clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn bkpramlpen(&mut self) -> BKPRAMLPEN_W<'_, AHB4LPENRrs> {
        BKPRAMLPEN_W::new(self, 28)
    }
    ///Bit 29 - SmartRun domain SRAM clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn srdsramlpen(&mut self) -> SRDSRAMLPEN_W<'_, AHB4LPENRrs> {
        SRDSRAMLPEN_W::new(self, 29)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`ahb4lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#RCC:AHB4LPENR)*/
pub struct AHB4LPENRrs;
impl crate::RegisterSpec for AHB4LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb4lpenr::R`](R) reader structure
impl crate::Readable for AHB4LPENRrs {}
///`write(|w| ..)` method takes [`ahb4lpenr::W`](W) writer structure
impl crate::Writable for AHB4LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB4LPENR to value 0x3020_07ff
impl crate::Resettable for AHB4LPENRrs {
    const RESET_VALUE: u32 = 0x3020_07ff;
}
