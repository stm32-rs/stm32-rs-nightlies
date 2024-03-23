#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<ISRrs>;
#[doc = "Alarm A write flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAWFR {
    #[doc = "0: Alarm update not allowed"]
    UpdateNotAllowed = 0,
    #[doc = "1: Alarm update allowed"]
    UpdateAllowed = 1,
}
impl From<ALRAWFR> for bool {
    #[inline(always)]
    fn from(variant: ALRAWFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAWF` reader - Alarm A write flag"]
pub type ALRAWF_R = crate::BitReader<ALRAWFR>;
impl ALRAWF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALRAWFR {
        match self.bits {
            false => ALRAWFR::UpdateNotAllowed,
            true => ALRAWFR::UpdateAllowed,
        }
    }
    #[doc = "Alarm update not allowed"]
    #[inline(always)]
    pub fn is_update_not_allowed(&self) -> bool {
        *self == ALRAWFR::UpdateNotAllowed
    }
    #[doc = "Alarm update allowed"]
    #[inline(always)]
    pub fn is_update_allowed(&self) -> bool {
        *self == ALRAWFR::UpdateAllowed
    }
}
#[doc = "Field `ALRBWF` reader - Alarm B write flag"]
pub use ALRAWF_R as ALRBWF_R;
#[doc = "Wakeup timer write flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTWFR {
    #[doc = "0: Wakeup timer configuration update not allowed"]
    UpdateNotAllowed = 0,
    #[doc = "1: Wakeup timer configuration update allowed"]
    UpdateAllowed = 1,
}
impl From<WUTWFR> for bool {
    #[inline(always)]
    fn from(variant: WUTWFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTWF` reader - Wakeup timer write flag"]
pub type WUTWF_R = crate::BitReader<WUTWFR>;
impl WUTWF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUTWFR {
        match self.bits {
            false => WUTWFR::UpdateNotAllowed,
            true => WUTWFR::UpdateAllowed,
        }
    }
    #[doc = "Wakeup timer configuration update not allowed"]
    #[inline(always)]
    pub fn is_update_not_allowed(&self) -> bool {
        *self == WUTWFR::UpdateNotAllowed
    }
    #[doc = "Wakeup timer configuration update allowed"]
    #[inline(always)]
    pub fn is_update_allowed(&self) -> bool {
        *self == WUTWFR::UpdateAllowed
    }
}
#[doc = "Shift operation pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHPFR {
    #[doc = "0: No shift operation is pending"]
    NoShiftPending = 0,
    #[doc = "1: A shift operation is pending"]
    ShiftPending = 1,
}
impl From<SHPFR> for bool {
    #[inline(always)]
    fn from(variant: SHPFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHPF` reader - Shift operation pending"]
pub type SHPF_R = crate::BitReader<SHPFR>;
impl SHPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SHPFR {
        match self.bits {
            false => SHPFR::NoShiftPending,
            true => SHPFR::ShiftPending,
        }
    }
    #[doc = "No shift operation is pending"]
    #[inline(always)]
    pub fn is_no_shift_pending(&self) -> bool {
        *self == SHPFR::NoShiftPending
    }
    #[doc = "A shift operation is pending"]
    #[inline(always)]
    pub fn is_shift_pending(&self) -> bool {
        *self == SHPFR::ShiftPending
    }
}
#[doc = "Initialization status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITSR {
    #[doc = "0: Calendar has not been initialized"]
    NotInitalized = 0,
    #[doc = "1: Calendar has been initialized"]
    Initalized = 1,
}
impl From<INITSR> for bool {
    #[inline(always)]
    fn from(variant: INITSR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITS` reader - Initialization status flag"]
pub type INITS_R = crate::BitReader<INITSR>;
impl INITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INITSR {
        match self.bits {
            false => INITSR::NotInitalized,
            true => INITSR::Initalized,
        }
    }
    #[doc = "Calendar has not been initialized"]
    #[inline(always)]
    pub fn is_not_initalized(&self) -> bool {
        *self == INITSR::NotInitalized
    }
    #[doc = "Calendar has been initialized"]
    #[inline(always)]
    pub fn is_initalized(&self) -> bool {
        *self == INITSR::Initalized
    }
}
#[doc = "Registers synchronization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSFR {
    #[doc = "0: Calendar shadow registers not yet synchronized"]
    NotSynced = 0,
    #[doc = "1: Calendar shadow registers synchronized"]
    Synced = 1,
}
impl From<RSFR> for bool {
    #[inline(always)]
    fn from(variant: RSFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` reader - Registers synchronization flag"]
pub type RSF_R = crate::BitReader<RSFR>;
impl RSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSFR {
        match self.bits {
            false => RSFR::NotSynced,
            true => RSFR::Synced,
        }
    }
    #[doc = "Calendar shadow registers not yet synchronized"]
    #[inline(always)]
    pub fn is_not_synced(&self) -> bool {
        *self == RSFR::NotSynced
    }
    #[doc = "Calendar shadow registers synchronized"]
    #[inline(always)]
    pub fn is_synced(&self) -> bool {
        *self == RSFR::Synced
    }
}
#[doc = "Registers synchronization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSFW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    Clear = 0,
}
impl From<RSFW> for bool {
    #[inline(always)]
    fn from(variant: RSFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` writer - Registers synchronization flag"]
pub type RSF_W<'a, REG> = crate::BitWriter0C<'a, REG, RSFW>;
impl<'a, REG> RSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RSFW::Clear)
    }
}
#[doc = "Initialization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITFR {
    #[doc = "0: Calendar registers update is not allowed"]
    NotAllowed = 0,
    #[doc = "1: Calendar registers update is allowed"]
    Allowed = 1,
}
impl From<INITFR> for bool {
    #[inline(always)]
    fn from(variant: INITFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITF` reader - Initialization flag"]
pub type INITF_R = crate::BitReader<INITFR>;
impl INITF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INITFR {
        match self.bits {
            false => INITFR::NotAllowed,
            true => INITFR::Allowed,
        }
    }
    #[doc = "Calendar registers update is not allowed"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == INITFR::NotAllowed
    }
    #[doc = "Calendar registers update is allowed"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == INITFR::Allowed
    }
}
#[doc = "Initialization mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INIT {
    #[doc = "0: Free running mode"]
    FreeRunningMode = 0,
    #[doc = "1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
    InitMode = 1,
}
impl From<INIT> for bool {
    #[inline(always)]
    fn from(variant: INIT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIT` reader - Initialization mode"]
pub type INIT_R = crate::BitReader<INIT>;
impl INIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INIT {
        match self.bits {
            false => INIT::FreeRunningMode,
            true => INIT::InitMode,
        }
    }
    #[doc = "Free running mode"]
    #[inline(always)]
    pub fn is_free_running_mode(&self) -> bool {
        *self == INIT::FreeRunningMode
    }
    #[doc = "Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
    #[inline(always)]
    pub fn is_init_mode(&self) -> bool {
        *self == INIT::InitMode
    }
}
#[doc = "Field `INIT` writer - Initialization mode"]
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG, INIT>;
impl<'a, REG> INIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Free running mode"]
    #[inline(always)]
    pub fn free_running_mode(self) -> &'a mut crate::W<REG> {
        self.variant(INIT::FreeRunningMode)
    }
    #[doc = "Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
    #[inline(always)]
    pub fn init_mode(self) -> &'a mut crate::W<REG> {
        self.variant(INIT::InitMode)
    }
}
#[doc = "Alarm A flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAFR {
    #[doc = "1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR)"]
    Match = 1,
}
impl From<ALRAFR> for bool {
    #[inline(always)]
    fn from(variant: ALRAFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAF` reader - Alarm A flag"]
pub type ALRAF_R = crate::BitReader<ALRAFR>;
impl ALRAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ALRAFR> {
        match self.bits {
            true => Some(ALRAFR::Match),
            _ => None,
        }
    }
    #[doc = "This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR)"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ALRAFR::Match
    }
}
#[doc = "Alarm A flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAFW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    Clear = 0,
}
impl From<ALRAFW> for bool {
    #[inline(always)]
    fn from(variant: ALRAFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAF` writer - Alarm A flag"]
pub type ALRAF_W<'a, REG> = crate::BitWriter0C<'a, REG, ALRAFW>;
impl<'a, REG> ALRAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAFW::Clear)
    }
}
#[doc = "Alarm B flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRBFR {
    #[doc = "1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm B register (RTC_ALRMBR)"]
    Match = 1,
}
impl From<ALRBFR> for bool {
    #[inline(always)]
    fn from(variant: ALRBFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRBF` reader - Alarm B flag"]
pub type ALRBF_R = crate::BitReader<ALRBFR>;
impl ALRBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ALRBFR> {
        match self.bits {
            true => Some(ALRBFR::Match),
            _ => None,
        }
    }
    #[doc = "This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm B register (RTC_ALRMBR)"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ALRBFR::Match
    }
}
#[doc = "Alarm B flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRBFW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    Clear = 0,
}
impl From<ALRBFW> for bool {
    #[inline(always)]
    fn from(variant: ALRBFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRBF` writer - Alarm B flag"]
pub type ALRBF_W<'a, REG> = crate::BitWriter0C<'a, REG, ALRBFW>;
impl<'a, REG> ALRBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ALRBFW::Clear)
    }
}
#[doc = "Wakeup timer flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTFR {
    #[doc = "1: This flag is set by hardware when the wakeup auto-reload counter reaches 0"]
    Zero = 1,
}
impl From<WUTFR> for bool {
    #[inline(always)]
    fn from(variant: WUTFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTF` reader - Wakeup timer flag"]
pub type WUTF_R = crate::BitReader<WUTFR>;
impl WUTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUTFR> {
        match self.bits {
            true => Some(WUTFR::Zero),
            _ => None,
        }
    }
    #[doc = "This flag is set by hardware when the wakeup auto-reload counter reaches 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == WUTFR::Zero
    }
}
#[doc = "Wakeup timer flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTFW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    Clear = 0,
}
impl From<WUTFW> for bool {
    #[inline(always)]
    fn from(variant: WUTFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTF` writer - Wakeup timer flag"]
pub type WUTF_W<'a, REG> = crate::BitWriter0C<'a, REG, WUTFW>;
impl<'a, REG> WUTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WUTFW::Clear)
    }
}
#[doc = "Time-stamp flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSFR {
    #[doc = "1: This flag is set by hardware when a time-stamp event occurs"]
    TimestampEvent = 1,
}
impl From<TSFR> for bool {
    #[inline(always)]
    fn from(variant: TSFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSF` reader - Time-stamp flag"]
pub type TSF_R = crate::BitReader<TSFR>;
impl TSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSFR> {
        match self.bits {
            true => Some(TSFR::TimestampEvent),
            _ => None,
        }
    }
    #[doc = "This flag is set by hardware when a time-stamp event occurs"]
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        *self == TSFR::TimestampEvent
    }
}
#[doc = "Time-stamp flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSFW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    Clear = 0,
}
impl From<TSFW> for bool {
    #[inline(always)]
    fn from(variant: TSFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSF` writer - Time-stamp flag"]
pub type TSF_W<'a, REG> = crate::BitWriter0C<'a, REG, TSFW>;
impl<'a, REG> TSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TSFW::Clear)
    }
}
#[doc = "Time-stamp overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSOVFR {
    #[doc = "1: This flag is set by hardware when a time-stamp event occurs while TSF is already set"]
    Overflow = 1,
}
impl From<TSOVFR> for bool {
    #[inline(always)]
    fn from(variant: TSOVFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSOVF` reader - Time-stamp overflow flag"]
pub type TSOVF_R = crate::BitReader<TSOVFR>;
impl TSOVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSOVFR> {
        match self.bits {
            true => Some(TSOVFR::Overflow),
            _ => None,
        }
    }
    #[doc = "This flag is set by hardware when a time-stamp event occurs while TSF is already set"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == TSOVFR::Overflow
    }
}
#[doc = "Time-stamp overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSOVFW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    Clear = 0,
}
impl From<TSOVFW> for bool {
    #[inline(always)]
    fn from(variant: TSOVFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSOVF` writer - Time-stamp overflow flag"]
pub type TSOVF_W<'a, REG> = crate::BitWriter0C<'a, REG, TSOVFW>;
impl<'a, REG> TSOVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TSOVFW::Clear)
    }
}
#[doc = "RTC_TAMP1 detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1FR {
    #[doc = "1: This flag is set by hardware when a tamper detection event is detected on the RTC_TAMPx input"]
    Tampered = 1,
}
impl From<TAMP1FR> for bool {
    #[inline(always)]
    fn from(variant: TAMP1FR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1F` reader - RTC_TAMP1 detection flag"]
pub type TAMP1F_R = crate::BitReader<TAMP1FR>;
impl TAMP1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TAMP1FR> {
        match self.bits {
            true => Some(TAMP1FR::Tampered),
            _ => None,
        }
    }
    #[doc = "This flag is set by hardware when a tamper detection event is detected on the RTC_TAMPx input"]
    #[inline(always)]
    pub fn is_tampered(&self) -> bool {
        *self == TAMP1FR::Tampered
    }
}
#[doc = "RTC_TAMP1 detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1FW {
    #[doc = "0: Flag cleared by software writing 0"]
    Clear = 0,
}
impl From<TAMP1FW> for bool {
    #[inline(always)]
    fn from(variant: TAMP1FW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1F` writer - RTC_TAMP1 detection flag"]
pub type TAMP1F_W<'a, REG> = crate::BitWriter0C<'a, REG, TAMP1FW>;
impl<'a, REG> TAMP1F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag cleared by software writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1FW::Clear)
    }
}
#[doc = "Field `TAMP2F` reader - RTC_TAMP2 detection flag"]
pub use TAMP1F_R as TAMP2F_R;
#[doc = "Field `TAMP3F` reader - RTC_TAMP3 detection flag"]
pub use TAMP1F_R as TAMP3F_R;
#[doc = "Field `TAMP2F` writer - RTC_TAMP2 detection flag"]
pub use TAMP1F_W as TAMP2F_W;
#[doc = "Field `TAMP3F` writer - RTC_TAMP3 detection flag"]
pub use TAMP1F_W as TAMP3F_W;
#[doc = "Recalibration pending flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECALPFR {
    #[doc = "1: The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0"]
    Pending = 1,
}
impl From<RECALPFR> for bool {
    #[inline(always)]
    fn from(variant: RECALPFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECALPF` reader - Recalibration pending flag"]
pub type RECALPF_R = crate::BitReader<RECALPFR>;
impl RECALPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RECALPFR> {
        match self.bits {
            true => Some(RECALPFR::Pending),
            _ => None,
        }
    }
    #[doc = "The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECALPFR::Pending
    }
}
#[doc = "Field `RECALPF` writer - Recalibration pending flag"]
pub type RECALPF_W<'a, REG> = crate::BitWriter<'a, REG, RECALPFR>;
impl<'a, REG> RECALPF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(RECALPFR::Pending)
    }
}
impl R {
    #[doc = "Bit 0 - Alarm A write flag"]
    #[inline(always)]
    pub fn alrawf(&self) -> ALRAWF_R {
        ALRAWF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B write flag"]
    #[inline(always)]
    pub fn alrbwf(&self) -> ALRBWF_R {
        ALRBWF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer write flag"]
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Initialization status flag"]
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization flag"]
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    pub fn alrbf(&self) -> ALRBF_R {
        ALRBF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RTC_TAMP1 detection flag"]
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RTC_TAMP2 detection flag"]
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC_TAMP3 detection flag"]
    #[inline(always)]
    pub fn tamp3f(&self) -> TAMP3F_R {
        TAMP3F_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Recalibration pending flag"]
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<ISRrs> {
        RSF_W::new(self, 5)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<ISRrs> {
        INIT_W::new(self, 7)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    #[must_use]
    pub fn alraf(&mut self) -> ALRAF_W<ISRrs> {
        ALRAF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    #[must_use]
    pub fn alrbf(&mut self) -> ALRBF_W<ISRrs> {
        ALRBF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    #[must_use]
    pub fn wutf(&mut self) -> WUTF_W<ISRrs> {
        WUTF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<ISRrs> {
        TSF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsovf(&mut self) -> TSOVF_W<ISRrs> {
        TSOVF_W::new(self, 12)
    }
    #[doc = "Bit 13 - RTC_TAMP1 detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1f(&mut self) -> TAMP1F_W<ISRrs> {
        TAMP1F_W::new(self, 13)
    }
    #[doc = "Bit 14 - RTC_TAMP2 detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2f(&mut self) -> TAMP2F_W<ISRrs> {
        TAMP2F_W::new(self, 14)
    }
    #[doc = "Bit 15 - RTC_TAMP3 detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3f(&mut self) -> TAMP3F_W<ISRrs> {
        TAMP3F_W::new(self, 15)
    }
    #[doc = "Bit 16 - Recalibration pending flag"]
    #[inline(always)]
    #[must_use]
    pub fn recalpf(&mut self) -> RECALPF_W<ISRrs> {
        RECALPF_W::new(self, 16)
    }
}
#[doc = "RTC initialization and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff20;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
