#[doc = "Register `ISTR` reader"]
pub struct R(crate::R<ISTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISTR` writer"]
pub struct W(crate::W<ISTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ISTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP_ID` reader - Endpoint Identifier"]
pub struct EP_ID_R(crate::FieldReader<u8, u8>);
impl EP_ID_R {
    pub(crate) fn new(bits: u8) -> Self {
        EP_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_ID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Direction of transaction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR_A {
    #[doc = "0: data transmitted by the USB peripheral to the host PC"]
    TO = 0,
    #[doc = "1: data received by the USB peripheral from the host PC"]
    FROM = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Direction of transaction"]
pub struct DIR_R(crate::FieldReader<bool, DIR_A>);
impl DIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::TO,
            true => DIR_A::FROM,
        }
    }
    #[doc = "Checks if the value of the field is `TO`"]
    #[inline(always)]
    pub fn is_to(&self) -> bool {
        **self == DIR_A::TO
    }
    #[doc = "Checks if the value of the field is `FROM`"]
    #[inline(always)]
    pub fn is_from(&self) -> bool {
        **self == DIR_A::FROM
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<bool, DIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LPM L1 state request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L1REQ_A {
    #[doc = "1: LPM command to enter the L1 state is successfully received and acknowledged"]
    RECEIVED = 1,
}
impl From<L1REQ_A> for bool {
    #[inline(always)]
    fn from(variant: L1REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L1REQ` reader - LPM L1 state request"]
pub struct L1REQ_R(crate::FieldReader<bool, L1REQ_A>);
impl L1REQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        L1REQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<L1REQ_A> {
        match self.bits {
            true => Some(L1REQ_A::RECEIVED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        **self == L1REQ_A::RECEIVED
    }
}
impl core::ops::Deref for L1REQ_R {
    type Target = crate::FieldReader<bool, L1REQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L1REQ` writer - LPM L1 state request"]
pub struct L1REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> L1REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L1REQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LPM command to enter the L1 state is successfully received and acknowledged"]
    #[inline(always)]
    pub fn received(self) -> &'a mut W {
        self.variant(L1REQ_A::RECEIVED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Expected start frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESOF_A {
    #[doc = "1: an SOF packet is expected but not received"]
    EXPECTEDSTARTOFFRAME = 1,
}
impl From<ESOF_A> for bool {
    #[inline(always)]
    fn from(variant: ESOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESOF` reader - Expected start frame"]
pub struct ESOF_R(crate::FieldReader<bool, ESOF_A>);
impl ESOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ESOF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ESOF_A> {
        match self.bits {
            true => Some(ESOF_A::EXPECTEDSTARTOFFRAME),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EXPECTEDSTARTOFFRAME`"]
    #[inline(always)]
    pub fn is_expected_start_of_frame(&self) -> bool {
        **self == ESOF_A::EXPECTEDSTARTOFFRAME
    }
}
impl core::ops::Deref for ESOF_R {
    type Target = crate::FieldReader<bool, ESOF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESOF` writer - Expected start frame"]
pub struct ESOF_W<'a> {
    w: &'a mut W,
}
impl<'a> ESOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ESOF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "an SOF packet is expected but not received"]
    #[inline(always)]
    pub fn expected_start_of_frame(self) -> &'a mut W {
        self.variant(ESOF_A::EXPECTEDSTARTOFFRAME)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "start of frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF_A {
    #[doc = "1: beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus"]
    STARTOFFRAME = 1,
}
impl From<SOF_A> for bool {
    #[inline(always)]
    fn from(variant: SOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF` reader - start of frame"]
pub struct SOF_R(crate::FieldReader<bool, SOF_A>);
impl SOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SOF_A> {
        match self.bits {
            true => Some(SOF_A::STARTOFFRAME),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STARTOFFRAME`"]
    #[inline(always)]
    pub fn is_start_of_frame(&self) -> bool {
        **self == SOF_A::STARTOFFRAME
    }
}
impl core::ops::Deref for SOF_R {
    type Target = crate::FieldReader<bool, SOF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF` writer - start of frame"]
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus"]
    #[inline(always)]
    pub fn start_of_frame(self) -> &'a mut W {
        self.variant(SOF_A::STARTOFFRAME)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "reset request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_A {
    #[doc = "1: peripheral detects an active USB RESET signal at its inputs"]
    RESET = 1,
}
impl From<RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` reader - reset request"]
pub struct RESET_R(crate::FieldReader<bool, RESET_A>);
impl RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RESET_A> {
        match self.bits {
            true => Some(RESET_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == RESET_A::RESET
    }
}
impl core::ops::Deref for RESET_R {
    type Target = crate::FieldReader<bool, RESET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET` writer - reset request"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "peripheral detects an active USB RESET signal at its inputs"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RESET_A::RESET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Suspend mode request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSP_A {
    #[doc = "1: no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus"]
    SUSPEND = 1,
}
impl From<SUSP_A> for bool {
    #[inline(always)]
    fn from(variant: SUSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSP` reader - Suspend mode request"]
pub struct SUSP_R(crate::FieldReader<bool, SUSP_A>);
impl SUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SUSP_A> {
        match self.bits {
            true => Some(SUSP_A::SUSPEND),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        **self == SUSP_A::SUSPEND
    }
}
impl core::ops::Deref for SUSP_R {
    type Target = crate::FieldReader<bool, SUSP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSP` writer - Suspend mode request"]
pub struct SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUSP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(SUSP_A::SUSPEND)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUP_A {
    #[doc = "1: activity is detected that wakes up the USB peripheral"]
    WAKEUP = 1,
}
impl From<WKUP_A> for bool {
    #[inline(always)]
    fn from(variant: WKUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUP` reader - Wakeup"]
pub struct WKUP_R(crate::FieldReader<bool, WKUP_A>);
impl WKUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WKUP_A> {
        match self.bits {
            true => Some(WKUP_A::WAKEUP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP`"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        **self == WKUP_A::WAKEUP
    }
}
impl core::ops::Deref for WKUP_R {
    type Target = crate::FieldReader<bool, WKUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUP` writer - Wakeup"]
pub struct WKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "activity is detected that wakes up the USB peripheral"]
    #[inline(always)]
    pub fn wakeup(self) -> &'a mut W {
        self.variant(WKUP_A::WAKEUP)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR_A {
    #[doc = "1: One of No ANSwer, Cyclic Redundancy Check, Bit Stuffing or Framing format Violation error occurred"]
    ERROR = 1,
}
impl From<ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR` reader - Error"]
pub struct ERR_R(crate::FieldReader<bool, ERR_A>);
impl ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ERR_A> {
        match self.bits {
            true => Some(ERR_A::ERROR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == ERR_A::ERROR
    }
}
impl core::ops::Deref for ERR_R {
    type Target = crate::FieldReader<bool, ERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR` writer - Error"]
pub struct ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "One of No ANSwer, Cyclic Redundancy Check, Bit Stuffing or Framing format Violation error occurred"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(ERR_A::ERROR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Packet memory area over / underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMAOVR_A {
    #[doc = "1: microcontroller has not been able to respond in time to an USB memory request"]
    OVERRUN = 1,
}
impl From<PMAOVR_A> for bool {
    #[inline(always)]
    fn from(variant: PMAOVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMAOVR` reader - Packet memory area over / underrun"]
pub struct PMAOVR_R(crate::FieldReader<bool, PMAOVR_A>);
impl PMAOVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMAOVR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PMAOVR_A> {
        match self.bits {
            true => Some(PMAOVR_A::OVERRUN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        **self == PMAOVR_A::OVERRUN
    }
}
impl core::ops::Deref for PMAOVR_R {
    type Target = crate::FieldReader<bool, PMAOVR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMAOVR` writer - Packet memory area over / underrun"]
pub struct PMAOVR_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAOVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMAOVR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "microcontroller has not been able to respond in time to an USB memory request"]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut W {
        self.variant(PMAOVR_A::OVERRUN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Correct transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTR_A {
    #[doc = "1: endpoint has successfully completed a transaction"]
    COMPLETED = 1,
}
impl From<CTR_A> for bool {
    #[inline(always)]
    fn from(variant: CTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTR` reader - Correct transfer"]
pub struct CTR_R(crate::FieldReader<bool, CTR_A>);
impl CTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTR_A> {
        match self.bits {
            true => Some(CTR_A::COMPLETED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        **self == CTR_A::COMPLETED
    }
}
impl core::ops::Deref for CTR_R {
    type Target = crate::FieldReader<bool, CTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Endpoint Identifier"]
    #[inline(always)]
    pub fn ep_id(&self) -> EP_ID_R {
        EP_ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Direction of transaction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LPM L1 state request"]
    #[inline(always)]
    pub fn l1req(&self) -> L1REQ_R {
        L1REQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Expected start frame"]
    #[inline(always)]
    pub fn esof(&self) -> ESOF_R {
        ESOF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - start of frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - reset request"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Suspend mode request"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Wakeup"]
    #[inline(always)]
    pub fn wkup(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Error"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun"]
    #[inline(always)]
    pub fn pmaovr(&self) -> PMAOVR_R {
        PMAOVR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Correct transfer"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - LPM L1 state request"]
    #[inline(always)]
    pub fn l1req(&mut self) -> L1REQ_W {
        L1REQ_W { w: self }
    }
    #[doc = "Bit 8 - Expected start frame"]
    #[inline(always)]
    pub fn esof(&mut self) -> ESOF_W {
        ESOF_W { w: self }
    }
    #[doc = "Bit 9 - start of frame"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W { w: self }
    }
    #[doc = "Bit 10 - reset request"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 11 - Suspend mode request"]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W { w: self }
    }
    #[doc = "Bit 12 - Wakeup"]
    #[inline(always)]
    pub fn wkup(&mut self) -> WKUP_W {
        WKUP_W { w: self }
    }
    #[doc = "Bit 13 - Error"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
    #[doc = "Bit 14 - Packet memory area over / underrun"]
    #[inline(always)]
    pub fn pmaovr(&mut self) -> PMAOVR_W {
        PMAOVR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [istr](index.html) module"]
pub struct ISTR_SPEC;
impl crate::RegisterSpec for ISTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [istr::R](R) reader structure"]
impl crate::Readable for ISTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [istr::W](W) writer structure"]
impl crate::Writable for ISTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISTR to value 0"]
impl crate::Resettable for ISTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
