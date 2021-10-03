#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DFSDM_FLTx enable Data which are cleared by setting DFEN=0: register DFSDM_FLTxISR is set to the reset state register DFSDM_FLTxAWSR is set to the reset state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFEN_A {
    #[doc = "0: DFSDM_FLTx is disabled. All conversions of given DFSDM_FLTx are stopped immediately and all DFSDM_FLTx functions are stopped."]
    B_0X0 = 0,
    #[doc = "1: DFSDM_FLTx is enabled. If DFSDM_FLTx is enabled, then DFSDM_FLTx starts operating according to its setting."]
    B_0X1 = 1,
}
impl From<DFEN_A> for bool {
    #[inline(always)]
    fn from(variant: DFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFEN` reader - DFSDM_FLTx enable Data which are cleared by setting DFEN=0: register DFSDM_FLTxISR is set to the reset state register DFSDM_FLTxAWSR is set to the reset state"]
pub struct DFEN_R(crate::FieldReader<bool, DFEN_A>);
impl DFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFEN_A {
        match self.bits {
            false => DFEN_A::B_0X0,
            true => DFEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DFEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DFEN_A::B_0X1
    }
}
impl core::ops::Deref for DFEN_R {
    type Target = crate::FieldReader<bool, DFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFEN` writer - DFSDM_FLTx enable Data which are cleared by setting DFEN=0: register DFSDM_FLTxISR is set to the reset state register DFSDM_FLTxAWSR is set to the reset state"]
pub struct DFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DFSDM_FLTx is disabled. All conversions of given DFSDM_FLTx are stopped immediately and all DFSDM_FLTx functions are stopped."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DFEN_A::B_0X0)
    }
    #[doc = "DFSDM_FLTx is enabled. If DFSDM_FLTx is enabled, then DFSDM_FLTx starts operating according to its setting."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DFEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Start a conversion of the injected group of channels This bit is always read as '0â\u{80}\u{99}.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSWSTART_A {
    #[doc = "0: Writing '0â\u{80}\u{99} has no effect."]
    B_0X0 = 0,
    #[doc = "1: Writing '1â\u{80}\u{99} makes a request to convert the channels in the injected conversion group, causing JCIP to become '1â\u{80}\u{99} at the same time. If JCIP=1 already, then writing to JSWSTART has no effect. Writing '1â\u{80}\u{99} has no effect if JSYNC=1."]
    B_0X1 = 1,
}
impl From<JSWSTART_A> for bool {
    #[inline(always)]
    fn from(variant: JSWSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSWSTART` reader - Start a conversion of the injected group of channels This bit is always read as '0â\u{80}\u{99}."]
pub struct JSWSTART_R(crate::FieldReader<bool, JSWSTART_A>);
impl JSWSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        JSWSTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JSWSTART_A {
        match self.bits {
            false => JSWSTART_A::B_0X0,
            true => JSWSTART_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == JSWSTART_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == JSWSTART_A::B_0X1
    }
}
impl core::ops::Deref for JSWSTART_R {
    type Target = crate::FieldReader<bool, JSWSTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JSWSTART` writer - Start a conversion of the injected group of channels This bit is always read as '0â\u{80}\u{99}."]
pub struct JSWSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> JSWSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JSWSTART_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Writing '0â\u{80}\u{99} has no effect."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(JSWSTART_A::B_0X0)
    }
    #[doc = "Writing '1â\u{80}\u{99} makes a request to convert the channels in the injected conversion group, causing JCIP to become '1â\u{80}\u{99} at the same time. If JCIP=1 already, then writing to JSWSTART has no effect. Writing '1â\u{80}\u{99} has no effect if JSYNC=1."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(JSWSTART_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Launch an injected conversion synchronously with the DFSDM_FLT0 JSWSTART trigger This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSYNC_A {
    #[doc = "0: Do not launch an injected conversion synchronously with DFSDM_FLT0"]
    B_0X0 = 0,
    #[doc = "1: Launch an injected conversion in this DFSDM_FLTx at the very moment when an injected conversion is launched in DFSDM_FLT0 by its JSWSTART trigger"]
    B_0X1 = 1,
}
impl From<JSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: JSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSYNC` reader - Launch an injected conversion synchronously with the DFSDM_FLT0 JSWSTART trigger This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
pub struct JSYNC_R(crate::FieldReader<bool, JSYNC_A>);
impl JSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        JSYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JSYNC_A {
        match self.bits {
            false => JSYNC_A::B_0X0,
            true => JSYNC_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == JSYNC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == JSYNC_A::B_0X1
    }
}
impl core::ops::Deref for JSYNC_R {
    type Target = crate::FieldReader<bool, JSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JSYNC` writer - Launch an injected conversion synchronously with the DFSDM_FLT0 JSWSTART trigger This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
