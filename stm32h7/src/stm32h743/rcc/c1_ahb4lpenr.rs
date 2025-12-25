///Register `C1_AHB4LPENR` reader
pub type R = crate::R<C1_AHB4LPENRrs>;
///Register `C1_AHB4LPENR` writer
pub type W = crate::W<C1_AHB4LPENRrs>;
/**GPIO peripheral clock enable during CSleep mode

Value on reset: 0*/
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
///Field `GPIOALPEN` reader - GPIO peripheral clock enable during CSleep mode
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
///Field `GPIOALPEN` writer - GPIO peripheral clock enable during CSleep mode
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
///Field `GPIOBLPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIOBLPEN_R;
///Field `GPIOCLPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIOCLPEN_R;
///Field `GPIODLPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIODLPEN_R;
///Field `GPIOELPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIOELPEN_R;
///Field `GPIOFLPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIOFLPEN_R;
///Field `GPIOGLPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIOGLPEN_R;
///Field `GPIOHLPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIOHLPEN_R;
///Field `GPIOILPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIOILPEN_R;
///Field `GPIOJLPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIOJLPEN_R;
///Field `GPIOKLPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIOKLPEN_R;
///Field `CRCLPEN` reader - CRC peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as CRCLPEN_R;
///Field `BDMALPEN` reader - BDMA Clock Enable During CSleep Mode
pub use GPIOALPEN_R as BDMALPEN_R;
///Field `ADC3LPEN` reader - ADC3 Peripheral Clocks Enable During CSleep Mode
pub use GPIOALPEN_R as ADC3LPEN_R;
///Field `BKPRAMLPEN` reader - Backup RAM Clock Enable During CSleep Mode
pub use GPIOALPEN_R as BKPRAMLPEN_R;
///Field `SRAM4LPEN` reader - SRAM4 Clock Enable During CSleep Mode
pub use GPIOALPEN_R as SRAM4LPEN_R;
///Field `GPIOBLPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIOBLPEN_W;
///Field `GPIOCLPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIOCLPEN_W;
///Field `GPIODLPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIODLPEN_W;
///Field `GPIOELPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIOELPEN_W;
///Field `GPIOFLPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIOFLPEN_W;
///Field `GPIOGLPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIOGLPEN_W;
///Field `GPIOHLPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIOHLPEN_W;
///Field `GPIOILPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIOILPEN_W;
///Field `GPIOJLPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIOJLPEN_W;
///Field `GPIOKLPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIOKLPEN_W;
///Field `CRCLPEN` writer - CRC peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as CRCLPEN_W;
///Field `BDMALPEN` writer - BDMA Clock Enable During CSleep Mode
pub use GPIOALPEN_W as BDMALPEN_W;
///Field `ADC3LPEN` writer - ADC3 Peripheral Clocks Enable During CSleep Mode
pub use GPIOALPEN_W as ADC3LPEN_W;
///Field `BKPRAMLPEN` writer - Backup RAM Clock Enable During CSleep Mode
pub use GPIOALPEN_W as BKPRAMLPEN_W;
///Field `SRAM4LPEN` writer - SRAM4 Clock Enable During CSleep Mode
pub use GPIOALPEN_W as SRAM4LPEN_W;
impl R {
    ///Bit 0 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioilpen(&self) -> GPIOILPEN_R {
        GPIOILPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpiojlpen(&self) -> GPIOJLPEN_R {
        GPIOJLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioklpen(&self) -> GPIOKLPEN_R {
        GPIOKLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 19 - CRC peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn crclpen(&self) -> CRCLPEN_R {
        CRCLPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - BDMA Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn bdmalpen(&self) -> BDMALPEN_R {
        BDMALPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - ADC3 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn adc3lpen(&self) -> ADC3LPEN_R {
        ADC3LPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - Backup RAM Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn bkpramlpen(&self) -> BKPRAMLPEN_R {
        BKPRAMLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SRAM4 Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn sram4lpen(&self) -> SRAM4LPEN_R {
        SRAM4LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1_AHB4LPENR")
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
            .field("crclpen", &self.crclpen())
            .field("bdmalpen", &self.bdmalpen())
            .field("adc3lpen", &self.adc3lpen())
            .field("bkpramlpen", &self.bkpramlpen())
            .field("sram4lpen", &self.sram4lpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<'_, C1_AHB4LPENRrs> {
        GPIOALPEN_W::new(self, 0)
    }
    ///Bit 1 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<'_, C1_AHB4LPENRrs> {
        GPIOBLPEN_W::new(self, 1)
    }
    ///Bit 2 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<'_, C1_AHB4LPENRrs> {
        GPIOCLPEN_W::new(self, 2)
    }
    ///Bit 3 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<'_, C1_AHB4LPENRrs> {
        GPIODLPEN_W::new(self, 3)
    }
    ///Bit 4 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W<'_, C1_AHB4LPENRrs> {
        GPIOELPEN_W::new(self, 4)
    }
    ///Bit 5 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W<'_, C1_AHB4LPENRrs> {
        GPIOFLPEN_W::new(self, 5)
    }
    ///Bit 6 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W<'_, C1_AHB4LPENRrs> {
        GPIOGLPEN_W::new(self, 6)
    }
    ///Bit 7 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<'_, C1_AHB4LPENRrs> {
        GPIOHLPEN_W::new(self, 7)
    }
    ///Bit 8 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioilpen(&mut self) -> GPIOILPEN_W<'_, C1_AHB4LPENRrs> {
        GPIOILPEN_W::new(self, 8)
    }
    ///Bit 9 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpiojlpen(&mut self) -> GPIOJLPEN_W<'_, C1_AHB4LPENRrs> {
        GPIOJLPEN_W::new(self, 9)
    }
    ///Bit 10 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioklpen(&mut self) -> GPIOKLPEN_W<'_, C1_AHB4LPENRrs> {
        GPIOKLPEN_W::new(self, 10)
    }
    ///Bit 19 - CRC peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn crclpen(&mut self) -> CRCLPEN_W<'_, C1_AHB4LPENRrs> {
        CRCLPEN_W::new(self, 19)
    }
    ///Bit 21 - BDMA Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn bdmalpen(&mut self) -> BDMALPEN_W<'_, C1_AHB4LPENRrs> {
        BDMALPEN_W::new(self, 21)
    }
    ///Bit 24 - ADC3 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn adc3lpen(&mut self) -> ADC3LPEN_W<'_, C1_AHB4LPENRrs> {
        ADC3LPEN_W::new(self, 24)
    }
    ///Bit 28 - Backup RAM Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn bkpramlpen(&mut self) -> BKPRAMLPEN_W<'_, C1_AHB4LPENRrs> {
        BKPRAMLPEN_W::new(self, 28)
    }
    ///Bit 29 - SRAM4 Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn sram4lpen(&mut self) -> SRAM4LPEN_W<'_, C1_AHB4LPENRrs> {
        SRAM4LPEN_W::new(self, 29)
    }
}
/**RCC AHB4 Sleep Clock Register

You can [`read`](crate::Reg::read) this register and get [`c1_ahb4lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_ahb4lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#RCC:C1_AHB4LPENR)*/
pub struct C1_AHB4LPENRrs;
impl crate::RegisterSpec for C1_AHB4LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`c1_ahb4lpenr::R`](R) reader structure
impl crate::Readable for C1_AHB4LPENRrs {}
///`write(|w| ..)` method takes [`c1_ahb4lpenr::W`](W) writer structure
impl crate::Writable for C1_AHB4LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C1_AHB4LPENR to value 0
impl crate::Resettable for C1_AHB4LPENRrs {}
