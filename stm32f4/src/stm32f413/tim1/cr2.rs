#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OIS4` reader - Output Idle state 4"]
pub struct OIS4_R(crate::FieldReader<bool, bool>);
impl OIS4_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS4` writer - Output Idle state 4"]
pub struct OIS4_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS4_W<'a> {
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
#[doc = "Field `OIS3N` reader - Output Idle state 3"]
pub struct OIS3N_R(crate::FieldReader<bool, bool>);
impl OIS3N_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS3N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS3N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS3N` writer - Output Idle state 3"]
pub struct OIS3N_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS3N_W<'a> {
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
#[doc = "Field `OIS3` reader - Output Idle state 3"]
pub struct OIS3_R(crate::FieldReader<bool, bool>);
impl OIS3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS3` writer - Output Idle state 3"]
pub struct OIS3_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS3_W<'a> {
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
#[doc = "Field `OIS2N` reader - Output Idle state 2"]
pub struct OIS2N_R(crate::FieldReader<bool, bool>);
impl OIS2N_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS2N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS2N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS2N` writer - Output Idle state 2"]
pub struct OIS2N_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS2N_W<'a> {
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
#[doc = "Field `OIS2` reader - Output Idle state 2"]
pub struct OIS2_R(crate::FieldReader<bool, bool>);
impl OIS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS2` writer - Output Idle state 2"]
pub struct OIS2_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS2_W<'a> {
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
#[doc = "Field `OIS1N` reader - Output Idle state 1"]
pub struct OIS1N_R(crate::FieldReader<bool, bool>);
impl OIS1N_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS1N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS1N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS1N` writer - Output Idle state 1"]
pub struct OIS1N_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS1N_W<'a> {
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
#[doc = "Field `OIS1` reader - Output Idle state 1"]
pub struct OIS1_R(crate::FieldReader<bool, bool>);
impl OIS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS1` writer - Output Idle state 1"]
pub struct OIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS1_W<'a> {
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
#[doc = "TI1 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TI1S_A {
    #[doc = "0: The TIMx_CH1 pin is connected to TI1 input"]
    NORMAL = 0,
    #[doc = "1: The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
    XOR = 1,
}
impl From<TI1S_A> for bool {
    #[inline(always)]
    fn from(variant: TI1S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TI1S` reader - TI1 selection"]
