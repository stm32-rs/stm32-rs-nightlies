///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
/**Alarm %s write flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAWFR {
    ///0: Alarm update not allowed
    UpdateNotAllowed = 0,
    ///1: Alarm update allowed
    UpdateAllowed = 1,
}
impl From<ALRAWFR> for bool {
    #[inline(always)]
    fn from(variant: ALRAWFR) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRWF(A,B)` reader - Alarm %s write flag
pub type ALRWF_R = crate::BitReader<ALRAWFR>;
impl ALRWF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALRAWFR {
        match self.bits {
            false => ALRAWFR::UpdateNotAllowed,
            true => ALRAWFR::UpdateAllowed,
        }
    }
    ///Alarm update not allowed
    #[inline(always)]
    pub fn is_update_not_allowed(&self) -> bool {
        *self == ALRAWFR::UpdateNotAllowed
    }
    ///Alarm update allowed
    #[inline(always)]
    pub fn is_update_allowed(&self) -> bool {
        *self == ALRAWFR::UpdateAllowed
    }
}
/**Wakeup timer write flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTWFR {
    ///0: Wakeup timer configuration update not allowed
    UpdateNotAllowed = 0,
    ///1: Wakeup timer configuration update allowed
    UpdateAllowed = 1,
}
impl From<WUTWFR> for bool {
    #[inline(always)]
    fn from(variant: WUTWFR) -> Self {
        variant as u8 != 0
    }
}
///Field `WUTWF` reader - Wakeup timer write flag
pub type WUTWF_R = crate::BitReader<WUTWFR>;
impl WUTWF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUTWFR {
        match self.bits {
            false => WUTWFR::UpdateNotAllowed,
            true => WUTWFR::UpdateAllowed,
        }
    }
    ///Wakeup timer configuration update not allowed
    #[inline(always)]
    pub fn is_update_not_allowed(&self) -> bool {
        *self == WUTWFR::UpdateNotAllowed
    }
    ///Wakeup timer configuration update allowed
    #[inline(always)]
    pub fn is_update_allowed(&self) -> bool {
        *self == WUTWFR::UpdateAllowed
    }
}
/**Shift operation pending

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHPFR {
    ///0: No shift operation is pending
    NoShiftPending = 0,
    ///1: A shift operation is pending
    ShiftPending = 1,
}
impl From<SHPFR> for bool {
    #[inline(always)]
    fn from(variant: SHPFR) -> Self {
        variant as u8 != 0
    }
}
///Field `SHPF` reader - Shift operation pending
pub type SHPF_R = crate::BitReader<SHPFR>;
impl SHPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SHPFR {
        match self.bits {
            false => SHPFR::NoShiftPending,
            true => SHPFR::ShiftPending,
        }
    }
    ///No shift operation is pending
    #[inline(always)]
    pub fn is_no_shift_pending(&self) -> bool {
        *self == SHPFR::NoShiftPending
    }
    ///A shift operation is pending
    #[inline(always)]
    pub fn is_shift_pending(&self) -> bool {
        *self == SHPFR::ShiftPending
    }
}
/**Initialization status flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITSR {
    ///0: Calendar has not been initialized
    NotInitalized = 0,
    ///1: Calendar has been initialized
    Initalized = 1,
}
impl From<INITSR> for bool {
    #[inline(always)]
    fn from(variant: INITSR) -> Self {
        variant as u8 != 0
    }
}
///Field `INITS` reader - Initialization status flag
pub type INITS_R = crate::BitReader<INITSR>;
impl INITS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INITSR {
        match self.bits {
            false => INITSR::NotInitalized,
            true => INITSR::Initalized,
        }
    }
    ///Calendar has not been initialized
    #[inline(always)]
    pub fn is_not_initalized(&self) -> bool {
        *self == INITSR::NotInitalized
    }
    ///Calendar has been initialized
    #[inline(always)]
    pub fn is_initalized(&self) -> bool {
        *self == INITSR::Initalized
    }
}
/**Registers synchronization flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSFR {
    ///0: Calendar shadow registers not yet synchronized
    NotSynced = 0,
    ///1: Calendar shadow registers synchronized
    Synced = 1,
}
impl From<RSFR> for bool {
    #[inline(always)]
    fn from(variant: RSFR) -> Self {
        variant as u8 != 0
    }
}
///Field `RSF` reader - Registers synchronization flag
pub type RSF_R = crate::BitReader<RSFR>;
impl RSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSFR {
        match self.bits {
            false => RSFR::NotSynced,
            true => RSFR::Synced,
        }
    }
    ///Calendar shadow registers not yet synchronized
    #[inline(always)]
    pub fn is_not_synced(&self) -> bool {
        *self == RSFR::NotSynced
    }
    ///Calendar shadow registers synchronized
    #[inline(always)]
    pub fn is_synced(&self) -> bool {
        *self == RSFR::Synced
    }
}
/**Registers synchronization flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSFW {
    ///0: This flag is cleared by software by writing 0
    Clear = 0,
}
impl From<RSFW> for bool {
    #[inline(always)]
    fn from(variant: RSFW) -> Self {
        variant as u8 != 0
    }
}
///Field `RSF` writer - Registers synchronization flag
pub type RSF_W<'a, REG> = crate::BitWriter0C<'a, REG, RSFW>;
impl<'a, REG> RSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This flag is cleared by software by writing 0
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RSFW::Clear)
    }
}
/**Initialization flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITFR {
    ///0: Calendar registers update is not allowed
    NotAllowed = 0,
    ///1: Calendar registers update is allowed
    Allowed = 1,
}
impl From<INITFR> for bool {
    #[inline(always)]
    fn from(variant: INITFR) -> Self {
        variant as u8 != 0
    }
}
///Field `INITF` reader - Initialization flag
pub type INITF_R = crate::BitReader<INITFR>;
impl INITF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INITFR {
        match self.bits {
            false => INITFR::NotAllowed,
            true => INITFR::Allowed,
        }
    }
    ///Calendar registers update is not allowed
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == INITFR::NotAllowed
    }
    ///Calendar registers update is allowed
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == INITFR::Allowed
    }
}
/**Initialization mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INIT {
    ///0: Free running mode
    FreeRunningMode = 0,
    ///1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.
    InitMode = 1,
}
impl From<INIT> for bool {
    #[inline(always)]
    fn from(variant: INIT) -> Self {
        variant as u8 != 0
    }
}
///Field `INIT` reader - Initialization mode
pub type INIT_R = crate::BitReader<INIT>;
impl INIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INIT {
        match self.bits {
            false => INIT::FreeRunningMode,
            true => INIT::InitMode,
        }
    }
    ///Free running mode
    #[inline(always)]
    pub fn is_free_running_mode(&self) -> bool {
        *self == INIT::FreeRunningMode
    }
    ///Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.
    #[inline(always)]
    pub fn is_init_mode(&self) -> bool {
        *self == INIT::InitMode
    }
}
///Field `INIT` writer - Initialization mode
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG, INIT>;
impl<'a, REG> INIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Free running mode
    #[inline(always)]
    pub fn free_running_mode(self) -> &'a mut crate::W<REG> {
        self.variant(INIT::FreeRunningMode)
    }
    ///Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.
    #[inline(always)]
    pub fn init_mode(self) -> &'a mut crate::W<REG> {
        self.variant(INIT::InitMode)
    }
}
/**Alarm %s flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAFR {
    ///1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm register (RTC_ALRMxR)
    Match = 1,
}
impl From<ALRAFR> for bool {
    #[inline(always)]
    fn from(variant: ALRAFR) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRF(A,B)` reader - Alarm %s flag
pub type ALRF_R = crate::BitReader<ALRAFR>;
impl ALRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ALRAFR> {
        match self.bits {
            true => Some(ALRAFR::Match),
            _ => None,
        }
    }
    ///This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm register (RTC_ALRMxR)
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ALRAFR::Match
    }
}
/**Alarm %s flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAFW {
    ///0: This flag is cleared by software by writing 0
    Clear = 0,
}
impl From<ALRAFW> for bool {
    #[inline(always)]
    fn from(variant: ALRAFW) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRF(A,B)` writer - Alarm %s flag
pub type ALRF_W<'a, REG> = crate::BitWriter0C<'a, REG, ALRAFW>;
impl<'a, REG> ALRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This flag is cleared by software by writing 0
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAFW::Clear)
    }
}
/**Wakeup timer flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTFR {
    ///1: This flag is set by hardware when the wakeup auto-reload counter reaches 0
    Zero = 1,
}
impl From<WUTFR> for bool {
    #[inline(always)]
    fn from(variant: WUTFR) -> Self {
        variant as u8 != 0
    }
}
///Field `WUTF` reader - Wakeup timer flag
pub type WUTF_R = crate::BitReader<WUTFR>;
impl WUTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUTFR> {
        match self.bits {
            true => Some(WUTFR::Zero),
            _ => None,
        }
    }
    ///This flag is set by hardware when the wakeup auto-reload counter reaches 0
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == WUTFR::Zero
    }
}
/**Wakeup timer flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTFW {
    ///0: This flag is cleared by software by writing 0
    Clear = 0,
}
impl From<WUTFW> for bool {
    #[inline(always)]
    fn from(variant: WUTFW) -> Self {
        variant as u8 != 0
    }
}
///Field `WUTF` writer - Wakeup timer flag
pub type WUTF_W<'a, REG> = crate::BitWriter0C<'a, REG, WUTFW>;
impl<'a, REG> WUTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This flag is cleared by software by writing 0
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WUTFW::Clear)
    }
}
/**Time-stamp flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSFR {
    ///1: This flag is set by hardware when a time-stamp event occurs
    TimestampEvent = 1,
}
impl From<TSFR> for bool {
    #[inline(always)]
    fn from(variant: TSFR) -> Self {
        variant as u8 != 0
    }
}
///Field `TSF` reader - Time-stamp flag
pub type TSF_R = crate::BitReader<TSFR>;
impl TSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSFR> {
        match self.bits {
            true => Some(TSFR::TimestampEvent),
            _ => None,
        }
    }
    ///This flag is set by hardware when a time-stamp event occurs
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        *self == TSFR::TimestampEvent
    }
}
/**Time-stamp flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSFW {
    ///0: This flag is cleared by software by writing 0
    Clear = 0,
}
impl From<TSFW> for bool {
    #[inline(always)]
    fn from(variant: TSFW) -> Self {
        variant as u8 != 0
    }
}
///Field `TSF` writer - Time-stamp flag
pub type TSF_W<'a, REG> = crate::BitWriter0C<'a, REG, TSFW>;
impl<'a, REG> TSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This flag is cleared by software by writing 0
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TSFW::Clear)
    }
}
/**Time-stamp overflow flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSOVFR {
    ///1: This flag is set by hardware when a time-stamp event occurs while TSF is already set
    Overflow = 1,
}
impl From<TSOVFR> for bool {
    #[inline(always)]
    fn from(variant: TSOVFR) -> Self {
        variant as u8 != 0
    }
}
///Field `TSOVF` reader - Time-stamp overflow flag
pub type TSOVF_R = crate::BitReader<TSOVFR>;
impl TSOVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSOVFR> {
        match self.bits {
            true => Some(TSOVFR::Overflow),
            _ => None,
        }
    }
    ///This flag is set by hardware when a time-stamp event occurs while TSF is already set
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == TSOVFR::Overflow
    }
}
/**Time-stamp overflow flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSOVFW {
    ///0: This flag is cleared by software by writing 0
    Clear = 0,
}
impl From<TSOVFW> for bool {
    #[inline(always)]
    fn from(variant: TSOVFW) -> Self {
        variant as u8 != 0
    }
}
///Field `TSOVF` writer - Time-stamp overflow flag
pub type TSOVF_W<'a, REG> = crate::BitWriter0C<'a, REG, TSOVFW>;
impl<'a, REG> TSOVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This flag is cleared by software by writing 0
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TSOVFW::Clear)
    }
}
/**RTC_TAMP1 detection flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1FR {
    ///1: This flag is set by hardware when a tamper detection event is detected on the RTC_TAMPx input
    Tampered = 1,
}
impl From<TAMP1FR> for bool {
    #[inline(always)]
    fn from(variant: TAMP1FR) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP1F` reader - RTC_TAMP1 detection flag
pub type TAMP1F_R = crate::BitReader<TAMP1FR>;
impl TAMP1F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TAMP1FR> {
        match self.bits {
            true => Some(TAMP1FR::Tampered),
            _ => None,
        }
    }
    ///This flag is set by hardware when a tamper detection event is detected on the RTC_TAMPx input
    #[inline(always)]
    pub fn is_tampered(&self) -> bool {
        *self == TAMP1FR::Tampered
    }
}
/**RTC_TAMP1 detection flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1FW {
    ///0: Flag cleared by software writing 0
    Clear = 0,
}
impl From<TAMP1FW> for bool {
    #[inline(always)]
    fn from(variant: TAMP1FW) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP1F` writer - RTC_TAMP1 detection flag
pub type TAMP1F_W<'a, REG> = crate::BitWriter0C<'a, REG, TAMP1FW>;
impl<'a, REG> TAMP1F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flag cleared by software writing 0
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1FW::Clear)
    }
}
///Field `TAMP2F` reader - RTC_TAMP2 detection flag
pub use TAMP1F_R as TAMP2F_R;
///Field `TAMP3F` reader - RTC_TAMP3 detection flag
pub use TAMP1F_R as TAMP3F_R;
///Field `TAMP2F` writer - RTC_TAMP2 detection flag
pub use TAMP1F_W as TAMP2F_W;
///Field `TAMP3F` writer - RTC_TAMP3 detection flag
pub use TAMP1F_W as TAMP3F_W;
/**Recalibration pending flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECALPFR {
    ///1: The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0
    Pending = 1,
}
impl From<RECALPFR> for bool {
    #[inline(always)]
    fn from(variant: RECALPFR) -> Self {
        variant as u8 != 0
    }
}
///Field `RECALPF` reader - Recalibration pending flag
pub type RECALPF_R = crate::BitReader<RECALPFR>;
impl RECALPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RECALPFR> {
        match self.bits {
            true => Some(RECALPFR::Pending),
            _ => None,
        }
    }
    ///The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECALPFR::Pending
    }
}
///Field `RECALPF` writer - Recalibration pending flag
pub type RECALPF_W<'a, REG> = crate::BitWriter<'a, REG, RECALPFR>;
impl<'a, REG> RECALPF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(RECALPFR::Pending)
    }
}
impl R {
    ///Alarm (A,B) write flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ALRAWF` field.</div>
    #[inline(always)]
    pub fn alrwf(&self, n: u8) -> ALRWF_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ALRWF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Alarm (A,B) write flag
    #[inline(always)]
    pub fn alrwf_iter(&self) -> impl Iterator<Item = ALRWF_R> + '_ {
        (0..2).map(move |n| ALRWF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Alarm A write flag
    #[inline(always)]
    pub fn alrawf(&self) -> ALRWF_R {
        ALRWF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm B write flag
    #[inline(always)]
    pub fn alrbwf(&self) -> ALRWF_R {
        ALRWF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup timer write flag
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shift operation pending
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Initialization status flag
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Registers synchronization flag
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Initialization flag
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Initialization mode
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Alarm (A,B) flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ALRAF` field.</div>
    #[inline(always)]
    pub fn alrf(&self, n: u8) -> ALRF_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ALRF_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Alarm (A,B) flag
    #[inline(always)]
    pub fn alrf_iter(&self) -> impl Iterator<Item = ALRF_R> + '_ {
        (0..2).map(move |n| ALRF_R::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    ///Bit 8 - Alarm A flag
    #[inline(always)]
    pub fn alraf(&self) -> ALRF_R {
        ALRF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Alarm B flag
    #[inline(always)]
    pub fn alrbf(&self) -> ALRF_R {
        ALRF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Wakeup timer flag
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Time-stamp flag
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Time-stamp overflow flag
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RTC_TAMP1 detection flag
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - RTC_TAMP2 detection flag
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - RTC_TAMP3 detection flag
    #[inline(always)]
    pub fn tamp3f(&self) -> TAMP3F_R {
        TAMP3F_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Recalibration pending flag
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("tamp1f", &self.tamp1f())
            .field("tamp2f", &self.tamp2f())
            .field("tsovf", &self.tsovf())
            .field("tsf", &self.tsf())
            .field("wutf", &self.wutf())
            .field("alraf", &self.alraf())
            .field("alrbf", &self.alrbf())
            .field("init", &self.init())
            .field("initf", &self.initf())
            .field("rsf", &self.rsf())
            .field("inits", &self.inits())
            .field("shpf", &self.shpf())
            .field("wutwf", &self.wutwf())
            .field("alrawf", &self.alrawf())
            .field("alrbwf", &self.alrbwf())
            .field("recalpf", &self.recalpf())
            .field("tamp3f", &self.tamp3f())
            .finish()
    }
}
impl W {
    ///Bit 5 - Registers synchronization flag
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<'_, ISRrs> {
        RSF_W::new(self, 5)
    }
    ///Bit 7 - Initialization mode
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<'_, ISRrs> {
        INIT_W::new(self, 7)
    }
    ///Alarm (A,B) flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ALRAF` field.</div>
    #[inline(always)]
    pub fn alrf(&mut self, n: u8) -> ALRF_W<'_, ISRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ALRF_W::new(self, n + 8)
    }
    ///Bit 8 - Alarm A flag
    #[inline(always)]
    pub fn alraf(&mut self) -> ALRF_W<'_, ISRrs> {
        ALRF_W::new(self, 8)
    }
    ///Bit 9 - Alarm B flag
    #[inline(always)]
    pub fn alrbf(&mut self) -> ALRF_W<'_, ISRrs> {
        ALRF_W::new(self, 9)
    }
    ///Bit 10 - Wakeup timer flag
    #[inline(always)]
    pub fn wutf(&mut self) -> WUTF_W<'_, ISRrs> {
        WUTF_W::new(self, 10)
    }
    ///Bit 11 - Time-stamp flag
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W<'_, ISRrs> {
        TSF_W::new(self, 11)
    }
    ///Bit 12 - Time-stamp overflow flag
    #[inline(always)]
    pub fn tsovf(&mut self) -> TSOVF_W<'_, ISRrs> {
        TSOVF_W::new(self, 12)
    }
    ///Bit 13 - RTC_TAMP1 detection flag
    #[inline(always)]
    pub fn tamp1f(&mut self) -> TAMP1F_W<'_, ISRrs> {
        TAMP1F_W::new(self, 13)
    }
    ///Bit 14 - RTC_TAMP2 detection flag
    #[inline(always)]
    pub fn tamp2f(&mut self) -> TAMP2F_W<'_, ISRrs> {
        TAMP2F_W::new(self, 14)
    }
    ///Bit 15 - RTC_TAMP3 detection flag
    #[inline(always)]
    pub fn tamp3f(&mut self) -> TAMP3F_W<'_, ISRrs> {
        TAMP3F_W::new(self, 15)
    }
    ///Bit 16 - Recalibration pending flag
    #[inline(always)]
    pub fn recalpf(&mut self) -> RECALPF_W<'_, ISRrs> {
        RECALPF_W::new(self, 16)
    }
}
/**RTC initialization and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#RTC:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff20;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
