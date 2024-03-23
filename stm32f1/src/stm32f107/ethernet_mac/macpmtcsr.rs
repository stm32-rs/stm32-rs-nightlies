#[doc = "Register `MACPMTCSR` reader"]
pub type R = crate::R<MACPMTCSRrs>;
#[doc = "Register `MACPMTCSR` writer"]
pub type W = crate::W<MACPMTCSRrs>;
#[doc = "Power down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD {
    #[doc = "1: All received frames will be dropped. Cleared automatically when a magic packet or wakeup frame is received"]
    Enabled = 1,
}
impl From<PD> for bool {
    #[inline(always)]
    fn from(variant: PD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD` reader - Power down"]
pub type PD_R = crate::BitReader<PD>;
impl PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD> {
        match self.bits {
            true => Some(PD::Enabled),
            _ => None,
        }
    }
    #[doc = "All received frames will be dropped. Cleared automatically when a magic packet or wakeup frame is received"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PD::Enabled
    }
}
#[doc = "Field `PD` writer - Power down"]
pub type PD_W<'a, REG> = crate::BitWriter<'a, REG, PD>;
impl<'a, REG> PD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All received frames will be dropped. Cleared automatically when a magic packet or wakeup frame is received"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PD::Enabled)
    }
}
#[doc = "Magic packet enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPE {
    #[doc = "0: No power management event generated due to Magic Packet reception"]
    Disabled = 0,
    #[doc = "1: Enable generation of a power management event due to Magic Packet reception"]
    Enabled = 1,
}
impl From<MPE> for bool {
    #[inline(always)]
    fn from(variant: MPE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPE` reader - Magic packet enable"]
pub type MPE_R = crate::BitReader<MPE>;
impl MPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MPE {
        match self.bits {
            false => MPE::Disabled,
            true => MPE::Enabled,
        }
    }
    #[doc = "No power management event generated due to Magic Packet reception"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MPE::Disabled
    }
    #[doc = "Enable generation of a power management event due to Magic Packet reception"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MPE::Enabled
    }
}
#[doc = "Field `MPE` writer - Magic packet enable"]
pub type MPE_W<'a, REG> = crate::BitWriter<'a, REG, MPE>;
impl<'a, REG> MPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No power management event generated due to Magic Packet reception"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MPE::Disabled)
    }
    #[doc = "Enable generation of a power management event due to Magic Packet reception"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MPE::Enabled)
    }
}
#[doc = "Wakeup frame enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WFE {
    #[doc = "0: No power management event generated due to wakeup frame reception"]
    Disabled = 0,
    #[doc = "1: Enable generation of a power management event due to wakeup frame reception"]
    Enabled = 1,
}
impl From<WFE> for bool {
    #[inline(always)]
    fn from(variant: WFE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WFE` reader - Wakeup frame enable"]
pub type WFE_R = crate::BitReader<WFE>;
impl WFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WFE {
        match self.bits {
            false => WFE::Disabled,
            true => WFE::Enabled,
        }
    }
    #[doc = "No power management event generated due to wakeup frame reception"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WFE::Disabled
    }
    #[doc = "Enable generation of a power management event due to wakeup frame reception"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WFE::Enabled
    }
}
#[doc = "Field `WFE` writer - Wakeup frame enable"]
pub type WFE_W<'a, REG> = crate::BitWriter<'a, REG, WFE>;
impl<'a, REG> WFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No power management event generated due to wakeup frame reception"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WFE::Disabled)
    }
    #[doc = "Enable generation of a power management event due to wakeup frame reception"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WFE::Enabled)
    }
}
#[doc = "Field `MPR` reader - Magic packet received"]
pub type MPR_R = crate::BitReader;
#[doc = "Field `MPR` writer - Magic packet received"]
pub type MPR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WFR` reader - Wakeup frame received"]
pub type WFR_R = crate::BitReader;
#[doc = "Field `WFR` writer - Wakeup frame received"]
pub type WFR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Global unicast\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GU {
    #[doc = "0: Normal operation"]
    Disabled = 0,
    #[doc = "1: Any unicast packet filtered by the MAC address recognition may be a wakeup frame"]
    Enabled = 1,
}
impl From<GU> for bool {
    #[inline(always)]
    fn from(variant: GU) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GU` reader - Global unicast"]
pub type GU_R = crate::BitReader<GU>;
impl GU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GU {
        match self.bits {
            false => GU::Disabled,
            true => GU::Enabled,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GU::Disabled
    }
    #[doc = "Any unicast packet filtered by the MAC address recognition may be a wakeup frame"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GU::Enabled
    }
}
#[doc = "Field `GU` writer - Global unicast"]
pub type GU_W<'a, REG> = crate::BitWriter<'a, REG, GU>;
impl<'a, REG> GU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GU::Disabled)
    }
    #[doc = "Any unicast packet filtered by the MAC address recognition may be a wakeup frame"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GU::Enabled)
    }
}
#[doc = "Wakeup frame filter register pointer reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WFFRPR {
    #[doc = "1: Reset wakeup frame filter register point to 0b000. Automatically cleared"]
    Reset = 1,
}
impl From<WFFRPR> for bool {
    #[inline(always)]
    fn from(variant: WFFRPR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WFFRPR` reader - Wakeup frame filter register pointer reset"]
pub type WFFRPR_R = crate::BitReader<WFFRPR>;
impl WFFRPR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WFFRPR> {
        match self.bits {
            true => Some(WFFRPR::Reset),
            _ => None,
        }
    }
    #[doc = "Reset wakeup frame filter register point to 0b000. Automatically cleared"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WFFRPR::Reset
    }
}
#[doc = "Field `WFFRPR` writer - Wakeup frame filter register pointer reset"]
pub type WFFRPR_W<'a, REG> = crate::BitWriter<'a, REG, WFFRPR>;
impl<'a, REG> WFFRPR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset wakeup frame filter register point to 0b000. Automatically cleared"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(WFFRPR::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Magic packet enable"]
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup frame enable"]
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Magic packet received"]
    #[inline(always)]
    pub fn mpr(&self) -> MPR_R {
        MPR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup frame received"]
    #[inline(always)]
    pub fn wfr(&self) -> WFR_R {
        WFR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    pub fn gu(&self) -> GU_R {
        GU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Wakeup frame filter register pointer reset"]
    #[inline(always)]
    pub fn wffrpr(&self) -> WFFRPR_R {
        WFFRPR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<MACPMTCSRrs> {
        PD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Magic packet enable"]
    #[inline(always)]
    #[must_use]
    pub fn mpe(&mut self) -> MPE_W<MACPMTCSRrs> {
        MPE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup frame enable"]
    #[inline(always)]
    #[must_use]
    pub fn wfe(&mut self) -> WFE_W<MACPMTCSRrs> {
        WFE_W::new(self, 2)
    }
    #[doc = "Bit 5 - Magic packet received"]
    #[inline(always)]
    #[must_use]
    pub fn mpr(&mut self) -> MPR_W<MACPMTCSRrs> {
        MPR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Wakeup frame received"]
    #[inline(always)]
    #[must_use]
    pub fn wfr(&mut self) -> WFR_W<MACPMTCSRrs> {
        WFR_W::new(self, 6)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    #[must_use]
    pub fn gu(&mut self) -> GU_W<MACPMTCSRrs> {
        GU_W::new(self, 9)
    }
    #[doc = "Bit 31 - Wakeup frame filter register pointer reset"]
    #[inline(always)]
    #[must_use]
    pub fn wffrpr(&mut self) -> WFFRPR_W<MACPMTCSRrs> {
        WFFRPR_W::new(self, 31)
    }
}
#[doc = "Ethernet MAC PMT control and status register (ETH_MACPMTCSR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macpmtcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macpmtcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACPMTCSRrs;
impl crate::RegisterSpec for MACPMTCSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macpmtcsr::R`](R) reader structure"]
impl crate::Readable for MACPMTCSRrs {}
#[doc = "`write(|w| ..)` method takes [`macpmtcsr::W`](W) writer structure"]
impl crate::Writable for MACPMTCSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACPMTCSR to value 0"]
impl crate::Resettable for MACPMTCSRrs {
    const RESET_VALUE: u32 = 0;
}