pub struct JSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> JSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JSYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Do not launch an injected conversion synchronously with DFSDM_FLT0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(JSYNC_A::B_0X0)
    }
    #[doc = "Launch an injected conversion in this DFSDM_FLTx at the very moment when an injected conversion is launched in DFSDM_FLT0 by its JSWSTART trigger"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(JSYNC_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Scanning conversion mode for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Writing JCHG if JSCAN=0 resets the channel selection to the lowest selected channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSCAN_A {
    #[doc = "0: One channel conversion is performed from the injected channel group and next the selected channel from this group is selected."]
    B_0X0 = 0,
    #[doc = "1: The series of conversions for the injected group channels is executed, starting over with the lowest selected channel."]
    B_0X1 = 1,
}
impl From<JSCAN_A> for bool {
    #[inline(always)]
    fn from(variant: JSCAN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSCAN` reader - Scanning conversion mode for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Writing JCHG if JSCAN=0 resets the channel selection to the lowest selected channel."]
pub struct JSCAN_R(crate::FieldReader<bool, JSCAN_A>);
impl JSCAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        JSCAN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JSCAN_A {
        match self.bits {
            false => JSCAN_A::B_0X0,
            true => JSCAN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == JSCAN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == JSCAN_A::B_0X1
    }
}
impl core::ops::Deref for JSCAN_R {
    type Target = crate::FieldReader<bool, JSCAN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JSCAN` writer - Scanning conversion mode for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Writing JCHG if JSCAN=0 resets the channel selection to the lowest selected channel."]
pub struct JSCAN_W<'a> {
    w: &'a mut W,
}
impl<'a> JSCAN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JSCAN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "One channel conversion is performed from the injected channel group and next the selected channel from this group is selected."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(JSCAN_A::B_0X0)
    }
    #[doc = "The series of conversions for the injected group channels is executed, starting over with the lowest selected channel."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(JSCAN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "DMA channel enabled to read data for the injected channel group This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JDMAEN_A {
    #[doc = "0: The DMA channel is not enabled to read injected data"]
    B_0X0 = 0,
    #[doc = "1: The DMA channel is enabled to read injected data"]
    B_0X1 = 1,
}
impl From<JDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: JDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JDMAEN` reader - DMA channel enabled to read data for the injected channel group This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
pub struct JDMAEN_R(crate::FieldReader<bool, JDMAEN_A>);
impl JDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        JDMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JDMAEN_A {
        match self.bits {
            false => JDMAEN_A::B_0X0,
            true => JDMAEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == JDMAEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == JDMAEN_A::B_0X1
    }
}
impl core::ops::Deref for JDMAEN_R {
    type Target = crate::FieldReader<bool, JDMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JDMAEN` writer - DMA channel enabled to read data for the injected channel group This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
pub struct JDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JDMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA channel is not enabled to read injected data"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(JDMAEN_A::B_0X0)
    }
    #[doc = "The DMA channel is enabled to read injected data"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(JDMAEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `JEXTSEL` reader - Trigger signal selection for launching injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Note: synchronous trigger has latency up to one fDFSDMCLK clock cycle (with deterministic jitter), asynchronous trigger has latency 2-3 fDFSDMCLK clock cycles (with jitter up to 1 cycle). DFSDM_FLTx 0x00 dfsdm_jtrg0 0x01 dfsdm_jtrg1 ... 0x1E dfsdm_jtrg30 0x1F dfsdm_jtrg31 Refer to . 0x0-0x1F: Trigger inputs selected by the following table (internal or external trigger)."]
