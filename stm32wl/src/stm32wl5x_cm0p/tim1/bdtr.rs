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
    ///0: No write protection
    Off = 0,
    ///1: Level 1 write protection
    Level1 = 1,
    ///2: Level 2 write protection
    Level2 = 2,
    ///3: Level 3 write protection
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
    ///No write protection
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LOCK::Off
    }
    ///Level 1 write protection
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == LOCK::Level1
    }
    ///Level 2 write protection
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == LOCK::Level2
    }
    ///Level 3 write protection
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
    ///No write protection
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Off)
    }
    ///Level 1 write protection
    #[inline(always)]
    pub fn level1(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Level1)
    }
    ///Level 2 write protection
    #[inline(always)]
    pub fn level2(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Level2)
    }
    ///Level 3 write protection
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
    ///0: OC/OCN outputs are disabled when inactive
    Disabled = 0,
    ///1: OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime
    Enabled = 1,
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
            false => OSSI::Disabled,
            true => OSSI::Enabled,
        }
    }
    ///OC/OCN outputs are disabled when inactive
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSSI::Disabled
    }
    ///OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSSI::Enabled
    }
}
///Field `OSSI` writer - Off-state selection for Idle mode
pub type OSSI_W<'a, REG> = crate::BitWriter<'a, REG, OSSI>;
impl<'a, REG> OSSI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OC/OCN outputs are disabled when inactive
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSSI::Disabled)
    }
    ///OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSSI::Enabled)
    }
}
/**Off-state selection for Run mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSR {
    ///0: OC/OCN outputs are disabled when inactive
    Disabled = 0,
    ///1: OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1
    Enabled = 1,
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
            false => OSSR::Disabled,
            true => OSSR::Enabled,
        }
    }
    ///OC/OCN outputs are disabled when inactive
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSSR::Disabled
    }
    ///OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSSR::Enabled
    }
}
///Field `OSSR` writer - Off-state selection for Run mode
pub type OSSR_W<'a, REG> = crate::BitWriter<'a, REG, OSSR>;
impl<'a, REG> OSSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OC/OCN outputs are disabled when inactive
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSSR::Disabled)
    }
    ///OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSSR::Enabled)
    }
}
/**Break enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKE {
    ///0: Break function disabled
    Disabled = 0,
    ///1: Break function enabled
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
    ///Break function disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BKE::Disabled
    }
    ///Break function enabled
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
    ///Break function disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKE::Disabled)
    }
    ///Break function enabled
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
    ///0: Break input BRK is active low
    ActiveLow = 0,
    ///1: Break input BRK is active high
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
    ///Break input BRK is active low
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == BKP::ActiveLow
    }
    ///Break input BRK is active high
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
    ///Break input BRK is active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(BKP::ActiveLow)
    }
    ///Break input BRK is active high
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
    Disabled = 0,
    ///1: MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)
    Enabled = 1,
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
            false => AOE::Disabled,
            true => AOE::Enabled,
        }
    }
    ///MOE can be set only by software
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AOE::Disabled
    }
    ///MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AOE::Enabled
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
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AOE::Disabled)
    }
    ///MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AOE::Enabled)
    }
}
/**Main output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOE {
    ///0: In response to a break 2 event OC and OCN outputs are disabled - In response to a break event or if MOE is written to 0 OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit
    Disabled = 0,
    ///1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)
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
            false => MOE::Disabled,
            true => MOE::Enabled,
        }
    }
    ///In response to a break 2 event OC and OCN outputs are disabled - In response to a break event or if MOE is written to 0 OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MOE::Disabled
    }
    ///OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)
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
    ///In response to a break 2 event OC and OCN outputs are disabled - In response to a break event or if MOE is written to 0 OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MOE::Disabled)
    }
    ///OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)
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
/**Break 2 filter

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BK2F {
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
impl From<BK2F> for u8 {
    #[inline(always)]
    fn from(variant: BK2F) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BK2F {
    type Ux = u8;
}
impl crate::IsEnum for BK2F {}
///Field `BK2F` reader - Break 2 filter
pub type BK2F_R = crate::FieldReader<BK2F>;
impl BK2F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2F {
        match self.bits {
            0 => BK2F::NoFilter,
            1 => BK2F::FckIntN2,
            2 => BK2F::FckIntN4,
            3 => BK2F::FckIntN8,
            4 => BK2F::FdtsDiv2N6,
            5 => BK2F::FdtsDiv2N8,
            6 => BK2F::FdtsDiv4N6,
            7 => BK2F::FdtsDiv4N8,
            8 => BK2F::FdtsDiv8N6,
            9 => BK2F::FdtsDiv8N8,
            10 => BK2F::FdtsDiv16N5,
            11 => BK2F::FdtsDiv16N6,
            12 => BK2F::FdtsDiv16N8,
            13 => BK2F::FdtsDiv32N5,
            14 => BK2F::FdtsDiv32N6,
            15 => BK2F::FdtsDiv32N8,
            _ => unreachable!(),
        }
    }
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == BK2F::NoFilter
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == BK2F::FckIntN2
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == BK2F::FckIntN4
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == BK2F::FckIntN8
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == BK2F::FdtsDiv2N6
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == BK2F::FdtsDiv2N8
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == BK2F::FdtsDiv4N6
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == BK2F::FdtsDiv4N8
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == BK2F::FdtsDiv8N6
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == BK2F::FdtsDiv8N8
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == BK2F::FdtsDiv16N5
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == BK2F::FdtsDiv16N6
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == BK2F::FdtsDiv16N8
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == BK2F::FdtsDiv32N5
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == BK2F::FdtsDiv32N6
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == BK2F::FdtsDiv32N8
    }
}
///Field `BK2F` writer - Break 2 filter
pub type BK2F_W<'a, REG> = crate::FieldWriter<'a, REG, 4, BK2F, crate::Safe>;
impl<'a, REG> BK2F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::NoFilter)
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FckIntN2)
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FckIntN4)
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FckIntN8)
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv2N6)
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv2N8)
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv4N6)
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv4N8)
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv8N6)
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv8N8)
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv16N5)
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv16N6)
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv16N8)
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv32N5)
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv32N6)
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv32N8)
    }
}
/**Break 2 enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2E {
    ///0: Break function disabled
    Disabled = 0,
    ///1: Break function enabled
    Enabled = 1,
}
impl From<BK2E> for bool {
    #[inline(always)]
    fn from(variant: BK2E) -> Self {
        variant as u8 != 0
    }
}
///Field `BK2E` reader - Break 2 enable
pub type BK2E_R = crate::BitReader<BK2E>;
impl BK2E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2E {
        match self.bits {
            false => BK2E::Disabled,
            true => BK2E::Enabled,
        }
    }
    ///Break function disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BK2E::Disabled
    }
    ///Break function enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BK2E::Enabled
    }
}
///Field `BK2E` writer - Break 2 enable
pub type BK2E_W<'a, REG> = crate::BitWriter<'a, REG, BK2E>;
impl<'a, REG> BK2E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break function disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2E::Disabled)
    }
    ///Break function enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2E::Enabled)
    }
}
/**Break 2 polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2P {
    ///0: Break input BRK2 is active low
    Low = 0,
    ///1: Break input BRK2 is active high
    High = 1,
}
impl From<BK2P> for bool {
    #[inline(always)]
    fn from(variant: BK2P) -> Self {
        variant as u8 != 0
    }
}
///Field `BK2P` reader - Break 2 polarity
pub type BK2P_R = crate::BitReader<BK2P>;
impl BK2P_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2P {
        match self.bits {
            false => BK2P::Low,
            true => BK2P::High,
        }
    }
    ///Break input BRK2 is active low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == BK2P::Low
    }
    ///Break input BRK2 is active high
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == BK2P::High
    }
}
///Field `BK2P` writer - Break 2 polarity
pub type BK2P_W<'a, REG> = crate::BitWriter<'a, REG, BK2P>;
impl<'a, REG> BK2P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break input BRK2 is active low
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(BK2P::Low)
    }
    ///Break input BRK2 is active high
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(BK2P::High)
    }
}
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
            .field("bk2p", &self.bk2p())
            .field("bk2e", &self.bk2e())
            .field("bk2f", &self.bk2f())
            .field("bkf", &self.bkf())
            .field("moe", &self.moe())
            .field("aoe", &self.aoe())
            .field("bkp", &self.bkp())
            .field("bke", &self.bke())
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
    #[must_use]
    pub fn dtg(&mut self) -> DTG_W<BDTRrs> {
        DTG_W::new(self, 0)
    }
    ///Bits 8:9 - Lock configuration
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<BDTRrs> {
        LOCK_W::new(self, 8)
    }
    ///Bit 10 - Off-state selection for Idle mode
    #[inline(always)]
    #[must_use]
    pub fn ossi(&mut self) -> OSSI_W<BDTRrs> {
        OSSI_W::new(self, 10)
    }
    ///Bit 11 - Off-state selection for Run mode
    #[inline(always)]
    #[must_use]
    pub fn ossr(&mut self) -> OSSR_W<BDTRrs> {
        OSSR_W::new(self, 11)
    }
    ///Bit 12 - Break enable
    #[inline(always)]
    #[must_use]
    pub fn bke(&mut self) -> BKE_W<BDTRrs> {
        BKE_W::new(self, 12)
    }
    ///Bit 13 - Break polarity
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<BDTRrs> {
        BKP_W::new(self, 13)
    }
    ///Bit 14 - Automatic output enable
    #[inline(always)]
    #[must_use]
    pub fn aoe(&mut self) -> AOE_W<BDTRrs> {
        AOE_W::new(self, 14)
    }
    ///Bit 15 - Main output enable
    #[inline(always)]
    #[must_use]
    pub fn moe(&mut self) -> MOE_W<BDTRrs> {
        MOE_W::new(self, 15)
    }
    ///Bits 16:19 - Break filter
    #[inline(always)]
    #[must_use]
    pub fn bkf(&mut self) -> BKF_W<BDTRrs> {
        BKF_W::new(self, 16)
    }
    ///Bits 20:23 - Break 2 filter
    #[inline(always)]
    #[must_use]
    pub fn bk2f(&mut self) -> BK2F_W<BDTRrs> {
        BK2F_W::new(self, 20)
    }
    ///Bit 24 - Break 2 enable
    #[inline(always)]
    #[must_use]
    pub fn bk2e(&mut self) -> BK2E_W<BDTRrs> {
        BK2E_W::new(self, 24)
    }
    ///Bit 25 - Break 2 polarity
    #[inline(always)]
    #[must_use]
    pub fn bk2p(&mut self) -> BK2P_W<BDTRrs> {
        BK2P_W::new(self, 25)
    }
    ///Bit 26 - BKDSRM
    #[inline(always)]
    #[must_use]
    pub fn bkdsrm(&mut self) -> BKDSRM_W<BDTRrs> {
        BKDSRM_W::new(self, 26)
    }
    ///Bit 27 - Break2 Disarm
    #[inline(always)]
    #[must_use]
    pub fn bk2dsrm(&mut self) -> BK2DSRM_W<BDTRrs> {
        BK2DSRM_W::new(self, 27)
    }
    ///Bit 28 - BKBID
    #[inline(always)]
    #[must_use]
    pub fn bkbid(&mut self) -> BKBID_W<BDTRrs> {
        BKBID_W::new(self, 28)
    }
    ///Bit 29 - Break2 bidirectional
    #[inline(always)]
    #[must_use]
    pub fn bk2bid(&mut self) -> BK2BID_W<BDTRrs> {
        BK2BID_W::new(self, 29)
    }
}
/**break and dead-time register

You can [`read`](crate::Reg::read) this register and get [`bdtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#TIM1:BDTR)*/
pub struct BDTRrs;
impl crate::RegisterSpec for BDTRrs {
    type Ux = u32;
}
///`read()` method returns [`bdtr::R`](R) reader structure
impl crate::Readable for BDTRrs {}
///`write(|w| ..)` method takes [`bdtr::W`](W) writer structure
impl crate::Writable for BDTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BDTR to value 0
impl crate::Resettable for BDTRrs {
    const RESET_VALUE: u32 = 0;
}
