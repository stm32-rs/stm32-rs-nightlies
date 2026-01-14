///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
/**End of operation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPR {
    ///0: No EOP operation occurred
    NoEvent = 0,
    ///1: An EOP event occurred
    Event = 1,
}
impl From<EOPR> for bool {
    #[inline(always)]
    fn from(variant: EOPR) -> Self {
        variant as u8 != 0
    }
}
///Field `EOP` reader - End of operation
pub type EOP_R = crate::BitReader<EOPR>;
impl EOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOPR {
        match self.bits {
            false => EOPR::NoEvent,
            true => EOPR::Event,
        }
    }
    ///No EOP operation occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == EOPR::NoEvent
    }
    ///An EOP event occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == EOPR::Event
    }
}
/**End of operation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<EOPW> for bool {
    #[inline(always)]
    fn from(variant: EOPW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOP` writer - End of operation
pub type EOP_W<'a, REG> = crate::BitWriter<'a, REG, EOPW>;
impl<'a, REG> EOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOPW::Clear)
    }
}
/**Operation error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPERRR {
    ///0: No memory opreation error happened
    NoError = 0,
    ///1: Memory operation error happened
    Error = 1,
}
impl From<OPERRR> for bool {
    #[inline(always)]
    fn from(variant: OPERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `OPERR` reader - Operation error
pub type OPERR_R = crate::BitReader<OPERRR>;
impl OPERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OPERRR {
        match self.bits {
            false => OPERRR::NoError,
            true => OPERRR::Error,
        }
    }
    ///No memory opreation error happened
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == OPERRR::NoError
    }
    ///Memory operation error happened
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == OPERRR::Error
    }
}
/**Operation error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPERRW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<OPERRW> for bool {
    #[inline(always)]
    fn from(variant: OPERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `OPERR` writer - Operation error
pub type OPERR_W<'a, REG> = crate::BitWriter<'a, REG, OPERRW>;
impl<'a, REG> OPERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OPERRW::Clear)
    }
}
/**Programming error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROGERRR {
    ///0: No size programming error happened
    NoError = 0,
    ///1: Programming error happened
    Error = 1,
}
impl From<PROGERRR> for bool {
    #[inline(always)]
    fn from(variant: PROGERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `PROGERR` reader - Programming error
pub type PROGERR_R = crate::BitReader<PROGERRR>;
impl PROGERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PROGERRR {
        match self.bits {
            false => PROGERRR::NoError,
            true => PROGERRR::Error,
        }
    }
    ///No size programming error happened
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PROGERRR::NoError
    }
    ///Programming error happened
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PROGERRR::Error
    }
}
/**Programming error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROGERRW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<PROGERRW> for bool {
    #[inline(always)]
    fn from(variant: PROGERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `PROGERR` writer - Programming error
pub type PROGERR_W<'a, REG> = crate::BitWriter<'a, REG, PROGERRW>;
impl<'a, REG> PROGERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PROGERRW::Clear)
    }
}
/**Write protected error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPERRR {
    ///0: No write protection error happened
    NoError = 0,
    ///1: Write protection error happened
    Error = 1,
}
impl From<WRPERRR> for bool {
    #[inline(always)]
    fn from(variant: WRPERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `WRPERR` reader - Write protected error
pub type WRPERR_R = crate::BitReader<WRPERRR>;
impl WRPERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WRPERRR {
        match self.bits {
            false => WRPERRR::NoError,
            true => WRPERRR::Error,
        }
    }
    ///No write protection error happened
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WRPERRR::NoError
    }
    ///Write protection error happened
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WRPERRR::Error
    }
}
/**Write protected error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPERRW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<WRPERRW> for bool {
    #[inline(always)]
    fn from(variant: WRPERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `WRPERR` writer - Write protected error
pub type WRPERR_W<'a, REG> = crate::BitWriter<'a, REG, WRPERRW>;
impl<'a, REG> WRPERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WRPERRW::Clear)
    }
}
/**Programming alignment error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAERRR {
    ///0: No programming alignment error happened
    NoError = 0,
    ///1: Programming alignment error happened
    Error = 1,
}
impl From<PGAERRR> for bool {
    #[inline(always)]
    fn from(variant: PGAERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `PGAERR` reader - Programming alignment error
pub type PGAERR_R = crate::BitReader<PGAERRR>;
impl PGAERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PGAERRR {
        match self.bits {
            false => PGAERRR::NoError,
            true => PGAERRR::Error,
        }
    }
    ///No programming alignment error happened
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGAERRR::NoError
    }
    ///Programming alignment error happened
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGAERRR::Error
    }
}
/**Programming alignment error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAERRW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<PGAERRW> for bool {
    #[inline(always)]
    fn from(variant: PGAERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `PGAERR` writer - Programming alignment error
pub type PGAERR_W<'a, REG> = crate::BitWriter<'a, REG, PGAERRW>;
impl<'a, REG> PGAERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PGAERRW::Clear)
    }
}
/**Size error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIZERRR {
    ///0: No size error happened
    NoError = 0,
    ///1: Size error happened
    Error = 1,
}
impl From<SIZERRR> for bool {
    #[inline(always)]
    fn from(variant: SIZERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `SIZERR` reader - Size error
pub type SIZERR_R = crate::BitReader<SIZERRR>;
impl SIZERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SIZERRR {
        match self.bits {
            false => SIZERRR::NoError,
            true => SIZERRR::Error,
        }
    }
    ///No size error happened
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == SIZERRR::NoError
    }
    ///Size error happened
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SIZERRR::Error
    }
}
/**Size error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIZERRW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<SIZERRW> for bool {
    #[inline(always)]
    fn from(variant: SIZERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `SIZERR` writer - Size error
pub type SIZERR_W<'a, REG> = crate::BitWriter<'a, REG, SIZERRW>;
impl<'a, REG> SIZERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SIZERRW::Clear)
    }
}
/**Programming sequence error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGSERRR {
    ///0: No fast programming sequence error happened
    NoError = 0,
    ///1: Fast programming sequence error happened
    Error = 1,
}
impl From<PGSERRR> for bool {
    #[inline(always)]
    fn from(variant: PGSERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `PGSERR` reader - Programming sequence error
pub type PGSERR_R = crate::BitReader<PGSERRR>;
impl PGSERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PGSERRR {
        match self.bits {
            false => PGSERRR::NoError,
            true => PGSERRR::Error,
        }
    }
    ///No fast programming sequence error happened
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGSERRR::NoError
    }
    ///Fast programming sequence error happened
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGSERRR::Error
    }
}
/**Programming sequence error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGSERRW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<PGSERRW> for bool {
    #[inline(always)]
    fn from(variant: PGSERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `PGSERR` writer - Programming sequence error
pub type PGSERR_W<'a, REG> = crate::BitWriter<'a, REG, PGSERRW>;
impl<'a, REG> PGSERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PGSERRW::Clear)
    }
}
/**Fast programming data miss error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MISSERRR {
    ///0: No fast programming data miss error happened
    NoError = 0,
    ///1: Fast programming data miss error happened
    Error = 1,
}
impl From<MISSERRR> for bool {
    #[inline(always)]
    fn from(variant: MISSERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `MISSERR` reader - Fast programming data miss error
pub type MISSERR_R = crate::BitReader<MISSERRR>;
impl MISSERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MISSERRR {
        match self.bits {
            false => MISSERRR::NoError,
            true => MISSERRR::Error,
        }
    }
    ///No fast programming data miss error happened
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == MISSERRR::NoError
    }
    ///Fast programming data miss error happened
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == MISSERRR::Error
    }
}
/**Fast programming data miss error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MISSERRW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<MISSERRW> for bool {
    #[inline(always)]
    fn from(variant: MISSERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `MISSERR` writer - Fast programming data miss error
pub type MISSERR_W<'a, REG> = crate::BitWriter<'a, REG, MISSERRW>;
impl<'a, REG> MISSERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(MISSERRW::Clear)
    }
}
/**Fast programming error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FASTERRR {
    ///0: No fast programming error happened
    NoError = 0,
    ///1: Fast programming error happened
    Error = 1,
}
impl From<FASTERRR> for bool {
    #[inline(always)]
    fn from(variant: FASTERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `FASTERR` reader - Fast programming error
pub type FASTERR_R = crate::BitReader<FASTERRR>;
impl FASTERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FASTERRR {
        match self.bits {
            false => FASTERRR::NoError,
            true => FASTERRR::Error,
        }
    }
    ///No fast programming error happened
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FASTERRR::NoError
    }
    ///Fast programming error happened
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FASTERRR::Error
    }
}
/**Fast programming error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FASTERRW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<FASTERRW> for bool {
    #[inline(always)]
    fn from(variant: FASTERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `FASTERR` writer - Fast programming error
pub type FASTERR_W<'a, REG> = crate::BitWriter<'a, REG, FASTERRW>;
impl<'a, REG> FASTERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FASTERRW::Clear)
    }
}
/**User Option OPTIVAL indication

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTNV {
    ///0: The OBL user option OPTVAL indicates "valid"
    Valid = 0,
    ///1: The OBL user option OPTVAL indicates "invalid"
    Invalid = 1,
}
impl From<OPTNV> for bool {
    #[inline(always)]
    fn from(variant: OPTNV) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTNV` reader - User Option OPTIVAL indication
pub type OPTNV_R = crate::BitReader<OPTNV>;
impl OPTNV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OPTNV {
        match self.bits {
            false => OPTNV::Valid,
            true => OPTNV::Invalid,
        }
    }
    ///The OBL user option OPTVAL indicates "valid"
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == OPTNV::Valid
    }
    ///The OBL user option OPTVAL indicates "invalid"
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == OPTNV::Invalid
    }
}
/**PCROP read error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERRR {
    ///0: No read-only error happened
    NoError = 0,
    ///1: Read-only error happened
    Error = 1,
}
impl From<RDERRR> for bool {
    #[inline(always)]
    fn from(variant: RDERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `RDERR` reader - PCROP read error
pub type RDERR_R = crate::BitReader<RDERRR>;
impl RDERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDERRR {
        match self.bits {
            false => RDERRR::NoError,
            true => RDERRR::Error,
        }
    }
    ///No read-only error happened
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RDERRR::NoError
    }
    ///Read-only error happened
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RDERRR::Error
    }
}
/**PCROP read error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERRW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<RDERRW> for bool {
    #[inline(always)]
    fn from(variant: RDERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `RDERR` writer - PCROP read error
pub type RDERR_W<'a, REG> = crate::BitWriter<'a, REG, RDERRW>;
impl<'a, REG> RDERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RDERRW::Clear)
    }
}
/**Option validity error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTVERRR {
    ///0: No error in option and engineering bits
    NoError = 0,
    ///1: Error in option and engineering bits
    Error = 1,
}
impl From<OPTVERRR> for bool {
    #[inline(always)]
    fn from(variant: OPTVERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTVERR` reader - Option validity error
pub type OPTVERR_R = crate::BitReader<OPTVERRR>;
impl OPTVERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OPTVERRR {
        match self.bits {
            false => OPTVERRR::NoError,
            true => OPTVERRR::Error,
        }
    }
    ///No error in option and engineering bits
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == OPTVERRR::NoError
    }
    ///Error in option and engineering bits
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == OPTVERRR::Error
    }
}
/**Option validity error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTVERRW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<OPTVERRW> for bool {
    #[inline(always)]
    fn from(variant: OPTVERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTVERR` writer - Option validity error
pub type OPTVERR_W<'a, REG> = crate::BitWriter<'a, REG, OPTVERRW>;
impl<'a, REG> OPTVERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OPTVERRW::Clear)
    }
}
/**Busy

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSY {
    ///0: No write/erase operation is in progress
    Inactive = 0,
    ///1: No write/erase operation is in progress
    Active = 1,
}
impl From<BSY> for bool {
    #[inline(always)]
    fn from(variant: BSY) -> Self {
        variant as u8 != 0
    }
}
///Field `BSY` reader - Busy
pub type BSY_R = crate::BitReader<BSY>;
impl BSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSY {
        match self.bits {
            false => BSY::Inactive,
            true => BSY::Active,
        }
    }
    ///No write/erase operation is in progress
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == BSY::Inactive
    }
    ///No write/erase operation is in progress
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BSY::Active
    }
}
/**Programming or erase configuration busy

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFGBSY {
    ///0: PG, PNB, PER, MER bits available for writing
    Free = 0,
    ///1: PG, PNB, PER, MER bits not available for writing (operation ongoing)
    Busy = 1,
}
impl From<CFGBSY> for bool {
    #[inline(always)]
    fn from(variant: CFGBSY) -> Self {
        variant as u8 != 0
    }
}
///Field `CFGBSY` reader - Programming or erase configuration busy
pub type CFGBSY_R = crate::BitReader<CFGBSY>;
impl CFGBSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CFGBSY {
        match self.bits {
            false => CFGBSY::Free,
            true => CFGBSY::Busy,
        }
    }
    ///PG, PNB, PER, MER bits available for writing
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == CFGBSY::Free
    }
    ///PG, PNB, PER, MER bits not available for writing (operation ongoing)
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == CFGBSY::Busy
    }
}
/**Programming / erase operation suspended

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PESD {
    ///0: Flash program and erase operations granted
    Granted = 0,
    ///1: Any new Flash program and erase operation is suspended until this bit is cleared. This bit is set when the PES bit in FLASH_ACR is set
    Suspended = 1,
}
impl From<PESD> for bool {
    #[inline(always)]
    fn from(variant: PESD) -> Self {
        variant as u8 != 0
    }
}
///Field `PESD` reader - Programming / erase operation suspended
pub type PESD_R = crate::BitReader<PESD>;
impl PESD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PESD {
        match self.bits {
            false => PESD::Granted,
            true => PESD::Suspended,
        }
    }
    ///Flash program and erase operations granted
    #[inline(always)]
    pub fn is_granted(&self) -> bool {
        *self == PESD::Granted
    }
    ///Any new Flash program and erase operation is suspended until this bit is cleared. This bit is set when the PES bit in FLASH_ACR is set
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == PESD::Suspended
    }
}
impl R {
    ///Bit 0 - End of operation
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Operation error
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Programming error
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Write protected error
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Programming alignment error
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Size error
    #[inline(always)]
    pub fn sizerr(&self) -> SIZERR_R {
        SIZERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Programming sequence error
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Fast programming data miss error
    #[inline(always)]
    pub fn misserr(&self) -> MISSERR_R {
        MISSERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Fast programming error
    #[inline(always)]
    pub fn fasterr(&self) -> FASTERR_R {
        FASTERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - User Option OPTIVAL indication
    #[inline(always)]
    pub fn optnv(&self) -> OPTNV_R {
        OPTNV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - PCROP read error
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Option validity error
    #[inline(always)]
    pub fn optverr(&self) -> OPTVERR_R {
        OPTVERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Busy
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Programming or erase configuration busy
    #[inline(always)]
    pub fn cfgbsy(&self) -> CFGBSY_R {
        CFGBSY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Programming / erase operation suspended
    #[inline(always)]
    pub fn pesd(&self) -> PESD_R {
        PESD_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("eop", &self.eop())
            .field("operr", &self.operr())
            .field("progerr", &self.progerr())
            .field("wrperr", &self.wrperr())
            .field("pgaerr", &self.pgaerr())
            .field("sizerr", &self.sizerr())
            .field("pgserr", &self.pgserr())
            .field("misserr", &self.misserr())
            .field("fasterr", &self.fasterr())
            .field("optnv", &self.optnv())
            .field("rderr", &self.rderr())
            .field("optverr", &self.optverr())
            .field("bsy", &self.bsy())
            .field("cfgbsy", &self.cfgbsy())
            .field("pesd", &self.pesd())
            .finish()
    }
}
impl W {
    ///Bit 0 - End of operation
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W<'_, SRrs> {
        EOP_W::new(self, 0)
    }
    ///Bit 1 - Operation error
    #[inline(always)]
    pub fn operr(&mut self) -> OPERR_W<'_, SRrs> {
        OPERR_W::new(self, 1)
    }
    ///Bit 3 - Programming error
    #[inline(always)]
    pub fn progerr(&mut self) -> PROGERR_W<'_, SRrs> {
        PROGERR_W::new(self, 3)
    }
    ///Bit 4 - Write protected error
    #[inline(always)]
    pub fn wrperr(&mut self) -> WRPERR_W<'_, SRrs> {
        WRPERR_W::new(self, 4)
    }
    ///Bit 5 - Programming alignment error
    #[inline(always)]
    pub fn pgaerr(&mut self) -> PGAERR_W<'_, SRrs> {
        PGAERR_W::new(self, 5)
    }
    ///Bit 6 - Size error
    #[inline(always)]
    pub fn sizerr(&mut self) -> SIZERR_W<'_, SRrs> {
        SIZERR_W::new(self, 6)
    }
    ///Bit 7 - Programming sequence error
    #[inline(always)]
    pub fn pgserr(&mut self) -> PGSERR_W<'_, SRrs> {
        PGSERR_W::new(self, 7)
    }
    ///Bit 8 - Fast programming data miss error
    #[inline(always)]
    pub fn misserr(&mut self) -> MISSERR_W<'_, SRrs> {
        MISSERR_W::new(self, 8)
    }
    ///Bit 9 - Fast programming error
    #[inline(always)]
    pub fn fasterr(&mut self) -> FASTERR_W<'_, SRrs> {
        FASTERR_W::new(self, 9)
    }
    ///Bit 14 - PCROP read error
    #[inline(always)]
    pub fn rderr(&mut self) -> RDERR_W<'_, SRrs> {
        RDERR_W::new(self, 14)
    }
    ///Bit 15 - Option validity error
    #[inline(always)]
    pub fn optverr(&mut self) -> OPTVERR_W<'_, SRrs> {
        OPTVERR_W::new(self, 15)
    }
}
/**Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
