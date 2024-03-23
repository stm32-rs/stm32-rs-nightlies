#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "End of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPR {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when one or more Flash memory operation (programming / erase) has been completed successfully"]
    Error = 1,
}
impl From<EOPR> for bool {
    #[inline(always)]
    fn from(variant: EOPR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOP` reader - End of operation"]
pub type EOP_R = crate::BitReader<EOPR>;
impl EOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOPR {
        match self.bits {
            false => EOPR::NoError,
            true => EOPR::Error,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == EOPR::NoError
    }
    #[doc = "Set by hardware when one or more Flash memory operation (programming / erase) has been completed successfully"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == EOPR::Error
    }
}
#[doc = "End of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPW {
    #[doc = "1: Cleared by writing 1"]
    Clear = 1,
}
impl From<EOPW> for bool {
    #[inline(always)]
    fn from(variant: EOPW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOP` writer - End of operation"]
pub type EOP_W<'a, REG> = crate::BitWriter<'a, REG, EOPW>;
impl<'a, REG> EOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOPW::Clear)
    }
}
#[doc = "Operation error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPERRR {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when a Flash memory operation (program / erase) completes unsuccessfully"]
    Error = 1,
}
impl From<OPERRR> for bool {
    #[inline(always)]
    fn from(variant: OPERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPERR` reader - Operation error"]
pub type OPERR_R = crate::BitReader<OPERRR>;
impl OPERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPERRR {
        match self.bits {
            false => OPERRR::NoError,
            true => OPERRR::Error,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == OPERRR::NoError
    }
    #[doc = "Set by hardware when a Flash memory operation (program / erase) completes unsuccessfully"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == OPERRR::Error
    }
}
#[doc = "Operation error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPERRW {
    #[doc = "1: Cleared by writing 1"]
    Clear = 1,
}
impl From<OPERRW> for bool {
    #[inline(always)]
    fn from(variant: OPERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPERR` writer - Operation error"]
pub type OPERR_W<'a, REG> = crate::BitWriter<'a, REG, OPERRW>;
impl<'a, REG> OPERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OPERRW::Clear)
    }
}
#[doc = "Programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROGERRR {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when a double-word address to be programmed contains a value different from '0xFFFF FFFF' before programming, except if the data to write is '0x0000 0000'"]
    Error = 1,
}
impl From<PROGERRR> for bool {
    #[inline(always)]
    fn from(variant: PROGERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROGERR` reader - Programming error"]
pub type PROGERR_R = crate::BitReader<PROGERRR>;
impl PROGERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PROGERRR {
        match self.bits {
            false => PROGERRR::NoError,
            true => PROGERRR::Error,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PROGERRR::NoError
    }
    #[doc = "Set by hardware when a double-word address to be programmed contains a value different from '0xFFFF FFFF' before programming, except if the data to write is '0x0000 0000'"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PROGERRR::Error
    }
}
#[doc = "Programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROGERRW {
    #[doc = "1: Cleared by writing 1"]
    Clear = 1,
}
impl From<PROGERRW> for bool {
    #[inline(always)]
    fn from(variant: PROGERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROGERR` writer - Programming error"]
pub type PROGERR_W<'a, REG> = crate::BitWriter<'a, REG, PROGERRW>;
impl<'a, REG> PROGERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PROGERRW::Clear)
    }
}
#[doc = "Write protected error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPERRR {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when an address to be erased/programmed belongs to a writeprotected part (by WRP, PCROP or RDP level 1) of the Flash memory"]
    Error = 1,
}
impl From<WRPERRR> for bool {
    #[inline(always)]
    fn from(variant: WRPERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRPERR` reader - Write protected error"]
pub type WRPERR_R = crate::BitReader<WRPERRR>;
impl WRPERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WRPERRR {
        match self.bits {
            false => WRPERRR::NoError,
            true => WRPERRR::Error,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WRPERRR::NoError
    }
    #[doc = "Set by hardware when an address to be erased/programmed belongs to a writeprotected part (by WRP, PCROP or RDP level 1) of the Flash memory"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WRPERRR::Error
    }
}
#[doc = "Write protected error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPERRW {
    #[doc = "1: Cleared by writing 1"]
    Clear = 1,
}
impl From<WRPERRW> for bool {
    #[inline(always)]
    fn from(variant: WRPERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRPERR` writer - Write protected error"]
pub type WRPERR_W<'a, REG> = crate::BitWriter<'a, REG, WRPERRW>;
impl<'a, REG> WRPERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WRPERRW::Clear)
    }
}
#[doc = "Programming alignment error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAERRR {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when the data to program cannot be contained in the same 64-bit Flash memory row in case of standard programming, or if there is a change of page during fast programming"]
    Error = 1,
}
impl From<PGAERRR> for bool {
    #[inline(always)]
    fn from(variant: PGAERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGAERR` reader - Programming alignment error"]
pub type PGAERR_R = crate::BitReader<PGAERRR>;
impl PGAERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PGAERRR {
        match self.bits {
            false => PGAERRR::NoError,
            true => PGAERRR::Error,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGAERRR::NoError
    }
    #[doc = "Set by hardware when the data to program cannot be contained in the same 64-bit Flash memory row in case of standard programming, or if there is a change of page during fast programming"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGAERRR::Error
    }
}
#[doc = "Programming alignment error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAERRW {
    #[doc = "1: Cleared by writing 1"]
    Clear = 1,
}
impl From<PGAERRW> for bool {
    #[inline(always)]
    fn from(variant: PGAERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGAERR` writer - Programming alignment error"]
pub type PGAERR_W<'a, REG> = crate::BitWriter<'a, REG, PGAERRW>;
impl<'a, REG> PGAERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PGAERRW::Clear)
    }
}
#[doc = "Size error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIZERRR {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when the size of the access is a byte or half-word during a program or a fast program sequence. Only double word programming is allowed (consequently: word access)"]
    Error = 1,
}
impl From<SIZERRR> for bool {
    #[inline(always)]
    fn from(variant: SIZERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIZERR` reader - Size error"]
pub type SIZERR_R = crate::BitReader<SIZERRR>;
impl SIZERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SIZERRR {
        match self.bits {
            false => SIZERRR::NoError,
            true => SIZERRR::Error,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == SIZERRR::NoError
    }
    #[doc = "Set by hardware when the size of the access is a byte or half-word during a program or a fast program sequence. Only double word programming is allowed (consequently: word access)"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SIZERRR::Error
    }
}
#[doc = "Size error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIZERRW {
    #[doc = "1: Cleared by writing 1"]
    Clear = 1,
}
impl From<SIZERRW> for bool {
    #[inline(always)]
    fn from(variant: SIZERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIZERR` writer - Size error"]
pub type SIZERR_W<'a, REG> = crate::BitWriter<'a, REG, SIZERRW>;
impl<'a, REG> SIZERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SIZERRW::Clear)
    }
}
#[doc = "Programming sequence error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGSERRR {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when a write access to the Flash memory is performed by the code while PG or FSTPG have not been set previously. Set also by hardware when PROGERR, SIZERR, PGAERR, WRPERR, MISSERR or FASTERR is set due to a previous programming error. Set also when trying to perform bank erase when DBANK=0 (or DB1M = 0)"]
    Error = 1,
}
impl From<PGSERRR> for bool {
    #[inline(always)]
    fn from(variant: PGSERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGSERR` reader - Programming sequence error"]
pub type PGSERR_R = crate::BitReader<PGSERRR>;
impl PGSERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PGSERRR {
        match self.bits {
            false => PGSERRR::NoError,
            true => PGSERRR::Error,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGSERRR::NoError
    }
    #[doc = "Set by hardware when a write access to the Flash memory is performed by the code while PG or FSTPG have not been set previously. Set also by hardware when PROGERR, SIZERR, PGAERR, WRPERR, MISSERR or FASTERR is set due to a previous programming error. Set also when trying to perform bank erase when DBANK=0 (or DB1M = 0)"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGSERRR::Error
    }
}
#[doc = "Programming sequence error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGSERRW {
    #[doc = "1: Cleared by writing 1"]
    Clear = 1,
}
impl From<PGSERRW> for bool {
    #[inline(always)]
    fn from(variant: PGSERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGSERR` writer - Programming sequence error"]
pub type PGSERR_W<'a, REG> = crate::BitWriter<'a, REG, PGSERRW>;
impl<'a, REG> PGSERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PGSERRW::Clear)
    }
}
#[doc = "Fast programming data miss error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MISERRR {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: In fast programming mode, 32 double words must be sent to Flash successively, and the new data must be sent to the Flash logic control before the current data is fully programmed. MISSERR is set by hardware when the new data is not present in time"]
    Error = 1,
}
impl From<MISERRR> for bool {
    #[inline(always)]
    fn from(variant: MISERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MISERR` reader - Fast programming data miss error"]
pub type MISERR_R = crate::BitReader<MISERRR>;
impl MISERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MISERRR {
        match self.bits {
            false => MISERRR::NoError,
            true => MISERRR::Error,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == MISERRR::NoError
    }
    #[doc = "In fast programming mode, 32 double words must be sent to Flash successively, and the new data must be sent to the Flash logic control before the current data is fully programmed. MISSERR is set by hardware when the new data is not present in time"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == MISERRR::Error
    }
}
#[doc = "Fast programming data miss error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MISERRW {
    #[doc = "1: Cleared by writing 1"]
    Clear = 1,
}
impl From<MISERRW> for bool {
    #[inline(always)]
    fn from(variant: MISERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MISERR` writer - Fast programming data miss error"]
pub type MISERR_W<'a, REG> = crate::BitWriter<'a, REG, MISERRW>;
impl<'a, REG> MISERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(MISERRW::Clear)
    }
}
#[doc = "Fast programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FASTERRR {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when a fast programming sequence (activated by FSTPG) is interrupted due to an error (alignment, size, write protection or data miss). The corresponding status bit (PGAERR, SIZERR, WRPERR or MISSERR) is set at the same time"]
    Error = 1,
}
impl From<FASTERRR> for bool {
    #[inline(always)]
    fn from(variant: FASTERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTERR` reader - Fast programming error"]
pub type FASTERR_R = crate::BitReader<FASTERRR>;
impl FASTERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FASTERRR {
        match self.bits {
            false => FASTERRR::NoError,
            true => FASTERRR::Error,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FASTERRR::NoError
    }
    #[doc = "Set by hardware when a fast programming sequence (activated by FSTPG) is interrupted due to an error (alignment, size, write protection or data miss). The corresponding status bit (PGAERR, SIZERR, WRPERR or MISSERR) is set at the same time"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FASTERRR::Error
    }
}
#[doc = "Fast programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FASTERRW {
    #[doc = "1: Cleared by writing 1"]
    Clear = 1,
}
impl From<FASTERRW> for bool {
    #[inline(always)]
    fn from(variant: FASTERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTERR` writer - Fast programming error"]
pub type FASTERR_W<'a, REG> = crate::BitWriter<'a, REG, FASTERRW>;
impl<'a, REG> FASTERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FASTERRW::Clear)
    }
}
#[doc = "PCROP read error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERRR {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when an address to be read through the D-bus belongs to a read protected area of the Flash (PCROP protection)"]
    Error = 1,
}
impl From<RDERRR> for bool {
    #[inline(always)]
    fn from(variant: RDERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDERR` reader - PCROP read error"]
pub type RDERR_R = crate::BitReader<RDERRR>;
impl RDERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDERRR {
        match self.bits {
            false => RDERRR::NoError,
            true => RDERRR::Error,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RDERRR::NoError
    }
    #[doc = "Set by hardware when an address to be read through the D-bus belongs to a read protected area of the Flash (PCROP protection)"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RDERRR::Error
    }
}
#[doc = "PCROP read error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERRW {
    #[doc = "1: Cleared by writing 1"]
    Clear = 1,
}
impl From<RDERRW> for bool {
    #[inline(always)]
    fn from(variant: RDERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDERR` writer - PCROP read error"]
pub type RDERR_W<'a, REG> = crate::BitWriter<'a, REG, RDERRW>;
impl<'a, REG> RDERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RDERRW::Clear)
    }
}
#[doc = "Option validity error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTVERRR {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when the options read may not be the one configured by the user. If option haven’t been properly loaded, OPTVERR is set again after each system reset"]
    Error = 1,
}
impl From<OPTVERRR> for bool {
    #[inline(always)]
    fn from(variant: OPTVERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTVERR` reader - Option validity error"]
pub type OPTVERR_R = crate::BitReader<OPTVERRR>;
impl OPTVERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPTVERRR {
        match self.bits {
            false => OPTVERRR::NoError,
            true => OPTVERRR::Error,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == OPTVERRR::NoError
    }
    #[doc = "Set by hardware when the options read may not be the one configured by the user. If option haven’t been properly loaded, OPTVERR is set again after each system reset"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == OPTVERRR::Error
    }
}
#[doc = "Option validity error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTVERRW {
    #[doc = "1: Cleared by writing 1"]
    Clear = 1,
}
impl From<OPTVERRW> for bool {
    #[inline(always)]
    fn from(variant: OPTVERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTVERR` writer - Option validity error"]
pub type OPTVERR_W<'a, REG> = crate::BitWriter<'a, REG, OPTVERRW>;
impl<'a, REG> OPTVERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OPTVERRW::Clear)
    }
}
#[doc = "Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSY {
    #[doc = "0: Not busy"]
    NotBusy = 0,
    #[doc = "1: Busy"]
    Busy = 1,
}
impl From<BSY> for bool {
    #[inline(always)]
    fn from(variant: BSY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSY` reader - Busy"]
pub type BSY_R = crate::BitReader<BSY>;
impl BSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BSY {
        match self.bits {
            false => BSY::NotBusy,
            true => BSY::Busy,
        }
    }
    #[doc = "Not busy"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BSY::NotBusy
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BSY::Busy
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEMPTY {
    #[doc = "0: The bit value is toggling"]
    Toggling = 0,
    #[doc = "1: No effect"]
    NoEffect = 1,
}
impl From<PEMPTY> for bool {
    #[inline(always)]
    fn from(variant: PEMPTY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEMPTY` reader - "]
pub type PEMPTY_R = crate::BitReader<PEMPTY>;
impl PEMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEMPTY {
        match self.bits {
            false => PEMPTY::Toggling,
            true => PEMPTY::NoEffect,
        }
    }
    #[doc = "The bit value is toggling"]
    #[inline(always)]
    pub fn is_toggling(&self) -> bool {
        *self == PEMPTY::Toggling
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PEMPTY::NoEffect
    }
}
#[doc = "Field `PEMPTY` writer - "]
pub type PEMPTY_W<'a, REG> = crate::BitWriter<'a, REG, PEMPTY>;
impl<'a, REG> PEMPTY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The bit value is toggling"]
    #[inline(always)]
    pub fn toggling(self) -> &'a mut crate::W<REG> {
        self.variant(PEMPTY::Toggling)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PEMPTY::NoEffect)
    }
}
impl R {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operation error"]
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Programming error"]
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protected error"]
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Programming alignment error"]
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Size error"]
    #[inline(always)]
    pub fn sizerr(&self) -> SIZERR_R {
        SIZERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Programming sequence error"]
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fast programming data miss error"]
    #[inline(always)]
    pub fn miserr(&self) -> MISERR_R {
        MISERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fast programming error"]
    #[inline(always)]
    pub fn fasterr(&self) -> FASTERR_R {
        FASTERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - PCROP read error"]
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Option validity error"]
    #[inline(always)]
    pub fn optverr(&self) -> OPTVERR_R {
        OPTVERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pempty(&self) -> PEMPTY_R {
        PEMPTY_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    #[must_use]
    pub fn eop(&mut self) -> EOP_W<SRrs> {
        EOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Operation error"]
    #[inline(always)]
    #[must_use]
    pub fn operr(&mut self) -> OPERR_W<SRrs> {
        OPERR_W::new(self, 1)
    }
    #[doc = "Bit 3 - Programming error"]
    #[inline(always)]
    #[must_use]
    pub fn progerr(&mut self) -> PROGERR_W<SRrs> {
        PROGERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write protected error"]
    #[inline(always)]
    #[must_use]
    pub fn wrperr(&mut self) -> WRPERR_W<SRrs> {
        WRPERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Programming alignment error"]
    #[inline(always)]
    #[must_use]
    pub fn pgaerr(&mut self) -> PGAERR_W<SRrs> {
        PGAERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Size error"]
    #[inline(always)]
    #[must_use]
    pub fn sizerr(&mut self) -> SIZERR_W<SRrs> {
        SIZERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Programming sequence error"]
    #[inline(always)]
    #[must_use]
    pub fn pgserr(&mut self) -> PGSERR_W<SRrs> {
        PGSERR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Fast programming data miss error"]
    #[inline(always)]
    #[must_use]
    pub fn miserr(&mut self) -> MISERR_W<SRrs> {
        MISERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Fast programming error"]
    #[inline(always)]
    #[must_use]
    pub fn fasterr(&mut self) -> FASTERR_W<SRrs> {
        FASTERR_W::new(self, 9)
    }
    #[doc = "Bit 14 - PCROP read error"]
    #[inline(always)]
    #[must_use]
    pub fn rderr(&mut self) -> RDERR_W<SRrs> {
        RDERR_W::new(self, 14)
    }
    #[doc = "Bit 15 - Option validity error"]
    #[inline(always)]
    #[must_use]
    pub fn optverr(&mut self) -> OPTVERR_W<SRrs> {
        OPTVERR_W::new(self, 15)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn pempty(&mut self) -> PEMPTY_W<SRrs> {
        PEMPTY_W::new(self, 17)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
