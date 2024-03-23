#[doc = "Register `BCR4` reader"]
pub type R = crate::R<BCR4rs>;
#[doc = "Register `BCR4` writer"]
pub type W = crate::W<BCR4rs>;
#[doc = "MBKEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MBKEN {
    #[doc = "0: Corresponding memory bank is disabled"]
    Disabled = 0,
    #[doc = "1: Corresponding memory bank is enabled"]
    Enabled = 1,
}
impl From<MBKEN> for bool {
    #[inline(always)]
    fn from(variant: MBKEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBKEN` reader - MBKEN"]
pub type MBKEN_R = crate::BitReader<MBKEN>;
impl MBKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MBKEN {
        match self.bits {
            false => MBKEN::Disabled,
            true => MBKEN::Enabled,
        }
    }
    #[doc = "Corresponding memory bank is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MBKEN::Disabled
    }
    #[doc = "Corresponding memory bank is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MBKEN::Enabled
    }
}
#[doc = "Field `MBKEN` writer - MBKEN"]
pub type MBKEN_W<'a, REG> = crate::BitWriter<'a, REG, MBKEN>;
impl<'a, REG> MBKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding memory bank is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MBKEN::Disabled)
    }
    #[doc = "Corresponding memory bank is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MBKEN::Enabled)
    }
}
#[doc = "MUXEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUXEN {
    #[doc = "0: Address/Data non-multiplexed"]
    Disabled = 0,
    #[doc = "1: Address/Data multiplexed on databus"]
    Enabled = 1,
}
impl From<MUXEN> for bool {
    #[inline(always)]
    fn from(variant: MUXEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUXEN` reader - MUXEN"]
pub type MUXEN_R = crate::BitReader<MUXEN>;
impl MUXEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MUXEN {
        match self.bits {
            false => MUXEN::Disabled,
            true => MUXEN::Enabled,
        }
    }
    #[doc = "Address/Data non-multiplexed"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MUXEN::Disabled
    }
    #[doc = "Address/Data multiplexed on databus"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MUXEN::Enabled
    }
}
#[doc = "Field `MUXEN` writer - MUXEN"]
pub type MUXEN_W<'a, REG> = crate::BitWriter<'a, REG, MUXEN>;
impl<'a, REG> MUXEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Address/Data non-multiplexed"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MUXEN::Disabled)
    }
    #[doc = "Address/Data multiplexed on databus"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MUXEN::Enabled)
    }
}
#[doc = "MTYP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MTYP {
    #[doc = "0: SRAM memory type"]
    Sram = 0,
    #[doc = "1: PSRAM (CRAM) memory type"]
    Psram = 1,
    #[doc = "2: NOR Flash/OneNAND Flash"]
    Flash = 2,
}
impl From<MTYP> for u8 {
    #[inline(always)]
    fn from(variant: MTYP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MTYP {
    type Ux = u8;
}
#[doc = "Field `MTYP` reader - MTYP"]
pub type MTYP_R = crate::FieldReader<MTYP>;
impl MTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MTYP> {
        match self.bits {
            0 => Some(MTYP::Sram),
            1 => Some(MTYP::Psram),
            2 => Some(MTYP::Flash),
            _ => None,
        }
    }
    #[doc = "SRAM memory type"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == MTYP::Sram
    }
    #[doc = "PSRAM (CRAM) memory type"]
    #[inline(always)]
    pub fn is_psram(&self) -> bool {
        *self == MTYP::Psram
    }
    #[doc = "NOR Flash/OneNAND Flash"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == MTYP::Flash
    }
}
#[doc = "Field `MTYP` writer - MTYP"]
pub type MTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MTYP>;
impl<'a, REG> MTYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SRAM memory type"]
    #[inline(always)]
    pub fn sram(self) -> &'a mut crate::W<REG> {
        self.variant(MTYP::Sram)
    }
    #[doc = "PSRAM (CRAM) memory type"]
    #[inline(always)]
    pub fn psram(self) -> &'a mut crate::W<REG> {
        self.variant(MTYP::Psram)
    }
    #[doc = "NOR Flash/OneNAND Flash"]
    #[inline(always)]
    pub fn flash(self) -> &'a mut crate::W<REG> {
        self.variant(MTYP::Flash)
    }
}
#[doc = "MWID\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MWID {
    #[doc = "0: Memory data bus width 8 bits"]
    Bits8 = 0,
    #[doc = "1: Memory data bus width 16 bits"]
    Bits16 = 1,
    #[doc = "2: Memory data bus width 32 bits"]
    Bits32 = 2,
}
impl From<MWID> for u8 {
    #[inline(always)]
    fn from(variant: MWID) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MWID {
    type Ux = u8;
}
#[doc = "Field `MWID` reader - MWID"]
pub type MWID_R = crate::FieldReader<MWID>;
impl MWID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MWID> {
        match self.bits {
            0 => Some(MWID::Bits8),
            1 => Some(MWID::Bits16),
            2 => Some(MWID::Bits32),
            _ => None,
        }
    }
    #[doc = "Memory data bus width 8 bits"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == MWID::Bits8
    }
    #[doc = "Memory data bus width 16 bits"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == MWID::Bits16
    }
    #[doc = "Memory data bus width 32 bits"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == MWID::Bits32
    }
}
#[doc = "Field `MWID` writer - MWID"]
pub type MWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MWID>;
impl<'a, REG> MWID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Memory data bus width 8 bits"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(MWID::Bits8)
    }
    #[doc = "Memory data bus width 16 bits"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(MWID::Bits16)
    }
    #[doc = "Memory data bus width 32 bits"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(MWID::Bits32)
    }
}
#[doc = "FACCEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FACCEN {
    #[doc = "0: Corresponding NOR Flash memory access is disabled"]
    Disabled = 0,
    #[doc = "1: Corresponding NOR Flash memory access is enabled"]
    Enabled = 1,
}
impl From<FACCEN> for bool {
    #[inline(always)]
    fn from(variant: FACCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FACCEN` reader - FACCEN"]
pub type FACCEN_R = crate::BitReader<FACCEN>;
impl FACCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FACCEN {
        match self.bits {
            false => FACCEN::Disabled,
            true => FACCEN::Enabled,
        }
    }
    #[doc = "Corresponding NOR Flash memory access is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FACCEN::Disabled
    }
    #[doc = "Corresponding NOR Flash memory access is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FACCEN::Enabled
    }
}
#[doc = "Field `FACCEN` writer - FACCEN"]
pub type FACCEN_W<'a, REG> = crate::BitWriter<'a, REG, FACCEN>;
impl<'a, REG> FACCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding NOR Flash memory access is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FACCEN::Disabled)
    }
    #[doc = "Corresponding NOR Flash memory access is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FACCEN::Enabled)
    }
}
#[doc = "BURSTEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BURSTEN {
    #[doc = "0: Burst mode disabled"]
    Disabled = 0,
    #[doc = "1: Burst mode enabled"]
    Enabled = 1,
}
impl From<BURSTEN> for bool {
    #[inline(always)]
    fn from(variant: BURSTEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BURSTEN` reader - BURSTEN"]
pub type BURSTEN_R = crate::BitReader<BURSTEN>;
impl BURSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BURSTEN {
        match self.bits {
            false => BURSTEN::Disabled,
            true => BURSTEN::Enabled,
        }
    }
    #[doc = "Burst mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BURSTEN::Disabled
    }
    #[doc = "Burst mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BURSTEN::Enabled
    }
}
#[doc = "Field `BURSTEN` writer - BURSTEN"]
pub type BURSTEN_W<'a, REG> = crate::BitWriter<'a, REG, BURSTEN>;
impl<'a, REG> BURSTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Burst mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BURSTEN::Disabled)
    }
    #[doc = "Burst mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BURSTEN::Enabled)
    }
}
#[doc = "WAITPOL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITPOL {
    #[doc = "0: NWAIT active low"]
    ActiveLow = 0,
    #[doc = "1: NWAIT active high"]
    ActiveHigh = 1,
}
impl From<WAITPOL> for bool {
    #[inline(always)]
    fn from(variant: WAITPOL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAITPOL` reader - WAITPOL"]
pub type WAITPOL_R = crate::BitReader<WAITPOL>;
impl WAITPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAITPOL {
        match self.bits {
            false => WAITPOL::ActiveLow,
            true => WAITPOL::ActiveHigh,
        }
    }
    #[doc = "NWAIT active low"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == WAITPOL::ActiveLow
    }
    #[doc = "NWAIT active high"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == WAITPOL::ActiveHigh
    }
}
#[doc = "Field `WAITPOL` writer - WAITPOL"]
pub type WAITPOL_W<'a, REG> = crate::BitWriter<'a, REG, WAITPOL>;
impl<'a, REG> WAITPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NWAIT active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(WAITPOL::ActiveLow)
    }
    #[doc = "NWAIT active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(WAITPOL::ActiveHigh)
    }
}
#[doc = "Field `WRAPMOD` reader - WRAPMOD"]
pub type WRAPMOD_R = crate::BitReader;
#[doc = "Field `WRAPMOD` writer - WRAPMOD"]
pub type WRAPMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "WAITCFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITCFG {
    #[doc = "0: NWAIT signal is active one data cycle before wait state"]
    BeforeWaitState = 0,
    #[doc = "1: NWAIT signal is active during wait state"]
    DuringWaitState = 1,
}
impl From<WAITCFG> for bool {
    #[inline(always)]
    fn from(variant: WAITCFG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAITCFG` reader - WAITCFG"]
pub type WAITCFG_R = crate::BitReader<WAITCFG>;
impl WAITCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAITCFG {
        match self.bits {
            false => WAITCFG::BeforeWaitState,
            true => WAITCFG::DuringWaitState,
        }
    }
    #[doc = "NWAIT signal is active one data cycle before wait state"]
    #[inline(always)]
    pub fn is_before_wait_state(&self) -> bool {
        *self == WAITCFG::BeforeWaitState
    }
    #[doc = "NWAIT signal is active during wait state"]
    #[inline(always)]
    pub fn is_during_wait_state(&self) -> bool {
        *self == WAITCFG::DuringWaitState
    }
}
#[doc = "Field `WAITCFG` writer - WAITCFG"]
pub type WAITCFG_W<'a, REG> = crate::BitWriter<'a, REG, WAITCFG>;
impl<'a, REG> WAITCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NWAIT signal is active one data cycle before wait state"]
    #[inline(always)]
    pub fn before_wait_state(self) -> &'a mut crate::W<REG> {
        self.variant(WAITCFG::BeforeWaitState)
    }
    #[doc = "NWAIT signal is active during wait state"]
    #[inline(always)]
    pub fn during_wait_state(self) -> &'a mut crate::W<REG> {
        self.variant(WAITCFG::DuringWaitState)
    }
}
#[doc = "WREN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WREN {
    #[doc = "0: Write operations disabled for the bank by the FMC"]
    Disabled = 0,
    #[doc = "1: Write operations enabled for the bank by the FMC"]
    Enabled = 1,
}
impl From<WREN> for bool {
    #[inline(always)]
    fn from(variant: WREN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WREN` reader - WREN"]
pub type WREN_R = crate::BitReader<WREN>;
impl WREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WREN {
        match self.bits {
            false => WREN::Disabled,
            true => WREN::Enabled,
        }
    }
    #[doc = "Write operations disabled for the bank by the FMC"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WREN::Disabled
    }
    #[doc = "Write operations enabled for the bank by the FMC"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WREN::Enabled
    }
}
#[doc = "Field `WREN` writer - WREN"]
pub type WREN_W<'a, REG> = crate::BitWriter<'a, REG, WREN>;
impl<'a, REG> WREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write operations disabled for the bank by the FMC"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WREN::Disabled)
    }
    #[doc = "Write operations enabled for the bank by the FMC"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WREN::Enabled)
    }
}
#[doc = "WAITEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITEN {
    #[doc = "0: Values inside the FMC_BWTR are taken into account"]
    Disabled = 0,
    #[doc = "1: NWAIT signal enabled"]
    Enabled = 1,
}
impl From<WAITEN> for bool {
    #[inline(always)]
    fn from(variant: WAITEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAITEN` reader - WAITEN"]
pub type WAITEN_R = crate::BitReader<WAITEN>;
impl WAITEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAITEN {
        match self.bits {
            false => WAITEN::Disabled,
            true => WAITEN::Enabled,
        }
    }
    #[doc = "Values inside the FMC_BWTR are taken into account"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAITEN::Disabled
    }
    #[doc = "NWAIT signal enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAITEN::Enabled
    }
}
#[doc = "Field `WAITEN` writer - WAITEN"]
pub type WAITEN_W<'a, REG> = crate::BitWriter<'a, REG, WAITEN>;
impl<'a, REG> WAITEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Values inside the FMC_BWTR are taken into account"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAITEN::Disabled)
    }
    #[doc = "NWAIT signal enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAITEN::Enabled)
    }
}
#[doc = "EXTMOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTMOD {
    #[doc = "0: Values inside the FMC_BWTR are not taken into account"]
    Disabled = 0,
    #[doc = "1: Values inside the FMC_BWTR are taken into account"]
    Enabled = 1,
}
impl From<EXTMOD> for bool {
    #[inline(always)]
    fn from(variant: EXTMOD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTMOD` reader - EXTMOD"]
pub type EXTMOD_R = crate::BitReader<EXTMOD>;
impl EXTMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTMOD {
        match self.bits {
            false => EXTMOD::Disabled,
            true => EXTMOD::Enabled,
        }
    }
    #[doc = "Values inside the FMC_BWTR are not taken into account"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTMOD::Disabled
    }
    #[doc = "Values inside the FMC_BWTR are taken into account"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXTMOD::Enabled
    }
}
#[doc = "Field `EXTMOD` writer - EXTMOD"]
pub type EXTMOD_W<'a, REG> = crate::BitWriter<'a, REG, EXTMOD>;
impl<'a, REG> EXTMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Values inside the FMC_BWTR are not taken into account"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTMOD::Disabled)
    }
    #[doc = "Values inside the FMC_BWTR are taken into account"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTMOD::Enabled)
    }
}
#[doc = "ASYNCWAIT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASYNCWAIT {
    #[doc = "0: Wait signal not used in asynchronous mode"]
    Disabled = 0,
    #[doc = "1: Wait signal used even in asynchronous mode"]
    Enabled = 1,
}
impl From<ASYNCWAIT> for bool {
    #[inline(always)]
    fn from(variant: ASYNCWAIT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASYNCWAIT` reader - ASYNCWAIT"]
pub type ASYNCWAIT_R = crate::BitReader<ASYNCWAIT>;
impl ASYNCWAIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASYNCWAIT {
        match self.bits {
            false => ASYNCWAIT::Disabled,
            true => ASYNCWAIT::Enabled,
        }
    }
    #[doc = "Wait signal not used in asynchronous mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ASYNCWAIT::Disabled
    }
    #[doc = "Wait signal used even in asynchronous mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ASYNCWAIT::Enabled
    }
}
#[doc = "Field `ASYNCWAIT` writer - ASYNCWAIT"]
pub type ASYNCWAIT_W<'a, REG> = crate::BitWriter<'a, REG, ASYNCWAIT>;
impl<'a, REG> ASYNCWAIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wait signal not used in asynchronous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCWAIT::Disabled)
    }
    #[doc = "Wait signal used even in asynchronous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCWAIT::Enabled)
    }
}
#[doc = "CRAM page size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CPSIZE {
    #[doc = "0: No burst split when crossing page boundary"]
    NoBurstSplit = 0,
    #[doc = "1: 128 bytes CRAM page size"]
    Bytes128 = 1,
    #[doc = "2: 256 bytes CRAM page size"]
    Bytes256 = 2,
    #[doc = "3: 512 bytes CRAM page size"]
    Bytes512 = 3,
    #[doc = "4: 1024 bytes CRAM page size"]
    Bytes1024 = 4,
}
impl From<CPSIZE> for u8 {
    #[inline(always)]
    fn from(variant: CPSIZE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CPSIZE {
    type Ux = u8;
}
#[doc = "Field `CPSIZE` reader - CRAM page size"]
pub type CPSIZE_R = crate::FieldReader<CPSIZE>;
impl CPSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CPSIZE> {
        match self.bits {
            0 => Some(CPSIZE::NoBurstSplit),
            1 => Some(CPSIZE::Bytes128),
            2 => Some(CPSIZE::Bytes256),
            3 => Some(CPSIZE::Bytes512),
            4 => Some(CPSIZE::Bytes1024),
            _ => None,
        }
    }
    #[doc = "No burst split when crossing page boundary"]
    #[inline(always)]
    pub fn is_no_burst_split(&self) -> bool {
        *self == CPSIZE::NoBurstSplit
    }
    #[doc = "128 bytes CRAM page size"]
    #[inline(always)]
    pub fn is_bytes128(&self) -> bool {
        *self == CPSIZE::Bytes128
    }
    #[doc = "256 bytes CRAM page size"]
    #[inline(always)]
    pub fn is_bytes256(&self) -> bool {
        *self == CPSIZE::Bytes256
    }
    #[doc = "512 bytes CRAM page size"]
    #[inline(always)]
    pub fn is_bytes512(&self) -> bool {
        *self == CPSIZE::Bytes512
    }
    #[doc = "1024 bytes CRAM page size"]
    #[inline(always)]
    pub fn is_bytes1024(&self) -> bool {
        *self == CPSIZE::Bytes1024
    }
}
#[doc = "Field `CPSIZE` writer - CRAM page size"]
pub type CPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CPSIZE>;
impl<'a, REG> CPSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No burst split when crossing page boundary"]
    #[inline(always)]
    pub fn no_burst_split(self) -> &'a mut crate::W<REG> {
        self.variant(CPSIZE::NoBurstSplit)
    }
    #[doc = "128 bytes CRAM page size"]
    #[inline(always)]
    pub fn bytes128(self) -> &'a mut crate::W<REG> {
        self.variant(CPSIZE::Bytes128)
    }
    #[doc = "256 bytes CRAM page size"]
    #[inline(always)]
    pub fn bytes256(self) -> &'a mut crate::W<REG> {
        self.variant(CPSIZE::Bytes256)
    }
    #[doc = "512 bytes CRAM page size"]
    #[inline(always)]
    pub fn bytes512(self) -> &'a mut crate::W<REG> {
        self.variant(CPSIZE::Bytes512)
    }
    #[doc = "1024 bytes CRAM page size"]
    #[inline(always)]
    pub fn bytes1024(self) -> &'a mut crate::W<REG> {
        self.variant(CPSIZE::Bytes1024)
    }
}
#[doc = "CBURSTRW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBURSTRW {
    #[doc = "0: Write operations are always performed in asynchronous mode"]
    Disabled = 0,
    #[doc = "1: Write operations are performed in synchronous mode"]
    Enabled = 1,
}
impl From<CBURSTRW> for bool {
    #[inline(always)]
    fn from(variant: CBURSTRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBURSTRW` reader - CBURSTRW"]
pub type CBURSTRW_R = crate::BitReader<CBURSTRW>;
impl CBURSTRW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CBURSTRW {
        match self.bits {
            false => CBURSTRW::Disabled,
            true => CBURSTRW::Enabled,
        }
    }
    #[doc = "Write operations are always performed in asynchronous mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CBURSTRW::Disabled
    }
    #[doc = "Write operations are performed in synchronous mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CBURSTRW::Enabled
    }
}
#[doc = "Field `CBURSTRW` writer - CBURSTRW"]
pub type CBURSTRW_W<'a, REG> = crate::BitWriter<'a, REG, CBURSTRW>;
impl<'a, REG> CBURSTRW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write operations are always performed in asynchronous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CBURSTRW::Disabled)
    }
    #[doc = "Write operations are performed in synchronous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CBURSTRW::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - MBKEN"]
    #[inline(always)]
    pub fn mbken(&self) -> MBKEN_R {
        MBKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MUXEN"]
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - MTYP"]
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - MWID"]
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - FACCEN"]
    #[inline(always)]
    pub fn faccen(&self) -> FACCEN_R {
        FACCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - BURSTEN"]
    #[inline(always)]
    pub fn bursten(&self) -> BURSTEN_R {
        BURSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WAITPOL"]
    #[inline(always)]
    pub fn waitpol(&self) -> WAITPOL_R {
        WAITPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - WRAPMOD"]
    #[inline(always)]
    pub fn wrapmod(&self) -> WRAPMOD_R {
        WRAPMOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WAITCFG"]
    #[inline(always)]
    pub fn waitcfg(&self) -> WAITCFG_R {
        WAITCFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - WREN"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - WAITEN"]
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EXTMOD"]
    #[inline(always)]
    pub fn extmod(&self) -> EXTMOD_R {
        EXTMOD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ASYNCWAIT"]
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - CRAM page size"]
    #[inline(always)]
    pub fn cpsize(&self) -> CPSIZE_R {
        CPSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - CBURSTRW"]
    #[inline(always)]
    pub fn cburstrw(&self) -> CBURSTRW_R {
        CBURSTRW_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MBKEN"]
    #[inline(always)]
    #[must_use]
    pub fn mbken(&mut self) -> MBKEN_W<BCR4rs> {
        MBKEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - MUXEN"]
    #[inline(always)]
    #[must_use]
    pub fn muxen(&mut self) -> MUXEN_W<BCR4rs> {
        MUXEN_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - MTYP"]
    #[inline(always)]
    #[must_use]
    pub fn mtyp(&mut self) -> MTYP_W<BCR4rs> {
        MTYP_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - MWID"]
    #[inline(always)]
    #[must_use]
    pub fn mwid(&mut self) -> MWID_W<BCR4rs> {
        MWID_W::new(self, 4)
    }
    #[doc = "Bit 6 - FACCEN"]
    #[inline(always)]
    #[must_use]
    pub fn faccen(&mut self) -> FACCEN_W<BCR4rs> {
        FACCEN_W::new(self, 6)
    }
    #[doc = "Bit 8 - BURSTEN"]
    #[inline(always)]
    #[must_use]
    pub fn bursten(&mut self) -> BURSTEN_W<BCR4rs> {
        BURSTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - WAITPOL"]
    #[inline(always)]
    #[must_use]
    pub fn waitpol(&mut self) -> WAITPOL_W<BCR4rs> {
        WAITPOL_W::new(self, 9)
    }
    #[doc = "Bit 10 - WRAPMOD"]
    #[inline(always)]
    #[must_use]
    pub fn wrapmod(&mut self) -> WRAPMOD_W<BCR4rs> {
        WRAPMOD_W::new(self, 10)
    }
    #[doc = "Bit 11 - WAITCFG"]
    #[inline(always)]
    #[must_use]
    pub fn waitcfg(&mut self) -> WAITCFG_W<BCR4rs> {
        WAITCFG_W::new(self, 11)
    }
    #[doc = "Bit 12 - WREN"]
    #[inline(always)]
    #[must_use]
    pub fn wren(&mut self) -> WREN_W<BCR4rs> {
        WREN_W::new(self, 12)
    }
    #[doc = "Bit 13 - WAITEN"]
    #[inline(always)]
    #[must_use]
    pub fn waiten(&mut self) -> WAITEN_W<BCR4rs> {
        WAITEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - EXTMOD"]
    #[inline(always)]
    #[must_use]
    pub fn extmod(&mut self) -> EXTMOD_W<BCR4rs> {
        EXTMOD_W::new(self, 14)
    }
    #[doc = "Bit 15 - ASYNCWAIT"]
    #[inline(always)]
    #[must_use]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W<BCR4rs> {
        ASYNCWAIT_W::new(self, 15)
    }
    #[doc = "Bits 16:18 - CRAM page size"]
    #[inline(always)]
    #[must_use]
    pub fn cpsize(&mut self) -> CPSIZE_W<BCR4rs> {
        CPSIZE_W::new(self, 16)
    }
    #[doc = "Bit 19 - CBURSTRW"]
    #[inline(always)]
    #[must_use]
    pub fn cburstrw(&mut self) -> CBURSTRW_W<BCR4rs> {
        CBURSTRW_W::new(self, 19)
    }
}
#[doc = "SRAM/NOR-Flash chip-select control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCR4rs;
impl crate::RegisterSpec for BCR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcr4::R`](R) reader structure"]
impl crate::Readable for BCR4rs {}
#[doc = "`write(|w| ..)` method takes [`bcr4::W`](W) writer structure"]
impl crate::Writable for BCR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCR4 to value 0x30d0"]
impl crate::Resettable for BCR4rs {
    const RESET_VALUE: u32 = 0x30d0;
}
