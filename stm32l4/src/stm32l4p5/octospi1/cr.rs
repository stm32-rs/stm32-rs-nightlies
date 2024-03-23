#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    #[doc = "0: OCTOSPI disabled"]
    Disabled = 0,
    #[doc = "1: OCTOSPI enabled"]
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader<EN>;
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN {
        match self.bits {
            false => EN::Disabled,
            true => EN::Enabled,
        }
    }
    #[doc = "OCTOSPI disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    #[doc = "OCTOSPI enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OCTOSPI disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    #[doc = "OCTOSPI enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
#[doc = "Abort request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT {
    #[doc = "0: No abort requested"]
    NotRequested = 0,
    #[doc = "1: Abort requested"]
    Requested = 1,
}
impl From<ABORT> for bool {
    #[inline(always)]
    fn from(variant: ABORT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT` reader - Abort request"]
pub type ABORT_R = crate::BitReader<ABORT>;
impl ABORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ABORT {
        match self.bits {
            false => ABORT::NotRequested,
            true => ABORT::Requested,
        }
    }
    #[doc = "No abort requested"]
    #[inline(always)]
    pub fn is_not_requested(&self) -> bool {
        *self == ABORT::NotRequested
    }
    #[doc = "Abort requested"]
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == ABORT::Requested
    }
}
#[doc = "Field `ABORT` writer - Abort request"]
pub type ABORT_W<'a, REG> = crate::BitWriter<'a, REG, ABORT>;
impl<'a, REG> ABORT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No abort requested"]
    #[inline(always)]
    pub fn not_requested(self) -> &'a mut crate::W<REG> {
        self.variant(ABORT::NotRequested)
    }
    #[doc = "Abort requested"]
    #[inline(always)]
    pub fn requested(self) -> &'a mut crate::W<REG> {
        self.variant(ABORT::Requested)
    }
}
#[doc = "DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN {
    #[doc = "0: DMA disabled for Indirect mode"]
    Disabled = 0,
    #[doc = "1: DMA enabled for Indirect mode"]
    Enabled = 1,
}
impl From<DMAEN> for bool {
    #[inline(always)]
    fn from(variant: DMAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DMAEN_R = crate::BitReader<DMAEN>;
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN {
        match self.bits {
            false => DMAEN::Disabled,
            true => DMAEN::Enabled,
        }
    }
    #[doc = "DMA disabled for Indirect mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN::Disabled
    }
    #[doc = "DMA enabled for Indirect mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN::Enabled
    }
}
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA disabled for Indirect mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Disabled)
    }
    #[doc = "DMA enabled for Indirect mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Enabled)
    }
}
#[doc = "Timeout counter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCEN {
    #[doc = "0: Timeout counter is disabled, and thus the chip-select (NCS) remains active indefinitely after an access in Memory-mapped mode"]
    Disabled = 0,
    #[doc = "1: Timeout counter is enabled, and thus the chip-select is released in the Memory-mapped mode after TIMEOUT\\[15:0\\]
cycles of external device inactivity"]
    Enabled = 1,
}
impl From<TCEN> for bool {
    #[inline(always)]
    fn from(variant: TCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCEN` reader - Timeout counter enable"]
pub type TCEN_R = crate::BitReader<TCEN>;
impl TCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCEN {
        match self.bits {
            false => TCEN::Disabled,
            true => TCEN::Enabled,
        }
    }
    #[doc = "Timeout counter is disabled, and thus the chip-select (NCS) remains active indefinitely after an access in Memory-mapped mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCEN::Disabled
    }
    #[doc = "Timeout counter is enabled, and thus the chip-select is released in the Memory-mapped mode after TIMEOUT\\[15:0\\]
cycles of external device inactivity"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCEN::Enabled
    }
}
#[doc = "Field `TCEN` writer - Timeout counter enable"]
pub type TCEN_W<'a, REG> = crate::BitWriter<'a, REG, TCEN>;
impl<'a, REG> TCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timeout counter is disabled, and thus the chip-select (NCS) remains active indefinitely after an access in Memory-mapped mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCEN::Disabled)
    }
    #[doc = "Timeout counter is enabled, and thus the chip-select is released in the Memory-mapped mode after TIMEOUT\\[15:0\\]