pub struct JEXTSEL_R(crate::FieldReader<u8, u8>);
impl JEXTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        JEXTSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEXTSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEXTSEL` writer - Trigger signal selection for launching injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Note: synchronous trigger has latency up to one fDFSDMCLK clock cycle (with deterministic jitter), asynchronous trigger has latency 2-3 fDFSDMCLK clock cycles (with jitter up to 1 cycle). DFSDM_FLTx 0x00 dfsdm_jtrg0 0x01 dfsdm_jtrg1 ... 0x1E dfsdm_jtrg30 0x1F dfsdm_jtrg31 Refer to . 0x0-0x1F: Trigger inputs selected by the following table (internal or external trigger)."]
pub struct JEXTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Trigger enable and trigger edge selection for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum JEXTEN_A {
    #[doc = "0: Trigger detection is disabled"]
    B_0X0 = 0,
    #[doc = "1: Each rising edge on the selected trigger makes a request to launch an injected conversion"]
    B_0X1 = 1,
    #[doc = "2: Each falling edge on the selected trigger makes a request to launch an injected conversion"]
    B_0X2 = 2,
    #[doc = "3: Both rising edges and falling edges on the selected trigger make requests to launch injected conversions"]
    B_0X3 = 3,
}
impl From<JEXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: JEXTEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `JEXTEN` reader - Trigger enable and trigger edge selection for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
pub struct JEXTEN_R(crate::FieldReader<u8, JEXTEN_A>);
impl JEXTEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        JEXTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEXTEN_A {
        match self.bits {
            0 => JEXTEN_A::B_0X0,
            1 => JEXTEN_A::B_0X1,
            2 => JEXTEN_A::B_0X2,
            3 => JEXTEN_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == JEXTEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == JEXTEN_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == JEXTEN_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == JEXTEN_A::B_0X3
    }
}
impl core::ops::Deref for JEXTEN_R {
    type Target = crate::FieldReader<u8, JEXTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEXTEN` writer - Trigger enable and trigger edge selection for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
pub struct JEXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JEXTEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Trigger detection is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(JEXTEN_A::B_0X0)
    }
    #[doc = "Each rising edge on the selected trigger makes a request to launch an injected conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(JEXTEN_A::B_0X1)
    }
    #[doc = "Each falling edge on the selected trigger makes a request to launch an injected conversion"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(JEXTEN_A::B_0X2)
    }
    #[doc = "Both rising edges and falling edges on the selected trigger make requests to launch injected conversions"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(JEXTEN_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Software start of a conversion on the regular channel This bit is always read as '0â\u{80}\u{99}.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSWSTART_A {
    #[doc = "0: Writing '0â\u{80}\u{99} has no effect"]
    B_0X0 = 0,
    #[doc = "1: Writing '1â\u{80}\u{99} makes a request to start a conversion on the regular channel and causes RCIP to become '1â\u{80}\u{99}. If RCIP=1 already, writing to RSWSTART has no effect. Writing '1â\u{80}\u{99} has no effect if RSYNC=1."]
    B_0X1 = 1,
}
impl From<RSWSTART_A> for bool {
    #[inline(always)]
    fn from(variant: RSWSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSWSTART` reader - Software start of a conversion on the regular channel This bit is always read as '0â\u{80}\u{99}."]
pub struct RSWSTART_R(crate::FieldReader<bool, RSWSTART_A>);
impl RSWSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSWSTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSWSTART_A {
        match self.bits {
            false => RSWSTART_A::B_0X0,
            true => RSWSTART_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RSWSTART_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RSWSTART_A::B_0X1
    }
}
impl core::ops::Deref for RSWSTART_R {
    type Target = crate::FieldReader<bool, RSWSTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSWSTART` writer - Software start of a conversion on the regular channel This bit is always read as '0â\u{80}\u{99}."]
pub struct RSWSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RSWSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSWSTART_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Writing '0â\u{80}\u{99} has no effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RSWSTART_A::B_0X0)
    }
    #[doc = "Writing '1â\u{80}\u{99} makes a request to start a conversion on the regular channel and causes RCIP to become '1â\u{80}\u{99}. If RCIP=1 already, writing to RSWSTART has no effect. Writing '1â\u{80}\u{99} has no effect if RSYNC=1."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RSWSTART_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Continuous mode selection for regular conversions Writing '0â\u{80}\u{99} to this bit while a continuous regular conversion is already in progress stops the continuous mode immediately.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCONT_A {
    #[doc = "0: The regular channel is converted just once for each conversion request"]
    B_0X0 = 0,
    #[doc = "1: The regular channel is converted repeatedly after each conversion request"]
    B_0X1 = 1,
}
impl From<RCONT_A> for bool {
    #[inline(always)]
    fn from(variant: RCONT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCONT` reader - Continuous mode selection for regular conversions Writing '0â\u{80}\u{99} to this bit while a continuous regular conversion is already in progress stops the continuous mode immediately."]
pub struct RCONT_R(crate::FieldReader<bool, RCONT_A>);
impl RCONT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCONT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCONT_A {
        match self.bits {
            false => RCONT_A::B_0X0,
            true => RCONT_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RCONT_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RCONT_A::B_0X1
    }
}
impl core::ops::Deref for RCONT_R {
    type Target = crate::FieldReader<bool, RCONT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCONT` writer - Continuous mode selection for regular conversions Writing '0â\u{80}\u{99} to this bit while a continuous regular conversion is already in progress stops the continuous mode immediately."]
pub struct RCONT_W<'a> {
    w: &'a mut W,
}
impl<'a> RCONT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCONT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The regular channel is converted just once for each conversion request"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RCONT_A::B_0X0)
    }
    #[doc = "The regular channel is converted repeatedly after each conversion request"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RCONT_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Launch regular conversion synchronously with DFSDM_FLT0 This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSYNC_A {
    #[doc = "0: Do not launch a regular conversion synchronously with DFSDM_FLT0"]
    B_0X0 = 0,
    #[doc = "1: Launch a regular conversion in this DFSDM_FLTx at the very moment when a regular conversion is launched in DFSDM_FLT0"]
    B_0X1 = 1,
}
impl From<RSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: RSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSYNC` reader - Launch regular conversion synchronously with DFSDM_FLT0 This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
pub struct RSYNC_R(crate::FieldReader<bool, RSYNC_A>);
impl RSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSYNC_A {
        match self.bits {
            false => RSYNC_A::B_0X0,
            true => RSYNC_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RSYNC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RSYNC_A::B_0X1
    }
}
impl core::ops::Deref for RSYNC_R {
    type Target = crate::FieldReader<bool, RSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSYNC` writer - Launch regular conversion synchronously with DFSDM_FLT0 This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
