#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Firewall pre alarm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPAW {
    #[doc = "0: Any code executed outside the protected segment when the Firewall is opened will generate a system reset"]
    PreArmReset = 0,
    #[doc = "1: Any code executed outside the protected segment will close the Firewall"]
    PreArmSet = 1,
}
impl From<FPAW> for bool {
    #[inline(always)]
    fn from(variant: FPAW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPA` reader - Firewall pre alarm"]
pub type FPA_R = crate::BitReader<FPAW>;
impl FPA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPAW {
        match self.bits {
            false => FPAW::PreArmReset,
            true => FPAW::PreArmSet,
        }
    }
    #[doc = "Any code executed outside the protected segment when the Firewall is opened will generate a system reset"]
    #[inline(always)]
    pub fn is_pre_arm_reset(&self) -> bool {
        *self == FPAW::PreArmReset
    }
    #[doc = "Any code executed outside the protected segment will close the Firewall"]
    #[inline(always)]
    pub fn is_pre_arm_set(&self) -> bool {
        *self == FPAW::PreArmSet
    }
}
#[doc = "Field `FPA` writer - Firewall pre alarm"]
pub type FPA_W<'a, REG> = crate::BitWriter<'a, REG, FPAW>;
impl<'a, REG> FPA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Any code executed outside the protected segment when the Firewall is opened will generate a system reset"]
    #[inline(always)]
    pub fn pre_arm_reset(self) -> &'a mut crate::W<REG> {
        self.variant(FPAW::PreArmReset)
    }
    #[doc = "Any code executed outside the protected segment will close the Firewall"]
    #[inline(always)]
    pub fn pre_arm_set(self) -> &'a mut crate::W<REG> {
        self.variant(FPAW::PreArmSet)
    }
}
#[doc = "Volatile data shared\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDSR {
    #[doc = "0: Volatile data segment is not shared and cannot be hit by a non protected executable code when the Firewall is closed"]
    NotShared = 0,
    #[doc = "1: Volatile data segment is shared with non protected application code"]
    Shared = 1,
}
impl From<VDSR> for bool {
    #[inline(always)]
    fn from(variant: VDSR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDS` reader - Volatile data shared"]
pub type VDS_R = crate::BitReader<VDSR>;
impl VDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VDSR {
        match self.bits {
            false => VDSR::NotShared,
            true => VDSR::Shared,
        }
    }
    #[doc = "Volatile data segment is not shared and cannot be hit by a non protected executable code when the Firewall is closed"]
    #[inline(always)]
    pub fn is_not_shared(&self) -> bool {
        *self == VDSR::NotShared
    }
    #[doc = "Volatile data segment is shared with non protected application code"]
    #[inline(always)]
    pub fn is_shared(&self) -> bool {
        *self == VDSR::Shared
    }
}
#[doc = "Volatile data shared\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDSW {
    #[doc = "0: Resets volatile data shared bit"]
    Reset = 0,
}
impl From<VDSW> for bool {
    #[inline(always)]
    fn from(variant: VDSW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDS` writer - Volatile data shared"]
pub type VDS_W<'a, REG> = crate::BitWriter<'a, REG, VDSW>;
impl<'a, REG> VDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resets volatile data shared bit"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(VDSW::Reset)
    }
}
#[doc = "Volatile data execution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDER {
    #[doc = "0: Volatile data segment cannot be executed if VDS = 0"]
    NotExecutable = 0,
    #[doc = "1: Volatile data segment is declared executable whatever VDS bit value"]
    Executable = 1,
}
impl From<VDER> for bool {
    #[inline(always)]
    fn from(variant: VDER) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDE` reader - Volatile data execution"]
pub type VDE_R = crate::BitReader<VDER>;
impl VDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VDER {
        match self.bits {
            false => VDER::NotExecutable,
            true => VDER::Executable,
        }
    }
    #[doc = "Volatile data segment cannot be executed if VDS = 0"]
    #[inline(always)]
    pub fn is_not_executable(&self) -> bool {
        *self == VDER::NotExecutable
    }
    #[doc = "Volatile data segment is declared executable whatever VDS bit value"]
    #[inline(always)]
    pub fn is_executable(&self) -> bool {
        *self == VDER::Executable
    }
}
#[doc = "Volatile data execution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDEW {
    #[doc = "0: Resets volatile data execution bit"]
    Reset = 0,
}
impl From<VDEW> for bool {
    #[inline(always)]
    fn from(variant: VDEW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDE` writer - Volatile data execution"]
pub type VDE_W<'a, REG> = crate::BitWriter<'a, REG, VDEW>;
impl<'a, REG> VDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resets volatile data execution bit"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(VDEW::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - Firewall pre alarm"]
    #[inline(always)]
    pub fn fpa(&self) -> FPA_R {
        FPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Volatile data shared"]
    #[inline(always)]
    pub fn vds(&self) -> VDS_R {
        VDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Volatile data execution"]
    #[inline(always)]
    pub fn vde(&self) -> VDE_R {
        VDE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Firewall pre alarm"]
    #[inline(always)]
    #[must_use]
    pub fn fpa(&mut self) -> FPA_W<CRrs> {
        FPA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Volatile data shared"]
    #[inline(always)]
    #[must_use]
    pub fn vds(&mut self) -> VDS_W<CRrs> {
        VDS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Volatile data execution"]
    #[inline(always)]
    #[must_use]
    pub fn vde(&mut self) -> VDE_W<CRrs> {
        VDE_W::new(self, 2)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
