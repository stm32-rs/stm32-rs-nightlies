///Register `OPTCR` reader
pub type R = crate::R<OPTCRrs>;
///Register `OPTCR` writer
pub type W = crate::W<OPTCRrs>;
/**Option lock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTLOCKR {
    ///0: The write and erase operations in the Option bytes area are disabled
    Unlocked = 0,
    ///1: The write and erase operations in the Option bytes area are enabled
    Locked = 1,
}
impl From<OPTLOCKR> for bool {
    #[inline(always)]
    fn from(variant: OPTLOCKR) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTLOCK` reader - Option lock
pub type OPTLOCK_R = crate::BitReader<OPTLOCKR>;
impl OPTLOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OPTLOCKR {
        match self.bits {
            false => OPTLOCKR::Unlocked,
            true => OPTLOCKR::Locked,
        }
    }
    ///The write and erase operations in the Option bytes area are disabled
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == OPTLOCKR::Unlocked
    }
    ///The write and erase operations in the Option bytes area are enabled
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == OPTLOCKR::Locked
    }
}
/**Option lock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTLOCKW {
    ///1: Lock the FLASH_OPTCR register
    Set = 1,
}
impl From<OPTLOCKW> for bool {
    #[inline(always)]
    fn from(variant: OPTLOCKW) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTLOCK` writer - Option lock
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG, OPTLOCKW>;
impl<'a, REG> OPTLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Lock the FLASH_OPTCR register
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(OPTLOCKW::Set)
    }
}
/**Option start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTSTRTR {
    ///0: Cleared when BSY bit is cleared in SR
    Complete = 0,
    ///1: Options modification requested
    Requested = 1,
}
impl From<OPTSTRTR> for bool {
    #[inline(always)]
    fn from(variant: OPTSTRTR) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTSTRT` reader - Option start
pub type OPTSTRT_R = crate::BitReader<OPTSTRTR>;
impl OPTSTRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OPTSTRTR {
        match self.bits {
            false => OPTSTRTR::Complete,
            true => OPTSTRTR::Requested,
        }
    }
    ///Cleared when BSY bit is cleared in SR
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == OPTSTRTR::Complete
    }
    ///Options modification requested
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == OPTSTRTR::Requested
    }
}
/**Option start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTSTRTW {
    ///1: This bit triggers an options operation when set
    Set = 1,
}
impl From<OPTSTRTW> for bool {
    #[inline(always)]
    fn from(variant: OPTSTRTW) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTSTRT` writer - Option start
pub type OPTSTRT_W<'a, REG> = crate::BitWriter<'a, REG, OPTSTRTW>;
impl<'a, REG> OPTSTRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This bit triggers an options operation when set
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(OPTSTRTW::Set)
    }
}
/**BOR reset Level

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOR_LEV {
    ///0: Reset threshold level for POR/PDR (around 1.7V)
    BorOff = 0,
    ///1: Reset threshold level for VBOR1 (around 2.2 V)
    BorLevel1 = 1,
    ///2: Reset threshold level for VBOR2 (around 2.5 V)
    BorLevel2 = 2,
    ///3: Reset threshold level for VBOR3 (around 2.8 V)
    BorLevel3 = 3,
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
impl crate::IsEnum for BOR_LEV {}
///Field `BOR_LEV` reader - BOR reset Level
pub type BOR_LEV_R = crate::FieldReader<BOR_LEV>;
impl BOR_LEV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BOR_LEV {
        match self.bits {
            0 => BOR_LEV::BorOff,
            1 => BOR_LEV::BorLevel1,
            2 => BOR_LEV::BorLevel2,
            3 => BOR_LEV::BorLevel3,
            _ => unreachable!(),
        }
    }
    ///Reset threshold level for POR/PDR (around 1.7V)
    #[inline(always)]
    pub fn is_bor_off(&self) -> bool {
        *self == BOR_LEV::BorOff
    }
    ///Reset threshold level for VBOR1 (around 2.2 V)
    #[inline(always)]
    pub fn is_bor_level1(&self) -> bool {
        *self == BOR_LEV::BorLevel1
    }
    ///Reset threshold level for VBOR2 (around 2.5 V)
    #[inline(always)]
    pub fn is_bor_level2(&self) -> bool {
        *self == BOR_LEV::BorLevel2
    }
    ///Reset threshold level for VBOR3 (around 2.8 V)
    #[inline(always)]
    pub fn is_bor_level3(&self) -> bool {
        *self == BOR_LEV::BorLevel3
    }
}
///Field `BOR_LEV` writer - BOR reset Level
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BOR_LEV, crate::Safe>;
impl<'a, REG> BOR_LEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reset threshold level for POR/PDR (around 1.7V)
    #[inline(always)]
    pub fn bor_off(self) -> &'a mut crate::W<REG> {
        self.variant(BOR_LEV::BorOff)
    }
    ///Reset threshold level for VBOR1 (around 2.2 V)
    #[inline(always)]
    pub fn bor_level1(self) -> &'a mut crate::W<REG> {
        self.variant(BOR_LEV::BorLevel1)
    }
    ///Reset threshold level for VBOR2 (around 2.5 V)
    #[inline(always)]
    pub fn bor_level2(self) -> &'a mut crate::W<REG> {
        self.variant(BOR_LEV::BorLevel2)
    }
    ///Reset threshold level for VBOR3 (around 2.8 V)
    #[inline(always)]
    pub fn bor_level3(self) -> &'a mut crate::W<REG> {
        self.variant(BOR_LEV::BorLevel3)
    }
}
/**WDG_SW User option bytes

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDG_SW {
    ///0: Hardware watchdog
    Hardware = 0,
    ///1: Software watchdog
    Software = 1,
}
impl From<WDG_SW> for bool {
    #[inline(always)]
    fn from(variant: WDG_SW) -> Self {
        variant as u8 != 0
    }
}
///Field `WDG_SW` reader - WDG_SW User option bytes
pub type WDG_SW_R = crate::BitReader<WDG_SW>;
impl WDG_SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WDG_SW {
        match self.bits {
            false => WDG_SW::Hardware,
            true => WDG_SW::Software,
        }
    }
    ///Hardware watchdog
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == WDG_SW::Hardware
    }
    ///Software watchdog
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == WDG_SW::Software
    }
}
///Field `WDG_SW` writer - WDG_SW User option bytes
pub type WDG_SW_W<'a, REG> = crate::BitWriter<'a, REG, WDG_SW>;
impl<'a, REG> WDG_SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Hardware watchdog
    #[inline(always)]
    pub fn hardware(self) -> &'a mut crate::W<REG> {
        self.variant(WDG_SW::Hardware)
    }
    ///Software watchdog
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(WDG_SW::Software)
    }
}
/**nRST_STOP User option bytes

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_RST_STOP {
    ///0: Reset generated when entering Stop mode
    Reset = 0,
    ///1: No reset generated
    NoReset = 1,
}
impl From<N_RST_STOP> for bool {
    #[inline(always)]
    fn from(variant: N_RST_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `nRST_STOP` reader - nRST_STOP User option bytes
pub type N_RST_STOP_R = crate::BitReader<N_RST_STOP>;
impl N_RST_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> N_RST_STOP {
        match self.bits {
            false => N_RST_STOP::Reset,
            true => N_RST_STOP::NoReset,
        }
    }
    ///Reset generated when entering Stop mode
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == N_RST_STOP::Reset
    }
    ///No reset generated
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == N_RST_STOP::NoReset
    }
}
///Field `nRST_STOP` writer - nRST_STOP User option bytes
pub type N_RST_STOP_W<'a, REG> = crate::BitWriter<'a, REG, N_RST_STOP>;
impl<'a, REG> N_RST_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset generated when entering Stop mode
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(N_RST_STOP::Reset)
    }
    ///No reset generated
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(N_RST_STOP::NoReset)
    }
}
/**nRST_STDBY User option bytes

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_RST_STDBY {
    ///0: Reset generated when entering Standby mode
    Reset = 0,
    ///1: No reset generated
    NoReset = 1,
}
impl From<N_RST_STDBY> for bool {
    #[inline(always)]
    fn from(variant: N_RST_STDBY) -> Self {
        variant as u8 != 0
    }
}
///Field `nRST_STDBY` reader - nRST_STDBY User option bytes
pub type N_RST_STDBY_R = crate::BitReader<N_RST_STDBY>;
impl N_RST_STDBY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> N_RST_STDBY {
        match self.bits {
            false => N_RST_STDBY::Reset,
            true => N_RST_STDBY::NoReset,
        }
    }
    ///Reset generated when entering Standby mode
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == N_RST_STDBY::Reset
    }
    ///No reset generated
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == N_RST_STDBY::NoReset
    }
}
///Field `nRST_STDBY` writer - nRST_STDBY User option bytes
pub type N_RST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG, N_RST_STDBY>;
impl<'a, REG> N_RST_STDBY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset generated when entering Standby mode
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(N_RST_STDBY::Reset)
    }
    ///No reset generated
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(N_RST_STDBY::NoReset)
    }
}
/**Read protect

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDP {
    ///170: Read protection not active
    Level0 = 170,
    ///204: Chip read protection active
    Level2 = 204,
    ///0: Read protection of memories active
    Level1 = 0,
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
impl crate::IsEnum for RDP {}
///Field `RDP` reader - Read protect
pub type RDP_R = crate::FieldReader<RDP>;
impl RDP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDP {
        match self.bits {
            170 => RDP::Level0,
            204 => RDP::Level2,
            _ => RDP::Level1,
        }
    }
    ///Read protection not active
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == RDP::Level0
    }
    ///Chip read protection active
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == RDP::Level2
    }
    ///Read protection of memories active
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        matches!(self.variant(), RDP::Level1)
    }
}
///Field `RDP` writer - Read protect
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8, RDP, crate::Safe>;
impl<'a, REG> RDP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Read protection not active
    #[inline(always)]
    pub fn level0(self) -> &'a mut crate::W<REG> {
        self.variant(RDP::Level0)
    }
    ///Chip read protection active
    #[inline(always)]
    pub fn level2(self) -> &'a mut crate::W<REG> {
        self.variant(RDP::Level2)
    }
    ///Read protection of memories active
    #[inline(always)]
    pub fn level1(self) -> &'a mut crate::W<REG> {
        self.variant(RDP::Level1)
    }
}
/**Not write protect

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_WRP0 {
    ///0: Write protection active on sector %s
    Active = 0,
    ///1: Write protection inactive on sector %s
    Inactive = 1,
}
impl From<N_WRP0> for bool {
    #[inline(always)]
    fn from(variant: N_WRP0) -> Self {
        variant as u8 != 0
    }
}
///Field `nWRP(0-11)` reader - Not write protect
pub type N_WRP_R = crate::BitReader<N_WRP0>;
impl N_WRP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> N_WRP0 {
        match self.bits {
            false => N_WRP0::Active,
            true => N_WRP0::Inactive,
        }
    }
    ///Write protection active on sector %s
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == N_WRP0::Active
    }
    ///Write protection inactive on sector %s
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == N_WRP0::Inactive
    }
}
///Field `nWRP(0-11)` writer - Not write protect
pub type N_WRP_W<'a, REG> = crate::BitWriter<'a, REG, N_WRP0>;
impl<'a, REG> N_WRP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write protection active on sector %s
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(N_WRP0::Active)
    }
    ///Write protection inactive on sector %s
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(N_WRP0::Inactive)
    }
}
/**Selection of Protection Mode of nWPR bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPRMOD {
    ///0: nWPRi bits used for Write protection on sector i
    Disabled = 0,
    ///1: nWPRi bits used for PCROP protection on sector i
    Enabled = 1,
}
impl From<SPRMOD> for bool {
    #[inline(always)]
    fn from(variant: SPRMOD) -> Self {
        variant as u8 != 0
    }
}
///Field `SPRMOD` reader - Selection of Protection Mode of nWPR bits
pub type SPRMOD_R = crate::BitReader<SPRMOD>;
impl SPRMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPRMOD {
        match self.bits {
            false => SPRMOD::Disabled,
            true => SPRMOD::Enabled,
        }
    }
    ///nWPRi bits used for Write protection on sector i
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPRMOD::Disabled
    }
    ///nWPRi bits used for PCROP protection on sector i
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPRMOD::Enabled
    }
}
///Field `SPRMOD` writer - Selection of Protection Mode of nWPR bits
pub type SPRMOD_W<'a, REG> = crate::BitWriter<'a, REG, SPRMOD>;
impl<'a, REG> SPRMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///nWPRi bits used for Write protection on sector i
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPRMOD::Disabled)
    }
    ///nWPRi bits used for PCROP protection on sector i
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPRMOD::Enabled)
    }
}
impl R {
    ///Bit 0 - Option lock
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Option start
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - BOR reset Level
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 5 - WDG_SW User option bytes
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - nRST_STOP User option bytes
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - nRST_STDBY User option bytes
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Read protect
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Not write protect
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `nWRP0` field.</div>
    #[inline(always)]
    pub fn n_wrp(&self, n: u8) -> N_WRP_R {
        #[allow(clippy::no_effect)]
        [(); 12][n as usize];
        N_WRP_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Not write protect
    #[inline(always)]
    pub fn n_wrp_iter(&self) -> impl Iterator<Item = N_WRP_R> + '_ {
        (0..12).map(move |n| N_WRP_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    ///Bit 16 - Not write protect
    #[inline(always)]
    pub fn n_wrp0(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Not write protect
    #[inline(always)]
    pub fn n_wrp1(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Not write protect
    #[inline(always)]
    pub fn n_wrp2(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Not write protect
    #[inline(always)]
    pub fn n_wrp3(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Not write protect
    #[inline(always)]
    pub fn n_wrp4(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Not write protect
    #[inline(always)]
    pub fn n_wrp5(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Not write protect
    #[inline(always)]
    pub fn n_wrp6(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Not write protect
    #[inline(always)]
    pub fn n_wrp7(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Not write protect
    #[inline(always)]
    pub fn n_wrp8(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Not write protect
    #[inline(always)]
    pub fn n_wrp9(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Not write protect
    #[inline(always)]
    pub fn n_wrp10(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Not write protect
    #[inline(always)]
    pub fn n_wrp11(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 31 - Selection of Protection Mode of nWPR bits
    #[inline(always)]
    pub fn sprmod(&self) -> SPRMOD_R {
        SPRMOD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTCR")
            .field("optlock", &self.optlock())
            .field("optstrt", &self.optstrt())
            .field("bor_lev", &self.bor_lev())
            .field("wdg_sw", &self.wdg_sw())
            .field("n_rst_stop", &self.n_rst_stop())
            .field("n_rst_stdby", &self.n_rst_stdby())
            .field("rdp", &self.rdp())
            .field("sprmod", &self.sprmod())
            .field("n_wrp0", &self.n_wrp0())
            .field("n_wrp1", &self.n_wrp1())
            .field("n_wrp2", &self.n_wrp2())
            .field("n_wrp3", &self.n_wrp3())
            .field("n_wrp4", &self.n_wrp4())
            .field("n_wrp5", &self.n_wrp5())
            .field("n_wrp6", &self.n_wrp6())
            .field("n_wrp7", &self.n_wrp7())
            .field("n_wrp8", &self.n_wrp8())
            .field("n_wrp9", &self.n_wrp9())
            .field("n_wrp10", &self.n_wrp10())
            .field("n_wrp11", &self.n_wrp11())
            .finish()
    }
}
impl W {
    ///Bit 0 - Option lock
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W<'_, OPTCRrs> {
        OPTLOCK_W::new(self, 0)
    }
    ///Bit 1 - Option start
    #[inline(always)]
    pub fn optstrt(&mut self) -> OPTSTRT_W<'_, OPTCRrs> {
        OPTSTRT_W::new(self, 1)
    }
    ///Bits 2:3 - BOR reset Level
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<'_, OPTCRrs> {
        BOR_LEV_W::new(self, 2)
    }
    ///Bit 5 - WDG_SW User option bytes
    #[inline(always)]
    pub fn wdg_sw(&mut self) -> WDG_SW_W<'_, OPTCRrs> {
        WDG_SW_W::new(self, 5)
    }
    ///Bit 6 - nRST_STOP User option bytes
    #[inline(always)]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<'_, OPTCRrs> {
        N_RST_STOP_W::new(self, 6)
    }
    ///Bit 7 - nRST_STDBY User option bytes
    #[inline(always)]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<'_, OPTCRrs> {
        N_RST_STDBY_W::new(self, 7)
    }
    ///Bits 8:15 - Read protect
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<'_, OPTCRrs> {
        RDP_W::new(self, 8)
    }
    ///Not write protect
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `nWRP0` field.</div>
    #[inline(always)]
    pub fn n_wrp(&mut self, n: u8) -> N_WRP_W<'_, OPTCRrs> {
        #[allow(clippy::no_effect)]
        [(); 12][n as usize];
        N_WRP_W::new(self, n + 16)
    }
    ///Bit 16 - Not write protect
    #[inline(always)]
    pub fn n_wrp0(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 16)
    }
    ///Bit 17 - Not write protect
    #[inline(always)]
    pub fn n_wrp1(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 17)
    }
    ///Bit 18 - Not write protect
    #[inline(always)]
    pub fn n_wrp2(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 18)
    }
    ///Bit 19 - Not write protect
    #[inline(always)]
    pub fn n_wrp3(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 19)
    }
    ///Bit 20 - Not write protect
    #[inline(always)]
    pub fn n_wrp4(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 20)
    }
    ///Bit 21 - Not write protect
    #[inline(always)]
    pub fn n_wrp5(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 21)
    }
    ///Bit 22 - Not write protect
    #[inline(always)]
    pub fn n_wrp6(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 22)
    }
    ///Bit 23 - Not write protect
    #[inline(always)]
    pub fn n_wrp7(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 23)
    }
    ///Bit 24 - Not write protect
    #[inline(always)]
    pub fn n_wrp8(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 24)
    }
    ///Bit 25 - Not write protect
    #[inline(always)]
    pub fn n_wrp9(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 25)
    }
    ///Bit 26 - Not write protect
    #[inline(always)]
    pub fn n_wrp10(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 26)
    }
    ///Bit 27 - Not write protect
    #[inline(always)]
    pub fn n_wrp11(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 27)
    }
    ///Bit 31 - Selection of Protection Mode of nWPR bits
    #[inline(always)]
    pub fn sprmod(&mut self) -> SPRMOD_W<'_, OPTCRrs> {
        SPRMOD_W::new(self, 31)
    }
}
/**Flash option control register

You can [`read`](crate::Reg::read) this register and get [`optcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F411.html#FLASH:OPTCR)*/
pub struct OPTCRrs;
impl crate::RegisterSpec for OPTCRrs {
    type Ux = u32;
}
///`read()` method returns [`optcr::R`](R) reader structure
impl crate::Readable for OPTCRrs {}
///`write(|w| ..)` method takes [`optcr::W`](W) writer structure
impl crate::Writable for OPTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTCR to value 0x14
impl crate::Resettable for OPTCRrs {
    const RESET_VALUE: u32 = 0x14;
}