pub struct RSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Do not launch a regular conversion synchronously with DFSDM_FLT0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RSYNC_A::B_0X0)
    }
    #[doc = "Launch a regular conversion in this DFSDM_FLTx at the very moment when a regular conversion is launched in DFSDM_FLT0"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RSYNC_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "DMA channel enabled to read data for the regular conversion This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDMAEN_A {
    #[doc = "0: The DMA channel is not enabled to read regular data"]
    B_0X0 = 0,
    #[doc = "1: The DMA channel is enabled to read regular data"]
    B_0X1 = 1,
}
impl From<RDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDMAEN` reader - DMA channel enabled to read data for the regular conversion This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
pub struct RDMAEN_R(crate::FieldReader<bool, RDMAEN_A>);
impl RDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDMAEN_A {
        match self.bits {
            false => RDMAEN_A::B_0X0,
            true => RDMAEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RDMAEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RDMAEN_A::B_0X1
    }
}
impl core::ops::Deref for RDMAEN_R {
    type Target = crate::FieldReader<bool, RDMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDMAEN` writer - DMA channel enabled to read data for the regular conversion This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
pub struct RDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA channel is not enabled to read regular data"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RDMAEN_A::B_0X0)
    }
    #[doc = "The DMA channel is enabled to read regular data"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RDMAEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Regular channel selection ... 7: Channel 7 is selected as the regular channel Writing these bits when RCIP=1 takes effect when the next regular conversion begins. This is especially useful in continuous mode (when RCONT=1). It also affects regular conversions which are pending (due to ongoing injected conversion).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RCH_A {
    #[doc = "0: Channel 0 is selected as the regular channel"]
    B_0X0 = 0,
    #[doc = "1: Channel 1 is selected as the regular channel"]
    B_0X1 = 1,
}
impl From<RCH_A> for u8 {
    #[inline(always)]
    fn from(variant: RCH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RCH` reader - Regular channel selection ... 7: Channel 7 is selected as the regular channel Writing these bits when RCIP=1 takes effect when the next regular conversion begins. This is especially useful in continuous mode (when RCONT=1). It also affects regular conversions which are pending (due to ongoing injected conversion)."]
pub struct RCH_R(crate::FieldReader<u8, RCH_A>);
impl RCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RCH_A> {
        match self.bits {
            0 => Some(RCH_A::B_0X0),
            1 => Some(RCH_A::B_0X1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RCH_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RCH_A::B_0X1
    }
}
impl core::ops::Deref for RCH_R {
    type Target = crate::FieldReader<u8, RCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCH` writer - Regular channel selection ... 7: Channel 7 is selected as the regular channel Writing these bits when RCIP=1 takes effect when the next regular conversion begins. This is especially useful in continuous mode (when RCONT=1). It also affects regular conversions which are pending (due to ongoing injected conversion)."]
pub struct RCH_W<'a> {
    w: &'a mut W,
}
impl<'a> RCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Channel 0 is selected as the regular channel"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RCH_A::B_0X0)
    }
    #[doc = "Channel 1 is selected as the regular channel"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RCH_A::B_0X1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Fast conversion mode selection for regular conversions When converting a regular conversion in continuous mode, having enabled the fast mode causes each conversion (except the first) to execute faster than in standard mode. This bit has no effect on conversions which are not continuous. This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). if FAST=0 (or first conversion in continuous mode if FAST=1): t = \\[FOSR * (IOSR-1 + FORD) + FORD\\]
