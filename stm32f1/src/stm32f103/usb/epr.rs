#[doc = "Register `EP%sR` reader"]
pub struct R(crate::R<EPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP%sR` writer"]
pub struct W(crate::W<EPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPR_SPEC>;
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
impl From<crate::W<EPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EA` reader - Endpoint address"]
pub struct EA_R(crate::FieldReader<u8, u8>);
impl EA_R {
    pub(crate) fn new(bits: u8) -> Self {
        EA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EA` writer - Endpoint address"]
pub struct EA_W<'a> {
    w: &'a mut W,
}
impl<'a> EA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Status bits, for transmission transfers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STAT_TX_A {
    #[doc = "0: all transmission requests addressed to this endpoint are ignored"]
    DISABLED = 0,
    #[doc = "1: the endpoint is stalled and all transmission requests result in a STALL handshake"]
    STALL = 1,
    #[doc = "2: the endpoint is naked and all transmission requests result in a NAK handshake"]
    NAK = 2,
    #[doc = "3: this endpoint is enabled for transmission"]
    VALID = 3,
}
impl From<STAT_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: STAT_TX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STAT_TX` reader - Status bits, for transmission transfers"]
pub struct STAT_TX_R(crate::FieldReader<u8, STAT_TX_A>);
impl STAT_TX_R {
    pub(crate) fn new(bits: u8) -> Self {
        STAT_TX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_TX_A {
        match self.bits {
            0 => STAT_TX_A::DISABLED,
            1 => STAT_TX_A::STALL,
            2 => STAT_TX_A::NAK,
            3 => STAT_TX_A::VALID,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == STAT_TX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        **self == STAT_TX_A::STALL
    }
    #[doc = "Checks if the value of the field is `NAK`"]
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        **self == STAT_TX_A::NAK
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        **self == STAT_TX_A::VALID
    }
}
impl core::ops::Deref for STAT_TX_R {
    type Target = crate::FieldReader<u8, STAT_TX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STAT_TX` writer - Status bits, for transmission transfers"]
pub struct STAT_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STAT_TX_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "all transmission requests addressed to this endpoint are ignored"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STAT_TX_A::DISABLED)
    }
    #[doc = "the endpoint is stalled and all transmission requests result in a STALL handshake"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut W {
        self.variant(STAT_TX_A::STALL)
    }
    #[doc = "the endpoint is naked and all transmission requests result in a NAK handshake"]
    #[inline(always)]
    pub fn nak(self) -> &'a mut W {
        self.variant(STAT_TX_A::NAK)
    }
    #[doc = "this endpoint is enabled for transmission"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(STAT_TX_A::VALID)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `DTOG_TX` reader - Data Toggle, for transmission transfers"]
pub struct DTOG_TX_R(crate::FieldReader<bool, bool>);
impl DTOG_TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTOG_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTOG_TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTOG_TX` writer - Data Toggle, for transmission transfers"]
pub struct DTOG_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOG_TX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CTR_TX` reader - Correct Transfer for transmission"]
pub struct CTR_TX_R(crate::FieldReader<bool, bool>);
impl CTR_TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR_TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR_TX` writer - Correct Transfer for transmission"]
pub struct CTR_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_TX_W<'a> {
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
#[doc = "Field `EP_KIND` reader - Endpoint kind"]
pub struct EP_KIND_R(crate::FieldReader<bool, bool>);
impl EP_KIND_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_KIND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_KIND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_KIND` writer - Endpoint kind"]
pub struct EP_KIND_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_KIND_W<'a> {
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
#[doc = "Endpoint type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EP_TYPE_A {
    #[doc = "0: Bulk endpoint"]
    BULK = 0,
    #[doc = "1: Control endpoint"]
    CONTROL = 1,
    #[doc = "2: Iso endpoint"]
    ISO = 2,
    #[doc = "3: Interrupt endpoint"]
    INTERRUPT = 3,
}
impl From<EP_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: EP_TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EP_TYPE` reader - Endpoint type"]
pub struct EP_TYPE_R(crate::FieldReader<u8, EP_TYPE_A>);
impl EP_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        EP_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP_TYPE_A {
        match self.bits {
            0 => EP_TYPE_A::BULK,
            1 => EP_TYPE_A::CONTROL,
            2 => EP_TYPE_A::ISO,
            3 => EP_TYPE_A::INTERRUPT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BULK`"]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        **self == EP_TYPE_A::BULK
    }
    #[doc = "Checks if the value of the field is `CONTROL`"]
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        **self == EP_TYPE_A::CONTROL
    }
    #[doc = "Checks if the value of the field is `ISO`"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        **self == EP_TYPE_A::ISO
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        **self == EP_TYPE_A::INTERRUPT
    }
}
impl core::ops::Deref for EP_TYPE_R {
    type Target = crate::FieldReader<u8, EP_TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_TYPE` writer - Endpoint type"]
pub struct EP_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP_TYPE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Bulk endpoint"]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut W {
        self.variant(EP_TYPE_A::BULK)
    }
    #[doc = "Control endpoint"]
    #[inline(always)]
    pub fn control(self) -> &'a mut W {
        self.variant(EP_TYPE_A::CONTROL)
    }
    #[doc = "Iso endpoint"]
    #[inline(always)]
    pub fn iso(self) -> &'a mut W {
        self.variant(EP_TYPE_A::ISO)
    }
    #[doc = "Interrupt endpoint"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EP_TYPE_A::INTERRUPT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `SETUP` reader - Setup transaction completed"]
pub struct SETUP_R(crate::FieldReader<bool, bool>);
impl SETUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETUP` writer - Setup transaction completed"]
pub struct SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_W<'a> {
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
#[doc = "Status bits, for reception transfers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STAT_RX_A {
    #[doc = "0: all reception requests addressed to this endpoint are ignored"]
    DISABLED = 0,
    #[doc = "1: the endpoint is stalled and all reception requests result in a STALL handshake"]
    STALL = 1,
    #[doc = "2: the endpoint is naked and all reception requests result in a NAK handshake"]
    NAK = 2,
    #[doc = "3: this endpoint is enabled for reception"]
    VALID = 3,
}
impl From<STAT_RX_A> for u8 {
    #[inline(always)]
    fn from(variant: STAT_RX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STAT_RX` reader - Status bits, for reception transfers"]
pub struct STAT_RX_R(crate::FieldReader<u8, STAT_RX_A>);
impl STAT_RX_R {
    pub(crate) fn new(bits: u8) -> Self {
        STAT_RX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_RX_A {
        match self.bits {
            0 => STAT_RX_A::DISABLED,
            1 => STAT_RX_A::STALL,
            2 => STAT_RX_A::NAK,
            3 => STAT_RX_A::VALID,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == STAT_RX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        **self == STAT_RX_A::STALL
    }
    #[doc = "Checks if the value of the field is `NAK`"]
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        **self == STAT_RX_A::NAK
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        **self == STAT_RX_A::VALID
    }
}
impl core::ops::Deref for STAT_RX_R {
    type Target = crate::FieldReader<u8, STAT_RX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STAT_RX` writer - Status bits, for reception transfers"]
pub struct STAT_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT_RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STAT_RX_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "all reception requests addressed to this endpoint are ignored"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STAT_RX_A::DISABLED)
    }
    #[doc = "the endpoint is stalled and all reception requests result in a STALL handshake"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut W {
        self.variant(STAT_RX_A::STALL)
    }
    #[doc = "the endpoint is naked and all reception requests result in a NAK handshake"]
    #[inline(always)]
    pub fn nak(self) -> &'a mut W {
        self.variant(STAT_RX_A::NAK)
    }
    #[doc = "this endpoint is enabled for reception"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(STAT_RX_A::VALID)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `DTOG_RX` reader - Data Toggle, for reception transfers"]
pub struct DTOG_RX_R(crate::FieldReader<bool, bool>);
impl DTOG_RX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTOG_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTOG_RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTOG_RX` writer - Data Toggle, for reception transfers"]
pub struct DTOG_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOG_RX_W<'a> {
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
#[doc = "Field `CTR_RX` reader - Correct transfer for reception"]
pub struct CTR_RX_R(crate::FieldReader<bool, bool>);
impl CTR_RX_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR_RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR_RX` writer - Correct transfer for reception"]
pub struct CTR_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_RX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    pub fn ea(&self) -> EA_R {
        EA_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline(always)]
    pub fn stat_tx(&self) -> STAT_TX_R {
        STAT_TX_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Data Toggle, for transmission transfers"]
    #[inline(always)]
    pub fn dtog_tx(&self) -> DTOG_TX_R {
        DTOG_TX_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Correct Transfer for transmission"]
    #[inline(always)]
    pub fn ctr_tx(&self) -> CTR_TX_R {
        CTR_TX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint kind"]
    #[inline(always)]
    pub fn ep_kind(&self) -> EP_KIND_R {
        EP_KIND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Endpoint type"]
    #[inline(always)]
    pub fn ep_type(&self) -> EP_TYPE_R {
        EP_TYPE_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers"]
    #[inline(always)]
    pub fn stat_rx(&self) -> STAT_RX_R {
        STAT_RX_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers"]
    #[inline(always)]
    pub fn dtog_rx(&self) -> DTOG_RX_R {
        DTOG_RX_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Correct transfer for reception"]
    #[inline(always)]
    pub fn ctr_rx(&self) -> CTR_RX_R {
        CTR_RX_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    pub fn ea(&mut self) -> EA_W {
        EA_W { w: self }
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline(always)]
    pub fn stat_tx(&mut self) -> STAT_TX_W {
        STAT_TX_W { w: self }
    }
    #[doc = "Bit 6 - Data Toggle, for transmission transfers"]
    #[inline(always)]
    pub fn dtog_tx(&mut self) -> DTOG_TX_W {
        DTOG_TX_W { w: self }
    }
    #[doc = "Bit 7 - Correct Transfer for transmission"]
    #[inline(always)]
    pub fn ctr_tx(&mut self) -> CTR_TX_W {
        CTR_TX_W { w: self }
    }
    #[doc = "Bit 8 - Endpoint kind"]
    #[inline(always)]
    pub fn ep_kind(&mut self) -> EP_KIND_W {
        EP_KIND_W { w: self }
    }
    #[doc = "Bits 9:10 - Endpoint type"]
    #[inline(always)]
    pub fn ep_type(&mut self) -> EP_TYPE_W {
        EP_TYPE_W { w: self }
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    pub fn setup(&mut self) -> SETUP_W {
        SETUP_W { w: self }
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers"]
    #[inline(always)]
    pub fn stat_rx(&mut self) -> STAT_RX_W {
        STAT_RX_W { w: self }
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers"]
    #[inline(always)]
    pub fn dtog_rx(&mut self) -> DTOG_RX_W {
        DTOG_RX_W { w: self }
    }
    #[doc = "Bit 15 - Correct transfer for reception"]
    #[inline(always)]
    pub fn ctr_rx(&mut self) -> CTR_RX_W {
        CTR_RX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint %s register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epr](index.html) module"]
pub struct EPR_SPEC;
impl crate::RegisterSpec for EPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epr::R](R) reader structure"]
impl crate::Readable for EPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epr::W](W) writer structure"]
impl crate::Writable for EPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP%sR to value 0"]
impl crate::Resettable for EPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
