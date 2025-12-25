///Register `AHB2ENR` reader
pub type R = crate::R<AHB2ENRrs>;
///Register `AHB2ENR` writer
pub type W = crate::W<AHB2ENRrs>;
/**GPIOA clock enable Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<GPIOAEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOAEN` reader - GPIOA clock enable Set and reset by software.
pub type GPIOAEN_R = crate::BitReader<GPIOAEN>;
impl GPIOAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOAEN {
        match self.bits {
            false => GPIOAEN::Disabled,
            true => GPIOAEN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN::Enabled
    }
}
///Field `GPIOAEN` writer - GPIOA clock enable Set and reset by software.
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOAEN>;
impl<'a, REG> GPIOAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Enabled)
    }
}
///Field `GPIOBEN` reader - GPIOB clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOBEN_R;
///Field `GPIOCEN` reader - GPIOC clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOCEN_R;
///Field `GPIODEN` reader - GPIOD clock enable Set and reset by software.
pub use GPIOAEN_R as GPIODEN_R;
///Field `GPIOEEN` reader - GPIOE clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOEEN_R;
///Field `GPIOFEN` reader - GPIOF clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOFEN_R;
///Field `GPIOGEN` reader - GPIOG clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOGEN_R;
///Field `GPIOHEN` reader - GPIOH clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOHEN_R;
///Field `GPIOIEN` reader - GPIOI clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOIEN_R;
///Field `ADCEN` reader - ADC1 and 2 peripherals clock enable
pub use GPIOAEN_R as ADCEN_R;
///Field `DAC1EN` reader - DAC clock enable
pub use GPIOAEN_R as DAC1EN_R;
///Field `DCMI_PSSIEN` reader - digital camera interface clock enable (DCMI or PSSI depending which interface is active) Set and reset by software.
pub use GPIOAEN_R as DCMI_PSSIEN_R;
///Field `HASHEN` reader - HASH clock enable Set and reset by software.
pub use GPIOAEN_R as HASHEN_R;
///Field `RNGEN` reader - RNG clock enable Set and reset by software.
pub use GPIOAEN_R as RNGEN_R;
///Field `SRAM3EN` reader - SRAM3 clock enable Set and reset by software.
pub use GPIOAEN_R as SRAM3EN_R;
///Field `SRAM2EN` reader - SRAM2 clock enable Set and reset by software.
pub use GPIOAEN_R as SRAM2EN_R;
///Field `GPIOBEN` writer - GPIOB clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOBEN_W;
///Field `GPIOCEN` writer - GPIOC clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOCEN_W;
///Field `GPIODEN` writer - GPIOD clock enable Set and reset by software.
pub use GPIOAEN_W as GPIODEN_W;
///Field `GPIOEEN` writer - GPIOE clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOEEN_W;
///Field `GPIOFEN` writer - GPIOF clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOFEN_W;
///Field `GPIOGEN` writer - GPIOG clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOGEN_W;
///Field `GPIOHEN` writer - GPIOH clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOHEN_W;
///Field `GPIOIEN` writer - GPIOI clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOIEN_W;
///Field `ADCEN` writer - ADC1 and 2 peripherals clock enable
pub use GPIOAEN_W as ADCEN_W;
///Field `DAC1EN` writer - DAC clock enable
pub use GPIOAEN_W as DAC1EN_W;
///Field `DCMI_PSSIEN` writer - digital camera interface clock enable (DCMI or PSSI depending which interface is active) Set and reset by software.
pub use GPIOAEN_W as DCMI_PSSIEN_W;
///Field `HASHEN` writer - HASH clock enable Set and reset by software.
pub use GPIOAEN_W as HASHEN_W;
///Field `RNGEN` writer - RNG clock enable Set and reset by software.
pub use GPIOAEN_W as RNGEN_W;
///Field `SRAM3EN` writer - SRAM3 clock enable Set and reset by software.
pub use GPIOAEN_W as SRAM3EN_W;
///Field `SRAM2EN` writer - SRAM2 clock enable Set and reset by software.
pub use GPIOAEN_W as SRAM2EN_W;
impl R {
    ///Bit 0 - GPIOA clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIOB clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIOC clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIOD clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPIOE clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIOF clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIOG clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIOH clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GPIOI clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - ADC1 and 2 peripherals clock enable
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DAC clock enable
    #[inline(always)]
    pub fn dac1en(&self) -> DAC1EN_R {
        DAC1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - digital camera interface clock enable (DCMI or PSSI depending which interface is active) Set and reset by software.
    #[inline(always)]
    pub fn dcmi_pssien(&self) -> DCMI_PSSIEN_R {
        DCMI_PSSIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 17 - HASH clock enable Set and reset by software.
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - RNG clock enable Set and reset by software.
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 30 - SRAM3 clock enable Set and reset by software.
    #[inline(always)]
    pub fn sram3en(&self) -> SRAM3EN_R {
        SRAM3EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SRAM2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn sram2en(&self) -> SRAM2EN_R {
        SRAM2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2ENR")
            .field("gpioaen", &self.gpioaen())
            .field("gpioben", &self.gpioben())
            .field("gpiocen", &self.gpiocen())
            .field("gpioden", &self.gpioden())
            .field("gpioeen", &self.gpioeen())
            .field("gpiofen", &self.gpiofen())
            .field("gpiogen", &self.gpiogen())
            .field("gpiohen", &self.gpiohen())
            .field("gpioien", &self.gpioien())
            .field("adcen", &self.adcen())
            .field("dac1en", &self.dac1en())
            .field("dcmi_pssien", &self.dcmi_pssien())
            .field("hashen", &self.hashen())
            .field("rngen", &self.rngen())
            .field("sram3en", &self.sram3en())
            .field("sram2en", &self.sram2en())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPIOA clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<'_, AHB2ENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    ///Bit 1 - GPIOB clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<'_, AHB2ENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    ///Bit 2 - GPIOC clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<'_, AHB2ENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    ///Bit 3 - GPIOD clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W<'_, AHB2ENRrs> {
        GPIODEN_W::new(self, 3)
    }
    ///Bit 4 - GPIOE clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<'_, AHB2ENRrs> {
        GPIOEEN_W::new(self, 4)
    }
    ///Bit 5 - GPIOF clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<'_, AHB2ENRrs> {
        GPIOFEN_W::new(self, 5)
    }
    ///Bit 6 - GPIOG clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<'_, AHB2ENRrs> {
        GPIOGEN_W::new(self, 6)
    }
    ///Bit 7 - GPIOH clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<'_, AHB2ENRrs> {
        GPIOHEN_W::new(self, 7)
    }
    ///Bit 8 - GPIOI clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioien(&mut self) -> GPIOIEN_W<'_, AHB2ENRrs> {
        GPIOIEN_W::new(self, 8)
    }
    ///Bit 10 - ADC1 and 2 peripherals clock enable
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W<'_, AHB2ENRrs> {
        ADCEN_W::new(self, 10)
    }
    ///Bit 11 - DAC clock enable
    #[inline(always)]
    pub fn dac1en(&mut self) -> DAC1EN_W<'_, AHB2ENRrs> {
        DAC1EN_W::new(self, 11)
    }
    ///Bit 12 - digital camera interface clock enable (DCMI or PSSI depending which interface is active) Set and reset by software.
    #[inline(always)]
    pub fn dcmi_pssien(&mut self) -> DCMI_PSSIEN_W<'_, AHB2ENRrs> {
        DCMI_PSSIEN_W::new(self, 12)
    }
    ///Bit 17 - HASH clock enable Set and reset by software.
    #[inline(always)]
    pub fn hashen(&mut self) -> HASHEN_W<'_, AHB2ENRrs> {
        HASHEN_W::new(self, 17)
    }
    ///Bit 18 - RNG clock enable Set and reset by software.
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<'_, AHB2ENRrs> {
        RNGEN_W::new(self, 18)
    }
    ///Bit 30 - SRAM3 clock enable Set and reset by software.
    #[inline(always)]
    pub fn sram3en(&mut self) -> SRAM3EN_W<'_, AHB2ENRrs> {
        SRAM3EN_W::new(self, 30)
    }
    ///Bit 31 - SRAM2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn sram2en(&mut self) -> SRAM2EN_W<'_, AHB2ENRrs> {
        SRAM2EN_W::new(self, 31)
    }
}
/**RCC AHB2 peripheral clock register

You can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#RCC:AHB2ENR)*/
pub struct AHB2ENRrs;
impl crate::RegisterSpec for AHB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2enr::R`](R) reader structure
impl crate::Readable for AHB2ENRrs {}
///`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure
impl crate::Writable for AHB2ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2ENR to value 0xc000_0000
impl crate::Resettable for AHB2ENRrs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