pub struct TI1S_R(crate::FieldReader<bool, TI1S_A>);
impl TI1S_R {
    pub(crate) fn new(bits: bool) -> Self {
        TI1S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TI1S_A {
        match self.bits {
            false => TI1S_A::NORMAL,
            true => TI1S_A::XOR,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == TI1S_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `XOR`"]
    #[inline(always)]
    pub fn is_xor(&self) -> bool {
        **self == TI1S_A::XOR
    }
}
impl core::ops::Deref for TI1S_R {
    type Target = crate::FieldReader<bool, TI1S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI1S` writer - TI1 selection"]
pub struct TI1S_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TI1S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TI1S_A::NORMAL)
    }
    #[doc = "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
    #[inline(always)]
    pub fn xor(self) -> &'a mut W {
        self.variant(TI1S_A::XOR)
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
#[doc = "Master mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MMS_A {
    #[doc = "0: The UG bit from the TIMx_EGR register is used as trigger output"]
    RESET = 0,
    #[doc = "1: The counter enable signal, CNT_EN, is used as trigger output"]
    ENABLE = 1,
    #[doc = "2: The update event is selected as trigger output"]
    UPDATE = 2,
    #[doc = "3: The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
    COMPAREPULSE = 3,
    #[doc = "4: OC1REF signal is used as trigger output"]
    COMPAREOC1 = 4,
    #[doc = "5: OC2REF signal is used as trigger output"]
    COMPAREOC2 = 5,
    #[doc = "6: OC3REF signal is used as trigger output"]
    COMPAREOC3 = 6,
    #[doc = "7: OC4REF signal is used as trigger output"]
    COMPAREOC4 = 7,
}
impl From<MMS_A> for u8 {
    #[inline(always)]
    fn from(variant: MMS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MMS` reader - Master mode selection"]
pub struct MMS_R(crate::FieldReader<u8, MMS_A>);
impl MMS_R {
    pub(crate) fn new(bits: u8) -> Self {
        MMS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMS_A {
        match self.bits {
            0 => MMS_A::RESET,
            1 => MMS_A::ENABLE,
            2 => MMS_A::UPDATE,
            3 => MMS_A::COMPAREPULSE,
            4 => MMS_A::COMPAREOC1,
            5 => MMS_A::COMPAREOC2,
            6 => MMS_A::COMPAREOC3,
            7 => MMS_A::COMPAREOC4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == MMS_A::RESET
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == MMS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        **self == MMS_A::UPDATE
    }
    #[doc = "Checks if the value of the field is `COMPAREPULSE`"]
    #[inline(always)]
    pub fn is_compare_pulse(&self) -> bool {
        **self == MMS_A::COMPAREPULSE
    }
    #[doc = "Checks if the value of the field is `COMPAREOC1`"]
    #[inline(always)]
    pub fn is_compare_oc1(&self) -> bool {
        **self == MMS_A::COMPAREOC1
    }
    #[doc = "Checks if the value of the field is `COMPAREOC2`"]
    #[inline(always)]
    pub fn is_compare_oc2(&self) -> bool {
        **self == MMS_A::COMPAREOC2
    }
    #[doc = "Checks if the value of the field is `COMPAREOC3`"]
    #[inline(always)]
    pub fn is_compare_oc3(&self) -> bool {
        **self == MMS_A::COMPAREOC3
    }
    #[doc = "Checks if the value of the field is `COMPAREOC4`"]
    #[inline(always)]
    pub fn is_compare_oc4(&self) -> bool {
        **self == MMS_A::COMPAREOC4
    }
}
impl core::ops::Deref for MMS_R {
    type Target = crate::FieldReader<u8, MMS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMS` writer - Master mode selection"]
pub struct MMS_W<'a> {
    w: &'a mut W,
}
impl<'a> MMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The UG bit from the TIMx_EGR register is used as trigger output"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MMS_A::RESET)
    }
    #[doc = "The counter enable signal, CNT_EN, is used as trigger output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MMS_A::ENABLE)
    }
    #[doc = "The update event is selected as trigger output"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(MMS_A::UPDATE)
    }
    #[doc = "The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
    #[inline(always)]
    pub fn compare_pulse(self) -> &'a mut W {
        self.variant(MMS_A::COMPAREPULSE)
    }
    #[doc = "OC1REF signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_oc1(self) -> &'a mut W {
        self.variant(MMS_A::COMPAREOC1)
    }
    #[doc = "OC2REF signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_oc2(self) -> &'a mut W {
        self.variant(MMS_A::COMPAREOC2)
    }
    #[doc = "OC3REF signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_oc3(self) -> &'a mut W {
        self.variant(MMS_A::COMPAREOC3)
    }
    #[doc = "OC4REF signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_oc4(self) -> &'a mut W {
        self.variant(MMS_A::COMPAREOC4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Capture/compare DMA selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCDS_A {
    #[doc = "0: CCx DMA request sent when CCx event occurs"]
    ONCOMPARE = 0,
    #[doc = "1: CCx DMA request sent when update event occurs"]
    ONUPDATE = 1,
}
impl From<CCDS_A> for bool {
    #[inline(always)]
    fn from(variant: CCDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCDS` reader - Capture/compare DMA selection"]
pub struct CCDS_R(crate::FieldReader<bool, CCDS_A>);
impl CCDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCDS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCDS_A {
        match self.bits {
            false => CCDS_A::ONCOMPARE,
            true => CCDS_A::ONUPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `ONCOMPARE`"]
    #[inline(always)]
    pub fn is_on_compare(&self) -> bool {
        **self == CCDS_A::ONCOMPARE
    }
    #[doc = "Checks if the value of the field is `ONUPDATE`"]
    #[inline(always)]
    pub fn is_on_update(&self) -> bool {
        **self == CCDS_A::ONUPDATE
    }
}
impl core::ops::Deref for CCDS_R {
    type Target = crate::FieldReader<bool, CCDS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCDS` writer - Capture/compare DMA selection"]
pub struct CCDS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCDS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCDS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn on_compare(self) -> &'a mut W {
        self.variant(CCDS_A::ONCOMPARE)
    }
    #[doc = "CCx DMA request sent when update event occurs"]
    #[inline(always)]
    pub fn on_update(self) -> &'a mut W {
        self.variant(CCDS_A::ONUPDATE)
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
#[doc = "Field `CCUS` reader - Capture/compare control update selection"]
pub struct CCUS_R(crate::FieldReader<bool, bool>);
impl CCUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCUS` writer - Capture/compare control update selection"]
pub struct CCUS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCUS_W<'a> {
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
#[doc = "Field `CCPC` reader - Capture/compare preloaded control"]
pub struct CCPC_R(crate::FieldReader<bool, bool>);
impl CCPC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCPC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCPC` writer - Capture/compare preloaded control"]
pub struct CCPC_W<'a> {
    w: &'a mut W,
}
impl<'a> CCPC_W<'a> {
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
impl R {
    #[doc = "Bit 14 - Output Idle state 4"]
    #[inline(always)]
    pub fn ois4(&self) -> OIS4_R {
        OIS4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Output Idle state 3"]
    #[inline(always)]
    pub fn ois3n(&self) -> OIS3N_R {
        OIS3N_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Output Idle state 3"]
    #[inline(always)]
    pub fn ois3(&self) -> OIS3_R {
        OIS3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Output Idle state 2"]
    #[inline(always)]
    pub fn ois2n(&self) -> OIS2N_R {
        OIS2N_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Output Idle state 2"]
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Output Idle state 4"]
    #[inline(always)]
    pub fn ois4(&mut self) -> OIS4_W {
        OIS4_W { w: self }
    }
    #[doc = "Bit 13 - Output Idle state 3"]
    #[inline(always)]
    pub fn ois3n(&mut self) -> OIS3N_W {
        OIS3N_W { w: self }
    }
    #[doc = "Bit 12 - Output Idle state 3"]
    #[inline(always)]
    pub fn ois3(&mut self) -> OIS3_W {
        OIS3_W { w: self }
    }
    #[doc = "Bit 11 - Output Idle state 2"]
    #[inline(always)]
    pub fn ois2n(&mut self) -> OIS2N_W {
        OIS2N_W { w: self }
    }
    #[doc = "Bit 10 - Output Idle state 2"]
    #[inline(always)]
    pub fn ois2(&mut self) -> OIS2_W {
        OIS2_W { w: self }
    }
    #[doc = "Bit 9 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1n(&mut self) -> OIS1N_W {
        OIS1N_W { w: self }
    }
    #[doc = "Bit 8 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1(&mut self) -> OIS1_W {
        OIS1_W { w: self }
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    pub fn ti1s(&mut self) -> TI1S_W {
        TI1S_W { w: self }
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W {
        MMS_W { w: self }
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&mut self) -> CCDS_W {
        CCDS_W { w: self }
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
    #[inline(always)]
    pub fn ccus(&mut self) -> CCUS_W {
        CCUS_W { w: self }
    }
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline(always)]
    pub fn ccpc(&mut self) -> CCPC_W {
        CCPC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
