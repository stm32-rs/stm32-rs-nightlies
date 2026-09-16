///Register `AHB1LPENR` reader
pub type R = crate::R<AHB1LPENRrs>;
///Register `AHB1LPENR` writer
pub type W = crate::W<AHB1LPENRrs>;
/**IO port A clock enable during sleep mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOALPEN {
    ///0: Selected module is disabled during Sleep mode
    DisabledInSleep = 0,
    ///1: Selected module is enabled during Sleep mode
    EnabledInSleep = 1,
}
impl From<GPIOALPEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOALPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOALPEN` reader - IO port A clock enable during sleep mode
pub type GPIOALPEN_R = crate::BitReader<GPIOALPEN>;
impl GPIOALPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOALPEN {
        match self.bits {
            false => GPIOALPEN::DisabledInSleep,
            true => GPIOALPEN::EnabledInSleep,
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == GPIOALPEN::DisabledInSleep
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == GPIOALPEN::EnabledInSleep
    }
}
///Field `GPIOALPEN` writer - IO port A clock enable during sleep mode
pub type GPIOALPEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOALPEN>;
impl<'a, REG> GPIOALPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOALPEN::DisabledInSleep)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOALPEN::EnabledInSleep)
    }
}
///Field `GPIOBLPEN` reader - IO port B clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOBLPEN_R;
///Field `GPIOCLPEN` reader - IO port C clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOCLPEN_R;
///Field `GPIODLPEN` reader - IO port D clock enable during Sleep mode
pub use GPIOALPEN_R as GPIODLPEN_R;
///Field `GPIOELPEN` reader - IO port E clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOELPEN_R;
///Field `GPIOFLPEN` reader - IO port F clock enable during sleep mode
pub use GPIOALPEN_R as GPIOFLPEN_R;
///Field `GPIOGLPEN` reader - IO port G clock enable during sleep mode
pub use GPIOALPEN_R as GPIOGLPEN_R;
///Field `GPIOHLPEN` reader - IO port H clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOHLPEN_R;
///Field `CRCLPEN` reader - CRC clock enable during Sleep mode
pub use GPIOALPEN_R as CRCLPEN_R;
///Field `FLITFLPEN` reader - Flash interface clock enable during Sleep mode
pub use GPIOALPEN_R as FLITFLPEN_R;
///Field `SRAM1LPEN` reader - SRAM 1interface clock enable during Sleep mode
pub use GPIOALPEN_R as SRAM1LPEN_R;
///Field `SRAM2LPEN` reader - SRAM2interface clock enable during Sleep mode
pub use GPIOALPEN_R as SRAM2LPEN_R;
///Field `DMA1LPEN` reader - DMA1 clock enable during Sleep mode
pub use GPIOALPEN_R as DMA1LPEN_R;
///Field `DMA2LPEN` reader - DMA2 clock enable during Sleep mode
pub use GPIOALPEN_R as DMA2LPEN_R;
///Field `GPIOBLPEN` writer - IO port B clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOBLPEN_W;
///Field `GPIOCLPEN` writer - IO port C clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOCLPEN_W;
///Field `GPIODLPEN` writer - IO port D clock enable during Sleep mode
pub use GPIOALPEN_W as GPIODLPEN_W;
///Field `GPIOELPEN` writer - IO port E clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOELPEN_W;
///Field `GPIOFLPEN` writer - IO port F clock enable during sleep mode
pub use GPIOALPEN_W as GPIOFLPEN_W;
///Field `GPIOGLPEN` writer - IO port G clock enable during sleep mode
pub use GPIOALPEN_W as GPIOGLPEN_W;
///Field `GPIOHLPEN` writer - IO port H clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOHLPEN_W;
///Field `CRCLPEN` writer - CRC clock enable during Sleep mode
pub use GPIOALPEN_W as CRCLPEN_W;
///Field `FLITFLPEN` writer - Flash interface clock enable during Sleep mode
pub use GPIOALPEN_W as FLITFLPEN_W;
///Field `SRAM1LPEN` writer - SRAM 1interface clock enable during Sleep mode
pub use GPIOALPEN_W as SRAM1LPEN_W;
///Field `SRAM2LPEN` writer - SRAM2interface clock enable during Sleep mode
pub use GPIOALPEN_W as SRAM2LPEN_W;
///Field `DMA1LPEN` writer - DMA1 clock enable during Sleep mode
pub use GPIOALPEN_W as DMA1LPEN_W;
///Field `DMA2LPEN` writer - DMA2 clock enable during Sleep mode
pub use GPIOALPEN_W as DMA2LPEN_W;
impl R {
    ///Bit 0 - IO port A clock enable during sleep mode
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port F clock enable during sleep mode
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G clock enable during sleep mode
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port H clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable during Sleep mode
    #[inline(always)]
    pub fn crclpen(&self) -> CRCLPEN_R {
        CRCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - Flash interface clock enable during Sleep mode
    #[inline(always)]
    pub fn flitflpen(&self) -> FLITFLPEN_R {
        FLITFLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SRAM 1interface clock enable during Sleep mode
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SRAM2interface clock enable during Sleep mode
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - DMA1 clock enable during Sleep mode
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - DMA2 clock enable during Sleep mode
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1LPENR")
            .field("gpioalpen", &self.gpioalpen())
            .field("gpioblpen", &self.gpioblpen())
            .field("gpioclpen", &self.gpioclpen())
            .field("gpiodlpen", &self.gpiodlpen())
            .field("gpioelpen", &self.gpioelpen())
            .field("gpioflpen", &self.gpioflpen())
            .field("gpioglpen", &self.gpioglpen())
            .field("gpiohlpen", &self.gpiohlpen())
            .field("crclpen", &self.crclpen())
            .field("flitflpen", &self.flitflpen())
            .field("sram1lpen", &self.sram1lpen())
            .field("dma1lpen", &self.dma1lpen())
            .field("dma2lpen", &self.dma2lpen())
            .field("sram2lpen", &self.sram2lpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A clock enable during sleep mode
    #[inline(always)]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<'_, AHB1LPENRrs> {
        GPIOALPEN_W::new(self, 0)
    }
    ///Bit 1 - IO port B clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<'_, AHB1LPENRrs> {
        GPIOBLPEN_W::new(self, 1)
    }
    ///Bit 2 - IO port C clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<'_, AHB1LPENRrs> {
        GPIOCLPEN_W::new(self, 2)
    }
    ///Bit 3 - IO port D clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<'_, AHB1LPENRrs> {
        GPIODLPEN_W::new(self, 3)
    }
    ///Bit 4 - IO port E clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W<'_, AHB1LPENRrs> {
        GPIOELPEN_W::new(self, 4)
    }
    ///Bit 5 - IO port F clock enable during sleep mode
    #[inline(always)]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W<'_, AHB1LPENRrs> {
        GPIOFLPEN_W::new(self, 5)
    }
    ///Bit 6 - IO port G clock enable during sleep mode
    #[inline(always)]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W<'_, AHB1LPENRrs> {
        GPIOGLPEN_W::new(self, 6)
    }
    ///Bit 7 - IO port H clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<'_, AHB1LPENRrs> {
        GPIOHLPEN_W::new(self, 7)
    }
    ///Bit 12 - CRC clock enable during Sleep mode
    #[inline(always)]
    pub fn crclpen(&mut self) -> CRCLPEN_W<'_, AHB1LPENRrs> {
        CRCLPEN_W::new(self, 12)
    }
    ///Bit 15 - Flash interface clock enable during Sleep mode
    #[inline(always)]
    pub fn flitflpen(&mut self) -> FLITFLPEN_W<'_, AHB1LPENRrs> {
        FLITFLPEN_W::new(self, 15)
    }
    ///Bit 16 - SRAM 1interface clock enable during Sleep mode
    #[inline(always)]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W<'_, AHB1LPENRrs> {
        SRAM1LPEN_W::new(self, 16)
    }
    ///Bit 17 - SRAM2interface clock enable during Sleep mode
    #[inline(always)]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W<'_, AHB1LPENRrs> {
        SRAM2LPEN_W::new(self, 17)
    }
    ///Bit 21 - DMA1 clock enable during Sleep mode
    #[inline(always)]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W<'_, AHB1LPENRrs> {
        DMA1LPEN_W::new(self, 21)
    }
    ///Bit 22 - DMA2 clock enable during Sleep mode
    #[inline(always)]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W<'_, AHB1LPENRrs> {
        DMA2LPEN_W::new(self, 22)
    }
}
/**AHB1 peripheral clock enable in low power mode register

You can [`read`](crate::Reg::read) this register and get [`ahb1lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F413.html#RCC:AHB1LPENR)*/
pub struct AHB1LPENRrs;
impl crate::RegisterSpec for AHB1LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1lpenr::R`](R) reader structure
impl crate::Readable for AHB1LPENRrs {}
///`write(|w| ..)` method takes [`ahb1lpenr::W`](W) writer structure
impl crate::Writable for AHB1LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1LPENR to value 0x7e67_91ff
impl crate::Resettable for AHB1LPENRrs {
    const RESET_VALUE: u32 = 0x7e67_91ff;
}
