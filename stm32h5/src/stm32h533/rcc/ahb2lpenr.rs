///Register `AHB2LPENR` reader
pub type R = crate::R<AHB2LPENRrs>;
///Register `AHB2LPENR` writer
pub type W = crate::W<AHB2LPENRrs>;
/**GPIOA clock enable during Sleep mode

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
///Field `GPIOALPEN` reader - GPIOA clock enable during Sleep mode
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
///Field `GPIOALPEN` writer - GPIOA clock enable during Sleep mode
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
///Field `GPIOBLPEN` reader - GPIOB clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOBLPEN_R;
///Field `GPIOCLPEN` reader - GPIOC clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOCLPEN_R;
///Field `GPIODLPEN` reader - GPIOD clock enable during Sleep mode
pub use GPIOALPEN_R as GPIODLPEN_R;
///Field `GPIOELPEN` reader - GPIOE clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOELPEN_R;
///Field `GPIOFLPEN` reader - GPIOF clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOFLPEN_R;
///Field `GPIOGLPEN` reader - GPIOG clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOGLPEN_R;
///Field `GPIOHLPEN` reader - GPIOH clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOHLPEN_R;
///Field `GPIOILPEN` reader - GPIOI clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOILPEN_R;
///Field `ADCLPEN` reader - ADC1 and 2 peripherals clock enable during Sleep mode
pub use GPIOALPEN_R as ADCLPEN_R;
///Field `DAC1LPEN` reader - DAC clock enable during Sleep mode
pub use GPIOALPEN_R as DAC1LPEN_R;
///Field `DCMI_PSSILPEN` reader - digital camera interface clock enable during Sleep mode (DCMI or PSSI depending which interface is active)
pub use GPIOALPEN_R as DCMI_PSSILPEN_R;
///Field `AESLPEN` reader - AES clock enable during Sleep mode
pub use GPIOALPEN_R as AESLPEN_R;
///Field `HASHLPEN` reader - HASH clock enable during Sleep mode
pub use GPIOALPEN_R as HASHLPEN_R;
///Field `RNGLPEN` reader - RNG clock enable during Sleep mode
pub use GPIOALPEN_R as RNGLPEN_R;
///Field `PKALPEN` reader - PKA clock enable during Sleep mode
pub use GPIOALPEN_R as PKALPEN_R;
///Field `SAESLPEN` reader - SAES clock enable during Sleep mode
pub use GPIOALPEN_R as SAESLPEN_R;
///Field `SRAM2LPEN` reader - SRAM2 clock enable during Sleep mode
pub use GPIOALPEN_R as SRAM2LPEN_R;
///Field `SRAM3LPEN` reader - SRAM3 clock enable during Sleep mode
pub use GPIOALPEN_R as SRAM3LPEN_R;
///Field `GPIOBLPEN` writer - GPIOB clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOBLPEN_W;
///Field `GPIOCLPEN` writer - GPIOC clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOCLPEN_W;
///Field `GPIODLPEN` writer - GPIOD clock enable during Sleep mode
pub use GPIOALPEN_W as GPIODLPEN_W;
///Field `GPIOELPEN` writer - GPIOE clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOELPEN_W;
///Field `GPIOFLPEN` writer - GPIOF clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOFLPEN_W;
///Field `GPIOGLPEN` writer - GPIOG clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOGLPEN_W;
///Field `GPIOHLPEN` writer - GPIOH clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOHLPEN_W;
///Field `GPIOILPEN` writer - GPIOI clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOILPEN_W;
///Field `ADCLPEN` writer - ADC1 and 2 peripherals clock enable during Sleep mode
pub use GPIOALPEN_W as ADCLPEN_W;
///Field `DAC1LPEN` writer - DAC clock enable during Sleep mode
pub use GPIOALPEN_W as DAC1LPEN_W;
///Field `DCMI_PSSILPEN` writer - digital camera interface clock enable during Sleep mode (DCMI or PSSI depending which interface is active)
pub use GPIOALPEN_W as DCMI_PSSILPEN_W;
///Field `AESLPEN` writer - AES clock enable during Sleep mode
pub use GPIOALPEN_W as AESLPEN_W;
///Field `HASHLPEN` writer - HASH clock enable during Sleep mode
pub use GPIOALPEN_W as HASHLPEN_W;
///Field `RNGLPEN` writer - RNG clock enable during Sleep mode
pub use GPIOALPEN_W as RNGLPEN_W;
///Field `PKALPEN` writer - PKA clock enable during Sleep mode
pub use GPIOALPEN_W as PKALPEN_W;
///Field `SAESLPEN` writer - SAES clock enable during Sleep mode
pub use GPIOALPEN_W as SAESLPEN_W;
///Field `SRAM2LPEN` writer - SRAM2 clock enable during Sleep mode
pub use GPIOALPEN_W as SRAM2LPEN_W;
///Field `SRAM3LPEN` writer - SRAM3 clock enable during Sleep mode
pub use GPIOALPEN_W as SRAM3LPEN_W;
impl R {
    ///Bit 0 - GPIOA clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIOB clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIOC clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIOD clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPIOE clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIOF clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIOG clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIOH clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GPIOI clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioilpen(&self) -> GPIOILPEN_R {
        GPIOILPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - ADC1 and 2 peripherals clock enable during Sleep mode
    #[inline(always)]
    pub fn adclpen(&self) -> ADCLPEN_R {
        ADCLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DAC clock enable during Sleep mode
    #[inline(always)]
    pub fn dac1lpen(&self) -> DAC1LPEN_R {
        DAC1LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - digital camera interface clock enable during Sleep mode (DCMI or PSSI depending which interface is active)
    #[inline(always)]
    pub fn dcmi_pssilpen(&self) -> DCMI_PSSILPEN_R {
        DCMI_PSSILPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - AES clock enable during Sleep mode
    #[inline(always)]
    pub fn aeslpen(&self) -> AESLPEN_R {
        AESLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HASH clock enable during Sleep mode
    #[inline(always)]
    pub fn hashlpen(&self) -> HASHLPEN_R {
        HASHLPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - RNG clock enable during Sleep mode
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PKA clock enable during Sleep mode
    #[inline(always)]
    pub fn pkalpen(&self) -> PKALPEN_R {
        PKALPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SAES clock enable during Sleep mode
    #[inline(always)]
    pub fn saeslpen(&self) -> SAESLPEN_R {
        SAESLPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 30 - SRAM2 clock enable during Sleep mode
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SRAM3 clock enable during Sleep mode
    #[inline(always)]
    pub fn sram3lpen(&self) -> SRAM3LPEN_R {
        SRAM3LPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2LPENR")
            .field("gpioalpen", &self.gpioalpen())
            .field("gpioblpen", &self.gpioblpen())
            .field("gpioclpen", &self.gpioclpen())
            .field("gpiodlpen", &self.gpiodlpen())
            .field("gpioelpen", &self.gpioelpen())
            .field("gpioflpen", &self.gpioflpen())
            .field("gpioglpen", &self.gpioglpen())
            .field("gpiohlpen", &self.gpiohlpen())
            .field("gpioilpen", &self.gpioilpen())
            .field("adclpen", &self.adclpen())
            .field("dac1lpen", &self.dac1lpen())
            .field("dcmi_pssilpen", &self.dcmi_pssilpen())
            .field("aeslpen", &self.aeslpen())
            .field("hashlpen", &self.hashlpen())
            .field("rnglpen", &self.rnglpen())
            .field("pkalpen", &self.pkalpen())
            .field("saeslpen", &self.saeslpen())
            .field("sram2lpen", &self.sram2lpen())
            .field("sram3lpen", &self.sram3lpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPIOA clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<'_, AHB2LPENRrs> {
        GPIOALPEN_W::new(self, 0)
    }
    ///Bit 1 - GPIOB clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<'_, AHB2LPENRrs> {
        GPIOBLPEN_W::new(self, 1)
    }
    ///Bit 2 - GPIOC clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<'_, AHB2LPENRrs> {
        GPIOCLPEN_W::new(self, 2)
    }
    ///Bit 3 - GPIOD clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<'_, AHB2LPENRrs> {
        GPIODLPEN_W::new(self, 3)
    }
    ///Bit 4 - GPIOE clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W<'_, AHB2LPENRrs> {
        GPIOELPEN_W::new(self, 4)
    }
    ///Bit 5 - GPIOF clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W<'_, AHB2LPENRrs> {
        GPIOFLPEN_W::new(self, 5)
    }
    ///Bit 6 - GPIOG clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W<'_, AHB2LPENRrs> {
        GPIOGLPEN_W::new(self, 6)
    }
    ///Bit 7 - GPIOH clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<'_, AHB2LPENRrs> {
        GPIOHLPEN_W::new(self, 7)
    }
    ///Bit 8 - GPIOI clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioilpen(&mut self) -> GPIOILPEN_W<'_, AHB2LPENRrs> {
        GPIOILPEN_W::new(self, 8)
    }
    ///Bit 10 - ADC1 and 2 peripherals clock enable during Sleep mode
    #[inline(always)]
    pub fn adclpen(&mut self) -> ADCLPEN_W<'_, AHB2LPENRrs> {
        ADCLPEN_W::new(self, 10)
    }
    ///Bit 11 - DAC clock enable during Sleep mode
    #[inline(always)]
    pub fn dac1lpen(&mut self) -> DAC1LPEN_W<'_, AHB2LPENRrs> {
        DAC1LPEN_W::new(self, 11)
    }
    ///Bit 12 - digital camera interface clock enable during Sleep mode (DCMI or PSSI depending which interface is active)
    #[inline(always)]
    pub fn dcmi_pssilpen(&mut self) -> DCMI_PSSILPEN_W<'_, AHB2LPENRrs> {
        DCMI_PSSILPEN_W::new(self, 12)
    }
    ///Bit 16 - AES clock enable during Sleep mode
    #[inline(always)]
    pub fn aeslpen(&mut self) -> AESLPEN_W<'_, AHB2LPENRrs> {
        AESLPEN_W::new(self, 16)
    }
    ///Bit 17 - HASH clock enable during Sleep mode
    #[inline(always)]
    pub fn hashlpen(&mut self) -> HASHLPEN_W<'_, AHB2LPENRrs> {
        HASHLPEN_W::new(self, 17)
    }
    ///Bit 18 - RNG clock enable during Sleep mode
    #[inline(always)]
    pub fn rnglpen(&mut self) -> RNGLPEN_W<'_, AHB2LPENRrs> {
        RNGLPEN_W::new(self, 18)
    }
    ///Bit 19 - PKA clock enable during Sleep mode
    #[inline(always)]
    pub fn pkalpen(&mut self) -> PKALPEN_W<'_, AHB2LPENRrs> {
        PKALPEN_W::new(self, 19)
    }
    ///Bit 20 - SAES clock enable during Sleep mode
    #[inline(always)]
    pub fn saeslpen(&mut self) -> SAESLPEN_W<'_, AHB2LPENRrs> {
        SAESLPEN_W::new(self, 20)
    }
    ///Bit 30 - SRAM2 clock enable during Sleep mode
    #[inline(always)]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W<'_, AHB2LPENRrs> {
        SRAM2LPEN_W::new(self, 30)
    }
    ///Bit 31 - SRAM3 clock enable during Sleep mode
    #[inline(always)]
    pub fn sram3lpen(&mut self) -> SRAM3LPEN_W<'_, AHB2LPENRrs> {
        SRAM3LPEN_W::new(self, 31)
    }
}
/**RCC AHB2 sleep clock register

You can [`read`](crate::Reg::read) this register and get [`ahb2lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#RCC:AHB2LPENR)*/
pub struct AHB2LPENRrs;
impl crate::RegisterSpec for AHB2LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2lpenr::R`](R) reader structure
impl crate::Readable for AHB2LPENRrs {}
///`write(|w| ..)` method takes [`ahb2lpenr::W`](W) writer structure
impl crate::Writable for AHB2LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2LPENR to value 0xc01f_1cff
impl crate::Resettable for AHB2LPENRrs {
    const RESET_VALUE: u32 = 0xc01f_1cff;
}
