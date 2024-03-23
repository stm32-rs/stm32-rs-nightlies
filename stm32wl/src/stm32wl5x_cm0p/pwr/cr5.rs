#[doc = "Register `CR5` reader"]
pub type R = crate::R<CR5rs>;
#[doc = "Register `CR5` writer"]
pub type W = crate::W<CR5rs>;
#[doc = "Enable Radio End Of Life detector enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFEOLEN {
    #[doc = "0: Radio end-of-life detector disabled"]
    Disabled = 0,
    #[doc = "1: Radio end-of-life detector enabled"]
    Enabled = 1,
}
impl From<RFEOLEN> for bool {
    #[inline(always)]
    fn from(variant: RFEOLEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFEOLEN` reader - Enable Radio End Of Life detector enabled"]
pub type RFEOLEN_R = crate::BitReader<RFEOLEN>;
impl RFEOLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFEOLEN {
        match self.bits {
            false => RFEOLEN::Disabled,
            true => RFEOLEN::Enabled,
        }
    }
    #[doc = "Radio end-of-life detector disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RFEOLEN::Disabled
    }
    #[doc = "Radio end-of-life detector enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RFEOLEN::Enabled
    }
}
#[doc = "Field `RFEOLEN` writer - Enable Radio End Of Life detector enabled"]
pub type RFEOLEN_W<'a, REG> = crate::BitWriter<'a, REG, RFEOLEN>;
impl<'a, REG> RFEOLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Radio end-of-life detector disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RFEOLEN::Disabled)
    }
    #[doc = "Radio end-of-life detector enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RFEOLEN::Enabled)
    }
}
#[doc = "Enable SMPS Step Down converter SMPS mode enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEN {
    #[doc = "0: SMPS step-down converter SMPS mode disabled (LDO mode enabled)"]
    Disabled = 0,
    #[doc = "1: SMPS step-down converter SMPS mode enabled"]
    Enabled = 1,
}
impl From<SMPSEN> for bool {
    #[inline(always)]
    fn from(variant: SMPSEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEN` reader - Enable SMPS Step Down converter SMPS mode enabled."]
pub type SMPSEN_R = crate::BitReader<SMPSEN>;
impl SMPSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEN {
        match self.bits {
            false => SMPSEN::Disabled,
            true => SMPSEN::Enabled,
        }
    }
    #[doc = "SMPS step-down converter SMPS mode disabled (LDO mode enabled)"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMPSEN::Disabled
    }
    #[doc = "SMPS step-down converter SMPS mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMPSEN::Enabled
    }
}
#[doc = "Field `SMPSEN` writer - Enable SMPS Step Down converter SMPS mode enabled."]
pub type SMPSEN_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEN>;
impl<'a, REG> SMPSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMPS step-down converter SMPS mode disabled (LDO mode enabled)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEN::Disabled)
    }
    #[doc = "SMPS step-down converter SMPS mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEN::Enabled)
    }
}
impl R {
    #[doc = "Bit 14 - Enable Radio End Of Life detector enabled"]
    #[inline(always)]
    pub fn rfeolen(&self) -> RFEOLEN_R {
        RFEOLEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable SMPS Step Down converter SMPS mode enabled."]
    #[inline(always)]
    pub fn smpsen(&self) -> SMPSEN_R {
        SMPSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Enable Radio End Of Life detector enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rfeolen(&mut self) -> RFEOLEN_W<CR5rs> {
        RFEOLEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable SMPS Step Down converter SMPS mode enabled."]
    #[inline(always)]
    #[must_use]
    pub fn smpsen(&mut self) -> SMPSEN_W<CR5rs> {
        SMPSEN_W::new(self, 15)
    }
}
#[doc = "Power control register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR5rs;
impl crate::RegisterSpec for CR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr5::R`](R) reader structure"]
impl crate::Readable for CR5rs {}
#[doc = "`write(|w| ..)` method takes [`cr5::W`](W) writer structure"]
impl crate::Writable for CR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR5 to value 0"]
impl crate::Resettable for CR5rs {
    const RESET_VALUE: u32 = 0;
}
