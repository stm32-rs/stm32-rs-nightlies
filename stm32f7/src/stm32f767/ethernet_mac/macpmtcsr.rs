///Register `MACPMTCSR` reader
pub type R = crate::R<MACPMTCSRrs>;
///Register `MACPMTCSR` writer
pub type W = crate::W<MACPMTCSRrs>;
/**Power down

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD {
    ///1: All received frames will be dropped. Cleared automatically when a magic packet or wakeup frame is received
    Enabled = 1,
}
impl From<PD> for bool {
    #[inline(always)]
    fn from(variant: PD) -> Self {
        variant as u8 != 0
    }
}
///Field `PD` reader - Power down
pub type PD_R = crate::BitReader<PD>;
impl PD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD> {
        match self.bits {
            true => Some(PD::Enabled),
            _ => None,
        }
    }
    ///All received frames will be dropped. Cleared automatically when a magic packet or wakeup frame is received
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PD::Enabled
    }
}
///Field `PD` writer - Power down
pub type PD_W<'a, REG> = crate::BitWriter<'a, REG, PD>;
impl<'a, REG> PD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///All received frames will be dropped. Cleared automatically when a magic packet or wakeup frame is received
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PD::Enabled)
    }
}
/**Magic packet enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPE {
    ///0: No power management event generated due to Magic Packet reception
    Disabled = 0,
    ///1: Enable generation of a power management event due to Magic Packet reception
    Enabled = 1,
}
impl From<MPE> for bool {
    #[inline(always)]
    fn from(variant: MPE) -> Self {
        variant as u8 != 0
    }
}
///Field `MPE` reader - Magic packet enable
pub type MPE_R = crate::BitReader<MPE>;
impl MPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MPE {
        match self.bits {
            false => MPE::Disabled,
            true => MPE::Enabled,
        }
    }
    ///No power management event generated due to Magic Packet reception
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MPE::Disabled
    }
    ///Enable generation of a power management event due to Magic Packet reception
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MPE::Enabled
    }
}
///Field `MPE` writer - Magic packet enable
pub type MPE_W<'a, REG> = crate::BitWriter<'a, REG, MPE>;
impl<'a, REG> MPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No power management event generated due to Magic Packet reception
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MPE::Disabled)
    }
    ///Enable generation of a power management event due to Magic Packet reception
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MPE::Enabled)
    }
}
/**Wakeup frame enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WFE {
    ///0: No power management event generated due to wakeup frame reception
    Disabled = 0,
    ///1: Enable generation of a power management event due to wakeup frame reception
    Enabled = 1,
}
impl From<WFE> for bool {
    #[inline(always)]
    fn from(variant: WFE) -> Self {
        variant as u8 != 0
    }
}
///Field `WFE` reader - Wakeup frame enable
pub type WFE_R = crate::BitReader<WFE>;
impl WFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WFE {
        match self.bits {
            false => WFE::Disabled,
            true => WFE::Enabled,
        }
    }
    ///No power management event generated due to wakeup frame reception
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WFE::Disabled
    }
    ///Enable generation of a power management event due to wakeup frame reception
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WFE::Enabled
    }
}
///Field `WFE` writer - Wakeup frame enable
pub type WFE_W<'a, REG> = crate::BitWriter<'a, REG, WFE>;
impl<'a, REG> WFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No power management event generated due to wakeup frame reception
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WFE::Disabled)
    }
    ///Enable generation of a power management event due to wakeup frame reception
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WFE::Enabled)
    }
}
///Field `MPR` reader - Magic packet received
pub type MPR_R = crate::BitReader;
///Field `MPR` writer - Magic packet received
pub type MPR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WFR` reader - Wakeup frame received
pub type WFR_R = crate::BitReader;
///Field `WFR` writer - Wakeup frame received
pub type WFR_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Global unicast

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GU {
    ///0: Normal operation
    Disabled = 0,
    ///1: Any unicast packet filtered by the MAC address recognition may be a wakeup frame
    Enabled = 1,
}
impl From<GU> for bool {
    #[inline(always)]
    fn from(variant: GU) -> Self {
        variant as u8 != 0
    }
}
///Field `GU` reader - Global unicast
pub type GU_R = crate::BitReader<GU>;
impl GU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GU {
        match self.bits {
            false => GU::Disabled,
            true => GU::Enabled,
        }
    }
    ///Normal operation
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GU::Disabled
    }
    ///Any unicast packet filtered by the MAC address recognition may be a wakeup frame
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GU::Enabled
    }
}
///Field `GU` writer - Global unicast
pub type GU_W<'a, REG> = crate::BitWriter<'a, REG, GU>;
impl<'a, REG> GU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal operation
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GU::Disabled)
    }
    ///Any unicast packet filtered by the MAC address recognition may be a wakeup frame
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GU::Enabled)
    }
}
/**Wakeup frame filter register pointer reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WFFRPR {
    ///1: Reset wakeup frame filter register point to 0b000. Automatically cleared
    Reset = 1,
}
impl From<WFFRPR> for bool {
    #[inline(always)]
    fn from(variant: WFFRPR) -> Self {
        variant as u8 != 0
    }
}
///Field `WFFRPR` reader - Wakeup frame filter register pointer reset
pub type WFFRPR_R = crate::BitReader<WFFRPR>;
impl WFFRPR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<WFFRPR> {
        match self.bits {
            true => Some(WFFRPR::Reset),
            _ => None,
        }
    }
    ///Reset wakeup frame filter register point to 0b000. Automatically cleared
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WFFRPR::Reset
    }
}
///Field `WFFRPR` writer - Wakeup frame filter register pointer reset
pub type WFFRPR_W<'a, REG> = crate::BitWriter<'a, REG, WFFRPR>;
impl<'a, REG> WFFRPR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset wakeup frame filter register point to 0b000. Automatically cleared
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(WFFRPR::Reset)
    }
}
impl R {
    ///Bit 0 - Power down
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Magic packet enable
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup frame enable
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - Magic packet received
    #[inline(always)]
    pub fn mpr(&self) -> MPR_R {
        MPR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Wakeup frame received
    #[inline(always)]
    pub fn wfr(&self) -> WFR_R {
        WFR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - Global unicast
    #[inline(always)]
    pub fn gu(&self) -> GU_R {
        GU_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 31 - Wakeup frame filter register pointer reset
    #[inline(always)]
    pub fn wffrpr(&self) -> WFFRPR_R {
        WFFRPR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPMTCSR")
            .field("pd", &self.pd())
            .field("mpe", &self.mpe())
            .field("wfe", &self.wfe())
            .field("mpr", &self.mpr())
            .field("wfr", &self.wfr())
            .field("gu", &self.gu())
            .field("wffrpr", &self.wffrpr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Power down
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W<'_, MACPMTCSRrs> {
        PD_W::new(self, 0)
    }
    ///Bit 1 - Magic packet enable
    #[inline(always)]
    pub fn mpe(&mut self) -> MPE_W<'_, MACPMTCSRrs> {
        MPE_W::new(self, 1)
    }
    ///Bit 2 - Wakeup frame enable
    #[inline(always)]
    pub fn wfe(&mut self) -> WFE_W<'_, MACPMTCSRrs> {
        WFE_W::new(self, 2)
    }
    ///Bit 5 - Magic packet received
    #[inline(always)]
    pub fn mpr(&mut self) -> MPR_W<'_, MACPMTCSRrs> {
        MPR_W::new(self, 5)
    }
    ///Bit 6 - Wakeup frame received
    #[inline(always)]
    pub fn wfr(&mut self) -> WFR_W<'_, MACPMTCSRrs> {
        WFR_W::new(self, 6)
    }
    ///Bit 9 - Global unicast
    #[inline(always)]
    pub fn gu(&mut self) -> GU_W<'_, MACPMTCSRrs> {
        GU_W::new(self, 9)
    }
    ///Bit 31 - Wakeup frame filter register pointer reset
    #[inline(always)]
    pub fn wffrpr(&mut self) -> WFFRPR_W<'_, MACPMTCSRrs> {
        WFFRPR_W::new(self, 31)
    }
}
/**Ethernet MAC PMT control and status register

You can [`read`](crate::Reg::read) this register and get [`macpmtcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpmtcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#Ethernet_MAC:MACPMTCSR)*/
pub struct MACPMTCSRrs;
impl crate::RegisterSpec for MACPMTCSRrs {
    type Ux = u32;
}
///`read()` method returns [`macpmtcsr::R`](R) reader structure
impl crate::Readable for MACPMTCSRrs {}
///`write(|w| ..)` method takes [`macpmtcsr::W`](W) writer structure
impl crate::Writable for MACPMTCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPMTCSR to value 0
impl crate::Resettable for MACPMTCSRrs {}
