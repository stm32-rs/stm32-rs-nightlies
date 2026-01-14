///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
/**TXIS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXIS {
    ///0: New Tx data write not required
    NotRequired = 0,
    ///1: New Tx data write required
    Required = 1,
}
impl From<TXIS> for bool {
    #[inline(always)]
    fn from(variant: TXIS) -> Self {
        variant as u8 != 0
    }
}
///Field `TXIS` reader - TXIS
pub type TXIS_R = crate::BitReader<TXIS>;
impl TXIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXIS {
        match self.bits {
            false => TXIS::NotRequired,
            true => TXIS::Required,
        }
    }
    ///New Tx data write not required
    #[inline(always)]
    pub fn is_not_required(&self) -> bool {
        *self == TXIS::NotRequired
    }
    ///New Tx data write required
    #[inline(always)]
    pub fn is_required(&self) -> bool {
        *self == TXIS::Required
    }
}
///Field `TXIS` writer - TXIS
pub type TXIS_W<'a, REG> = crate::BitWriter<'a, REG, TXIS>;
impl<'a, REG> TXIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///New Tx data write not required
    #[inline(always)]
    pub fn not_required(self) -> &'a mut crate::W<REG> {
        self.variant(TXIS::NotRequired)
    }
    ///New Tx data write required
    #[inline(always)]
    pub fn required(self) -> &'a mut crate::W<REG> {
        self.variant(TXIS::Required)
    }
}
/**TXMSGDISC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXMSGDISC {
    ///0: No Tx message discarded
    NotDiscarded = 0,
    ///1: Tx message discarded
    Discarded = 1,
}
impl From<TXMSGDISC> for bool {
    #[inline(always)]
    fn from(variant: TXMSGDISC) -> Self {
        variant as u8 != 0
    }
}
///Field `TXMSGDISC` reader - TXMSGDISC
pub type TXMSGDISC_R = crate::BitReader<TXMSGDISC>;
impl TXMSGDISC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXMSGDISC {
        match self.bits {
            false => TXMSGDISC::NotDiscarded,
            true => TXMSGDISC::Discarded,
        }
    }
    ///No Tx message discarded
    #[inline(always)]
    pub fn is_not_discarded(&self) -> bool {
        *self == TXMSGDISC::NotDiscarded
    }
    ///Tx message discarded
    #[inline(always)]
    pub fn is_discarded(&self) -> bool {
        *self == TXMSGDISC::Discarded
    }
}
///Field `TXMSGDISC` writer - TXMSGDISC
pub type TXMSGDISC_W<'a, REG> = crate::BitWriter<'a, REG, TXMSGDISC>;
impl<'a, REG> TXMSGDISC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No Tx message discarded
    #[inline(always)]
    pub fn not_discarded(self) -> &'a mut crate::W<REG> {
        self.variant(TXMSGDISC::NotDiscarded)
    }
    ///Tx message discarded
    #[inline(always)]
    pub fn discarded(self) -> &'a mut crate::W<REG> {
        self.variant(TXMSGDISC::Discarded)
    }
}
/**TXMSGSENT

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXMSGSENT {
    ///0: No Tx message completed
    NotCompleted = 0,
    ///1: Tx message completed
    Completed = 1,
}
impl From<TXMSGSENT> for bool {
    #[inline(always)]
    fn from(variant: TXMSGSENT) -> Self {
        variant as u8 != 0
    }
}
///Field `TXMSGSENT` reader - TXMSGSENT
pub type TXMSGSENT_R = crate::BitReader<TXMSGSENT>;
impl TXMSGSENT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXMSGSENT {
        match self.bits {
            false => TXMSGSENT::NotCompleted,
            true => TXMSGSENT::Completed,
        }
    }
    ///No Tx message completed
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == TXMSGSENT::NotCompleted
    }
    ///Tx message completed
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == TXMSGSENT::Completed
    }
}
///Field `TXMSGSENT` writer - TXMSGSENT
pub type TXMSGSENT_W<'a, REG> = crate::BitWriter<'a, REG, TXMSGSENT>;
impl<'a, REG> TXMSGSENT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No Tx message completed
    #[inline(always)]
    pub fn not_completed(self) -> &'a mut crate::W<REG> {
        self.variant(TXMSGSENT::NotCompleted)
    }
    ///Tx message completed
    #[inline(always)]
    pub fn completed(self) -> &'a mut crate::W<REG> {
        self.variant(TXMSGSENT::Completed)
    }
}
/**TXMSGABT

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXMSGABT {
    ///0: No transmit message abort
    NoAbort = 0,
    ///1: Transmit message abort
    Abort = 1,
}
impl From<TXMSGABT> for bool {
    #[inline(always)]
    fn from(variant: TXMSGABT) -> Self {
        variant as u8 != 0
    }
}
///Field `TXMSGABT` reader - TXMSGABT
pub type TXMSGABT_R = crate::BitReader<TXMSGABT>;
impl TXMSGABT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXMSGABT {
        match self.bits {
            false => TXMSGABT::NoAbort,
            true => TXMSGABT::Abort,
        }
    }
    ///No transmit message abort
    #[inline(always)]
    pub fn is_no_abort(&self) -> bool {
        *self == TXMSGABT::NoAbort
    }
    ///Transmit message abort
    #[inline(always)]
    pub fn is_abort(&self) -> bool {
        *self == TXMSGABT::Abort
    }
}
///Field `TXMSGABT` writer - TXMSGABT
pub type TXMSGABT_W<'a, REG> = crate::BitWriter<'a, REG, TXMSGABT>;
impl<'a, REG> TXMSGABT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No transmit message abort
    #[inline(always)]
    pub fn no_abort(self) -> &'a mut crate::W<REG> {
        self.variant(TXMSGABT::NoAbort)
    }
    ///Transmit message abort
    #[inline(always)]
    pub fn abort(self) -> &'a mut crate::W<REG> {
        self.variant(TXMSGABT::Abort)
    }
}
/**HRSTDISC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRSTDISC {
    ///0: No Hard Reset discarded
    NotDiscarded = 0,
    ///1: Hard Reset discarded
    Discarded = 1,
}
impl From<HRSTDISC> for bool {
    #[inline(always)]
    fn from(variant: HRSTDISC) -> Self {
        variant as u8 != 0
    }
}
///Field `HRSTDISC` reader - HRSTDISC
pub type HRSTDISC_R = crate::BitReader<HRSTDISC>;
impl HRSTDISC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HRSTDISC {
        match self.bits {
            false => HRSTDISC::NotDiscarded,
            true => HRSTDISC::Discarded,
        }
    }
    ///No Hard Reset discarded
    #[inline(always)]
    pub fn is_not_discarded(&self) -> bool {
        *self == HRSTDISC::NotDiscarded
    }
    ///Hard Reset discarded
    #[inline(always)]
    pub fn is_discarded(&self) -> bool {
        *self == HRSTDISC::Discarded
    }
}
///Field `HRSTDISC` writer - HRSTDISC
pub type HRSTDISC_W<'a, REG> = crate::BitWriter<'a, REG, HRSTDISC>;
impl<'a, REG> HRSTDISC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No Hard Reset discarded
    #[inline(always)]
    pub fn not_discarded(self) -> &'a mut crate::W<REG> {
        self.variant(HRSTDISC::NotDiscarded)
    }
    ///Hard Reset discarded
    #[inline(always)]
    pub fn discarded(self) -> &'a mut crate::W<REG> {
        self.variant(HRSTDISC::Discarded)
    }
}
/**HRSTSENT

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRSTSENT {
    ///0: No Hard Reset message sent
    NotSent = 0,
    ///1: Hard Reset message sent
    Sent = 1,
}
impl From<HRSTSENT> for bool {
    #[inline(always)]
    fn from(variant: HRSTSENT) -> Self {
        variant as u8 != 0
    }
}
///Field `HRSTSENT` reader - HRSTSENT
pub type HRSTSENT_R = crate::BitReader<HRSTSENT>;
impl HRSTSENT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HRSTSENT {
        match self.bits {
            false => HRSTSENT::NotSent,
            true => HRSTSENT::Sent,
        }
    }
    ///No Hard Reset message sent
    #[inline(always)]
    pub fn is_not_sent(&self) -> bool {
        *self == HRSTSENT::NotSent
    }
    ///Hard Reset message sent
    #[inline(always)]
    pub fn is_sent(&self) -> bool {
        *self == HRSTSENT::Sent
    }
}
///Field `HRSTSENT` writer - HRSTSENT
pub type HRSTSENT_W<'a, REG> = crate::BitWriter<'a, REG, HRSTSENT>;
impl<'a, REG> HRSTSENT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No Hard Reset message sent
    #[inline(always)]
    pub fn not_sent(self) -> &'a mut crate::W<REG> {
        self.variant(HRSTSENT::NotSent)
    }
    ///Hard Reset message sent
    #[inline(always)]
    pub fn sent(self) -> &'a mut crate::W<REG> {
        self.variant(HRSTSENT::Sent)
    }
}
/**TXUND

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXUND {
    ///0: No Tx data underrun detected
    NoUnderrun = 0,
    ///1: Tx data underrun detected
    Underrun = 1,
}
impl From<TXUND> for bool {
    #[inline(always)]
    fn from(variant: TXUND) -> Self {
        variant as u8 != 0
    }
}
///Field `TXUND` reader - TXUND
pub type TXUND_R = crate::BitReader<TXUND>;
impl TXUND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXUND {
        match self.bits {
            false => TXUND::NoUnderrun,
            true => TXUND::Underrun,
        }
    }
    ///No Tx data underrun detected
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == TXUND::NoUnderrun
    }
    ///Tx data underrun detected
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == TXUND::Underrun
    }
}
///Field `TXUND` writer - TXUND
pub type TXUND_W<'a, REG> = crate::BitWriter<'a, REG, TXUND>;
impl<'a, REG> TXUND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No Tx data underrun detected
    #[inline(always)]
    pub fn no_underrun(self) -> &'a mut crate::W<REG> {
        self.variant(TXUND::NoUnderrun)
    }
    ///Tx data underrun detected
    #[inline(always)]
    pub fn underrun(self) -> &'a mut crate::W<REG> {
        self.variant(TXUND::Underrun)
    }
}
/**RXNE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNE {
    ///0: Rx data register empty
    Empty = 0,
    ///1: Rx data register not empty
    NotEmpty = 1,
}
impl From<RXNE> for bool {
    #[inline(always)]
    fn from(variant: RXNE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXNE` reader - RXNE
pub type RXNE_R = crate::BitReader<RXNE>;
impl RXNE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXNE {
        match self.bits {
            false => RXNE::Empty,
            true => RXNE::NotEmpty,
        }
    }
    ///Rx data register empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXNE::Empty
    }
    ///Rx data register not empty
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXNE::NotEmpty
    }
}
///Field `RXNE` writer - RXNE
pub type RXNE_W<'a, REG> = crate::BitWriter<'a, REG, RXNE>;
impl<'a, REG> RXNE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rx data register empty
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(RXNE::Empty)
    }
    ///Rx data register not empty
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(RXNE::NotEmpty)
    }
}
/**RXORDDET

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXORDDET {
    ///0: No ordered set detected
    NoOrderedSet = 0,
    ///1: Ordered set detected
    OrderedSet = 1,
}
impl From<RXORDDET> for bool {
    #[inline(always)]
    fn from(variant: RXORDDET) -> Self {
        variant as u8 != 0
    }
}
///Field `RXORDDET` reader - RXORDDET
pub type RXORDDET_R = crate::BitReader<RXORDDET>;
impl RXORDDET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXORDDET {
        match self.bits {
            false => RXORDDET::NoOrderedSet,
            true => RXORDDET::OrderedSet,
        }
    }
    ///No ordered set detected
    #[inline(always)]
    pub fn is_no_ordered_set(&self) -> bool {
        *self == RXORDDET::NoOrderedSet
    }
    ///Ordered set detected
    #[inline(always)]
    pub fn is_ordered_set(&self) -> bool {
        *self == RXORDDET::OrderedSet
    }
}
///Field `RXORDDET` writer - RXORDDET
pub type RXORDDET_W<'a, REG> = crate::BitWriter<'a, REG, RXORDDET>;
impl<'a, REG> RXORDDET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No ordered set detected
    #[inline(always)]
    pub fn no_ordered_set(self) -> &'a mut crate::W<REG> {
        self.variant(RXORDDET::NoOrderedSet)
    }
    ///Ordered set detected
    #[inline(always)]
    pub fn ordered_set(self) -> &'a mut crate::W<REG> {
        self.variant(RXORDDET::OrderedSet)
    }
}
/**RXHRSTDET

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXHRSTDET {
    ///0: Hard Reset not received
    NoHardReset = 0,
    ///1: Hard Reset received
    HardReset = 1,
}
impl From<RXHRSTDET> for bool {
    #[inline(always)]
    fn from(variant: RXHRSTDET) -> Self {
        variant as u8 != 0
    }
}
///Field `RXHRSTDET` reader - RXHRSTDET
pub type RXHRSTDET_R = crate::BitReader<RXHRSTDET>;
impl RXHRSTDET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXHRSTDET {
        match self.bits {
            false => RXHRSTDET::NoHardReset,
            true => RXHRSTDET::HardReset,
        }
    }
    ///Hard Reset not received
    #[inline(always)]
    pub fn is_no_hard_reset(&self) -> bool {
        *self == RXHRSTDET::NoHardReset
    }
    ///Hard Reset received
    #[inline(always)]
    pub fn is_hard_reset(&self) -> bool {
        *self == RXHRSTDET::HardReset
    }
}
///Field `RXHRSTDET` writer - RXHRSTDET
pub type RXHRSTDET_W<'a, REG> = crate::BitWriter<'a, REG, RXHRSTDET>;
impl<'a, REG> RXHRSTDET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Hard Reset not received
    #[inline(always)]
    pub fn no_hard_reset(self) -> &'a mut crate::W<REG> {
        self.variant(RXHRSTDET::NoHardReset)
    }
    ///Hard Reset received
    #[inline(always)]
    pub fn hard_reset(self) -> &'a mut crate::W<REG> {
        self.variant(RXHRSTDET::HardReset)
    }
}
/**RXOVR

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXOVR {
    ///0: No overflow
    NoOverflow = 0,
    ///1: Overflow
    Overflow = 1,
}
impl From<RXOVR> for bool {
    #[inline(always)]
    fn from(variant: RXOVR) -> Self {
        variant as u8 != 0
    }
}
///Field `RXOVR` reader - RXOVR
pub type RXOVR_R = crate::BitReader<RXOVR>;
impl RXOVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXOVR {
        match self.bits {
            false => RXOVR::NoOverflow,
            true => RXOVR::Overflow,
        }
    }
    ///No overflow
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == RXOVR::NoOverflow
    }
    ///Overflow
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == RXOVR::Overflow
    }
}
///Field `RXOVR` writer - RXOVR
pub type RXOVR_W<'a, REG> = crate::BitWriter<'a, REG, RXOVR>;
impl<'a, REG> RXOVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No overflow
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(RXOVR::NoOverflow)
    }
    ///Overflow
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(RXOVR::Overflow)
    }
}
/**RXMSGEND

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXMSGEND {
    ///0: No new Rx message received
    NoNewMessage = 0,
    ///1: A new Rx message received
    NewMessage = 1,
}
impl From<RXMSGEND> for bool {
    #[inline(always)]
    fn from(variant: RXMSGEND) -> Self {
        variant as u8 != 0
    }
}
///Field `RXMSGEND` reader - RXMSGEND
pub type RXMSGEND_R = crate::BitReader<RXMSGEND>;
impl RXMSGEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXMSGEND {
        match self.bits {
            false => RXMSGEND::NoNewMessage,
            true => RXMSGEND::NewMessage,
        }
    }
    ///No new Rx message received
    #[inline(always)]
    pub fn is_no_new_message(&self) -> bool {
        *self == RXMSGEND::NoNewMessage
    }
    ///A new Rx message received
    #[inline(always)]
    pub fn is_new_message(&self) -> bool {
        *self == RXMSGEND::NewMessage
    }
}
///Field `RXMSGEND` writer - RXMSGEND
pub type RXMSGEND_W<'a, REG> = crate::BitWriter<'a, REG, RXMSGEND>;
impl<'a, REG> RXMSGEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No new Rx message received
    #[inline(always)]
    pub fn no_new_message(self) -> &'a mut crate::W<REG> {
        self.variant(RXMSGEND::NoNewMessage)
    }
    ///A new Rx message received
    #[inline(always)]
    pub fn new_message(self) -> &'a mut crate::W<REG> {
        self.variant(RXMSGEND::NewMessage)
    }
}
/**RXERR

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXERR {
    ///0: No error detected
    NoError = 0,
    ///1: Error(s) detected
    Error = 1,
}
impl From<RXERR> for bool {
    #[inline(always)]
    fn from(variant: RXERR) -> Self {
        variant as u8 != 0
    }
}
///Field `RXERR` reader - RXERR
pub type RXERR_R = crate::BitReader<RXERR>;
impl RXERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXERR {
        match self.bits {
            false => RXERR::NoError,
            true => RXERR::Error,
        }
    }
    ///No error detected
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RXERR::NoError
    }
    ///Error(s) detected
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RXERR::Error
    }
}
///Field `RXERR` writer - RXERR
pub type RXERR_W<'a, REG> = crate::BitWriter<'a, REG, RXERR>;
impl<'a, REG> RXERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No error detected
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(RXERR::NoError)
    }
    ///Error(s) detected
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(RXERR::Error)
    }
}
/**TYPECEVT1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TYPECEVT1 {
    ///0: No new event
    NoNewEvent = 0,
    ///1: A new Type-C event occurred
    NewEvent = 1,
}
impl From<TYPECEVT1> for bool {
    #[inline(always)]
    fn from(variant: TYPECEVT1) -> Self {
        variant as u8 != 0
    }
}
///Field `TYPECEVT1` reader - TYPECEVT1
pub type TYPECEVT1_R = crate::BitReader<TYPECEVT1>;
impl TYPECEVT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TYPECEVT1 {
        match self.bits {
            false => TYPECEVT1::NoNewEvent,
            true => TYPECEVT1::NewEvent,
        }
    }
    ///No new event
    #[inline(always)]
    pub fn is_no_new_event(&self) -> bool {
        *self == TYPECEVT1::NoNewEvent
    }
    ///A new Type-C event occurred
    #[inline(always)]
    pub fn is_new_event(&self) -> bool {
        *self == TYPECEVT1::NewEvent
    }
}
///Field `TYPECEVT1` writer - TYPECEVT1
pub type TYPECEVT1_W<'a, REG> = crate::BitWriter<'a, REG, TYPECEVT1>;
impl<'a, REG> TYPECEVT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No new event
    #[inline(always)]
    pub fn no_new_event(self) -> &'a mut crate::W<REG> {
        self.variant(TYPECEVT1::NoNewEvent)
    }
    ///A new Type-C event occurred
    #[inline(always)]
    pub fn new_event(self) -> &'a mut crate::W<REG> {
        self.variant(TYPECEVT1::NewEvent)
    }
}
///Field `TYPECEVT2` reader - TYPECEVT2
pub use TYPECEVT1_R as TYPECEVT2_R;
///Field `TYPECEVT2` writer - TYPECEVT2
pub use TYPECEVT1_W as TYPECEVT2_W;
/**TYPEC_VSTATE_CC1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TYPEC_VSTATE_CC1 {
    ///0: Lowest
    Lowest = 0,
    ///1: Low
    Low = 1,
    ///2: High
    High = 2,
    ///3: Highest
    Highest = 3,
}
impl From<TYPEC_VSTATE_CC1> for u8 {
    #[inline(always)]
    fn from(variant: TYPEC_VSTATE_CC1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TYPEC_VSTATE_CC1 {
    type Ux = u8;
}
impl crate::IsEnum for TYPEC_VSTATE_CC1 {}
///Field `TYPEC_VSTATE_CC1` reader - TYPEC_VSTATE_CC1
pub type TYPEC_VSTATE_CC1_R = crate::FieldReader<TYPEC_VSTATE_CC1>;
impl TYPEC_VSTATE_CC1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TYPEC_VSTATE_CC1 {
        match self.bits {
            0 => TYPEC_VSTATE_CC1::Lowest,
            1 => TYPEC_VSTATE_CC1::Low,
            2 => TYPEC_VSTATE_CC1::High,
            3 => TYPEC_VSTATE_CC1::Highest,
            _ => unreachable!(),
        }
    }
    ///Lowest
    #[inline(always)]
    pub fn is_lowest(&self) -> bool {
        *self == TYPEC_VSTATE_CC1::Lowest
    }
    ///Low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == TYPEC_VSTATE_CC1::Low
    }
    ///High
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == TYPEC_VSTATE_CC1::High
    }
    ///Highest
    #[inline(always)]
    pub fn is_highest(&self) -> bool {
        *self == TYPEC_VSTATE_CC1::Highest
    }
}
///Field `TYPEC_VSTATE_CC1` writer - TYPEC_VSTATE_CC1
pub type TYPEC_VSTATE_CC1_W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, TYPEC_VSTATE_CC1, crate::Safe>;
impl<'a, REG> TYPEC_VSTATE_CC1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Lowest
    #[inline(always)]
    pub fn lowest(self) -> &'a mut crate::W<REG> {
        self.variant(TYPEC_VSTATE_CC1::Lowest)
    }
    ///Low
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(TYPEC_VSTATE_CC1::Low)
    }
    ///High
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(TYPEC_VSTATE_CC1::High)
    }
    ///Highest
    #[inline(always)]
    pub fn highest(self) -> &'a mut crate::W<REG> {
        self.variant(TYPEC_VSTATE_CC1::Highest)
    }
}
///Field `TYPEC_VSTATE_CC2` reader - TYPEC_VSTATE_CC2
pub use TYPEC_VSTATE_CC1_R as TYPEC_VSTATE_CC2_R;
///Field `TYPEC_VSTATE_CC2` writer - TYPEC_VSTATE_CC2
pub use TYPEC_VSTATE_CC1_W as TYPEC_VSTATE_CC2_W;
/**FRSEVT

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRSEVT {
    ///0: No new event
    NoNewEvent = 0,
    ///1: New FRS receive event occurred
    NewEvent = 1,
}
impl From<FRSEVT> for bool {
    #[inline(always)]
    fn from(variant: FRSEVT) -> Self {
        variant as u8 != 0
    }
}
///Field `FRSEVT` reader - FRSEVT
pub type FRSEVT_R = crate::BitReader<FRSEVT>;
impl FRSEVT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FRSEVT {
        match self.bits {
            false => FRSEVT::NoNewEvent,
            true => FRSEVT::NewEvent,
        }
    }
    ///No new event
    #[inline(always)]
    pub fn is_no_new_event(&self) -> bool {
        *self == FRSEVT::NoNewEvent
    }
    ///New FRS receive event occurred
    #[inline(always)]
    pub fn is_new_event(&self) -> bool {
        *self == FRSEVT::NewEvent
    }
}
///Field `FRSEVT` writer - FRSEVT
pub type FRSEVT_W<'a, REG> = crate::BitWriter<'a, REG, FRSEVT>;
impl<'a, REG> FRSEVT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No new event
    #[inline(always)]
    pub fn no_new_event(self) -> &'a mut crate::W<REG> {
        self.variant(FRSEVT::NoNewEvent)
    }
    ///New FRS receive event occurred
    #[inline(always)]
    pub fn new_event(self) -> &'a mut crate::W<REG> {
        self.variant(FRSEVT::NewEvent)
    }
}
impl R {
    ///Bit 0 - TXIS
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TXMSGDISC
    #[inline(always)]
    pub fn txmsgdisc(&self) -> TXMSGDISC_R {
        TXMSGDISC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TXMSGSENT
    #[inline(always)]
    pub fn txmsgsent(&self) -> TXMSGSENT_R {
        TXMSGSENT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TXMSGABT
    #[inline(always)]
    pub fn txmsgabt(&self) -> TXMSGABT_R {
        TXMSGABT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HRSTDISC
    #[inline(always)]
    pub fn hrstdisc(&self) -> HRSTDISC_R {
        HRSTDISC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HRSTSENT
    #[inline(always)]
    pub fn hrstsent(&self) -> HRSTSENT_R {
        HRSTSENT_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TXUND
    #[inline(always)]
    pub fn txund(&self) -> TXUND_R {
        TXUND_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - RXNE
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - RXORDDET
    #[inline(always)]
    pub fn rxorddet(&self) -> RXORDDET_R {
        RXORDDET_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RXHRSTDET
    #[inline(always)]
    pub fn rxhrstdet(&self) -> RXHRSTDET_R {
        RXHRSTDET_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - RXOVR
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - RXMSGEND
    #[inline(always)]
    pub fn rxmsgend(&self) -> RXMSGEND_R {
        RXMSGEND_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RXERR
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - TYPECEVT1
    #[inline(always)]
    pub fn typecevt1(&self) -> TYPECEVT1_R {
        TYPECEVT1_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TYPECEVT2
    #[inline(always)]
    pub fn typecevt2(&self) -> TYPECEVT2_R {
        TYPECEVT2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - TYPEC_VSTATE_CC1
    #[inline(always)]
    pub fn typec_vstate_cc1(&self) -> TYPEC_VSTATE_CC1_R {
        TYPEC_VSTATE_CC1_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - TYPEC_VSTATE_CC2
    #[inline(always)]
    pub fn typec_vstate_cc2(&self) -> TYPEC_VSTATE_CC2_R {
        TYPEC_VSTATE_CC2_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 20 - FRSEVT
    #[inline(always)]
    pub fn frsevt(&self) -> FRSEVT_R {
        FRSEVT_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("txis", &self.txis())
            .field("txmsgdisc", &self.txmsgdisc())
            .field("txmsgsent", &self.txmsgsent())
            .field("txmsgabt", &self.txmsgabt())
            .field("hrstdisc", &self.hrstdisc())
            .field("hrstsent", &self.hrstsent())
            .field("txund", &self.txund())
            .field("rxne", &self.rxne())
            .field("rxorddet", &self.rxorddet())
            .field("rxhrstdet", &self.rxhrstdet())
            .field("rxovr", &self.rxovr())
            .field("rxmsgend", &self.rxmsgend())
            .field("rxerr", &self.rxerr())
            .field("typecevt1", &self.typecevt1())
            .field("typecevt2", &self.typecevt2())
            .field("typec_vstate_cc1", &self.typec_vstate_cc1())
            .field("typec_vstate_cc2", &self.typec_vstate_cc2())
            .field("frsevt", &self.frsevt())
            .finish()
    }
}
impl W {
    ///Bit 0 - TXIS
    #[inline(always)]
    pub fn txis(&mut self) -> TXIS_W<'_, SRrs> {
        TXIS_W::new(self, 0)
    }
    ///Bit 1 - TXMSGDISC
    #[inline(always)]
    pub fn txmsgdisc(&mut self) -> TXMSGDISC_W<'_, SRrs> {
        TXMSGDISC_W::new(self, 1)
    }
    ///Bit 2 - TXMSGSENT
    #[inline(always)]
    pub fn txmsgsent(&mut self) -> TXMSGSENT_W<'_, SRrs> {
        TXMSGSENT_W::new(self, 2)
    }
    ///Bit 3 - TXMSGABT
    #[inline(always)]
    pub fn txmsgabt(&mut self) -> TXMSGABT_W<'_, SRrs> {
        TXMSGABT_W::new(self, 3)
    }
    ///Bit 4 - HRSTDISC
    #[inline(always)]
    pub fn hrstdisc(&mut self) -> HRSTDISC_W<'_, SRrs> {
        HRSTDISC_W::new(self, 4)
    }
    ///Bit 5 - HRSTSENT
    #[inline(always)]
    pub fn hrstsent(&mut self) -> HRSTSENT_W<'_, SRrs> {
        HRSTSENT_W::new(self, 5)
    }
    ///Bit 6 - TXUND
    #[inline(always)]
    pub fn txund(&mut self) -> TXUND_W<'_, SRrs> {
        TXUND_W::new(self, 6)
    }
    ///Bit 8 - RXNE
    #[inline(always)]
    pub fn rxne(&mut self) -> RXNE_W<'_, SRrs> {
        RXNE_W::new(self, 8)
    }
    ///Bit 9 - RXORDDET
    #[inline(always)]
    pub fn rxorddet(&mut self) -> RXORDDET_W<'_, SRrs> {
        RXORDDET_W::new(self, 9)
    }
    ///Bit 10 - RXHRSTDET
    #[inline(always)]
    pub fn rxhrstdet(&mut self) -> RXHRSTDET_W<'_, SRrs> {
        RXHRSTDET_W::new(self, 10)
    }
    ///Bit 11 - RXOVR
    #[inline(always)]
    pub fn rxovr(&mut self) -> RXOVR_W<'_, SRrs> {
        RXOVR_W::new(self, 11)
    }
    ///Bit 12 - RXMSGEND
    #[inline(always)]
    pub fn rxmsgend(&mut self) -> RXMSGEND_W<'_, SRrs> {
        RXMSGEND_W::new(self, 12)
    }
    ///Bit 13 - RXERR
    #[inline(always)]
    pub fn rxerr(&mut self) -> RXERR_W<'_, SRrs> {
        RXERR_W::new(self, 13)
    }
    ///Bit 14 - TYPECEVT1
    #[inline(always)]
    pub fn typecevt1(&mut self) -> TYPECEVT1_W<'_, SRrs> {
        TYPECEVT1_W::new(self, 14)
    }
    ///Bit 15 - TYPECEVT2
    #[inline(always)]
    pub fn typecevt2(&mut self) -> TYPECEVT2_W<'_, SRrs> {
        TYPECEVT2_W::new(self, 15)
    }
    ///Bits 16:17 - TYPEC_VSTATE_CC1
    #[inline(always)]
    pub fn typec_vstate_cc1(&mut self) -> TYPEC_VSTATE_CC1_W<'_, SRrs> {
        TYPEC_VSTATE_CC1_W::new(self, 16)
    }
    ///Bits 18:19 - TYPEC_VSTATE_CC2
    #[inline(always)]
    pub fn typec_vstate_cc2(&mut self) -> TYPEC_VSTATE_CC2_W<'_, SRrs> {
        TYPEC_VSTATE_CC2_W::new(self, 18)
    }
    ///Bit 20 - FRSEVT
    #[inline(always)]
    pub fn frsevt(&mut self) -> FRSEVT_W<'_, SRrs> {
        FRSEVT_W::new(self, 20)
    }
}
/**UCPD Status Register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G4A1.html#UCPD1:SR)*/
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
