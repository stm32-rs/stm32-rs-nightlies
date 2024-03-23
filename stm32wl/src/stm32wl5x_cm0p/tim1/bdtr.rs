#[doc = "Register `BDTR` reader"]
pub type R = crate::R<BDTRrs>;
#[doc = "Register `BDTR` writer"]
pub type W = crate::W<BDTRrs>;
#[doc = "Field `DTG` reader - Dead-time generator setup"]
pub type DTG_R = crate::FieldReader;
#[doc = "Field `DTG` writer - Dead-time generator setup"]
pub type DTG_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Lock configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK {
    #[doc = "0: No write protection"]
    Off = 0,
    #[doc = "1: Level 1 write protection"]
    Level1 = 1,
    #[doc = "2: Level 2 write protection"]
    Level2 = 2,
    #[doc = "3: Level 3 write protection"]
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
#[doc = "Field `LOCK` reader - Lock configuration"]
pub type LOCK_R = crate::FieldReader<LOCK>;
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "No write protection"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LOCK::Off
    }
    #[doc = "Level 1 write protection"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == LOCK::Level1
    }
    #[doc = "Level 2 write protection"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == LOCK::Level2
    }
    #[doc = "Level 3 write protection"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == LOCK::Level3
    }
}
#[doc = "Field `LOCK` writer - Lock configuration"]
pub type LOCK_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, LOCK>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No write protection"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Off)
    }
    #[doc = "Level 1 write protection"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Level1)
    }
    #[doc = "Level 2 write protection"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Level2)
    }
    #[doc = "Level 3 write protection"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Level3)
    }
}
#[doc = "Off-state selection for Idle mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSI {
    #[doc = "0: OC/OCN outputs are disabled when inactive"]
    Disabled = 0,
    #[doc = "1: OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime"]
    Enabled = 1,
}
impl From<OSSI> for bool {
    #[inline(always)]
    fn from(variant: OSSI) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSSI` reader - Off-state selection for Idle mode"]
pub type OSSI_R = crate::BitReader<OSSI>;
impl OSSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSSI {
        match self.bits {
            false => OSSI::Disabled,
            true => OSSI::Enabled,
        }
    }
    #[doc = "OC/OCN outputs are disabled when inactive"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSSI::Disabled
    }
    #[doc = "OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSSI::Enabled
    }
}
#[doc = "Field `OSSI` writer - Off-state selection for Idle mode"]
pub type OSSI_W<'a, REG> = crate::BitWriter<'a, REG, OSSI>;
impl<'a, REG> OSSI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC/OCN outputs are disabled when inactive"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSSI::Disabled)
    }
    #[doc = "OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSSI::Enabled)
    }
}
#[doc = "Off-state selection for Run mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSR {
    #[doc = "0: OC/OCN outputs are disabled when inactive"]
    Disabled = 0,
    #[doc = "1: OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1"]
    Enabled = 1,
}
impl From<OSSR> for bool {
    #[inline(always)]
    fn from(variant: OSSR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSSR` reader - Off-state selection for Run mode"]
pub type OSSR_R = crate::BitReader<OSSR>;
impl OSSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSSR {
        match self.bits {
            false => OSSR::Disabled,
            true => OSSR::Enabled,
        }
    }
    #[doc = "OC/OCN outputs are disabled when inactive"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSSR::Disabled
    }
    #[doc = "OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSSR::Enabled
    }
}
#[doc = "Field `OSSR` writer - Off-state selection for Run mode"]
pub type OSSR_W<'a, REG> = crate::BitWriter<'a, REG, OSSR>;
impl<'a, REG> OSSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC/OCN outputs are disabled when inactive"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSSR::Disabled)
    }
    #[doc = "OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSSR::Enabled)
    }
}
#[doc = "Break enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKE {
    #[doc = "0: Break function disabled"]
    Disabled = 0,
    #[doc = "1: Break function enabled"]
    Enabled = 1,
}
impl From<BKE> for bool {
    #[inline(always)]
    fn from(variant: BKE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKE` reader - Break enable"]
pub type BKE_R = crate::BitReader<BKE>;
impl BKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKE {
        match self.bits {
            false => BKE::Disabled,
            true => BKE::Enabled,
        }
    }
    #[doc = "Break function disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BKE::Disabled
    }
    #[doc = "Break function enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BKE::Enabled
    }
}
#[doc = "Field `BKE` writer - Break enable"]
pub type BKE_W<'a, REG> = crate::BitWriter<'a, REG, BKE>;
impl<'a, REG> BKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break function disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKE::Disabled)
    }
    #[doc = "Break function enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKE::Enabled)
    }
}
#[doc = "Break polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKP {
    #[doc = "0: Break input BRK is active low"]
    ActiveLow = 0,
    #[doc = "1: Break input BRK is active high"]
    ActiveHigh = 1,
}
impl From<BKP> for bool {
    #[inline(always)]
    fn from(variant: BKP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKP` reader - Break polarity"]
pub type BKP_R = crate::BitReader<BKP>;
impl BKP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKP {
        match self.bits {
            false => BKP::ActiveLow,
            true => BKP::ActiveHigh,
        }
    }
    #[doc = "Break input BRK is active low"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == BKP::ActiveLow
    }
    #[doc = "Break input BRK is active high"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == BKP::ActiveHigh
    }
}
#[doc = "Field `BKP` writer - Break polarity"]
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG, BKP>;
impl<'a, REG> BKP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input BRK is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(BKP::ActiveLow)
    }
    #[doc = "Break input BRK is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(BKP::ActiveHigh)
    }
}
#[doc = "Automatic output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AOE {
    #[doc = "0: MOE can be set only by software"]
    Disabled = 0,
    #[doc = "1: MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)"]
    Enabled = 1,
}
impl From<AOE> for bool {
    #[inline(always)]
    fn from(variant: AOE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AOE` reader - Automatic output enable"]
pub type AOE_R = crate::BitReader<AOE>;
impl AOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AOE {
        match self.bits {
            false => AOE::Disabled,
            true => AOE::Enabled,
        }
    }
    #[doc = "MOE can be set only by software"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AOE::Disabled
    }
    #[doc = "MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AOE::Enabled
    }
}
#[doc = "Field `AOE` writer - Automatic output enable"]
pub type AOE_W<'a, REG> = crate::BitWriter<'a, REG, AOE>;
impl<'a, REG> AOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MOE can be set only by software"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AOE::Disabled)
    }
    #[doc = "MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AOE::Enabled)
    }
}
#[doc = "Main output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOE {
    #[doc = "0: In response to a break 2 event OC and OCN outputs are disabled - In response to a break event or if MOE is written to 0 OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit"]
    Disabled = 0,
    #[doc = "1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)"]
    Enabled = 1,
}
impl From<MOE> for bool {
    #[inline(always)]
    fn from(variant: MOE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MOE` reader - Main output enable"]
pub type MOE_R = crate::BitReader<MOE>;
impl MOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MOE {
        match self.bits {
            false => MOE::Disabled,
            true => MOE::Enabled,
        }
    }
    #[doc = "In response to a break 2 event OC and OCN outputs are disabled - In response to a break event or if MOE is written to 0 OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MOE::Disabled
    }
    #[doc = "OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MOE::Enabled
    }
}
#[doc = "Field `MOE` writer - Main output enable"]
pub type MOE_W<'a, REG> = crate::BitWriter<'a, REG, MOE>;
impl<'a, REG> MOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In response to a break 2 event OC and OCN outputs are disabled - In response to a break event or if MOE is written to 0 OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MOE::Disabled)
    }
    #[doc = "OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MOE::Enabled)
    }
}
#[doc = "Break filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BKF {
    #[doc = "0: No filter, sampling is done at fDTS"]
    NoFilter = 0,
    #[doc = "1: fSAMPLING=fCK_INT, N=2"]
    FckIntN2 = 1,
    #[doc = "2: fSAMPLING=fCK_INT, N=4"]
    FckIntN4 = 2,
    #[doc = "3: fSAMPLING=fCK_INT, N=8"]
    FckIntN8 = 3,
    #[doc = "4: fSAMPLING=fDTS/2, N=6"]
    FdtsDiv2N6 = 4,
    #[doc = "5: fSAMPLING=fDTS/2, N=8"]
    FdtsDiv2N8 = 5,
    #[doc = "6: fSAMPLING=fDTS/4, N=6"]
    FdtsDiv4N6 = 6,
    #[doc = "7: fSAMPLING=fDTS/4, N=8"]
    FdtsDiv4N8 = 7,
    #[doc = "8: fSAMPLING=fDTS/8, N=6"]
    FdtsDiv8N6 = 8,
    #[doc = "9: fSAMPLING=fDTS/8, N=8"]
    FdtsDiv8N8 = 9,
    #[doc = "10: fSAMPLING=fDTS/16, N=5"]
    FdtsDiv16N5 = 10,
    #[doc = "11: fSAMPLING=fDTS/16, N=6"]
    FdtsDiv16N6 = 11,
    #[doc = "12: fSAMPLING=fDTS/16, N=8"]
    FdtsDiv16N8 = 12,
    #[doc = "13: fSAMPLING=fDTS/32, N=5"]
    FdtsDiv32N5 = 13,
    #[doc = "14: fSAMPLING=fDTS/32, N=6"]
    FdtsDiv32N6 = 14,
    #[doc = "15: fSAMPLING=fDTS/32, N=8"]
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
#[doc = "Field `BKF` reader - Break filter"]
pub type BKF_R = crate::FieldReader<BKF>;
impl BKF_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == BKF::NoFilter
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == BKF::FckIntN2
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == BKF::FckIntN4
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == BKF::FckIntN8
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == BKF::FdtsDiv2N6
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == BKF::FdtsDiv2N8
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == BKF::FdtsDiv4N6
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == BKF::FdtsDiv4N8
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == BKF::FdtsDiv8N6
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == BKF::FdtsDiv8N8
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == BKF::FdtsDiv16N5
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == BKF::FdtsDiv16N6
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == BKF::FdtsDiv16N8
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == BKF::FdtsDiv32N5
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == BKF::FdtsDiv32N6
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == BKF::FdtsDiv32N8
    }
}
#[doc = "Field `BKF` writer - Break filter"]
pub type BKF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, BKF>;
impl<'a, REG> BKF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::NoFilter)
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FckIntN2)
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FckIntN4)
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FckIntN8)
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv2N6)
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv2N8)
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv4N6)
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv4N8)
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv8N6)
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv8N8)
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv16N5)
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv16N6)
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv16N8)
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv32N5)
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv32N6)
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::FdtsDiv32N8)
    }
}
#[doc = "Break 2 filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BK2F {
    #[doc = "0: No filter, sampling is done at fDTS"]
    NoFilter = 0,
    #[doc = "1: fSAMPLING=fCK_INT, N=2"]
    FckIntN2 = 1,
    #[doc = "2: fSAMPLING=fCK_INT, N=4"]
    FckIntN4 = 2,
    #[doc = "3: fSAMPLING=fCK_INT, N=8"]
    FckIntN8 = 3,
    #[doc = "4: fSAMPLING=fDTS/2, N=6"]
    FdtsDiv2N6 = 4,
    #[doc = "5: fSAMPLING=fDTS/2, N=8"]
    FdtsDiv2N8 = 5,
    #[doc = "6: fSAMPLING=fDTS/4, N=6"]
    FdtsDiv4N6 = 6,
    #[doc = "7: fSAMPLING=fDTS/4, N=8"]
    FdtsDiv4N8 = 7,
    #[doc = "8: fSAMPLING=fDTS/8, N=6"]
    FdtsDiv8N6 = 8,
    #[doc = "9: fSAMPLING=fDTS/8, N=8"]
    FdtsDiv8N8 = 9,
    #[doc = "10: fSAMPLING=fDTS/16, N=5"]
    FdtsDiv16N5 = 10,
    #[doc = "11: fSAMPLING=fDTS/16, N=6"]
    FdtsDiv16N6 = 11,
    #[doc = "12: fSAMPLING=fDTS/16, N=8"]
    FdtsDiv16N8 = 12,
    #[doc = "13: fSAMPLING=fDTS/32, N=5"]
    FdtsDiv32N5 = 13,
    #[doc = "14: fSAMPLING=fDTS/32, N=6"]
    FdtsDiv32N6 = 14,
    #[doc = "15: fSAMPLING=fDTS/32, N=8"]
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
#[doc = "Field `BK2F` reader - Break 2 filter"]
pub type BK2F_R = crate::FieldReader<BK2F>;
impl BK2F_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == BK2F::NoFilter
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == BK2F::FckIntN2
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == BK2F::FckIntN4
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == BK2F::FckIntN8
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == BK2F::FdtsDiv2N6
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == BK2F::FdtsDiv2N8
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == BK2F::FdtsDiv4N6
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == BK2F::FdtsDiv4N8
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == BK2F::FdtsDiv8N6
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == BK2F::FdtsDiv8N8
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == BK2F::FdtsDiv16N5
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == BK2F::FdtsDiv16N6
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == BK2F::FdtsDiv16N8
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == BK2F::FdtsDiv32N5
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == BK2F::FdtsDiv32N6
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == BK2F::FdtsDiv32N8
    }
}
#[doc = "Field `BK2F` writer - Break 2 filter"]
pub type BK2F_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, BK2F>;
impl<'a, REG> BK2F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::NoFilter)
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FckIntN2)
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FckIntN4)
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FckIntN8)
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv2N6)
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv2N8)
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv4N6)
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv4N8)
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv8N6)
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv8N8)
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv16N5)
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv16N6)
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv16N8)
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv32N5)
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv32N6)
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::FdtsDiv32N8)
    }
}
#[doc = "Break 2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2E {
    #[doc = "0: Break function disabled"]
    Disabled = 0,
    #[doc = "1: Break function enabled"]
    Enabled = 1,
}
impl From<BK2E> for bool {
    #[inline(always)]
    fn from(variant: BK2E) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2E` reader - Break 2 enable"]
pub type BK2E_R = crate::BitReader<BK2E>;
impl BK2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2E {
        match self.bits {
            false => BK2E::Disabled,
            true => BK2E::Enabled,
        }
    }
    #[doc = "Break function disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BK2E::Disabled
    }
    #[doc = "Break function enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BK2E::Enabled
    }
}
#[doc = "Field `BK2E` writer - Break 2 enable"]
pub type BK2E_W<'a, REG> = crate::BitWriter<'a, REG, BK2E>;
impl<'a, REG> BK2E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break function disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2E::Disabled)
    }
    #[doc = "Break function enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2E::Enabled)
    }
}
#[doc = "Break 2 polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2P {
    #[doc = "0: Break input BRK2 is active low"]
    Low = 0,
    #[doc = "1: Break input BRK2 is active high"]
    High = 1,
}
impl From<BK2P> for bool {
    #[inline(always)]
    fn from(variant: BK2P) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2P` reader - Break 2 polarity"]
pub type BK2P_R = crate::BitReader<BK2P>;
impl BK2P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2P {
        match self.bits {
            false => BK2P::Low,
            true => BK2P::High,
        }
    }
    #[doc = "Break input BRK2 is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == BK2P::Low
    }
    #[doc = "Break input BRK2 is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == BK2P::High
    }
}
#[doc = "Field `BK2P` writer - Break 2 polarity"]
pub type BK2P_W<'a, REG> = crate::BitWriter<'a, REG, BK2P>;
impl<'a, REG> BK2P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input BRK2 is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(BK2P::Low)
    }
    #[doc = "Break input BRK2 is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(BK2P::High)
    }
}
#[doc = "BKDSRM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKDSRM {
    #[doc = "0: Break input BRK is armed"]
    Armed = 0,
    #[doc = "1: Break input BRK is disarmed"]
    Disarmed = 1,
}
impl From<BKDSRM> for bool {
    #[inline(always)]
    fn from(variant: BKDSRM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKDSRM` reader - BKDSRM"]
pub type BKDSRM_R = crate::BitReader<BKDSRM>;
impl BKDSRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKDSRM {
        match self.bits {
            false => BKDSRM::Armed,
            true => BKDSRM::Disarmed,
        }
    }
    #[doc = "Break input BRK is armed"]
    #[inline(always)]
    pub fn is_armed(&self) -> bool {
        *self == BKDSRM::Armed
    }
    #[doc = "Break input BRK is disarmed"]
    #[inline(always)]
    pub fn is_disarmed(&self) -> bool {
        *self == BKDSRM::Disarmed
    }
}
#[doc = "Field `BKDSRM` writer - BKDSRM"]
pub type BKDSRM_W<'a, REG> = crate::BitWriter<'a, REG, BKDSRM>;
impl<'a, REG> BKDSRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input BRK is armed"]
    #[inline(always)]
    pub fn armed(self) -> &'a mut crate::W<REG> {
        self.variant(BKDSRM::Armed)
    }
    #[doc = "Break input BRK is disarmed"]
    #[inline(always)]
    pub fn disarmed(self) -> &'a mut crate::W<REG> {
        self.variant(BKDSRM::Disarmed)
    }
}
#[doc = "Break2 Disarm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2DSRM {
    #[doc = "0: Break input BRK2 is armed"]
    Armed = 0,
    #[doc = "1: Break input BRK2 is disarmed"]
    Disarmed = 1,
}
impl From<BK2DSRM> for bool {
    #[inline(always)]
    fn from(variant: BK2DSRM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2DSRM` reader - Break2 Disarm"]
pub type BK2DSRM_R = crate::BitReader<BK2DSRM>;
impl BK2DSRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2DSRM {
        match self.bits {
            false => BK2DSRM::Armed,
            true => BK2DSRM::Disarmed,
        }
    }
    #[doc = "Break input BRK2 is armed"]
    #[inline(always)]
    pub fn is_armed(&self) -> bool {
        *self == BK2DSRM::Armed
    }
    #[doc = "Break input BRK2 is disarmed"]
    #[inline(always)]
    pub fn is_disarmed(&self) -> bool {
        *self == BK2DSRM::Disarmed
    }
}
#[doc = "Field `BK2DSRM` writer - Break2 Disarm"]
pub type BK2DSRM_W<'a, REG> = crate::BitWriter<'a, REG, BK2DSRM>;
impl<'a, REG> BK2DSRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input BRK2 is armed"]
    #[inline(always)]
    pub fn armed(self) -> &'a mut crate::W<REG> {
        self.variant(BK2DSRM::Armed)
    }
    #[doc = "Break input BRK2 is disarmed"]
    #[inline(always)]
    pub fn disarmed(self) -> &'a mut crate::W<REG> {
        self.variant(BK2DSRM::Disarmed)
    }
}
#[doc = "BKBID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKBID {
    #[doc = "0: Break input BRK in input mode"]
    Input = 0,
    #[doc = "1: Break input BRK in bidirectional mode"]
    Bidirectional = 1,
}
impl From<BKBID> for bool {
    #[inline(always)]
    fn from(variant: BKBID) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKBID` reader - BKBID"]
pub type BKBID_R = crate::BitReader<BKBID>;
impl BKBID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKBID {
        match self.bits {
            false => BKBID::Input,
            true => BKBID::Bidirectional,
        }
    }
    #[doc = "Break input BRK in input mode"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == BKBID::Input
    }
    #[doc = "Break input BRK in bidirectional mode"]
    #[inline(always)]
    pub fn is_bidirectional(&self) -> bool {
        *self == BKBID::Bidirectional
    }
}
#[doc = "Field `BKBID` writer - BKBID"]
pub type BKBID_W<'a, REG> = crate::BitWriter<'a, REG, BKBID>;
impl<'a, REG> BKBID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input BRK in input mode"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(BKBID::Input)
    }
    #[doc = "Break input BRK in bidirectional mode"]
    #[inline(always)]
    pub fn bidirectional(self) -> &'a mut crate::W<REG> {
        self.variant(BKBID::Bidirectional)
    }
}
#[doc = "Break2 bidirectional\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2BID {
    #[doc = "0: Break input BRK2 in input mode"]
    Input = 0,
    #[doc = "1: Break input BRK2 in bidirectional mode"]
    Bidirectional = 1,
}
impl From<BK2BID> for bool {
    #[inline(always)]
    fn from(variant: BK2BID) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2BID` reader - Break2 bidirectional"]
pub type BK2BID_R = crate::BitReader<BK2BID>;
impl BK2BID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2BID {
        match self.bits {
            false => BK2BID::Input,
            true => BK2BID::Bidirectional,
        }
    }
    #[doc = "Break input BRK2 in input mode"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == BK2BID::Input
    }
    #[doc = "Break input BRK2 in bidirectional mode"]
    #[inline(always)]
    pub fn is_bidirectional(&self) -> bool {
        *self == BK2BID::Bidirectional
    }
}
#[doc = "Field `BK2BID` writer - Break2 bidirectional"]
pub type BK2BID_W<'a, REG> = crate::BitWriter<'a, REG, BK2BID>;
impl<'a, REG> BK2BID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input BRK2 in input mode"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(BK2BID::Input)
    }
    #[doc = "Break input BRK2 in bidirectional mode"]
    #[inline(always)]
    pub fn bidirectional(self) -> &'a mut crate::W<REG> {
        self.variant(BK2BID::Bidirectional)
    }
}
impl R {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Lock configuration"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Break filter"]
    #[inline(always)]
    pub fn bkf(&self) -> BKF_R {
        BKF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Break 2 filter"]
    #[inline(always)]
    pub fn bk2f(&self) -> BK2F_R {
        BK2F_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Break 2 enable"]
    #[inline(always)]
    pub fn bk2e(&self) -> BK2E_R {
        BK2E_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Break 2 polarity"]
    #[inline(always)]
    pub fn bk2p(&self) -> BK2P_R {
        BK2P_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - BKDSRM"]
    #[inline(always)]
    pub fn bkdsrm(&self) -> BKDSRM_R {
        BKDSRM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Break2 Disarm"]
    #[inline(always)]
    pub fn bk2dsrm(&self) -> BK2DSRM_R {
        BK2DSRM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - BKBID"]
    #[inline(always)]
    pub fn bkbid(&self) -> BKBID_R {
        BKBID_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Break2 bidirectional"]
    #[inline(always)]
    pub fn bk2bid(&self) -> BK2BID_R {
        BK2BID_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    #[must_use]
    pub fn dtg(&mut self) -> DTG_W<BDTRrs> {
        DTG_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Lock configuration"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<BDTRrs> {
        LOCK_W::new(self, 8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    #[must_use]
    pub fn ossi(&mut self) -> OSSI_W<BDTRrs> {
        OSSI_W::new(self, 10)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    #[must_use]
    pub fn ossr(&mut self) -> OSSR_W<BDTRrs> {
        OSSR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    #[must_use]
    pub fn bke(&mut self) -> BKE_W<BDTRrs> {
        BKE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<BDTRrs> {
        BKP_W::new(self, 13)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    #[must_use]
    pub fn aoe(&mut self) -> AOE_W<BDTRrs> {
        AOE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    #[must_use]
    pub fn moe(&mut self) -> MOE_W<BDTRrs> {
        MOE_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Break filter"]
    #[inline(always)]
    #[must_use]
    pub fn bkf(&mut self) -> BKF_W<BDTRrs> {
        BKF_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Break 2 filter"]
    #[inline(always)]
    #[must_use]
    pub fn bk2f(&mut self) -> BK2F_W<BDTRrs> {
        BK2F_W::new(self, 20)
    }
    #[doc = "Bit 24 - Break 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2e(&mut self) -> BK2E_W<BDTRrs> {
        BK2E_W::new(self, 24)
    }
    #[doc = "Bit 25 - Break 2 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2p(&mut self) -> BK2P_W<BDTRrs> {
        BK2P_W::new(self, 25)
    }
    #[doc = "Bit 26 - BKDSRM"]
    #[inline(always)]
    #[must_use]
    pub fn bkdsrm(&mut self) -> BKDSRM_W<BDTRrs> {
        BKDSRM_W::new(self, 26)
    }
    #[doc = "Bit 27 - Break2 Disarm"]
    #[inline(always)]
    #[must_use]
    pub fn bk2dsrm(&mut self) -> BK2DSRM_W<BDTRrs> {
        BK2DSRM_W::new(self, 27)
    }
    #[doc = "Bit 28 - BKBID"]
    #[inline(always)]
    #[must_use]
    pub fn bkbid(&mut self) -> BKBID_W<BDTRrs> {
        BKBID_W::new(self, 28)
    }
    #[doc = "Bit 29 - Break2 bidirectional"]
    #[inline(always)]
    #[must_use]
    pub fn bk2bid(&mut self) -> BK2BID_W<BDTRrs> {
        BK2BID_W::new(self, 29)
    }
}
#[doc = "break and dead-time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDTRrs;
impl crate::RegisterSpec for BDTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdtr::R`](R) reader structure"]
impl crate::Readable for BDTRrs {}
#[doc = "`write(|w| ..)` method takes [`bdtr::W`](W) writer structure"]
impl crate::Writable for BDTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BDTR to value 0"]
impl crate::Resettable for BDTRrs {
    const RESET_VALUE: u32 = 0;
}
