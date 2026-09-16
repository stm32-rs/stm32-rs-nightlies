///Register `SCCR` reader
pub type R = crate::R<SCCRrs>;
///Register `SCCR` writer
pub type W = crate::W<SCCRrs>;
/**power management unit bypass

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASS {
    ///0: Power management unit normal operation. Use the internal regulator.
    InternalRegulator = 0,
    ///1: Power management unit bypassed. Use the external power.
    Bypassed = 1,
}
impl From<BYPASS> for bool {
    #[inline(always)]
    fn from(variant: BYPASS) -> Self {
        variant as u8 != 0
    }
}
///Field `BYPASS` reader - power management unit bypass
pub type BYPASS_R = crate::BitReader<BYPASS>;
impl BYPASS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BYPASS {
        match self.bits {
            false => BYPASS::InternalRegulator,
            true => BYPASS::Bypassed,
        }
    }
    ///Power management unit normal operation. Use the internal regulator.
    #[inline(always)]
    pub fn is_internal_regulator(&self) -> bool {
        *self == BYPASS::InternalRegulator
    }
    ///Power management unit bypassed. Use the external power.
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == BYPASS::Bypassed
    }
}
///Field `BYPASS` writer - power management unit bypass
pub type BYPASS_W<'a, REG> = crate::BitWriter<'a, REG, BYPASS>;
impl<'a, REG> BYPASS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Power management unit normal operation. Use the internal regulator.
    #[inline(always)]
    pub fn internal_regulator(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS::InternalRegulator)
    }
    ///Power management unit bypassed. Use the external power.
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS::Bypassed)
    }
}
/**LDO enable The value is set by hardware when the package uses the LDO regulator.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDOENR {
    ///0: Package does not use LDO regulator
    Disabled = 0,
    ///1: Package uses LDO regulator
    Enabled = 1,
}
impl From<LDOENR> for bool {
    #[inline(always)]
    fn from(variant: LDOENR) -> Self {
        variant as u8 != 0
    }
}
///Field `LDOEN` reader - LDO enable The value is set by hardware when the package uses the LDO regulator.
pub type LDOEN_R = crate::BitReader<LDOENR>;
impl LDOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LDOENR {
        match self.bits {
            false => LDOENR::Disabled,
            true => LDOENR::Enabled,
        }
    }
    ///Package does not use LDO regulator
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LDOENR::Disabled
    }
    ///Package uses LDO regulator
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LDOENR::Enabled
    }
}
///Field `SMPSEN` reader - SMPS enable The value is set by hardware when the package uses the SMPS regulator.
pub type SMPSEN_R = crate::BitReader;
impl R {
    ///Bit 0 - power management unit bypass
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - LDO enable The value is set by hardware when the package uses the LDO regulator.
    #[inline(always)]
    pub fn ldoen(&self) -> LDOEN_R {
        LDOEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SMPS enable The value is set by hardware when the package uses the SMPS regulator.
    #[inline(always)]
    pub fn smpsen(&self) -> SMPSEN_R {
        SMPSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCCR")
            .field("bypass", &self.bypass())
            .field("ldoen", &self.ldoen())
            .field("smpsen", &self.smpsen())
            .finish()
    }
}
impl W {
    ///Bit 0 - power management unit bypass
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W<'_, SCCRrs> {
        BYPASS_W::new(self, 0)
    }
}
/**PWR supply configuration control register

You can [`read`](crate::Reg::read) this register and get [`sccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:SCCR)*/
pub struct SCCRrs;
impl crate::RegisterSpec for SCCRrs {
    type Ux = u32;
}
///`read()` method returns [`sccr::R`](R) reader structure
impl crate::Readable for SCCRrs {}
///`write(|w| ..)` method takes [`sccr::W`](W) writer structure
impl crate::Writable for SCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCCR to value 0
impl crate::Resettable for SCCRrs {}
