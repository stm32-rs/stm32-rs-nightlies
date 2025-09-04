///Register `OPTR` reader
pub type R = crate::R<OPTRrs>;
///Register `OPTR` writer
pub type W = crate::W<OPTRrs>;
///Field `RDP` reader - Read protection level
pub type RDP_R = crate::FieldReader;
///Field `RDP` writer - Read protection level
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BOR_LEV` reader - BOR reset Level
pub type BOR_LEV_R = crate::FieldReader;
///Field `BOR_LEV` writer - BOR reset Level
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `nRST_STOP` reader - nRST_STOP
pub type N_RST_STOP_R = crate::BitReader;
///Field `nRST_STOP` writer - nRST_STOP
pub type N_RST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nRST_STDBY` reader - nRST_STDBY
pub type N_RST_STDBY_R = crate::BitReader;
///Field `nRST_STDBY` writer - nRST_STDBY
pub type N_RST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Independent watchdog selection

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDG_SW {
    ///0: Hardware independent watchdog
    Hardware = 0,
    ///1: Software independent watchdog
    Software = 1,
}
impl From<IWDG_SW> for bool {
    #[inline(always)]
    fn from(variant: IWDG_SW) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDG_SW` reader - Independent watchdog selection
pub type IWDG_SW_R = crate::BitReader<IWDG_SW>;
impl IWDG_SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IWDG_SW {
        match self.bits {
            false => IWDG_SW::Hardware,
            true => IWDG_SW::Software,
        }
    }
    ///Hardware independent watchdog
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == IWDG_SW::Hardware
    }
    ///Software independent watchdog
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == IWDG_SW::Software
    }
}
///Field `IWDG_SW` writer - Independent watchdog selection
pub type IWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG, IWDG_SW>;
impl<'a, REG> IWDG_SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Hardware independent watchdog
    #[inline(always)]
    pub fn hardware(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_SW::Hardware)
    }
    ///Software independent watchdog
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_SW::Software)
    }
}
/**Independent watchdog counter freeze in Stop mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDG_STOP {
    ///0: Independent watchdog counter is frozen in Stop mode
    Frozen = 0,
    ///1: Independent watchdog counter is running in Stop mode
    Running = 1,
}
impl From<IWDG_STOP> for bool {
    #[inline(always)]
    fn from(variant: IWDG_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_R = crate::BitReader<IWDG_STOP>;
impl IWDG_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IWDG_STOP {
        match self.bits {
            false => IWDG_STOP::Frozen,
            true => IWDG_STOP::Running,
        }
    }
    ///Independent watchdog counter is frozen in Stop mode
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == IWDG_STOP::Frozen
    }
    ///Independent watchdog counter is running in Stop mode
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == IWDG_STOP::Running
    }
}
///Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG, IWDG_STOP>;
impl<'a, REG> IWDG_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Independent watchdog counter is frozen in Stop mode
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_STOP::Frozen)
    }
    ///Independent watchdog counter is running in Stop mode
    #[inline(always)]
    pub fn running(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_STOP::Running)
    }
}
/**Independent watchdog counter freeze in Standby mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDG_STDBY {
    ///0: Independent watchdog counter is frozen in Standby mode
    Frozen = 0,
    ///1: Independent watchdog counter is running in Standby mode
    Running = 1,
}
impl From<IWDG_STDBY> for bool {
    #[inline(always)]
    fn from(variant: IWDG_STDBY) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDG_STDBY` reader - Independent watchdog counter freeze in Standby mode
pub type IWDG_STDBY_R = crate::BitReader<IWDG_STDBY>;
impl IWDG_STDBY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IWDG_STDBY {
        match self.bits {
            false => IWDG_STDBY::Frozen,
            true => IWDG_STDBY::Running,
        }
    }
    ///Independent watchdog counter is frozen in Standby mode
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == IWDG_STDBY::Frozen
    }
    ///Independent watchdog counter is running in Standby mode
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == IWDG_STDBY::Running
    }
}
///Field `IWDG_STDBY` writer - Independent watchdog counter freeze in Standby mode
pub type IWDG_STDBY_W<'a, REG> = crate::BitWriter<'a, REG, IWDG_STDBY>;
impl<'a, REG> IWDG_STDBY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Independent watchdog counter is frozen in Standby mode
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_STDBY::Frozen)
    }
    ///Independent watchdog counter is running in Standby mode
    #[inline(always)]
    pub fn running(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_STDBY::Running)
    }
}
/**Window watchdog selection

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDG_SW {
    ///0: Hardware window watchdog
    Hardware = 0,
    ///1: Software window watchdog
    Software = 1,
}
impl From<WWDG_SW> for bool {
    #[inline(always)]
    fn from(variant: WWDG_SW) -> Self {
        variant as u8 != 0
    }
}
///Field `WWDG_SW` reader - Window watchdog selection
pub type WWDG_SW_R = crate::BitReader<WWDG_SW>;
impl WWDG_SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WWDG_SW {
        match self.bits {
            false => WWDG_SW::Hardware,
            true => WWDG_SW::Software,
        }
    }
    ///Hardware window watchdog
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == WWDG_SW::Hardware
    }
    ///Software window watchdog
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == WWDG_SW::Software
    }
}
///Field `WWDG_SW` writer - Window watchdog selection
pub type WWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG, WWDG_SW>;
impl<'a, REG> WWDG_SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Hardware window watchdog
    #[inline(always)]
    pub fn hardware(self) -> &'a mut crate::W<REG> {
        self.variant(WWDG_SW::Hardware)
    }
    ///Software window watchdog
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(WWDG_SW::Software)
    }
}
/**Dual-bank boot

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFB2 {
    ///0: Dual-bank boot disabled
    Disabled = 0,
    ///1: Dual-bank boot enabled
    Enabled = 1,
}
impl From<BFB2> for bool {
    #[inline(always)]
    fn from(variant: BFB2) -> Self {
        variant as u8 != 0
    }
}
///Field `BFB2` reader - Dual-bank boot
pub type BFB2_R = crate::BitReader<BFB2>;
impl BFB2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BFB2 {
        match self.bits {
            false => BFB2::Disabled,
            true => BFB2::Enabled,
        }
    }
    ///Dual-bank boot disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BFB2::Disabled
    }
    ///Dual-bank boot enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BFB2::Enabled
    }
}
///Field `BFB2` writer - Dual-bank boot
pub type BFB2_W<'a, REG> = crate::BitWriter<'a, REG, BFB2>;
impl<'a, REG> BFB2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Dual-bank boot disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BFB2::Disabled)
    }
    ///Dual-bank boot enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BFB2::Enabled)
    }
}
/**

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DB1M {
    ///0: Single Flash contiguous address in Bank 1
    SingleBank = 0,
    ///1: Dual-bank Flash with contiguous addresses
    DualBank = 1,
}
impl From<DB1M> for bool {
    #[inline(always)]
    fn from(variant: DB1M) -> Self {
        variant as u8 != 0
    }
}
///Field `DB1M` reader -
pub type DB1M_R = crate::BitReader<DB1M>;
impl DB1M_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DB1M {
        match self.bits {
            false => DB1M::SingleBank,
            true => DB1M::DualBank,
        }
    }
    ///Single Flash contiguous address in Bank 1
    #[inline(always)]
    pub fn is_single_bank(&self) -> bool {
        *self == DB1M::SingleBank
    }
    ///Dual-bank Flash with contiguous addresses
    #[inline(always)]
    pub fn is_dual_bank(&self) -> bool {
        *self == DB1M::DualBank
    }
}
///Field `DB1M` writer -
pub type DB1M_W<'a, REG> = crate::BitWriter<'a, REG, DB1M>;
impl<'a, REG> DB1M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single Flash contiguous address in Bank 1
    #[inline(always)]
    pub fn single_bank(self) -> &'a mut crate::W<REG> {
        self.variant(DB1M::SingleBank)
    }
    ///Dual-bank Flash with contiguous addresses
    #[inline(always)]
    pub fn dual_bank(self) -> &'a mut crate::W<REG> {
        self.variant(DB1M::DualBank)
    }
}
/**

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBANK {
    ///0: Single-bank mode with 128 bits data read width
    SingleBankMode = 0,
    ///1: Dual-bank mode with 64 bits data
    DualBankMode = 1,
}
impl From<DBANK> for bool {
    #[inline(always)]
    fn from(variant: DBANK) -> Self {
        variant as u8 != 0
    }
}
///Field `DBANK` reader -
pub type DBANK_R = crate::BitReader<DBANK>;
impl DBANK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBANK {
        match self.bits {
            false => DBANK::SingleBankMode,
            true => DBANK::DualBankMode,
        }
    }
    ///Single-bank mode with 128 bits data read width
    #[inline(always)]
    pub fn is_single_bank_mode(&self) -> bool {
        *self == DBANK::SingleBankMode
    }
    ///Dual-bank mode with 64 bits data
    #[inline(always)]
    pub fn is_dual_bank_mode(&self) -> bool {
        *self == DBANK::DualBankMode
    }
}
///Field `DBANK` writer -
pub type DBANK_W<'a, REG> = crate::BitWriter<'a, REG, DBANK>;
impl<'a, REG> DBANK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single-bank mode with 128 bits data read width
    #[inline(always)]
    pub fn single_bank_mode(self) -> &'a mut crate::W<REG> {
        self.variant(DBANK::SingleBankMode)
    }
    ///Dual-bank mode with 64 bits data
    #[inline(always)]
    pub fn dual_bank_mode(self) -> &'a mut crate::W<REG> {
        self.variant(DBANK::DualBankMode)
    }
}
///Field `nBOOT1` reader - Boot configuration
pub type N_BOOT1_R = crate::BitReader;
///Field `nBOOT1` writer - Boot configuration
pub type N_BOOT1_W<'a, REG> = crate::BitWriter<'a, REG>;
/**SRAM2 parity check enable

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2_PE {
    ///0: SRAM2 parity check enabled
    Enabled = 0,
    ///1: SRAM2 parity check disabled
    Disabled = 1,
}
impl From<SRAM2_PE> for bool {
    #[inline(always)]
    fn from(variant: SRAM2_PE) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM2_PE` reader - SRAM2 parity check enable
pub type SRAM2_PE_R = crate::BitReader<SRAM2_PE>;
impl SRAM2_PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAM2_PE {
        match self.bits {
            false => SRAM2_PE::Enabled,
            true => SRAM2_PE::Disabled,
        }
    }
    ///SRAM2 parity check enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM2_PE::Enabled
    }
    ///SRAM2 parity check disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM2_PE::Disabled
    }
}
///Field `SRAM2_PE` writer - SRAM2 parity check enable
pub type SRAM2_PE_W<'a, REG> = crate::BitWriter<'a, REG, SRAM2_PE>;
impl<'a, REG> SRAM2_PE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM2 parity check enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2_PE::Enabled)
    }
    ///SRAM2 parity check disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2_PE::Disabled)
    }
}
/**SRAM2 Erase when system reset

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2_RST {
    ///0: SRAM2 erased when a system reset occurs
    Enabled = 0,
    ///1: SRAM2 is not erased when a system reset occurs
    Disabled = 1,
}
impl From<SRAM2_RST> for bool {
    #[inline(always)]
    fn from(variant: SRAM2_RST) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM2_RST` reader - SRAM2 Erase when system reset
pub type SRAM2_RST_R = crate::BitReader<SRAM2_RST>;
impl SRAM2_RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAM2_RST {
        match self.bits {
            false => SRAM2_RST::Enabled,
            true => SRAM2_RST::Disabled,
        }
    }
    ///SRAM2 erased when a system reset occurs
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM2_RST::Enabled
    }
    ///SRAM2 is not erased when a system reset occurs
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM2_RST::Disabled
    }
}
///Field `SRAM2_RST` writer - SRAM2 Erase when system reset
pub type SRAM2_RST_W<'a, REG> = crate::BitWriter<'a, REG, SRAM2_RST>;
impl<'a, REG> SRAM2_RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM2 erased when a system reset occurs
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2_RST::Enabled)
    }
    ///SRAM2 is not erased when a system reset occurs
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2_RST::Disabled)
    }
}
/**Software BOOT0

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_SWBOOT0 {
    ///0: BOOT0 taken from the option bit nBOOT0
    OptionBit = 0,
    ///1: BOOT0 taken from PH3/BOOT0 pin
    Pin = 1,
}
impl From<N_SWBOOT0> for bool {
    #[inline(always)]
    fn from(variant: N_SWBOOT0) -> Self {
        variant as u8 != 0
    }
}
///Field `nSWBOOT0` reader - Software BOOT0
pub type N_SWBOOT0_R = crate::BitReader<N_SWBOOT0>;
impl N_SWBOOT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> N_SWBOOT0 {
        match self.bits {
            false => N_SWBOOT0::OptionBit,
            true => N_SWBOOT0::Pin,
        }
    }
    ///BOOT0 taken from the option bit nBOOT0
    #[inline(always)]
    pub fn is_option_bit(&self) -> bool {
        *self == N_SWBOOT0::OptionBit
    }
    ///BOOT0 taken from PH3/BOOT0 pin
    #[inline(always)]
    pub fn is_pin(&self) -> bool {
        *self == N_SWBOOT0::Pin
    }
}
///Field `nSWBOOT0` writer - Software BOOT0
pub type N_SWBOOT0_W<'a, REG> = crate::BitWriter<'a, REG, N_SWBOOT0>;
impl<'a, REG> N_SWBOOT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///BOOT0 taken from the option bit nBOOT0
    #[inline(always)]
    pub fn option_bit(self) -> &'a mut crate::W<REG> {
        self.variant(N_SWBOOT0::OptionBit)
    }
    ///BOOT0 taken from PH3/BOOT0 pin
    #[inline(always)]
    pub fn pin(self) -> &'a mut crate::W<REG> {
        self.variant(N_SWBOOT0::Pin)
    }
}
/**nBOOT0 option bit

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_BOOT0 {
    ///0: nBOOT0 = 0
    Disabled = 0,
    ///1: nBOOT0 = 1
    Enabled = 1,
}
impl From<N_BOOT0> for bool {
    #[inline(always)]
    fn from(variant: N_BOOT0) -> Self {
        variant as u8 != 0
    }
}
///Field `nBOOT0` reader - nBOOT0 option bit
pub type N_BOOT0_R = crate::BitReader<N_BOOT0>;
impl N_BOOT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> N_BOOT0 {
        match self.bits {
            false => N_BOOT0::Disabled,
            true => N_BOOT0::Enabled,
        }
    }
    ///nBOOT0 = 0
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == N_BOOT0::Disabled
    }
    ///nBOOT0 = 1
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == N_BOOT0::Enabled
    }
}
///Field `nBOOT0` writer - nBOOT0 option bit
pub type N_BOOT0_W<'a, REG> = crate::BitWriter<'a, REG, N_BOOT0>;
impl<'a, REG> N_BOOT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///nBOOT0 = 0
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(N_BOOT0::Disabled)
    }
    ///nBOOT0 = 1
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(N_BOOT0::Enabled)
    }
}
impl R {
    ///Bits 0:7 - Read protection level
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:10 - BOR reset Level
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 12 - nRST_STOP
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - nRST_STDBY
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Dual-bank boot
    #[inline(always)]
    pub fn bfb2(&self) -> BFB2_R {
        BFB2_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21
    #[inline(always)]
    pub fn db1m(&self) -> DB1M_R {
        DB1M_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22
    #[inline(always)]
    pub fn dbank(&self) -> DBANK_R {
        DBANK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Boot configuration
    #[inline(always)]
    pub fn n_boot1(&self) -> N_BOOT1_R {
        N_BOOT1_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SRAM2 parity check enable
    #[inline(always)]
    pub fn sram2_pe(&self) -> SRAM2_PE_R {
        SRAM2_PE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SRAM2 Erase when system reset
    #[inline(always)]
    pub fn sram2_rst(&self) -> SRAM2_RST_R {
        SRAM2_RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Software BOOT0
    #[inline(always)]
    pub fn n_swboot0(&self) -> N_SWBOOT0_R {
        N_SWBOOT0_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - nBOOT0 option bit
    #[inline(always)]
    pub fn n_boot0(&self) -> N_BOOT0_R {
        N_BOOT0_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTR")
            .field("rdp", &self.rdp())
            .field("bor_lev", &self.bor_lev())
            .field("n_rst_stop", &self.n_rst_stop())
            .field("n_rst_stdby", &self.n_rst_stdby())
            .field("iwdg_sw", &self.iwdg_sw())
            .field("iwdg_stop", &self.iwdg_stop())
            .field("iwdg_stdby", &self.iwdg_stdby())
            .field("wwdg_sw", &self.wwdg_sw())
            .field("bfb2", &self.bfb2())
            .field("n_boot1", &self.n_boot1())
            .field("sram2_pe", &self.sram2_pe())
            .field("sram2_rst", &self.sram2_rst())
            .field("dbank", &self.dbank())
            .field("db1m", &self.db1m())
            .field("n_swboot0", &self.n_swboot0())
            .field("n_boot0", &self.n_boot0())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Read protection level
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<OPTRrs> {
        RDP_W::new(self, 0)
    }
    ///Bits 8:10 - BOR reset Level
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<OPTRrs> {
        BOR_LEV_W::new(self, 8)
    }
    ///Bit 12 - nRST_STOP
    #[inline(always)]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<OPTRrs> {
        N_RST_STOP_W::new(self, 12)
    }
    ///Bit 13 - nRST_STDBY
    #[inline(always)]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<OPTRrs> {
        N_RST_STDBY_W::new(self, 13)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<OPTRrs> {
        IWDG_SW_W::new(self, 16)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<OPTRrs> {
        IWDG_STOP_W::new(self, 17)
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W<OPTRrs> {
        IWDG_STDBY_W::new(self, 18)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<OPTRrs> {
        WWDG_SW_W::new(self, 19)
    }
    ///Bit 20 - Dual-bank boot
    #[inline(always)]
    pub fn bfb2(&mut self) -> BFB2_W<OPTRrs> {
        BFB2_W::new(self, 20)
    }
    ///Bit 21
    #[inline(always)]
    pub fn db1m(&mut self) -> DB1M_W<OPTRrs> {
        DB1M_W::new(self, 21)
    }
    ///Bit 22
    #[inline(always)]
    pub fn dbank(&mut self) -> DBANK_W<OPTRrs> {
        DBANK_W::new(self, 22)
    }
    ///Bit 23 - Boot configuration
    #[inline(always)]
    pub fn n_boot1(&mut self) -> N_BOOT1_W<OPTRrs> {
        N_BOOT1_W::new(self, 23)
    }
    ///Bit 24 - SRAM2 parity check enable
    #[inline(always)]
    pub fn sram2_pe(&mut self) -> SRAM2_PE_W<OPTRrs> {
        SRAM2_PE_W::new(self, 24)
    }
    ///Bit 25 - SRAM2 Erase when system reset
    #[inline(always)]
    pub fn sram2_rst(&mut self) -> SRAM2_RST_W<OPTRrs> {
        SRAM2_RST_W::new(self, 25)
    }
    ///Bit 26 - Software BOOT0
    #[inline(always)]
    pub fn n_swboot0(&mut self) -> N_SWBOOT0_W<OPTRrs> {
        N_SWBOOT0_W::new(self, 26)
    }
    ///Bit 27 - nBOOT0 option bit
    #[inline(always)]
    pub fn n_boot0(&mut self) -> N_BOOT0_W<OPTRrs> {
        N_BOOT0_W::new(self, 27)
    }
}
/**Flash option register

You can [`read`](crate::Reg::read) this register and get [`optr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#FLASH:OPTR)*/
pub struct OPTRrs;
impl crate::RegisterSpec for OPTRrs {
    type Ux = u32;
}
///`read()` method returns [`optr::R`](R) reader structure
impl crate::Readable for OPTRrs {}
///`write(|w| ..)` method takes [`optr::W`](W) writer structure
impl crate::Writable for OPTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTR to value 0xffef_f8aa
impl crate::Resettable for OPTRrs {
    const RESET_VALUE: u32 = 0xffef_f8aa;
}
