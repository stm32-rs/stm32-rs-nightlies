#[doc = "Register `OPTR` reader"]
pub type R = crate::R<OPTRrs>;
#[doc = "Register `OPTR` writer"]
pub type W = crate::W<OPTRrs>;
#[doc = "Read protection level\n\nValue on reset: 170"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDP {
    #[doc = "136: Level 1, memories readout protection active (writes 0x88)"]
    Level1 = 136,
    #[doc = "170: Level 0, readout protection not active"]
    Level0 = 170,
    #[doc = "204: Level 2, chip readout protection active"]
    Level2 = 204,
}
impl From<RDP> for u8 {
    #[inline(always)]
    fn from(variant: RDP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RDP {
    type Ux = u8;
}
#[doc = "Field `RDP` reader - Read protection level"]
pub type RDP_R = crate::FieldReader<RDP>;
impl RDP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RDP> {
        match self.bits {
            136 => Some(RDP::Level1),
            170 => Some(RDP::Level0),
            204 => Some(RDP::Level2),
            _ => None,
        }
    }
    #[doc = "Level 1, memories readout protection active (writes 0x88)"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == RDP::Level1
    }
    #[doc = "Level 0, readout protection not active"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == RDP::Level0
    }
    #[doc = "Level 2, chip readout protection active"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == RDP::Level2
    }
}
#[doc = "Field `RDP` writer - Read protection level"]
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8, RDP>;
impl<'a, REG> RDP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Level 1, memories readout protection active (writes 0x88)"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut crate::W<REG> {
        self.variant(RDP::Level1)
    }
    #[doc = "Level 0, readout protection not active"]
    #[inline(always)]
    pub fn level0(self) -> &'a mut crate::W<REG> {
        self.variant(RDP::Level0)
    }
    #[doc = "Level 2, chip readout protection active"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut crate::W<REG> {
        self.variant(RDP::Level2)
    }
}
#[doc = "System security enabled flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESE {
    #[doc = "0: Security disabled"]
    Disabled = 0,
    #[doc = "1: Security enabled"]
    Enabled = 1,
}
impl From<ESE> for bool {
    #[inline(always)]
    fn from(variant: ESE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESE` reader - System security enabled flag"]
pub type ESE_R = crate::BitReader<ESE>;
impl ESE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ESE {
        match self.bits {
            false => ESE::Disabled,
            true => ESE::Enabled,
        }
    }
    #[doc = "Security disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ESE::Disabled
    }
    #[doc = "Security enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ESE::Enabled
    }
}
#[doc = "Field `ESE` writer - System security enabled flag"]
pub type ESE_W<'a, REG> = crate::BitWriter<'a, REG, ESE>;
impl<'a, REG> ESE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Security disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ESE::Disabled)
    }
    #[doc = "Security enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ESE::Enabled)
    }
}
#[doc = "BOR reset Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOR_LEV {
    #[doc = "0: BOR level 0. Reset level threshold is around 1.7 V"]
    Level0 = 0,
    #[doc = "1: BOR level 1. Reset level threshold is around 2.0 V"]
    Level1 = 1,
    #[doc = "2: BOR level 2. Reset level threshold is around 2.2 V"]
    Level2 = 2,
    #[doc = "3: BOR level 3. Reset level threshold is around 2.5 V"]
    Level3 = 3,
    #[doc = "4: BOR level 4. Reset level threshold is around 2.8 V"]
    Level4 = 4,
}
impl From<BOR_LEV> for u8 {
    #[inline(always)]
    fn from(variant: BOR_LEV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BOR_LEV {
    type Ux = u8;
}
#[doc = "Field `BOR_LEV` reader - BOR reset Level"]
pub type BOR_LEV_R = crate::FieldReader<BOR_LEV>;
impl BOR_LEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BOR_LEV> {
        match self.bits {
            0 => Some(BOR_LEV::Level0),
            1 => Some(BOR_LEV::Level1),
            2 => Some(BOR_LEV::Level2),
            3 => Some(BOR_LEV::Level3),
            4 => Some(BOR_LEV::Level4),
            _ => None,
        }
    }
    #[doc = "BOR level 0. Reset level threshold is around 1.7 V"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == BOR_LEV::Level0
    }
    #[doc = "BOR level 1. Reset level threshold is around 2.0 V"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == BOR_LEV::Level1
    }
    #[doc = "BOR level 2. Reset level threshold is around 2.2 V"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == BOR_LEV::Level2
    }
    #[doc = "BOR level 3. Reset level threshold is around 2.5 V"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == BOR_LEV::Level3
    }
    #[doc = "BOR level 4. Reset level threshold is around 2.8 V"]
    #[inline(always)]
    pub fn is_level4(&self) -> bool {
        *self == BOR_LEV::Level4
    }
}
#[doc = "Field `BOR_LEV` writer - BOR reset Level"]
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BOR_LEV>;
impl<'a, REG> BOR_LEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "BOR level 0. Reset level threshold is around 1.7 V"]
    #[inline(always)]
    pub fn level0(self) -> &'a mut crate::W<REG> {
        self.variant(BOR_LEV::Level0)
    }
    #[doc = "BOR level 1. Reset level threshold is around 2.0 V"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut crate::W<REG> {
        self.variant(BOR_LEV::Level1)
    }
    #[doc = "BOR level 2. Reset level threshold is around 2.2 V"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut crate::W<REG> {
        self.variant(BOR_LEV::Level2)
    }
    #[doc = "BOR level 3. Reset level threshold is around 2.5 V"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut crate::W<REG> {
        self.variant(BOR_LEV::Level3)
    }
    #[doc = "BOR level 4. Reset level threshold is around 2.8 V"]
    #[inline(always)]
    pub fn level4(self) -> &'a mut crate::W<REG> {
        self.variant(BOR_LEV::Level4)
    }
}
#[doc = "nRST_STOP\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_RST_STOP {
    #[doc = "0: Reset generated when entering the Standby mode"]
    Enabled = 0,
    #[doc = "1: No reset generated when entering the Standby mode"]
    Disabled = 1,
}
impl From<N_RST_STOP> for bool {
    #[inline(always)]
    fn from(variant: N_RST_STOP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `nRST_STOP` reader - nRST_STOP"]
pub type N_RST_STOP_R = crate::BitReader<N_RST_STOP>;
impl N_RST_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> N_RST_STOP {
        match self.bits {
            false => N_RST_STOP::Enabled,
            true => N_RST_STOP::Disabled,
        }
    }
    #[doc = "Reset generated when entering the Standby mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == N_RST_STOP::Enabled
    }
    #[doc = "No reset generated when entering the Standby mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == N_RST_STOP::Disabled
    }
}
#[doc = "Field `nRST_STOP` writer - nRST_STOP"]
pub type N_RST_STOP_W<'a, REG> = crate::BitWriter<'a, REG, N_RST_STOP>;
impl<'a, REG> N_RST_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset generated when entering the Standby mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(N_RST_STOP::Enabled)
    }
    #[doc = "No reset generated when entering the Standby mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(N_RST_STOP::Disabled)
    }
}
#[doc = "nRST_STDBY\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_RST_STDBY {
    #[doc = "0: Reset generated when entering the Standby mode"]
    Enabled = 0,
    #[doc = "1: No reset generated when entering the Standby mode"]
    Disabled = 1,
}
impl From<N_RST_STDBY> for bool {
    #[inline(always)]
    fn from(variant: N_RST_STDBY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub type N_RST_STDBY_R = crate::BitReader<N_RST_STDBY>;
impl N_RST_STDBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> N_RST_STDBY {
        match self.bits {
            false => N_RST_STDBY::Enabled,
            true => N_RST_STDBY::Disabled,
        }
    }
    #[doc = "Reset generated when entering the Standby mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == N_RST_STDBY::Enabled
    }
    #[doc = "No reset generated when entering the Standby mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == N_RST_STDBY::Disabled
    }
}
#[doc = "Field `nRST_STDBY` writer - nRST_STDBY"]
pub type N_RST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG, N_RST_STDBY>;
impl<'a, REG> N_RST_STDBY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset generated when entering the Standby mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(N_RST_STDBY::Enabled)
    }
    #[doc = "No reset generated when entering the Standby mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(N_RST_STDBY::Disabled)
    }
}
#[doc = "nRSTSHDW\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_RST_SHDW {
    #[doc = "0: Reset generated when entering the Shutdown mode"]
    Enabled = 0,
    #[doc = "1: No reset generated when entering the Shutdown mode"]
    Disabled = 1,
}
impl From<N_RST_SHDW> for bool {
    #[inline(always)]
    fn from(variant: N_RST_SHDW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `nRST_SHDW` reader - nRSTSHDW"]
pub type N_RST_SHDW_R = crate::BitReader<N_RST_SHDW>;
impl N_RST_SHDW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> N_RST_SHDW {
        match self.bits {
            false => N_RST_SHDW::Enabled,
            true => N_RST_SHDW::Disabled,
        }
    }
    #[doc = "Reset generated when entering the Shutdown mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == N_RST_SHDW::Enabled
    }
    #[doc = "No reset generated when entering the Shutdown mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == N_RST_SHDW::Disabled
    }
}
#[doc = "Field `nRST_SHDW` writer - nRSTSHDW"]
pub type N_RST_SHDW_W<'a, REG> = crate::BitWriter<'a, REG, N_RST_SHDW>;
impl<'a, REG> N_RST_SHDW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset generated when entering the Shutdown mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(N_RST_SHDW::Enabled)
    }
    #[doc = "No reset generated when entering the Shutdown mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(N_RST_SHDW::Disabled)
    }
}
#[doc = "Independent watchdog selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDG_SW {
    #[doc = "0: Hardware independent watchdog"]
    Hardware = 0,
    #[doc = "1: Software independent watchdog"]
    Software = 1,
}
impl From<IWDG_SW> for bool {
    #[inline(always)]
    fn from(variant: IWDG_SW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDG_SW` reader - Independent watchdog selection"]
pub type IWDG_SW_R = crate::BitReader<IWDG_SW>;
impl IWDG_SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IWDG_SW {
        match self.bits {
            false => IWDG_SW::Hardware,
            true => IWDG_SW::Software,
        }
    }
    #[doc = "Hardware independent watchdog"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == IWDG_SW::Hardware
    }
    #[doc = "Software independent watchdog"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == IWDG_SW::Software
    }
}
#[doc = "Field `IWDG_SW` writer - Independent watchdog selection"]
pub type IWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG, IWDG_SW>;
impl<'a, REG> IWDG_SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware independent watchdog"]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_SW::Hardware)
    }
    #[doc = "Software independent watchdog"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_SW::Software)
    }
}
#[doc = "Independent watchdog counter freeze in Stop mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDG_STOP {
    #[doc = "0: Independent watchdog counter frozen in Stop mode"]
    Frozen = 0,
    #[doc = "1: Independent watchdog counter running in Stop mode"]
    Running = 1,
}
impl From<IWDG_STOP> for bool {
    #[inline(always)]
    fn from(variant: IWDG_STOP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode"]
pub type IWDG_STOP_R = crate::BitReader<IWDG_STOP>;
impl IWDG_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IWDG_STOP {
        match self.bits {
            false => IWDG_STOP::Frozen,
            true => IWDG_STOP::Running,
        }
    }
    #[doc = "Independent watchdog counter frozen in Stop mode"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == IWDG_STOP::Frozen
    }
    #[doc = "Independent watchdog counter running in Stop mode"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == IWDG_STOP::Running
    }
}
#[doc = "Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode"]
pub type IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG, IWDG_STOP>;
impl<'a, REG> IWDG_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Independent watchdog counter frozen in Stop mode"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_STOP::Frozen)
    }
    #[doc = "Independent watchdog counter running in Stop mode"]
    #[inline(always)]
    pub fn running(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_STOP::Running)
    }
}
#[doc = "Independent watchdog counter freeze in Standby mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDG_STDBY {
    #[doc = "0: Independent watchdog counter frozen in Standby mode"]
    Frozen = 0,
    #[doc = "1: Independent watchdog counter running in Standby mode"]
    Running = 1,
}
impl From<IWDG_STDBY> for bool {
    #[inline(always)]
    fn from(variant: IWDG_STDBY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDG_STDBY` reader - Independent watchdog counter freeze in Standby mode"]
pub type IWDG_STDBY_R = crate::BitReader<IWDG_STDBY>;
impl IWDG_STDBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IWDG_STDBY {
        match self.bits {
            false => IWDG_STDBY::Frozen,
            true => IWDG_STDBY::Running,
        }
    }
    #[doc = "Independent watchdog counter frozen in Standby mode"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == IWDG_STDBY::Frozen
    }
    #[doc = "Independent watchdog counter running in Standby mode"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == IWDG_STDBY::Running
    }
}
#[doc = "Field `IWDG_STDBY` writer - Independent watchdog counter freeze in Standby mode"]
pub type IWDG_STDBY_W<'a, REG> = crate::BitWriter<'a, REG, IWDG_STDBY>;
impl<'a, REG> IWDG_STDBY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Independent watchdog counter frozen in Standby mode"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_STDBY::Frozen)
    }
    #[doc = "Independent watchdog counter running in Standby mode"]
    #[inline(always)]
    pub fn running(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_STDBY::Running)
    }
}
#[doc = "Window watchdog selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDG_SW {
    #[doc = "0: Hardware window watchdog"]
    Hardware = 0,
    #[doc = "1: Software window watchdog"]
    Software = 1,
}
impl From<WWDG_SW> for bool {
    #[inline(always)]
    fn from(variant: WWDG_SW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDG_SW` reader - Window watchdog selection"]
pub type WWDG_SW_R = crate::BitReader<WWDG_SW>;
impl WWDG_SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WWDG_SW {
        match self.bits {
            false => WWDG_SW::Hardware,
            true => WWDG_SW::Software,
        }
    }
    #[doc = "Hardware window watchdog"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == WWDG_SW::Hardware
    }
    #[doc = "Software window watchdog"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == WWDG_SW::Software
    }
}
#[doc = "Field `WWDG_SW` writer - Window watchdog selection"]
pub type WWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG, WWDG_SW>;
impl<'a, REG> WWDG_SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware window watchdog"]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut crate::W<REG> {
        self.variant(WWDG_SW::Hardware)
    }
    #[doc = "Software window watchdog"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(WWDG_SW::Software)
    }
}
#[doc = "Boot configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_BOOT1 {
    #[doc = "0: When nSWBOOT0 is cleared, select boot mode together with nBOOT0"]
    Clear = 0,
    #[doc = "1: When nSWBOOT0 is cleared, select boot mode together with nBOOT0"]
    Set = 1,
}
impl From<N_BOOT1> for bool {
    #[inline(always)]
    fn from(variant: N_BOOT1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `nBOOT1` reader - Boot configuration"]
pub type N_BOOT1_R = crate::BitReader<N_BOOT1>;
impl N_BOOT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> N_BOOT1 {
        match self.bits {
            false => N_BOOT1::Clear,
            true => N_BOOT1::Set,
        }
    }
    #[doc = "When nSWBOOT0 is cleared, select boot mode together with nBOOT0"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == N_BOOT1::Clear
    }
    #[doc = "When nSWBOOT0 is cleared, select boot mode together with nBOOT0"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == N_BOOT1::Set
    }
}
#[doc = "Field `nBOOT1` writer - Boot configuration"]
pub type N_BOOT1_W<'a, REG> = crate::BitWriter<'a, REG, N_BOOT1>;
impl<'a, REG> N_BOOT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When nSWBOOT0 is cleared, select boot mode together with nBOOT0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(N_BOOT1::Clear)
    }
    #[doc = "When nSWBOOT0 is cleared, select boot mode together with nBOOT0"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(N_BOOT1::Set)
    }
}
#[doc = "SRAM2 parity check enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2_PE {
    #[doc = "0: SRAM2 Parity check enabled"]
    Enabled = 0,
    #[doc = "1: SRAM2 Parity check disabled"]
    Disabled = 1,
}
impl From<SRAM2_PE> for bool {
    #[inline(always)]
    fn from(variant: SRAM2_PE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM2_PE` reader - SRAM2 parity check enable"]
pub type SRAM2_PE_R = crate::BitReader<SRAM2_PE>;
impl SRAM2_PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM2_PE {
        match self.bits {
            false => SRAM2_PE::Enabled,
            true => SRAM2_PE::Disabled,
        }
    }
    #[doc = "SRAM2 Parity check enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM2_PE::Enabled
    }
    #[doc = "SRAM2 Parity check disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM2_PE::Disabled
    }
}
#[doc = "Field `SRAM2_PE` writer - SRAM2 parity check enable"]
pub type SRAM2_PE_W<'a, REG> = crate::BitWriter<'a, REG, SRAM2_PE>;
impl<'a, REG> SRAM2_PE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM2 Parity check enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2_PE::Enabled)
    }
    #[doc = "SRAM2 Parity check disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2_PE::Disabled)
    }
}
#[doc = "SRAM2 Erase when system reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_RST {
    #[doc = "0: SRAM1 and SRAM2 erased when a system reset occurs"]
    Reset = 0,
    #[doc = "1: SRAM1 and SRAM2 not erased when a system reset occurs"]
    NotReset = 1,
}
impl From<SRAM_RST> for bool {
    #[inline(always)]
    fn from(variant: SRAM_RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_RST` reader - SRAM2 Erase when system reset"]
pub type SRAM_RST_R = crate::BitReader<SRAM_RST>;
impl SRAM_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM_RST {
        match self.bits {
            false => SRAM_RST::Reset,
            true => SRAM_RST::NotReset,
        }
    }
    #[doc = "SRAM1 and SRAM2 erased when a system reset occurs"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SRAM_RST::Reset
    }
    #[doc = "SRAM1 and SRAM2 not erased when a system reset occurs"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == SRAM_RST::NotReset
    }
}
#[doc = "Field `SRAM_RST` writer - SRAM2 Erase when system reset"]
pub type SRAM_RST_W<'a, REG> = crate::BitWriter<'a, REG, SRAM_RST>;
impl<'a, REG> SRAM_RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM1 and SRAM2 erased when a system reset occurs"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM_RST::Reset)
    }
    #[doc = "SRAM1 and SRAM2 not erased when a system reset occurs"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM_RST::NotReset)
    }
}
#[doc = "Software BOOT0 selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_SWBOOT0 {
    #[doc = "0: BOOT0 taken from nBOOT0 in this register"]
    Bit = 0,
    #[doc = "1: BOOT0 taken from GPIO PH3/BOOT0"]
    Pin = 1,
}
impl From<N_SWBOOT0> for bool {
    #[inline(always)]
    fn from(variant: N_SWBOOT0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `nSWBOOT0` reader - Software BOOT0 selection"]
pub type N_SWBOOT0_R = crate::BitReader<N_SWBOOT0>;
impl N_SWBOOT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> N_SWBOOT0 {
        match self.bits {
            false => N_SWBOOT0::Bit,
            true => N_SWBOOT0::Pin,
        }
    }
    #[doc = "BOOT0 taken from nBOOT0 in this register"]
    #[inline(always)]
    pub fn is_bit(&self) -> bool {
        *self == N_SWBOOT0::Bit
    }
    #[doc = "BOOT0 taken from GPIO PH3/BOOT0"]
    #[inline(always)]
    pub fn is_pin(&self) -> bool {
        *self == N_SWBOOT0::Pin
    }
}
#[doc = "Field `nSWBOOT0` writer - Software BOOT0 selection"]
pub type N_SWBOOT0_W<'a, REG> = crate::BitWriter<'a, REG, N_SWBOOT0>;
impl<'a, REG> N_SWBOOT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BOOT0 taken from nBOOT0 in this register"]
    #[inline(always)]
    pub fn bit_(self) -> &'a mut crate::W<REG> {
        self.variant(N_SWBOOT0::Bit)
    }
    #[doc = "BOOT0 taken from GPIO PH3/BOOT0"]
    #[inline(always)]
    pub fn pin(self) -> &'a mut crate::W<REG> {
        self.variant(N_SWBOOT0::Pin)
    }
}
#[doc = "nBOOT0 option bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_BOOT0 {
    #[doc = "0: When nSWBOOT0 is cleared, select boot mode together with nBOOT1"]
    Clear = 0,
    #[doc = "1: When nSWBOOT0 is cleared, select boot mode together with nBOOT1"]
    Set = 1,
}
impl From<N_BOOT0> for bool {
    #[inline(always)]
    fn from(variant: N_BOOT0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `nBOOT0` reader - nBOOT0 option bit"]
pub type N_BOOT0_R = crate::BitReader<N_BOOT0>;
impl N_BOOT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> N_BOOT0 {
        match self.bits {
            false => N_BOOT0::Clear,
            true => N_BOOT0::Set,
        }
    }
    #[doc = "When nSWBOOT0 is cleared, select boot mode together with nBOOT1"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == N_BOOT0::Clear
    }
    #[doc = "When nSWBOOT0 is cleared, select boot mode together with nBOOT1"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == N_BOOT0::Set
    }
}
#[doc = "Field `nBOOT0` writer - nBOOT0 option bit"]
pub type N_BOOT0_W<'a, REG> = crate::BitWriter<'a, REG, N_BOOT0>;
impl<'a, REG> N_BOOT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When nSWBOOT0 is cleared, select boot mode together with nBOOT1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(N_BOOT0::Clear)
    }
    #[doc = "When nSWBOOT0 is cleared, select boot mode together with nBOOT1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(N_BOOT0::Set)
    }
}
#[doc = "CPU1 CM4 Unique Boot entry enable option bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOT_LOCK {
    #[doc = "0: Boot lock is disabled"]
    Disabled = 0,
    #[doc = "1: Boot lock is enabled"]
    Enabled = 1,
}
impl From<BOOT_LOCK> for bool {
    #[inline(always)]
    fn from(variant: BOOT_LOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOT_LOCK` reader - CPU1 CM4 Unique Boot entry enable option bit"]
pub type BOOT_LOCK_R = crate::BitReader<BOOT_LOCK>;
impl BOOT_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BOOT_LOCK {
        match self.bits {
            false => BOOT_LOCK::Disabled,
            true => BOOT_LOCK::Enabled,
        }
    }
    #[doc = "Boot lock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOOT_LOCK::Disabled
    }
    #[doc = "Boot lock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOOT_LOCK::Enabled
    }
}
#[doc = "Field `BOOT_LOCK` writer - CPU1 CM4 Unique Boot entry enable option bit"]
pub type BOOT_LOCK_W<'a, REG> = crate::BitWriter<'a, REG, BOOT_LOCK>;
impl<'a, REG> BOOT_LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Boot lock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_LOCK::Disabled)
    }
    #[doc = "Boot lock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_LOCK::Enabled)
    }
}
#[doc = "Field `C2BOOT_LOCK` reader - CPU2 CM0+ Unique Boot entry enable option bit"]
pub type C2BOOT_LOCK_R = crate::BitReader;
#[doc = "Field `C2BOOT_LOCK` writer - CPU2 CM0+ Unique Boot entry enable option bit"]
pub type C2BOOT_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Read protection level"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - System security enabled flag"]
    #[inline(always)]
    pub fn ese(&self) -> ESE_R {
        ESE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - BOR reset Level"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - nRSTSHDW"]
    #[inline(always)]
    pub fn n_rst_shdw(&self) -> N_RST_SHDW_R {
        N_RST_SHDW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Window watchdog selection"]
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - Boot configuration"]
    #[inline(always)]
    pub fn n_boot1(&self) -> N_BOOT1_R {
        N_BOOT1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SRAM2 parity check enable"]
    #[inline(always)]
    pub fn sram2_pe(&self) -> SRAM2_PE_R {
        SRAM2_PE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SRAM2 Erase when system reset"]
    #[inline(always)]
    pub fn sram_rst(&self) -> SRAM_RST_R {
        SRAM_RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Software BOOT0 selection"]
    #[inline(always)]
    pub fn n_swboot0(&self) -> N_SWBOOT0_R {
        N_SWBOOT0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - nBOOT0 option bit"]
    #[inline(always)]
    pub fn n_boot0(&self) -> N_BOOT0_R {
        N_BOOT0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - CPU1 CM4 Unique Boot entry enable option bit"]
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU2 CM0+ Unique Boot entry enable option bit"]
    #[inline(always)]
    pub fn c2boot_lock(&self) -> C2BOOT_LOCK_R {
        C2BOOT_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read protection level"]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<OPTRrs> {
        RDP_W::new(self, 0)
    }
    #[doc = "Bit 8 - System security enabled flag"]
    #[inline(always)]
    #[must_use]
    pub fn ese(&mut self) -> ESE_W<OPTRrs> {
        ESE_W::new(self, 8)
    }
    #[doc = "Bits 9:11 - BOR reset Level"]
    #[inline(always)]
    #[must_use]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<OPTRrs> {
        BOR_LEV_W::new(self, 9)
    }
    #[doc = "Bit 12 - nRST_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<OPTRrs> {
        N_RST_STOP_W::new(self, 12)
    }
    #[doc = "Bit 13 - nRST_STDBY"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<OPTRrs> {
        N_RST_STDBY_W::new(self, 13)
    }
    #[doc = "Bit 14 - nRSTSHDW"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_shdw(&mut self) -> N_RST_SHDW_W<OPTRrs> {
        N_RST_SHDW_W::new(self, 14)
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<OPTRrs> {
        IWDG_SW_W::new(self, 16)
    }
    #[doc = "Bit 17 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<OPTRrs> {
        IWDG_STOP_W::new(self, 17)
    }
    #[doc = "Bit 18 - Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W<OPTRrs> {
        IWDG_STDBY_W::new(self, 18)
    }
    #[doc = "Bit 19 - Window watchdog selection"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<OPTRrs> {
        WWDG_SW_W::new(self, 19)
    }
    #[doc = "Bit 23 - Boot configuration"]
    #[inline(always)]
    #[must_use]
    pub fn n_boot1(&mut self) -> N_BOOT1_W<OPTRrs> {
        N_BOOT1_W::new(self, 23)
    }
    #[doc = "Bit 24 - SRAM2 parity check enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram2_pe(&mut self) -> SRAM2_PE_W<OPTRrs> {
        SRAM2_PE_W::new(self, 24)
    }
    #[doc = "Bit 25 - SRAM2 Erase when system reset"]
    #[inline(always)]
    #[must_use]
    pub fn sram_rst(&mut self) -> SRAM_RST_W<OPTRrs> {
        SRAM_RST_W::new(self, 25)
    }
    #[doc = "Bit 26 - Software BOOT0 selection"]
    #[inline(always)]
    #[must_use]
    pub fn n_swboot0(&mut self) -> N_SWBOOT0_W<OPTRrs> {
        N_SWBOOT0_W::new(self, 26)
    }
    #[doc = "Bit 27 - nBOOT0 option bit"]
    #[inline(always)]
    #[must_use]
    pub fn n_boot0(&mut self) -> N_BOOT0_W<OPTRrs> {
        N_BOOT0_W::new(self, 27)
    }
    #[doc = "Bit 30 - CPU1 CM4 Unique Boot entry enable option bit"]
    #[inline(always)]
    #[must_use]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W<OPTRrs> {
        BOOT_LOCK_W::new(self, 30)
    }
    #[doc = "Bit 31 - CPU2 CM0+ Unique Boot entry enable option bit"]
    #[inline(always)]
    #[must_use]
    pub fn c2boot_lock(&mut self) -> C2BOOT_LOCK_W<OPTRrs> {
        C2BOOT_LOCK_W::new(self, 31)
    }
}
#[doc = "Flash option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTRrs;
impl crate::RegisterSpec for OPTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optr::R`](R) reader structure"]
impl crate::Readable for OPTRrs {}
#[doc = "`write(|w| ..)` method takes [`optr::W`](W) writer structure"]
impl crate::Writable for OPTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTR to value 0x3fff_f0aa"]
impl crate::Resettable for OPTRrs {
    const RESET_VALUE: u32 = 0x3fff_f0aa;
}