cycles of external device inactivity"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCEN::Enabled)
    }
}
#[doc = "Dual-memory configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMM {
    #[doc = "0: Dual-quad configuration disabled"]
    Disabled = 0,
    #[doc = "1: Dual-quad configuration enabled"]
    Enabled = 1,
}
impl From<DMM> for bool {
    #[inline(always)]
    fn from(variant: DMM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMM` reader - Dual-memory configuration"]
pub type DMM_R = crate::BitReader<DMM>;
impl DMM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMM {
        match self.bits {
            false => DMM::Disabled,
            true => DMM::Enabled,
        }
    }
    #[doc = "Dual-quad configuration disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMM::Disabled
    }
    #[doc = "Dual-quad configuration enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMM::Enabled
    }
}
#[doc = "Field `DMM` writer - Dual-memory configuration"]
pub type DMM_W<'a, REG> = crate::BitWriter<'a, REG, DMM>;
impl<'a, REG> DMM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dual-quad configuration disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMM::Disabled)
    }
    #[doc = "Dual-quad configuration enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMM::Enabled)
    }
}
#[doc = "FLASH memory selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSEL {
    #[doc = "0: FLASH 1 selected (data exchanged over IO\\[3:0\\])"]
    Flash1 = 0,
    #[doc = "1: FLASH 2 selected (data exchanged over IO\\[7:4\\])"]
    Flash2 = 1,
}
impl From<FSEL> for bool {
    #[inline(always)]
    fn from(variant: FSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSEL` reader - FLASH memory selection"]
pub type FSEL_R = crate::BitReader<FSEL>;
impl FSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL {
        match self.bits {
            false => FSEL::Flash1,
            true => FSEL::Flash2,
        }
    }
    #[doc = "FLASH 1 selected (data exchanged over IO\\[3:0\\])"]
    #[inline(always)]
    pub fn is_flash1(&self) -> bool {
        *self == FSEL::Flash1
    }
    #[doc = "FLASH 2 selected (data exchanged over IO\\[7:4\\])"]
    #[inline(always)]
    pub fn is_flash2(&self) -> bool {
        *self == FSEL::Flash2
    }
}
#[doc = "Field `FSEL` writer - FLASH memory selection"]
pub type FSEL_W<'a, REG> = crate::BitWriter<'a, REG, FSEL>;
impl<'a, REG> FSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FLASH 1 selected (data exchanged over IO\\[3:0\\])"]
    #[inline(always)]
    pub fn flash1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL::Flash1)
    }
    #[doc = "FLASH 2 selected (data exchanged over IO\\[7:4\\])"]
    #[inline(always)]
    pub fn flash2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL::Flash2)
    }
}
#[doc = "Field `FTHRES` reader - IFO threshold level"]
pub type FTHRES_R = crate::FieldReader;
#[doc = "Field `FTHRES` writer - IFO threshold level"]
pub type FTHRES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Transfer error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<TEIE> for bool {
    #[inline(always)]
    fn from(variant: TEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIE` reader - Transfer error interrupt enable"]
pub type TEIE_R = crate::BitReader<TEIE>;
impl TEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEIE {
        match self.bits {
            false => TEIE::Disabled,
            true => TEIE::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEIE::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEIE::Enabled
    }
}
#[doc = "Field `TEIE` writer - Transfer error interrupt enable"]
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG, TEIE>;
impl<'a, REG> TEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::Enabled)
    }
}
#[doc = "Transfer complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<TCIE> for bool {
    #[inline(always)]
    fn from(variant: TCIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable"]
pub type TCIE_R = crate::BitReader<TCIE>;
impl TCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCIE {
        match self.bits {
            false => TCIE::Disabled,
            true => TCIE::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE::Enabled
    }
}
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Enabled)
    }
}
#[doc = "FIFO threshold interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTIE {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<FTIE> for bool {
    #[inline(always)]
    fn from(variant: FTIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTIE` reader - FIFO threshold interrupt enable"]
pub type FTIE_R = crate::BitReader<FTIE>;
impl FTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FTIE {
        match self.bits {
            false => FTIE::Disabled,
            true => FTIE::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FTIE::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FTIE::Enabled
    }
}
#[doc = "Field `FTIE` writer - FIFO threshold interrupt enable"]
pub type FTIE_W<'a, REG> = crate::BitWriter<'a, REG, FTIE>;
impl<'a, REG> FTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FTIE::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FTIE::Enabled)
    }
}
#[doc = "Status match interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMIE {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<SMIE> for bool {
    #[inline(always)]
    fn from(variant: SMIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMIE` reader - Status match interrupt enable"]
pub type SMIE_R = crate::BitReader<SMIE>;
impl SMIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMIE {
        match self.bits {
            false => SMIE::Disabled,
            true => SMIE::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMIE::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMIE::Enabled
    }
}
#[doc = "Field `SMIE` writer - Status match interrupt enable"]
pub type SMIE_W<'a, REG> = crate::BitWriter<'a, REG, SMIE>;
impl<'a, REG> SMIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMIE::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMIE::Enabled)
    }
}
#[doc = "TimeOut interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOIE {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<TOIE> for bool {
    #[inline(always)]
    fn from(variant: TOIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOIE` reader - TimeOut interrupt enable"]
pub type TOIE_R = crate::BitReader<TOIE>;
impl TOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TOIE {
        match self.bits {
            false => TOIE::Disabled,
            true => TOIE::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TOIE::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TOIE::Enabled
    }
}
#[doc = "Field `TOIE` writer - TimeOut interrupt enable"]
pub type TOIE_W<'a, REG> = crate::BitWriter<'a, REG, TOIE>;
impl<'a, REG> TOIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TOIE::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TOIE::Enabled)
    }
}
#[doc = "Automatic poll mode stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APMS {
    #[doc = "0: Automatic status-polling mode is stopped only by abort or by disabling the OCTOSPI"]
    Running = 0,
    #[doc = "1: Automatic status-polling mode stops as soon as there is a match"]
    StopMatch = 1,
}
impl From<APMS> for bool {
    #[inline(always)]
    fn from(variant: APMS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APMS` reader - Automatic poll mode stop"]
pub type APMS_R = crate::BitReader<APMS>;
impl APMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> APMS {
        match self.bits {
            false => APMS::Running,
            true => APMS::StopMatch,
        }
    }
    #[doc = "Automatic status-polling mode is stopped only by abort or by disabling the OCTOSPI"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == APMS::Running
    }
    #[doc = "Automatic status-polling mode stops as soon as there is a match"]
    #[inline(always)]
    pub fn is_stop_match(&self) -> bool {
        *self == APMS::StopMatch
    }
}
#[doc = "Field `APMS` writer - Automatic poll mode stop"]
pub type APMS_W<'a, REG> = crate::BitWriter<'a, REG, APMS>;
impl<'a, REG> APMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic status-polling mode is stopped only by abort or by disabling the OCTOSPI"]
    #[inline(always)]
    pub fn running(self) -> &'a mut crate::W<REG> {
        self.variant(APMS::Running)
    }
    #[doc = "Automatic status-polling mode stops as soon as there is a match"]
    #[inline(always)]
    pub fn stop_match(self) -> &'a mut crate::W<REG> {
        self.variant(APMS::StopMatch)
    }
}
#[doc = "Polling match mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMM {
    #[doc = "0: AND-match mode, SMF is set if all the unmasked bits received from the device match the corresponding bits in the match register"]
    AndmatchMode = 0,
    #[doc = "1: OR-match mode, SMF is set if any of the unmasked bits received from the device matches its corresponding bit in the match register"]
    Ormatchmode = 1,
}
impl From<PMM> for bool {
    #[inline(always)]
    fn from(variant: PMM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMM` reader - Polling match mode"]
pub type PMM_R = crate::BitReader<PMM>;
impl PMM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PMM {
        match self.bits {
            false => PMM::AndmatchMode,
            true => PMM::Ormatchmode,
        }
    }
    #[doc = "AND-match mode, SMF is set if all the unmasked bits received from the device match the corresponding bits in the match register"]
    #[inline(always)]
    pub fn is_andmatch_mode(&self) -> bool {
        *self == PMM::AndmatchMode
    }
    #[doc = "OR-match mode, SMF is set if any of the unmasked bits received from the device matches its corresponding bit in the match register"]
    #[inline(always)]
    pub fn is_ormatchmode(&self) -> bool {
        *self == PMM::Ormatchmode
    }
}
#[doc = "Field `PMM` writer - Polling match mode"]
pub type PMM_W<'a, REG> = crate::BitWriter<'a, REG, PMM>;
impl<'a, REG> PMM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AND-match mode, SMF is set if all the unmasked bits received from the device match the corresponding bits in the match register"]
    #[inline(always)]
    pub fn andmatch_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PMM::AndmatchMode)
    }
    #[doc = "OR-match mode, SMF is set if any of the unmasked bits received from the device matches its corresponding bit in the match register"]
    #[inline(always)]
    pub fn ormatchmode(self) -> &'a mut crate::W<REG> {
        self.variant(PMM::Ormatchmode)
    }
}
#[doc = "Functional mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FMODE {
    #[doc = "0: Indirect-write mode"]
    IndirectWrite = 0,
    #[doc = "1: Indirect-read mode"]
    IndirectRead = 1,
    #[doc = "2: Automatic status-polling mode"]
    AutomaticPolling = 2,
    #[doc = "3: Memory-mapped mode"]
    MemoryMapped = 3,
}
impl From<FMODE> for u8 {
    #[inline(always)]
    fn from(variant: FMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FMODE {
    type Ux = u8;
}
#[doc = "Field `FMODE` reader - Functional mode"]
pub type FMODE_R = crate::FieldReader<FMODE>;
impl FMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FMODE {
        match self.bits {
            0 => FMODE::IndirectWrite,
            1 => FMODE::IndirectRead,
            2 => FMODE::AutomaticPolling,
            3 => FMODE::MemoryMapped,
            _ => unreachable!(),
        }
    }
    #[doc = "Indirect-write mode"]
    #[inline(always)]
    pub fn is_indirect_write(&self) -> bool {
        *self == FMODE::IndirectWrite
    }
    #[doc = "Indirect-read mode"]
    #[inline(always)]
    pub fn is_indirect_read(&self) -> bool {
        *self == FMODE::IndirectRead
    }
    #[doc = "Automatic status-polling mode"]
    #[inline(always)]
    pub fn is_automatic_polling(&self) -> bool {
        *self == FMODE::AutomaticPolling
    }
    #[doc = "Memory-mapped mode"]
    #[inline(always)]
    pub fn is_memory_mapped(&self) -> bool {
        *self == FMODE::MemoryMapped
    }
}
#[doc = "Field `FMODE` writer - Functional mode"]
pub type FMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, FMODE>;
impl<'a, REG> FMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Indirect-write mode"]
    #[inline(always)]
    pub fn indirect_write(self) -> &'a mut crate::W<REG> {
        self.variant(FMODE::IndirectWrite)
    }
    #[doc = "Indirect-read mode"]
    #[inline(always)]
    pub fn indirect_read(self) -> &'a mut crate::W<REG> {
        self.variant(FMODE::IndirectRead)
    }
    #[doc = "Automatic status-polling mode"]
    #[inline(always)]
    pub fn automatic_polling(self) -> &'a mut crate::W<REG> {
        self.variant(FMODE::AutomaticPolling)
    }
    #[doc = "Memory-mapped mode"]
    #[inline(always)]
    pub fn memory_mapped(self) -> &'a mut crate::W<REG> {
        self.variant(FMODE::MemoryMapped)
    }
}
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Abort request"]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout counter enable"]
    #[inline(always)]
    pub fn tcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Dual-memory configuration"]
    #[inline(always)]
    pub fn dmm(&self) -> DMM_R {
        DMM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FLASH memory selection"]
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - IFO threshold level"]
    #[inline(always)]
    pub fn fthres(&self) -> FTHRES_R {
        FTHRES_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FIFO threshold interrupt enable"]
    #[inline(always)]
    pub fn ftie(&self) -> FTIE_R {
        FTIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Status match interrupt enable"]
    #[inline(always)]
    pub fn smie(&self) -> SMIE_R {
        SMIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TimeOut interrupt enable"]
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Automatic poll mode stop"]
    #[inline(always)]
    pub fn apms(&self) -> APMS_R {
        APMS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Polling match mode"]
    #[inline(always)]
    pub fn pmm(&self) -> PMM_R {
        PMM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Functional mode"]
    #[inline(always)]
    pub fn fmode(&self) -> FMODE_R {
        FMODE_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Abort request"]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<CRrs> {
        ABORT_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CRrs> {
        DMAEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timeout counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcen(&mut self) -> TCEN_W<CRrs> {
        TCEN_W::new(self, 3)
    }
    #[doc = "Bit 6 - Dual-memory configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dmm(&mut self) -> DMM_W<CRrs> {
        DMM_W::new(self, 6)
    }
    #[doc = "Bit 7 - FLASH memory selection"]
    #[inline(always)]
    #[must_use]
    pub fn fsel(&mut self) -> FSEL_W<CRrs> {
        FSEL_W::new(self, 7)
    }
    #[doc = "Bits 8:12 - IFO threshold level"]
    #[inline(always)]
    #[must_use]
    pub fn fthres(&mut self) -> FTHRES_W<CRrs> {
        FTHRES_W::new(self, 8)
    }
    #[doc = "Bit 16 - Transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<CRrs> {
        TEIE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<CRrs> {
        TCIE_W::new(self, 17)
    }
    #[doc = "Bit 18 - FIFO threshold interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ftie(&mut self) -> FTIE_W<CRrs> {
        FTIE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Status match interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn smie(&mut self) -> SMIE_W<CRrs> {
        SMIE_W::new(self, 19)
    }
    #[doc = "Bit 20 - TimeOut interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie(&mut self) -> TOIE_W<CRrs> {
        TOIE_W::new(self, 20)
    }
    #[doc = "Bit 22 - Automatic poll mode stop"]
    #[inline(always)]
    #[must_use]
    pub fn apms(&mut self) -> APMS_W<CRrs> {
        APMS_W::new(self, 22)
    }
    #[doc = "Bit 23 - Polling match mode"]
    #[inline(always)]
    #[must_use]
    pub fn pmm(&mut self) -> PMM_W<CRrs> {
        PMM_W::new(self, 23)
    }
    #[doc = "Bits 28:29 - Functional mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmode(&mut self) -> FMODE_W<CRrs> {
        FMODE_W::new(self, 28)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
