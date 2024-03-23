#[doc = "Register `SCCR` reader"]
pub type R = crate::R<SCCRrs>;
#[doc = "Register `SCCR` writer"]
pub type W = crate::W<SCCRrs>;
#[doc = "power management unit bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASS {
    #[doc = "0: Power management unit normal operation. Use the internal regulator."]
    InternalRegulator = 0,
    #[doc = "1: Power management unit bypassed. Use the external power."]
    Bypassed = 1,
}
impl From<BYPASS> for bool {
    #[inline(always)]
    fn from(variant: BYPASS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS` reader - power management unit bypass"]
pub type BYPASS_R = crate::BitReader<BYPASS>;
impl BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BYPASS {
        match self.bits {
            false => BYPASS::InternalRegulator,
            true => BYPASS::Bypassed,
        }
    }
    #[doc = "Power management unit normal operation. Use the internal regulator."]
    #[inline(always)]
    pub fn is_internal_regulator(&self) -> bool {
        *self == BYPASS::InternalRegulator
    }
    #[doc = "Power management unit bypassed. Use the external power."]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == BYPASS::Bypassed
    }
}
#[doc = "Field `BYPASS` writer - power management unit bypass"]
pub type BYPASS_W<'a, REG> = crate::BitWriter<'a, REG, BYPASS>;
impl<'a, REG> BYPASS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power management unit normal operation. Use the internal regulator."]
    #[inline(always)]
    pub fn internal_regulator(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS::InternalRegulator)
    }
    #[doc = "Power management unit bypassed. Use the external power."]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS::Bypassed)
    }
}
#[doc = "LDO enable The value is set by hardware when the package uses the LDO regulator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDOENR {
    #[doc = "0: Package does not use LDO regulator"]
    Disabled = 0,
    #[doc = "1: Package uses LDO regulator"]
    Enabled = 1,
}
impl From<LDOENR> for bool {
    #[inline(always)]
    fn from(variant: LDOENR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDOEN` reader - LDO enable The value is set by hardware when the package uses the LDO regulator."]
pub type LDOEN_R = crate::BitReader<LDOENR>;
impl LDOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LDOENR {
        match self.bits {
            false => LDOENR::Disabled,
            true => LDOENR::Enabled,
        }
    }
    #[doc = "Package does not use LDO regulator"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LDOENR::Disabled
    }
    #[doc = "Package uses LDO regulator"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LDOENR::Enabled
    }
}
#[doc = "Field `SMPSEN` reader - SMPS enable The value is set by hardware when the package uses the SMPS regulator."]
pub type SMPSEN_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - power management unit bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - LDO enable The value is set by hardware when the package uses the LDO regulator."]
    #[inline(always)]
    pub fn ldoen(&self) -> LDOEN_R {
        LDOEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SMPS enable The value is set by hardware when the package uses the SMPS regulator."]
    #[inline(always)]
    pub fn smpsen(&self) -> SMPSEN_R {
        SMPSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - power management unit bypass"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<SCCRrs> {
        BYPASS_W::new(self, 0)
    }
}
#[doc = "PWR supply configuration control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCCRrs;
impl crate::RegisterSpec for SCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sccr::R`](R) reader structure"]
impl crate::Readable for SCCRrs {}
#[doc = "`write(|w| ..)` method takes [`sccr::W`](W) writer structure"]
impl crate::Writable for SCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCCR to value 0"]
impl crate::Resettable for SCCRrs {
    const RESET_VALUE: u32 = 0;
}
