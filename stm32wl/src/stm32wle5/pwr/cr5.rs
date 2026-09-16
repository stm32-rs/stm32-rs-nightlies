///Register `CR5` reader
pub type R = crate::R<CR5rs>;
///Register `CR5` writer
pub type W = crate::W<CR5rs>;
/**Enable Radio End Of Life detector enabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFEOLEN {
    ///0: Radio end-of-life detector disabled
    Disabled = 0,
    ///1: Radio end-of-life detector enabled
    Enabled = 1,
}
impl From<RFEOLEN> for bool {
    #[inline(always)]
    fn from(variant: RFEOLEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RFEOLEN` reader - Enable Radio End Of Life detector enabled
pub type RFEOLEN_R = crate::BitReader<RFEOLEN>;
impl RFEOLEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFEOLEN {
        match self.bits {
            false => RFEOLEN::Disabled,
            true => RFEOLEN::Enabled,
        }
    }
    ///Radio end-of-life detector disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RFEOLEN::Disabled
    }
    ///Radio end-of-life detector enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RFEOLEN::Enabled
    }
}
///Field `RFEOLEN` writer - Enable Radio End Of Life detector enabled
pub type RFEOLEN_W<'a, REG> = crate::BitWriter<'a, REG, RFEOLEN>;
impl<'a, REG> RFEOLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Radio end-of-life detector disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RFEOLEN::Disabled)
    }
    ///Radio end-of-life detector enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RFEOLEN::Enabled)
    }
}
/**Enable SMPS Step Down converter SMPS mode enabled.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEN {
    ///0: SMPS step-down converter SMPS mode disabled (LDO mode enabled)
    Disabled = 0,
    ///1: SMPS step-down converter SMPS mode enabled
    Enabled = 1,
}
impl From<SMPSEN> for bool {
    #[inline(always)]
    fn from(variant: SMPSEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEN` reader - Enable SMPS Step Down converter SMPS mode enabled.
pub type SMPSEN_R = crate::BitReader<SMPSEN>;
impl SMPSEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEN {
        match self.bits {
            false => SMPSEN::Disabled,
            true => SMPSEN::Enabled,
        }
    }
    ///SMPS step-down converter SMPS mode disabled (LDO mode enabled)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMPSEN::Disabled
    }
    ///SMPS step-down converter SMPS mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMPSEN::Enabled
    }
}
///Field `SMPSEN` writer - Enable SMPS Step Down converter SMPS mode enabled.
pub type SMPSEN_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEN>;
impl<'a, REG> SMPSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SMPS step-down converter SMPS mode disabled (LDO mode enabled)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEN::Disabled)
    }
    ///SMPS step-down converter SMPS mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEN::Enabled)
    }
}
impl R {
    ///Bit 14 - Enable Radio End Of Life detector enabled
    #[inline(always)]
    pub fn rfeolen(&self) -> RFEOLEN_R {
        RFEOLEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Enable SMPS Step Down converter SMPS mode enabled.
    #[inline(always)]
    pub fn smpsen(&self) -> SMPSEN_R {
        SMPSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR5")
            .field("smpsen", &self.smpsen())
            .field("rfeolen", &self.rfeolen())
            .finish()
    }
}
impl W {
    ///Bit 14 - Enable Radio End Of Life detector enabled
    #[inline(always)]
    pub fn rfeolen(&mut self) -> RFEOLEN_W<'_, CR5rs> {
        RFEOLEN_W::new(self, 14)
    }
    ///Bit 15 - Enable SMPS Step Down converter SMPS mode enabled.
    #[inline(always)]
    pub fn smpsen(&mut self) -> SMPSEN_W<'_, CR5rs> {
        SMPSEN_W::new(self, 15)
    }
}
/**Power control register 5

You can [`read`](crate::Reg::read) this register and get [`cr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#PWR:CR5)*/
pub struct CR5rs;
impl crate::RegisterSpec for CR5rs {
    type Ux = u32;
}
///`read()` method returns [`cr5::R`](R) reader structure
impl crate::Readable for CR5rs {}
///`write(|w| ..)` method takes [`cr5::W`](W) writer structure
impl crate::Writable for CR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR5 to value 0
impl crate::Resettable for CR5rs {}
