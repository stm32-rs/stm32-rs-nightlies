///Register `AHB3SMENR` reader
pub type R = crate::R<AHB3SMENRrs>;
///Register `AHB3SMENR` writer
pub type W = crate::W<AHB3SMENRrs>;
/**Flexible memory controller clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMCSMEN {
    ///0: FMC clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: FMC clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<FMCSMEN> for bool {
    #[inline(always)]
    fn from(variant: FMCSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `FMCSMEN` reader - Flexible memory controller clocks enable during Sleep and Stop modes
pub type FMCSMEN_R = crate::BitReader<FMCSMEN>;
impl FMCSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FMCSMEN {
        match self.bits {
            false => FMCSMEN::Disabled,
            true => FMCSMEN::Enabled,
        }
    }
    ///FMC clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FMCSMEN::Disabled
    }
    ///FMC clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FMCSMEN::Enabled
    }
}
///Field `FMCSMEN` writer - Flexible memory controller clocks enable during Sleep and Stop modes
pub type FMCSMEN_W<'a, REG> = crate::BitWriter<'a, REG, FMCSMEN>;
impl<'a, REG> FMCSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FMC clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMCSMEN::Disabled)
    }
    ///FMC clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMCSMEN::Enabled)
    }
}
/**OctoSPI1 memory interface clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSPI1SMEN {
    ///0: OctoSPI1 clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: OctoSPI1 clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<OSPI1SMEN> for bool {
    #[inline(always)]
    fn from(variant: OSPI1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `OSPI1SMEN` reader - OctoSPI1 memory interface clocks enable during Sleep and Stop modes
pub type OSPI1SMEN_R = crate::BitReader<OSPI1SMEN>;
impl OSPI1SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPI1SMEN {
        match self.bits {
            false => OSPI1SMEN::Disabled,
            true => OSPI1SMEN::Enabled,
        }
    }
    ///OctoSPI1 clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSPI1SMEN::Disabled
    }
    ///OctoSPI1 clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSPI1SMEN::Enabled
    }
}
///Field `OSPI1SMEN` writer - OctoSPI1 memory interface clocks enable during Sleep and Stop modes
pub type OSPI1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, OSPI1SMEN>;
impl<'a, REG> OSPI1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OctoSPI1 clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSPI1SMEN::Disabled)
    }
    ///OctoSPI1 clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSPI1SMEN::Enabled)
    }
}
/**OctoSPI2 memory interface clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCTOSPI2 {
    ///0: OctoSPI2 clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: OctoSPI2 clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<OCTOSPI2> for bool {
    #[inline(always)]
    fn from(variant: OCTOSPI2) -> Self {
        variant as u8 != 0
    }
}
///Field `OCTOSPI2` reader - OctoSPI2 memory interface clocks enable during Sleep and Stop modes
pub type OCTOSPI2_R = crate::BitReader<OCTOSPI2>;
impl OCTOSPI2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OCTOSPI2 {
        match self.bits {
            false => OCTOSPI2::Disabled,
            true => OCTOSPI2::Enabled,
        }
    }
    ///OctoSPI2 clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OCTOSPI2::Disabled
    }
    ///OctoSPI2 clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OCTOSPI2::Enabled
    }
}
///Field `OCTOSPI2` writer - OctoSPI2 memory interface clocks enable during Sleep and Stop modes
pub type OCTOSPI2_W<'a, REG> = crate::BitWriter<'a, REG, OCTOSPI2>;
impl<'a, REG> OCTOSPI2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OctoSPI2 clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OCTOSPI2::Disabled)
    }
    ///OctoSPI2 clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OCTOSPI2::Enabled)
    }
}
impl R {
    ///Bit 0 - Flexible memory controller clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn fmcsmen(&self) -> FMCSMEN_R {
        FMCSMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - OctoSPI1 memory interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn ospi1smen(&self) -> OSPI1SMEN_R {
        OSPI1SMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - OctoSPI2 memory interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn octospi2(&self) -> OCTOSPI2_R {
        OCTOSPI2_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3SMENR")
            .field("fmcsmen", &self.fmcsmen())
            .field("octospi2", &self.octospi2())
            .field("ospi1smen", &self.ospi1smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Flexible memory controller clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn fmcsmen(&mut self) -> FMCSMEN_W<'_, AHB3SMENRrs> {
        FMCSMEN_W::new(self, 0)
    }
    ///Bit 8 - OctoSPI1 memory interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn ospi1smen(&mut self) -> OSPI1SMEN_W<'_, AHB3SMENRrs> {
        OSPI1SMEN_W::new(self, 8)
    }
    ///Bit 9 - OctoSPI2 memory interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn octospi2(&mut self) -> OCTOSPI2_W<'_, AHB3SMENRrs> {
        OCTOSPI2_W::new(self, 9)
    }
}
/**AHB3 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb3smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:AHB3SMENR)*/
pub struct AHB3SMENRrs;
impl crate::RegisterSpec for AHB3SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3smenr::R`](R) reader structure
impl crate::Readable for AHB3SMENRrs {}
///`write(|w| ..)` method takes [`ahb3smenr::W`](W) writer structure
impl crate::Writable for AHB3SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3SMENR to value 0x0301
impl crate::Resettable for AHB3SMENRrs {
    const RESET_VALUE: u32 = 0x0301;
}
