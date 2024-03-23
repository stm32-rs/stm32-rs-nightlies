#[doc = "Register `AHB3SMENR` reader"]
pub type R = crate::R<AHB3SMENRrs>;
#[doc = "Register `AHB3SMENR` writer"]
pub type W = crate::W<AHB3SMENRrs>;
#[doc = "Flexible memory controller clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMCSMEN {
    #[doc = "0: FMC clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: FMC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<FMCSMEN> for bool {
    #[inline(always)]
    fn from(variant: FMCSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMCSMEN` reader - Flexible memory controller clocks enable during Sleep and Stop modes"]
pub type FMCSMEN_R = crate::BitReader<FMCSMEN>;
impl FMCSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FMCSMEN {
        match self.bits {
            false => FMCSMEN::Disabled,
            true => FMCSMEN::Enabled,
        }
    }
    #[doc = "FMC clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FMCSMEN::Disabled
    }
    #[doc = "FMC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FMCSMEN::Enabled
    }
}
#[doc = "Field `FMCSMEN` writer - Flexible memory controller clocks enable during Sleep and Stop modes"]
pub type FMCSMEN_W<'a, REG> = crate::BitWriter<'a, REG, FMCSMEN>;
impl<'a, REG> FMCSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FMC clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMCSMEN::Disabled)
    }
    #[doc = "FMC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMCSMEN::Enabled)
    }
}
#[doc = "OctoSPI1 memory interface clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSPI1SMEN {
    #[doc = "0: OctoSPI1 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: OctoSPI1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<OSPI1SMEN> for bool {
    #[inline(always)]
    fn from(variant: OSPI1SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSPI1SMEN` reader - OctoSPI1 memory interface clocks enable during Sleep and Stop modes"]
pub type OSPI1SMEN_R = crate::BitReader<OSPI1SMEN>;
impl OSPI1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPI1SMEN {
        match self.bits {
            false => OSPI1SMEN::Disabled,
            true => OSPI1SMEN::Enabled,
        }
    }
    #[doc = "OctoSPI1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSPI1SMEN::Disabled
    }
    #[doc = "OctoSPI1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSPI1SMEN::Enabled
    }
}
#[doc = "Field `OSPI1SMEN` writer - OctoSPI1 memory interface clocks enable during Sleep and Stop modes"]
pub type OSPI1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, OSPI1SMEN>;
impl<'a, REG> OSPI1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OctoSPI1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSPI1SMEN::Disabled)
    }
    #[doc = "OctoSPI1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSPI1SMEN::Enabled)
    }
}
#[doc = "OctoSPI2 memory interface clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCTOSPI2 {
    #[doc = "0: OctoSPI2 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: OctoSPI2 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<OCTOSPI2> for bool {
    #[inline(always)]
    fn from(variant: OCTOSPI2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCTOSPI2` reader - OctoSPI2 memory interface clocks enable during Sleep and Stop modes"]
pub type OCTOSPI2_R = crate::BitReader<OCTOSPI2>;
impl OCTOSPI2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OCTOSPI2 {
        match self.bits {
            false => OCTOSPI2::Disabled,
            true => OCTOSPI2::Enabled,
        }
    }
    #[doc = "OctoSPI2 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OCTOSPI2::Disabled
    }
    #[doc = "OctoSPI2 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OCTOSPI2::Enabled
    }
}
#[doc = "Field `OCTOSPI2` writer - OctoSPI2 memory interface clocks enable during Sleep and Stop modes"]
pub type OCTOSPI2_W<'a, REG> = crate::BitWriter<'a, REG, OCTOSPI2>;
impl<'a, REG> OCTOSPI2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OctoSPI2 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OCTOSPI2::Disabled)
    }
    #[doc = "OctoSPI2 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OCTOSPI2::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Flexible memory controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn fmcsmen(&self) -> FMCSMEN_R {
        FMCSMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - OctoSPI1 memory interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn ospi1smen(&self) -> OSPI1SMEN_R {
        OSPI1SMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OctoSPI2 memory interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn octospi2(&self) -> OCTOSPI2_R {
        OCTOSPI2_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn fmcsmen(&mut self) -> FMCSMEN_W<AHB3SMENRrs> {
        FMCSMEN_W::new(self, 0)
    }
    #[doc = "Bit 8 - OctoSPI1 memory interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn ospi1smen(&mut self) -> OSPI1SMEN_W<AHB3SMENRrs> {
        OSPI1SMEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - OctoSPI2 memory interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn octospi2(&mut self) -> OCTOSPI2_W<AHB3SMENRrs> {
        OCTOSPI2_W::new(self, 9)
    }
}
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB3SMENRrs;
impl crate::RegisterSpec for AHB3SMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3smenr::R`](R) reader structure"]
impl crate::Readable for AHB3SMENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb3smenr::W`](W) writer structure"]
impl crate::Writable for AHB3SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB3SMENR to value 0x0301"]
impl crate::Resettable for AHB3SMENRrs {
    const RESET_VALUE: u32 = 0x0301;
}
