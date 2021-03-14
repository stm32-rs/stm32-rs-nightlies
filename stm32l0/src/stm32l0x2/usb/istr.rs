#[doc = "Reader of register ISTR"]
pub type R = crate::R<u32, super::ISTR>;
#[doc = "Writer for register ISTR"]
pub type W = crate::W<u32, super::ISTR>;
#[doc = "Register ISTR `reset()`'s with value 0"]
impl crate::ResetValue for super::ISTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CTR\n\nValue on reset: 0"]
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
#[doc = "Reader of field `CTR`"]
pub type CTR_R = crate::R<bool, CTR_A>;
impl CTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CTR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CTR_A::COMPLETED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == CTR_A::COMPLETED
    }
}
#[doc = "Write proxy for field `CTR`"]
pub struct CTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "endpoint has successfully completed a transaction"]
    #[inline(always)]
    pub fn completed(self) -> &'a mut W {
        self.variant(CTR_A::COMPLETED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "PMAOVR\n\nValue on reset: 0"]
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
#[doc = "Reader of field `PMAOVR`"]
pub type PMAOVR_R = crate::R<bool, PMAOVR_A>;
impl PMAOVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PMAOVR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PMAOVR_A::OVERRUN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == PMAOVR_A::OVERRUN
    }
}
#[doc = "Write proxy for field `PMAOVR`"]
pub struct PMAOVR_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAOVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMAOVR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "ERR\n\nValue on reset: 0"]
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
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, ERR_A>;
impl ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ERR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(ERR_A::ERROR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ERR_A::ERROR
    }
}
#[doc = "Write proxy for field `ERR`"]
pub struct ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "WKUP\n\nValue on reset: 0"]
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
#[doc = "Reader of field `WKUP`"]
pub type WKUP_R = crate::R<bool, WKUP_A>;
impl WKUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, WKUP_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(WKUP_A::WAKEUP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP`"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WKUP_A::WAKEUP
    }
}
#[doc = "Write proxy for field `WKUP`"]
pub struct WKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "SUSP\n\nValue on reset: 0"]
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
#[doc = "Reader of field `SUSP`"]
pub type SUSP_R = crate::R<bool, SUSP_A>;
impl SUSP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SUSP_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SUSP_A::SUSPEND),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == SUSP_A::SUSPEND
    }
}
#[doc = "Write proxy for field `SUSP`"]
pub struct SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUSP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "RESET\n\nValue on reset: 0"]
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
#[doc = "Reader of field `RESET`"]
pub type RESET_R = crate::R<bool, RESET_A>;
impl RESET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RESET_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(RESET_A::RESET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RESET_A::RESET
    }
}
#[doc = "Write proxy for field `RESET`"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "SOF\n\nValue on reset: 0"]
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
#[doc = "Reader of field `SOF`"]
pub type SOF_R = crate::R<bool, SOF_A>;
impl SOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SOF_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SOF_A::STARTOFFRAME),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STARTOFFRAME`"]
    #[inline(always)]
    pub fn is_start_of_frame(&self) -> bool {
        *self == SOF_A::STARTOFFRAME
    }
}
#[doc = "Write proxy for field `SOF`"]
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "ESOF\n\nValue on reset: 0"]
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
#[doc = "Reader of field `ESOF`"]
pub type ESOF_R = crate::R<bool, ESOF_A>;
impl ESOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ESOF_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(ESOF_A::EXPECTEDSTARTOFFRAME),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EXPECTEDSTARTOFFRAME`"]
    #[inline(always)]
    pub fn is_expected_start_of_frame(&self) -> bool {
        *self == ESOF_A::EXPECTEDSTARTOFFRAME
    }
}
#[doc = "Write proxy for field `ESOF`"]
pub struct ESOF_W<'a> {
    w: &'a mut W,
}
impl<'a> ESOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ESOF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "L1REQ\n\nValue on reset: 0"]
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
#[doc = "Reader of field `L1REQ`"]
pub type L1REQ_R = crate::R<bool, L1REQ_A>;
impl L1REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, L1REQ_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(L1REQ_A::RECEIVED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == L1REQ_A::RECEIVED
    }
}
#[doc = "Write proxy for field `L1REQ`"]
pub struct L1REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> L1REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L1REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "DIR\n\nValue on reset: 0"]
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
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, DIR_A>;
impl DIR_R {
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
        *self == DIR_A::TO
    }
    #[doc = "Checks if the value of the field is `FROM`"]
    #[inline(always)]
    pub fn is_from(&self) -> bool {
        *self == DIR_A::FROM
    }
}
#[doc = "Write proxy for field `DIR`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "data transmitted by the USB peripheral to the host PC"]
    #[inline(always)]
    pub fn to(self) -> &'a mut W {
        self.variant(DIR_A::TO)
    }
    #[doc = "data received by the USB peripheral from the host PC"]
    #[inline(always)]
    pub fn from(self) -> &'a mut W {
        self.variant(DIR_A::FROM)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `EP_ID`"]
pub type EP_ID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP_ID`"]
pub struct EP_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - CTR"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PMAOVR"]
    #[inline(always)]
    pub fn pmaovr(&self) -> PMAOVR_R {
        PMAOVR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ERR"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - WKUP"]
    #[inline(always)]
    pub fn wkup(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SUSP"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RESET"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SOF"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ESOF"]
    #[inline(always)]
    pub fn esof(&self) -> ESOF_R {
        ESOF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - L1REQ"]
    #[inline(always)]
    pub fn l1req(&self) -> L1REQ_R {
        L1REQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - EP_ID"]
    #[inline(always)]
    pub fn ep_id(&self) -> EP_ID_R {
        EP_ID_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - CTR"]
    #[inline(always)]
    pub fn ctr(&mut self) -> CTR_W {
        CTR_W { w: self }
    }
    #[doc = "Bit 14 - PMAOVR"]
    #[inline(always)]
    pub fn pmaovr(&mut self) -> PMAOVR_W {
        PMAOVR_W { w: self }
    }
    #[doc = "Bit 13 - ERR"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
    #[doc = "Bit 12 - WKUP"]
    #[inline(always)]
    pub fn wkup(&mut self) -> WKUP_W {
        WKUP_W { w: self }
    }
    #[doc = "Bit 11 - SUSP"]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W { w: self }
    }
    #[doc = "Bit 10 - RESET"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 9 - SOF"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W { w: self }
    }
    #[doc = "Bit 8 - ESOF"]
    #[inline(always)]
    pub fn esof(&mut self) -> ESOF_W {
        ESOF_W { w: self }
    }
    #[doc = "Bit 7 - L1REQ"]
    #[inline(always)]
    pub fn l1req(&mut self) -> L1REQ_W {
        L1REQ_W { w: self }
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bits 0:3 - EP_ID"]
    #[inline(always)]
    pub fn ep_id(&mut self) -> EP_ID_W {
        EP_ID_W { w: self }
    }
}
