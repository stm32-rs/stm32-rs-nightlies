#[doc = "Register `WKUPEPR` reader"]
pub struct R(crate::R<WKUPEPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKUPEPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKUPEPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKUPEPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKUPEPR` writer"]
pub struct W(crate::W<WKUPEPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKUPEPR_SPEC>;
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
impl From<crate::W<WKUPEPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKUPEPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUPEN1` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub struct WKUPEN1_R(crate::FieldReader<bool, bool>);
impl WKUPEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN1` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub struct WKUPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN1_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `WKUPEN2` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub struct WKUPEN2_R(crate::FieldReader<bool, bool>);
impl WKUPEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN2` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub struct WKUPEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `WKUPEN3` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub struct WKUPEN3_R(crate::FieldReader<bool, bool>);
impl WKUPEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN3` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub struct WKUPEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `WKUPEN4` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub struct WKUPEN4_R(crate::FieldReader<bool, bool>);
impl WKUPEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPEN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN4` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub struct WKUPEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `WKUPEN5` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub struct WKUPEN5_R(crate::FieldReader<bool, bool>);
impl WKUPEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPEN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN5` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub struct WKUPEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `WKUPEN6` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub struct WKUPEN6_R(crate::FieldReader<bool, bool>);
impl WKUPEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPEN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN6` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub struct WKUPEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `WKUPP1` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub struct WKUPP1_R(crate::FieldReader<bool, bool>);
impl WKUPP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPP1` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub struct WKUPP1_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPP1_W<'a> {
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
#[doc = "Field `WKUPP2` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub struct WKUPP2_R(crate::FieldReader<bool, bool>);
impl WKUPP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPP2` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub struct WKUPP2_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPP2_W<'a> {
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
#[doc = "Field `WKUPP3` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub struct WKUPP3_R(crate::FieldReader<bool, bool>);
impl WKUPP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPP3` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub struct WKUPP3_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPP3_W<'a> {
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
#[doc = "Field `WKUPP4` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub struct WKUPP4_R(crate::FieldReader<bool, bool>);
impl WKUPP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPP4` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub struct WKUPP4_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPP4_W<'a> {
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
#[doc = "Field `WKUPP5` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub struct WKUPP5_R(crate::FieldReader<bool, bool>);
impl WKUPP5_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPP5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPP5` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub struct WKUPP5_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPP5_W<'a> {
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
#[doc = "Field `WKUPP6` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub struct WKUPP6_R(crate::FieldReader<bool, bool>);
impl WKUPP6_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPP6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPP6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPP6` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub struct WKUPP6_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPP6_W<'a> {
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
#[doc = "Field `WKUPPUPD1` reader - Wakeup pin pull configuration"]
pub struct WKUPPUPD1_R(crate::FieldReader<u8, u8>);
impl WKUPPUPD1_R {
    pub(crate) fn new(bits: u8) -> Self {
        WKUPPUPD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPPUPD1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPPUPD1` writer - Wakeup pin pull configuration"]
pub struct WKUPPUPD1_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPPUPD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `WKUPPUPD2` reader - Wakeup pin pull configuration"]
pub struct WKUPPUPD2_R(crate::FieldReader<u8, u8>);
impl WKUPPUPD2_R {
    pub(crate) fn new(bits: u8) -> Self {
        WKUPPUPD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPPUPD2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPPUPD2` writer - Wakeup pin pull configuration"]
pub struct WKUPPUPD2_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPPUPD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `WKUPPUPD3` reader - Wakeup pin pull configuration"]
pub struct WKUPPUPD3_R(crate::FieldReader<u8, u8>);
impl WKUPPUPD3_R {
    pub(crate) fn new(bits: u8) -> Self {
        WKUPPUPD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPPUPD3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPPUPD3` writer - Wakeup pin pull configuration"]
pub struct WKUPPUPD3_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPPUPD3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `WKUPPUPD4` reader - Wakeup pin pull configuration"]
pub struct WKUPPUPD4_R(crate::FieldReader<u8, u8>);
impl WKUPPUPD4_R {
    pub(crate) fn new(bits: u8) -> Self {
        WKUPPUPD4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPPUPD4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPPUPD4` writer - Wakeup pin pull configuration"]
pub struct WKUPPUPD4_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPPUPD4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `WKUPPUPD5` reader - Wakeup pin pull configuration"]
pub struct WKUPPUPD5_R(crate::FieldReader<u8, u8>);
impl WKUPPUPD5_R {
    pub(crate) fn new(bits: u8) -> Self {
        WKUPPUPD5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPPUPD5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPPUPD5` writer - Wakeup pin pull configuration"]
pub struct WKUPPUPD5_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPPUPD5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `WKUPPUPD6` reader - Wakeup pin pull configuration for WKUP(truncate(n/2)-7) These bits define the I/O pad pull configuration used when WKUPEN(truncate(n/2)-7) = 1. The associated GPIO port pull configuration shall be set to the same value or to 00. The Wakeup pin pull configuration is kept in Standby mode."]
pub struct WKUPPUPD6_R(crate::FieldReader<u8, u8>);
impl WKUPPUPD6_R {
    pub(crate) fn new(bits: u8) -> Self {
        WKUPPUPD6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPPUPD6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPPUPD6` writer - Wakeup pin pull configuration for WKUP(truncate(n/2)-7) These bits define the I/O pad pull configuration used when WKUPEN(truncate(n/2)-7) = 1. The associated GPIO port pull configuration shall be set to the same value or to 00. The Wakeup pin pull configuration is kept in Standby mode."]
pub struct WKUPPUPD6_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPPUPD6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen1(&self) -> WKUPEN1_R {
        WKUPEN1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen2(&self) -> WKUPEN2_R {
        WKUPEN2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen3(&self) -> WKUPEN3_R {
        WKUPEN3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen4(&self) -> WKUPEN4_R {
        WKUPEN4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen5(&self) -> WKUPEN5_R {
        WKUPEN5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen6(&self) -> WKUPEN6_R {
        WKUPEN6_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp1(&self) -> WKUPP1_R {
        WKUPP1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp2(&self) -> WKUPP2_R {
        WKUPP2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp3(&self) -> WKUPP3_R {
        WKUPP3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp4(&self) -> WKUPP4_R {
        WKUPP4_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp5(&self) -> WKUPP5_R {
        WKUPP5_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp6(&self) -> WKUPP6_R {
        WKUPP6_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd1(&self) -> WKUPPUPD1_R {
        WKUPPUPD1_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd2(&self) -> WKUPPUPD2_R {
        WKUPPUPD2_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd3(&self) -> WKUPPUPD3_R {
        WKUPPUPD3_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd4(&self) -> WKUPPUPD4_R {
        WKUPPUPD4_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd5(&self) -> WKUPPUPD5_R {
        WKUPPUPD5_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Wakeup pin pull configuration for WKUP(truncate(n/2)-7) These bits define the I/O pad pull configuration used when WKUPEN(truncate(n/2)-7) = 1. The associated GPIO port pull configuration shall be set to the same value or to 00. The Wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn wkuppupd6(&self) -> WKUPPUPD6_R {
        WKUPPUPD6_R::new(((self.bits >> 26) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen1(&mut self) -> WKUPEN1_W {
        WKUPEN1_W { w: self }
    }
    #[doc = "Bit 1 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen2(&mut self) -> WKUPEN2_W {
        WKUPEN2_W { w: self }
    }
    #[doc = "Bit 2 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen3(&mut self) -> WKUPEN3_W {
        WKUPEN3_W { w: self }
    }
    #[doc = "Bit 3 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen4(&mut self) -> WKUPEN4_W {
        WKUPEN4_W { w: self }
    }
    #[doc = "Bit 4 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen5(&mut self) -> WKUPEN5_W {
        WKUPEN5_W { w: self }
    }
    #[doc = "Bit 5 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen6(&mut self) -> WKUPEN6_W {
        WKUPEN6_W { w: self }
    }
    #[doc = "Bit 8 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp1(&mut self) -> WKUPP1_W {
        WKUPP1_W { w: self }
    }
    #[doc = "Bit 9 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp2(&mut self) -> WKUPP2_W {
        WKUPP2_W { w: self }
    }
    #[doc = "Bit 10 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp3(&mut self) -> WKUPP3_W {
        WKUPP3_W { w: self }
    }
    #[doc = "Bit 11 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp4(&mut self) -> WKUPP4_W {
        WKUPP4_W { w: self }
    }
    #[doc = "Bit 12 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp5(&mut self) -> WKUPP5_W {
        WKUPP5_W { w: self }
    }
    #[doc = "Bit 13 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp6(&mut self) -> WKUPP6_W {
        WKUPP6_W { w: self }
    }
    #[doc = "Bits 16:17 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd1(&mut self) -> WKUPPUPD1_W {
        WKUPPUPD1_W { w: self }
    }
    #[doc = "Bits 18:19 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd2(&mut self) -> WKUPPUPD2_W {
        WKUPPUPD2_W { w: self }
    }
    #[doc = "Bits 20:21 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd3(&mut self) -> WKUPPUPD3_W {
        WKUPPUPD3_W { w: self }
    }
    #[doc = "Bits 22:23 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd4(&mut self) -> WKUPPUPD4_W {
        WKUPPUPD4_W { w: self }
    }
    #[doc = "Bits 24:25 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd5(&mut self) -> WKUPPUPD5_W {
        WKUPPUPD5_W { w: self }
    }
    #[doc = "Bits 26:27 - Wakeup pin pull configuration for WKUP(truncate(n/2)-7) These bits define the I/O pad pull configuration used when WKUPEN(truncate(n/2)-7) = 1. The associated GPIO port pull configuration shall be set to the same value or to 00. The Wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn wkuppupd6(&mut self) -> WKUPPUPD6_W {
        WKUPPUPD6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset only by system reset, not reset by wakeup from Standby mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkupepr](index.html) module"]
pub struct WKUPEPR_SPEC;
impl crate::RegisterSpec for WKUPEPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wkupepr::R](R) reader structure"]
impl crate::Readable for WKUPEPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkupepr::W](W) writer structure"]
impl crate::Writable for WKUPEPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WKUPEPR to value 0"]
impl crate::Resettable for WKUPEPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
