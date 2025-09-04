///Register `BDTR` reader
pub type R = crate::R<BDTRrs>;
///Register `BDTR` writer
pub type W = crate::W<BDTRrs>;
///Field `DTG` reader - Dead-time generator setup
pub type DTG_R = crate::FieldReader;
///Field `DTG` writer - Dead-time generator setup
pub type DTG_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
/**Lock configuration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK {
    ///0: No bit is write protected
    Off = 0,
    ///1: Any bits except MOE, OSSR, OSSI and LOCK in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register can no longer be written
    Level1 = 1,
    ///2: LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written
    Level2 = 2,
    ///3: LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written
    Level3 = 3,
}
impl From<LOCK> for u8 {
    #[inline(always)]
    fn from(variant: LOCK) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LOCK {
    type Ux = u8;
}
impl crate::IsEnum for LOCK {}
///Field `LOCK` reader - Lock configuration
pub type LOCK_R = crate::FieldReader<LOCK>;
impl LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LOCK {
        match self.bits {
            0 => LOCK::Off,
            1 => LOCK::Level1,
            2 => LOCK::Level2,
            3 => LOCK::Level3,
            _ => unreachable!(),
        }
    }
    ///No bit is write protected
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LOCK::Off
    }
    ///Any bits except MOE, OSSR, OSSI and LOCK in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register can no longer be written
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == LOCK::Level1
    }
    ///LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == LOCK::Level2
    }
    ///LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == LOCK::Level3
    }
}
///Field `LOCK` writer - Lock configuration
pub type LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LOCK, crate::Safe>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No bit is write protected
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Off)
    }
    ///Any bits except MOE, OSSR, OSSI and LOCK in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register can no longer be written
    #[inline(always)]
    pub fn level1(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Level1)
    }
    ///LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written
    #[inline(always)]
    pub fn level2(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Level2)
    }
    ///LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written
    #[inline(always)]
    pub fn level3(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Level3)
    }
}
/**Off-state selection for Idle mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSI {
    ///0: When inactive, OC/OCN outputs are disabled
    HiZ = 0,
    ///1: When inactive, OC/OCN outputs are forced to idle level
    IdleLevel = 1,
}
impl From<OSSI> for bool {
    #[inline(always)]
    fn from(variant: OSSI) -> Self {
        variant as u8 != 0
    }
}
///Field `OSSI` reader - Off-state selection for Idle mode
pub type OSSI_R = crate::BitReader<OSSI>;
impl OSSI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSSI {
        match self.bits {
            false => OSSI::HiZ,
            true => OSSI::IdleLevel,
        }
    }
    ///When inactive, OC/OCN outputs are disabled
    #[inline(always)]
    pub fn is_hi_z(&self) -> bool {
        *self == OSSI::HiZ
    }
    ///When inactive, OC/OCN outputs are forced to idle level
    #[inline(always)]
    pub fn is_idle_level(&self) -> bool {
        *self == OSSI::IdleLevel
    }
}
///Field `OSSI` writer - Off-state selection for Idle mode
pub type OSSI_W<'a, REG> = crate::BitWriter<'a, REG, OSSI>;
impl<'a, REG> OSSI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When inactive, OC/OCN outputs are disabled
    #[inline(always)]
    pub fn hi_z(self) -> &'a mut crate::W<REG> {
        self.variant(OSSI::HiZ)
    }
    ///When inactive, OC/OCN outputs are forced to idle level
    #[inline(always)]
    pub fn idle_level(self) -> &'a mut crate::W<REG> {
        self.variant(OSSI::IdleLevel)
    }
}
/**Off-state selection for Run mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSR {
    ///0: When inactive, OC/OCN outputs are disabled
    HiZ = 0,
    ///1: When inactive, OC/OCN outputs are enabled with their inactive level
    IdleLevel = 1,
}
impl From<OSSR> for bool {
    #[inline(always)]
    fn from(variant: OSSR) -> Self {
        variant as u8 != 0
    }
}
///Field `OSSR` reader - Off-state selection for Run mode
pub type OSSR_R = crate::BitReader<OSSR>;
impl OSSR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSSR {
        match self.bits {
            false => OSSR::HiZ,
            true => OSSR::IdleLevel,
        }
    }
    ///When inactive, OC/OCN outputs are disabled
    #[inline(always)]
    pub fn is_hi_z(&self) -> bool {
        *self == OSSR::HiZ
    }
    ///When inactive, OC/OCN outputs are enabled with their inactive level
    #[inline(always)]
    pub fn is_idle_level(&self) -> bool {
        *self == OSSR::IdleLevel
    }
}
///Field `OSSR` writer - Off-state selection for Run mode
pub type OSSR_W<'a, REG> = crate::BitWriter<'a, REG, OSSR>;
impl<'a, REG> OSSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When inactive, OC/OCN outputs are disabled
    #[inline(always)]
    pub fn hi_z(self) -> &'a mut crate::W<REG> {
        self.variant(OSSR::HiZ)
    }
    ///When inactive, OC/OCN outputs are enabled with their inactive level
    #[inline(always)]
    pub fn idle_level(self) -> &'a mut crate::W<REG> {
        self.variant(OSSR::IdleLevel)
    }
}
/**Break enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKE {
    ///0: Break function x disabled
    Disabled = 0,
    ///1: Break function x enabled
    Enabled = 1,
}
impl From<BKE> for bool {
    #[inline(always)]
    fn from(variant: BKE) -> Self {
        variant as u8 != 0
    }
}
///Field `BKE` reader - Break enable
pub type BKE_R = crate::BitReader<BKE>;
impl BKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKE {
        match self.bits {
            false => BKE::Disabled,
            true => BKE::Enabled,
        }
    }
    ///Break function x disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BKE::Disabled
    }
    ///Break function x enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BKE::Enabled
    }
}
///Field `BKE` writer - Break enable
pub type BKE_W<'a, REG> = crate::BitWriter<'a, REG, BKE>;
impl<'a, REG> BKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break function x disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKE::Disabled)
    }
    ///Break function x enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKE::Enabled)
    }
}
/**Break polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKP {
    ///0: Break input BRKx is active low
    ActiveLow = 0,
    ///1: Break input BRKx is active high
    ActiveHigh = 1,
}
impl From<BKP> for bool {
    #[inline(always)]
    fn from(variant: BKP) -> Self {
        variant as u8 != 0
    }
}
///Field `BKP` reader - Break polarity
pub type BKP_R = crate::BitReader<BKP>;
impl BKP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKP {
        match self.bits {
            false => BKP::ActiveLow,
            true => BKP::ActiveHigh,
        }
    }
    ///Break input BRKx is active low
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == BKP::ActiveLow
    }
    ///Break input BRKx is active high
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == BKP::ActiveHigh
    }
}
///Field `BKP` writer - Break polarity
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG, BKP>;
impl<'a, REG> BKP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break input BRKx is active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(BKP::ActiveLow)
    }
    ///Break input BRKx is active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(BKP::ActiveHigh)
    }
}
/**Automatic output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AOE {
    ///0: MOE can be set only by software
    Manual = 0,
    ///1: MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)
    Automatic = 1,
}
impl From<AOE> for bool {
    #[inline(always)]
    fn from(variant: AOE) -> Self {
        variant as u8 != 0
    }
}
///Field `AOE` reader - Automatic output enable
pub type AOE_R = crate::BitReader<AOE>;
impl AOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AOE {
        match self.bits {
            false => AOE::Manual,
            true => AOE::Automatic,
        }
    }
    ///MOE can be set only by software
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == AOE::Manual
    }
    ///MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == AOE::Automatic
    }
}
///Field `AOE` writer - Automatic output enable
pub type AOE_W<'a, REG> = crate::BitWriter<'a, REG, AOE>;
impl<'a, REG> AOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MOE can be set only by software
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(AOE::Manual)
    }
    ///MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(AOE::Automatic)
    }
}
/**Main output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOE {
    ///0: OC/OCN are disabled or forced idle depending on OSSI
    DisabledIdle = 0,
    ///1: OC/OCN are enabled if CCxE/CCxNE are set
    Enabled = 1,
}
impl From<MOE> for bool {
    #[inline(always)]
    fn from(variant: MOE) -> Self {
        variant as u8 != 0
    }
}
///Field `MOE` reader - Main output enable
pub type MOE_R = crate::BitReader<MOE>;
impl MOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MOE {
        match self.bits {
            false => MOE::DisabledIdle,
            true => MOE::Enabled,
        }
    }
    ///OC/OCN are disabled or forced idle depending on OSSI
    #[inline(always)]
    pub fn is_disabled_idle(&self) -> bool {
        *self == MOE::DisabledIdle
    }
    ///OC/OCN are enabled if CCxE/CCxNE are set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MOE::Enabled
    }
}
///Field `MOE` writer - Main output enable
pub type MOE_W<'a, REG> = crate::BitWriter<'a, REG, MOE>;
impl<'a, REG> MOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OC/OCN are disabled or forced idle depending on OSSI
    #[inline(always)]
    pub fn disabled_idle(self) -> &'a mut crate::W<REG> {
        self.variant(MOE::DisabledIdle)
    }
    ///OC/OCN are enabled if CCxE/CCxNE are set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MOE::Enabled)
    }
}
/**Break filter

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BKF {
    ///0: No filter, sampling is done at fDTS
    NoFilter = 0,
    ///1: fSAMPLING=fCK_INT, N=2
    FckIntN2 = 1,
    ///2: fSAMPLING=fCK_INT, N=4
    FckIntN4 = 2,
    ///3: fSAMPLING=fCK_INT, N=8
    FckIntN8 = 3,
    ///4: fSAMPLING=fDTS/2, N=6
    FdtsDiv2N6 = 4,
    ///5: fSAMPLING=fDTS/2, N=8
    FdtsDiv2N8 = 5,
    ///6: fSAMPLING=fDTS/4, N=6
    FdtsDiv4N6 = 6,
    ///7: fSAMPLING=fDTS/4, N=8
    FdtsDiv4N8 = 7,
    ///8: fSAMPLING=fDTS/8, N=6
    FdtsDiv8N6 = 8,
    ///9: fSAMPLING=fDTS/8, N=8
    FdtsDiv8N8 = 9,
    ///10: fSAMPLING=fDTS/16, N=5
    FdtsDiv16N5 = 10,
    ///11: fSAMPLING=fDTS/16, N=6
    FdtsDiv16N6 = 11,
    ///12: fSAMPLING=fDTS/16, N=8
    FdtsDiv16N8 = 12,
    ///13: fSAMPLING=fDTS/32, N=5
    FdtsDiv32N5 = 13,
    ///14: fSAMPLING=fDTS/32, N=6
    FdtsDiv32N6 = 14,
    ///15: fSAMPLING=fDTS/32, N=8
    FdtsDiv32N8 = 15,
}
impl From<BKF> for u8 {
    #[inline(always)]
    fn from(variant: BKF) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BKF {
    type Ux = u8;
}
impl crate::IsEnum for BKF {}
///Field `BKF` reader - Break filter
pub type BKF_R = crate::FieldReader<BKF>;
impl BKF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKF {
        match self.bits {
            0 => BKF::NoFilter,
            1 => BKF::FckIntN2,
            2 => BKF::FckIntN4,
            3 => BKF::FckIntN8,
            4 => BKF::FdtsDiv2N6,
            5 => BKF::FdtsDiv2N8,
            6 => BKF::FdtsDiv4N6,
            7 => BKF::FdtsDiv4N8,
            8 => BKF::FdtsDiv8N6,
            9 => BKF::FdtsDiv8N8,
            10 => BKF::FdtsDiv16N5,
            11 => BKF::FdtsDiv16N6,
            12 => BKF::FdtsDiv16N8,
            13 => BKF::FdtsDiv32N5,
            14 => BKF::FdtsDiv32N6,
            15 => BKF::FdtsDiv32N8,
            _ => unreachable!(),
        }
    }
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == BKF::NoFilter
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == BKF::FckIntN2
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == BKF::FckIntN4
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == BKF::FckIntN8
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == BKF::FdtsDiv2N6
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == BKF::FdtsDiv2N8
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == BKF::FdtsDiv4N6
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == BKF::FdtsDiv4N8
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == BKF::FdtsDiv8N6
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == BKF::FdtsDiv8N8
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == BKF::FdtsDiv16N5
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == BKF::FdtsDiv16N6
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == BKF::FdtsDiv16N8
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == BKF::FdtsDiv32N5
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == BKF::FdtsDiv32N6
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == BKF::FdtsDiv32N8
    }
}
///Field `BKF` writer - Break filter
pub type BKF_W<'a, REG> = crate::FieldWriter<'a, REG, 4, BKF, crate::Safe>;
impl<'a, REG> BKF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::NoFilter)
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FckIntN2)
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FckIntN4)
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FckIntN8)
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv2N6)
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv2N8)
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv4N6)
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv4N8)
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv8N6)
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv8N8)
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv16N5)
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv16N6)
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv16N8)
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv32N5)
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv32N6)
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv32N8)
    }
}
///Field `BK2E` reader - Break 2 enable
pub use BKE_R as BK2E_R;
///Field `BK2E` writer - Break 2 enable
pub use BKE_W as BK2E_W;
///Field `BK2F` reader - Break 2 filter
pub use BKF_R as BK2F_R;
///Field `BK2F` writer - Break 2 filter
pub use BKF_W as BK2F_W;
///Field `BK2P` reader - Break 2 polarity
pub use BKP_R as BK2P_R;
///Field `BK2P` writer - Break 2 polarity
pub use BKP_W as BK2P_W;
/**BKDSRM

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKDSRM {
    ///0: Break input BRK is armed
    Armed = 0,
    ///1: Break input BRK is disarmed
    Disarmed = 1,
}
impl From<BKDSRM> for bool {
    #[inline(always)]
    fn from(variant: BKDSRM) -> Self {
        variant as u8 != 0
    }
}
///Field `BKDSRM` reader - BKDSRM
pub type BKDSRM_R = crate::BitReader<BKDSRM>;
impl BKDSRM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKDSRM {
        match self.bits {
            false => BKDSRM::Armed,
            true => BKDSRM::Disarmed,
        }
    }
    ///Break input BRK is armed
    #[inline(always)]
    pub fn is_armed(&self) -> bool {
        *self == BKDSRM::Armed
    }
    ///Break input BRK is disarmed
    #[inline(always)]
    pub fn is_disarmed(&self) -> bool {
        *self == BKDSRM::Disarmed
    }
}
///Field `BKDSRM` writer - BKDSRM
pub type BKDSRM_W<'a, REG> = crate::BitWriter<'a, REG, BKDSRM>;
impl<'a, REG> BKDSRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break input BRK is armed
    #[inline(always)]
    pub fn armed(self) -> &'a mut crate::W<REG> {
        self.variant(BKDSRM::Armed)
    }
    ///Break input BRK is disarmed
    #[inline(always)]
    pub fn disarmed(self) -> &'a mut crate::W<REG> {
        self.variant(BKDSRM::Disarmed)
    }
}
/**Break2 Disarm

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2DSRM {
    ///0: Break input BRK2 is armed
    Armed = 0,
    ///1: Break input BRK2 is disarmed
    Disarmed = 1,
}
impl From<BK2DSRM> for bool {
    #[inline(always)]
    fn from(variant: BK2DSRM) -> Self {
        variant as u8 != 0
    }
}
///Field `BK2DSRM` reader - Break2 Disarm
pub type BK2DSRM_R = crate::BitReader<BK2DSRM>;
impl BK2DSRM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2DSRM {
        match self.bits {
            false => BK2DSRM::Armed,
            true => BK2DSRM::Disarmed,
        }
    }
    ///Break input BRK2 is armed
    #[inline(always)]
    pub fn is_armed(&self) -> bool {
        *self == BK2DSRM::Armed
    }
    ///Break input BRK2 is disarmed
    #[inline(always)]
    pub fn is_disarmed(&self) -> bool {
        *self == BK2DSRM::Disarmed
    }
}
///Field `BK2DSRM` writer - Break2 Disarm
pub type BK2DSRM_W<'a, REG> = crate::BitWriter<'a, REG, BK2DSRM>;
impl<'a, REG> BK2DSRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break input BRK2 is armed
    #[inline(always)]
    pub fn armed(self) -> &'a mut crate::W<REG> {
        self.variant(BK2DSRM::Armed)
    }
    ///Break input BRK2 is disarmed
    #[inline(always)]
    pub fn disarmed(self) -> &'a mut crate::W<REG> {
        self.variant(BK2DSRM::Disarmed)
    }
}
/**BKBID

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKBID {
    ///0: Break input BRK in input mode
    Input = 0,
    ///1: Break input BRK in bidirectional mode
    Bidirectional = 1,
}
impl From<BKBID> for bool {
    #[inline(always)]
    fn from(variant: BKBID) -> Self {
        variant as u8 != 0
    }
}
///Field `BKBID` reader - BKBID
pub type BKBID_R = crate::BitReader<BKBID>;
impl BKBID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKBID {
        match self.bits {
            false => BKBID::Input,
            true => BKBID::Bidirectional,
        }
    }
    ///Break input BRK in input mode
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == BKBID::Input
    }
    ///Break input BRK in bidirectional mode
    #[inline(always)]
    pub fn is_bidirectional(&self) -> bool {
        *self == BKBID::Bidirectional
    }
}
///Field `BKBID` writer - BKBID
pub type BKBID_W<'a, REG> = crate::BitWriter<'a, REG, BKBID>;
impl<'a, REG> BKBID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break input BRK in input mode
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(BKBID::Input)
    }
    ///Break input BRK in bidirectional mode
    #[inline(always)]
    pub fn bidirectional(self) -> &'a mut crate::W<REG> {
        self.variant(BKBID::Bidirectional)
    }
}
/**Break2 bidirectional

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2BID {
    ///0: Break input BRK2 in input mode
    Input = 0,
    ///1: Break input BRK2 in bidirectional mode
    Bidirectional = 1,
}
impl From<BK2BID> for bool {
    #[inline(always)]
    fn from(variant: BK2BID) -> Self {
        variant as u8 != 0
    }
}
///Field `BK2BID` reader - Break2 bidirectional
pub type BK2BID_R = crate::BitReader<BK2BID>;
impl BK2BID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2BID {
        match self.bits {
            false => BK2BID::Input,
            true => BK2BID::Bidirectional,
        }
    }
    ///Break input BRK2 in input mode
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == BK2BID::Input
    }
    ///Break input BRK2 in bidirectional mode
    #[inline(always)]
    pub fn is_bidirectional(&self) -> bool {
        *self == BK2BID::Bidirectional
    }
}
///Field `BK2BID` writer - Break2 bidirectional
pub type BK2BID_W<'a, REG> = crate::BitWriter<'a, REG, BK2BID>;
impl<'a, REG> BK2BID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break input BRK2 in input mode
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(BK2BID::Input)
    }
    ///Break input BRK2 in bidirectional mode
    #[inline(always)]
    pub fn bidirectional(self) -> &'a mut crate::W<REG> {
        self.variant(BK2BID::Bidirectional)
    }
}
impl R {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:9 - Lock configuration
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Off-state selection for Idle mode
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Off-state selection for Run mode
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Break enable
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Break polarity
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Automatic output enable
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Main output enable
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:19 - Break filter
    #[inline(always)]
    pub fn bkf(&self) -> BKF_R {
        BKF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Break 2 filter
    #[inline(always)]
    pub fn bk2f(&self) -> BK2F_R {
        BK2F_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bit 24 - Break 2 enable
    #[inline(always)]
    pub fn bk2e(&self) -> BK2E_R {
        BK2E_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Break 2 polarity
    #[inline(always)]
    pub fn bk2p(&self) -> BK2P_R {
        BK2P_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - BKDSRM
    #[inline(always)]
    pub fn bkdsrm(&self) -> BKDSRM_R {
        BKDSRM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Break2 Disarm
    #[inline(always)]
    pub fn bk2dsrm(&self) -> BK2DSRM_R {
        BK2DSRM_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - BKBID
    #[inline(always)]
    pub fn bkbid(&self) -> BKBID_R {
        BKBID_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Break2 bidirectional
    #[inline(always)]
    pub fn bk2bid(&self) -> BK2BID_R {
        BK2BID_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDTR")
            .field("bk2bid", &self.bk2bid())
            .field("bkbid", &self.bkbid())
            .field("bk2dsrm", &self.bk2dsrm())
            .field("bkdsrm", &self.bkdsrm())
            .field("bkp", &self.bkp())
            .field("bk2p", &self.bk2p())
            .field("bke", &self.bke())
            .field("bk2e", &self.bk2e())
            .field("bkf", &self.bkf())
            .field("bk2f", &self.bk2f())
            .field("moe", &self.moe())
            .field("aoe", &self.aoe())
            .field("ossr", &self.ossr())
            .field("ossi", &self.ossi())
            .field("lock", &self.lock())
            .field("dtg", &self.dtg())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtg(&mut self) -> DTG_W<BDTRrs> {
        DTG_W::new(self, 0)
    }
    ///Bits 8:9 - Lock configuration
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<BDTRrs> {
        LOCK_W::new(self, 8)
    }
    ///Bit 10 - Off-state selection for Idle mode
    #[inline(always)]
    pub fn ossi(&mut self) -> OSSI_W<BDTRrs> {
        OSSI_W::new(self, 10)
    }
    ///Bit 11 - Off-state selection for Run mode
    #[inline(always)]
    pub fn ossr(&mut self) -> OSSR_W<BDTRrs> {
        OSSR_W::new(self, 11)
    }
    ///Bit 12 - Break enable
    #[inline(always)]
    pub fn bke(&mut self) -> BKE_W<BDTRrs> {
        BKE_W::new(self, 12)
    }
    ///Bit 13 - Break polarity
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<BDTRrs> {
        BKP_W::new(self, 13)
    }
    ///Bit 14 - Automatic output enable
    #[inline(always)]
    pub fn aoe(&mut self) -> AOE_W<BDTRrs> {
        AOE_W::new(self, 14)
    }
    ///Bit 15 - Main output enable
    #[inline(always)]
    pub fn moe(&mut self) -> MOE_W<BDTRrs> {
        MOE_W::new(self, 15)
    }
    ///Bits 16:19 - Break filter
    #[inline(always)]
    pub fn bkf(&mut self) -> BKF_W<BDTRrs> {
        BKF_W::new(self, 16)
    }
    ///Bits 20:23 - Break 2 filter
    #[inline(always)]
    pub fn bk2f(&mut self) -> BK2F_W<BDTRrs> {
        BK2F_W::new(self, 20)
    }
    ///Bit 24 - Break 2 enable
    #[inline(always)]
    pub fn bk2e(&mut self) -> BK2E_W<BDTRrs> {
        BK2E_W::new(self, 24)
    }
    ///Bit 25 - Break 2 polarity
    #[inline(always)]
    pub fn bk2p(&mut self) -> BK2P_W<BDTRrs> {
        BK2P_W::new(self, 25)
    }
    ///Bit 26 - BKDSRM
    #[inline(always)]
    pub fn bkdsrm(&mut self) -> BKDSRM_W<BDTRrs> {
        BKDSRM_W::new(self, 26)
    }
    ///Bit 27 - Break2 Disarm
    #[inline(always)]
    pub fn bk2dsrm(&mut self) -> BK2DSRM_W<BDTRrs> {
        BK2DSRM_W::new(self, 27)
    }
    ///Bit 28 - BKBID
    #[inline(always)]
    pub fn bkbid(&mut self) -> BKBID_W<BDTRrs> {
        BKBID_W::new(self, 28)
    }
    ///Bit 29 - Break2 bidirectional
    #[inline(always)]
    pub fn bk2bid(&mut self) -> BK2BID_W<BDTRrs> {
        BK2BID_W::new(self, 29)
    }
}
/**break and dead-time register

You can [`read`](crate::Reg::read) this register and get [`bdtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#TIM1:BDTR)*/
pub struct BDTRrs;
impl crate::RegisterSpec for BDTRrs {
    type Ux = u32;
}
///`read()` method returns [`bdtr::R`](R) reader structure
impl crate::Readable for BDTRrs {}
///`write(|w| ..)` method takes [`bdtr::W`](W) writer structure
impl crate::Writable for BDTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDTR to value 0
impl crate::Resettable for BDTRrs {}