/ fCKIN ..... for Sincx filters t = \\[FOSR * (IOSR-1 + 4) + 2\\]
/ fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \\[FOSR * IOSR\\]
/ fCKIN in case if FOSR = FOSR\\[9:0\\]+1 = 1 (filter bypassed, active only integrator): t = IOSR / fCKIN (... but CNVCNT=0) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAST_A {
    #[doc = "0: Fast conversion mode disabled"]
    B_0X0 = 0,
    #[doc = "1: Fast conversion mode enabled"]
    B_0X1 = 1,
}
impl From<FAST_A> for bool {
    #[inline(always)]
    fn from(variant: FAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAST` reader - Fast conversion mode selection for regular conversions When converting a regular conversion in continuous mode, having enabled the fast mode causes each conversion (except the first) to execute faster than in standard mode. This bit has no effect on conversions which are not continuous. This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). if FAST=0 (or first conversion in continuous mode if FAST=1): t = \\[FOSR * (IOSR-1 + FORD) + FORD\\]
/ fCKIN ..... for Sincx filters t = \\[FOSR * (IOSR-1 + 4) + 2\\]
/ fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \\[FOSR * IOSR\\]
/ fCKIN in case if FOSR = FOSR\\[9:0\\]+1 = 1 (filter bypassed, active only integrator): t = IOSR / fCKIN (... but CNVCNT=0) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input."]
pub struct FAST_R(crate::FieldReader<bool, FAST_A>);
impl FAST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAST_A {
        match self.bits {
            false => FAST_A::B_0X0,
            true => FAST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == FAST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == FAST_A::B_0X1
    }
}
impl core::ops::Deref for FAST_R {
    type Target = crate::FieldReader<bool, FAST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAST` writer - Fast conversion mode selection for regular conversions When converting a regular conversion in continuous mode, having enabled the fast mode causes each conversion (except the first) to execute faster than in standard mode. This bit has no effect on conversions which are not continuous. This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). if FAST=0 (or first conversion in continuous mode if FAST=1): t = \\[FOSR * (IOSR-1 + FORD) + FORD\\]
/ fCKIN ..... for Sincx filters t = \\[FOSR * (IOSR-1 + 4) + 2\\]
/ fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \\[FOSR * IOSR\\]
/ fCKIN in case if FOSR = FOSR\\[9:0\\]+1 = 1 (filter bypassed, active only integrator): t = IOSR / fCKIN (... but CNVCNT=0) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input."]
pub struct FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> FAST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fast conversion mode disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FAST_A::B_0X0)
    }
    #[doc = "Fast conversion mode enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FAST_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Analog watchdog fast mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWFSEL_A {
    #[doc = "0: Analog watchdog on data output value (after the digital filter). The comparison is done after offset correction and shift"]
    B_0X0 = 0,
    #[doc = "1: Analog watchdog on channel transceivers value (after watchdog filter)"]
    B_0X1 = 1,
}
impl From<AWFSEL_A> for bool {
    #[inline(always)]
    fn from(variant: AWFSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWFSEL` reader - Analog watchdog fast mode select"]
pub struct AWFSEL_R(crate::FieldReader<bool, AWFSEL_A>);
impl AWFSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWFSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWFSEL_A {
        match self.bits {
            false => AWFSEL_A::B_0X0,
            true => AWFSEL_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == AWFSEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == AWFSEL_A::B_0X1
    }
}
impl core::ops::Deref for AWFSEL_R {
    type Target = crate::FieldReader<bool, AWFSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWFSEL` writer - Analog watchdog fast mode select"]
pub struct AWFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> AWFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWFSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Analog watchdog on data output value (after the digital filter). The comparison is done after offset correction and shift"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AWFSEL_A::B_0X0)
    }
    #[doc = "Analog watchdog on channel transceivers value (after watchdog filter)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AWFSEL_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DFSDM_FLTx enable Data which are cleared by setting DFEN=0: register DFSDM_FLTxISR is set to the reset state register DFSDM_FLTxAWSR is set to the reset state"]
    #[inline(always)]
    pub fn dfen(&self) -> DFEN_R {
        DFEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start a conversion of the injected group of channels This bit is always read as '0â\u{80}\u{99}."]
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM_FLT0 JSWSTART trigger This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
    #[inline(always)]
    pub fn jsync(&self) -> JSYNC_R {
        JSYNC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Scanning conversion mode for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Writing JCHG if JSCAN=0 resets the channel selection to the lowest selected channel."]
    #[inline(always)]
    pub fn jscan(&self) -> JSCAN_R {
        JSCAN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DMA channel enabled to read data for the injected channel group This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
    #[inline(always)]
    pub fn jdmaen(&self) -> JDMAEN_R {
        JDMAEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Trigger signal selection for launching injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Note: synchronous trigger has latency up to one fDFSDMCLK clock cycle (with deterministic jitter), asynchronous trigger has latency 2-3 fDFSDMCLK clock cycles (with jitter up to 1 cycle). DFSDM_FLTx 0x00 dfsdm_jtrg0 0x01 dfsdm_jtrg1 ... 0x1E dfsdm_jtrg30 0x1F dfsdm_jtrg31 Refer to . 0x0-0x1F: Trigger inputs selected by the following table (internal or external trigger)."]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 17 - Software start of a conversion on the regular channel This bit is always read as '0â\u{80}\u{99}."]
    #[inline(always)]
    pub fn rswstart(&self) -> RSWSTART_R {
        RSWSTART_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Continuous mode selection for regular conversions Writing '0â\u{80}\u{99} to this bit while a continuous regular conversion is already in progress stops the continuous mode immediately."]
    #[inline(always)]
    pub fn rcont(&self) -> RCONT_R {
        RCONT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Launch regular conversion synchronously with DFSDM_FLT0 This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
    #[inline(always)]
    pub fn rsync(&self) -> RSYNC_R {
        RSYNC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - DMA channel enabled to read data for the regular conversion This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
    #[inline(always)]
    pub fn rdmaen(&self) -> RDMAEN_R {
        RDMAEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Regular channel selection ... 7: Channel 7 is selected as the regular channel Writing these bits when RCIP=1 takes effect when the next regular conversion begins. This is especially useful in continuous mode (when RCONT=1). It also affects regular conversions which are pending (due to ongoing injected conversion)."]
    #[inline(always)]
    pub fn rch(&self) -> RCH_R {
        RCH_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 29 - Fast conversion mode selection for regular conversions When converting a regular conversion in continuous mode, having enabled the fast mode causes each conversion (except the first) to execute faster than in standard mode. This bit has no effect on conversions which are not continuous. This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). if FAST=0 (or first conversion in continuous mode if FAST=1): t = \\[FOSR * (IOSR-1 + FORD) + FORD\\]
/ fCKIN ..... for Sincx filters t = \\[FOSR * (IOSR-1 + 4) + 2\\]
/ fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \\[FOSR * IOSR\\]
/ fCKIN in case if FOSR = FOSR\\[9:0\\]+1 = 1 (filter bypassed, active only integrator): t = IOSR / fCKIN (... but CNVCNT=0) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input."]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Analog watchdog fast mode select"]
    #[inline(always)]
    pub fn awfsel(&self) -> AWFSEL_R {
        AWFSEL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DFSDM_FLTx enable Data which are cleared by setting DFEN=0: register DFSDM_FLTxISR is set to the reset state register DFSDM_FLTxAWSR is set to the reset state"]
    #[inline(always)]
    pub fn dfen(&mut self) -> DFEN_W {
        DFEN_W { w: self }
    }
    #[doc = "Bit 1 - Start a conversion of the injected group of channels This bit is always read as '0â\u{80}\u{99}."]
    #[inline(always)]
    pub fn jswstart(&mut self) -> JSWSTART_W {
        JSWSTART_W { w: self }
    }
    #[doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM_FLT0 JSWSTART trigger This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
    #[inline(always)]
    pub fn jsync(&mut self) -> JSYNC_W {
        JSYNC_W { w: self }
    }
    #[doc = "Bit 4 - Scanning conversion mode for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Writing JCHG if JSCAN=0 resets the channel selection to the lowest selected channel."]
    #[inline(always)]
    pub fn jscan(&mut self) -> JSCAN_W {
        JSCAN_W { w: self }
    }
    #[doc = "Bit 5 - DMA channel enabled to read data for the injected channel group This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
    #[inline(always)]
    pub fn jdmaen(&mut self) -> JDMAEN_W {
        JDMAEN_W { w: self }
    }
    #[doc = "Bits 8:12 - Trigger signal selection for launching injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Note: synchronous trigger has latency up to one fDFSDMCLK clock cycle (with deterministic jitter), asynchronous trigger has latency 2-3 fDFSDMCLK clock cycles (with jitter up to 1 cycle). DFSDM_FLTx 0x00 dfsdm_jtrg0 0x01 dfsdm_jtrg1 ... 0x1E dfsdm_jtrg30 0x1F dfsdm_jtrg31 Refer to . 0x0-0x1F: Trigger inputs selected by the following table (internal or external trigger)."]
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W {
        JEXTSEL_W { w: self }
    }
    #[doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
    #[inline(always)]
    pub fn jexten(&mut self) -> JEXTEN_W {
        JEXTEN_W { w: self }
    }
    #[doc = "Bit 17 - Software start of a conversion on the regular channel This bit is always read as '0â\u{80}\u{99}."]
    #[inline(always)]
    pub fn rswstart(&mut self) -> RSWSTART_W {
        RSWSTART_W { w: self }
    }
    #[doc = "Bit 18 - Continuous mode selection for regular conversions Writing '0â\u{80}\u{99} to this bit while a continuous regular conversion is already in progress stops the continuous mode immediately."]
    #[inline(always)]
    pub fn rcont(&mut self) -> RCONT_W {
        RCONT_W { w: self }
    }
    #[doc = "Bit 19 - Launch regular conversion synchronously with DFSDM_FLT0 This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
    #[inline(always)]
    pub fn rsync(&mut self) -> RSYNC_W {
        RSYNC_W { w: self }
    }
    #[doc = "Bit 21 - DMA channel enabled to read data for the regular conversion This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
    #[inline(always)]
    pub fn rdmaen(&mut self) -> RDMAEN_W {
        RDMAEN_W { w: self }
    }
    #[doc = "Bits 24:26 - Regular channel selection ... 7: Channel 7 is selected as the regular channel Writing these bits when RCIP=1 takes effect when the next regular conversion begins. This is especially useful in continuous mode (when RCONT=1). It also affects regular conversions which are pending (due to ongoing injected conversion)."]
    #[inline(always)]
    pub fn rch(&mut self) -> RCH_W {
        RCH_W { w: self }
    }
    #[doc = "Bit 29 - Fast conversion mode selection for regular conversions When converting a regular conversion in continuous mode, having enabled the fast mode causes each conversion (except the first) to execute faster than in standard mode. This bit has no effect on conversions which are not continuous. This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). if FAST=0 (or first conversion in continuous mode if FAST=1): t = \\[FOSR * (IOSR-1 + FORD) + FORD\\]
/ fCKIN ..... for Sincx filters t = \\[FOSR * (IOSR-1 + 4) + 2\\]
/ fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \\[FOSR * IOSR\\]
/ fCKIN in case if FOSR = FOSR\\[9:0\\]+1 = 1 (filter bypassed, active only integrator): t = IOSR / fCKIN (... but CNVCNT=0) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input."]
    #[inline(always)]
    pub fn fast(&mut self) -> FAST_W {
        FAST_W { w: self }
    }
    #[doc = "Bit 30 - Analog watchdog fast mode select"]
    #[inline(always)]
    pub fn awfsel(&mut self) -> AWFSEL_W {
        AWFSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
