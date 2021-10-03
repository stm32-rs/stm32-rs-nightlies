#[doc = "Register `ADC_CFGR2` reader"]
pub struct R(crate::R<ADC_CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CFGR2` writer"]
pub struct W(crate::W<ADC_CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CFGR2_SPEC>;
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
impl From<crate::W<ADC_CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVSE_A {
    #[doc = "0: Oversampler disabled"]
    B_0X0 = 0,
    #[doc = "1: Oversampler enabled"]
    B_0X1 = 1,
}
impl From<OVSE_A> for bool {
    #[inline(always)]
    fn from(variant: OVSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVSE` reader - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub struct OVSE_R(crate::FieldReader<bool, OVSE_A>);
impl OVSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVSE_A {
        match self.bits {
            false => OVSE_A::B_0X0,
            true => OVSE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == OVSE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == OVSE_A::B_0X1
    }
}
impl core::ops::Deref for OVSE_R {
    type Target = crate::FieldReader<bool, OVSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVSE` writer - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub struct OVSE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Oversampler disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OVSE_A::B_0X0)
    }
    #[doc = "Oversampler enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OVSE_A::B_0X1)
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
#[doc = "Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OVSR_A {
    #[doc = "0: 2x"]
    B_0X0 = 0,
    #[doc = "1: 4x"]
    B_0X1 = 1,
    #[doc = "2: 8x"]
    B_0X2 = 2,
    #[doc = "3: 16x"]
    B_0X3 = 3,
    #[doc = "4: 32x"]
    B_0X4 = 4,
    #[doc = "5: 64x"]
    B_0X5 = 5,
    #[doc = "6: 128x"]
    B_0X6 = 6,
    #[doc = "7: 256x"]
    B_0X7 = 7,
}
impl From<OVSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OVSR` reader - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub struct OVSR_R(crate::FieldReader<u8, OVSR_A>);
impl OVSR_R {
    pub(crate) fn new(bits: u8) -> Self {
        OVSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVSR_A {
        match self.bits {
            0 => OVSR_A::B_0X0,
            1 => OVSR_A::B_0X1,
            2 => OVSR_A::B_0X2,
            3 => OVSR_A::B_0X3,
            4 => OVSR_A::B_0X4,
            5 => OVSR_A::B_0X5,
            6 => OVSR_A::B_0X6,
            7 => OVSR_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == OVSR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == OVSR_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == OVSR_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == OVSR_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == OVSR_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == OVSR_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == OVSR_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == OVSR_A::B_0X7
    }
}
impl core::ops::Deref for OVSR_R {
    type Target = crate::FieldReader<u8, OVSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVSR` writer - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub struct OVSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVSR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2x"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OVSR_A::B_0X0)
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OVSR_A::B_0X1)
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(OVSR_A::B_0X2)
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(OVSR_A::B_0X3)
    }
    #[doc = "32x"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(OVSR_A::B_0X4)
    }
    #[doc = "64x"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(OVSR_A::B_0X5)
    }
    #[doc = "128x"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(OVSR_A::B_0X6)
    }
    #[doc = "256x"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(OVSR_A::B_0X7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OVSS_A {
    #[doc = "0: No shift"]
    B_0X0 = 0,
    #[doc = "1: Shift 1-bit"]
    B_0X1 = 1,
    #[doc = "2: Shift 2-bits"]
    B_0X2 = 2,
    #[doc = "3: Shift 3-bits"]
    B_0X3 = 3,
    #[doc = "4: Shift 4-bits"]
    B_0X4 = 4,
    #[doc = "5: Shift 5-bits"]
    B_0X5 = 5,
    #[doc = "6: Shift 6-bits"]
    B_0X6 = 6,
    #[doc = "7: Shift 7-bits"]
    B_0X7 = 7,
    #[doc = "8: Shift 8-bits"]
    B_0X8 = 8,
}
impl From<OVSS_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OVSS` reader - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub struct OVSS_R(crate::FieldReader<u8, OVSS_A>);
impl OVSS_R {
    pub(crate) fn new(bits: u8) -> Self {
        OVSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OVSS_A> {
        match self.bits {
            0 => Some(OVSS_A::B_0X0),
            1 => Some(OVSS_A::B_0X1),
            2 => Some(OVSS_A::B_0X2),
            3 => Some(OVSS_A::B_0X3),
            4 => Some(OVSS_A::B_0X4),
            5 => Some(OVSS_A::B_0X5),
            6 => Some(OVSS_A::B_0X6),
            7 => Some(OVSS_A::B_0X7),
            8 => Some(OVSS_A::B_0X8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == OVSS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == OVSS_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == OVSS_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == OVSS_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == OVSS_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == OVSS_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == OVSS_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == OVSS_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        **self == OVSS_A::B_0X8
    }
}
impl core::ops::Deref for OVSS_R {
    type Target = crate::FieldReader<u8, OVSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVSS` writer - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub struct OVSS_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVSS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No shift"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OVSS_A::B_0X0)
    }
    #[doc = "Shift 1-bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OVSS_A::B_0X1)
    }
    #[doc = "Shift 2-bits"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(OVSS_A::B_0X2)
    }
    #[doc = "Shift 3-bits"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(OVSS_A::B_0X3)
    }
    #[doc = "Shift 4-bits"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(OVSS_A::B_0X4)
    }
    #[doc = "Shift 5-bits"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(OVSS_A::B_0X5)
    }
    #[doc = "Shift 6-bits"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(OVSS_A::B_0X6)
    }
    #[doc = "Shift 7-bits"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(OVSS_A::B_0X7)
    }
    #[doc = "Shift 8-bits"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(OVSS_A::B_0X8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | ((value as u32 & 0x0f) << 5);
        self.w
    }
}
#[doc = "Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOVS_A {
    #[doc = "0: All oversampled conversions for a channel are done consecutively after a trigger"]
    B_0X0 = 0,
    #[doc = "1: Each oversampled conversion for a channel needs a trigger"]
    B_0X1 = 1,
}
impl From<TOVS_A> for bool {
    #[inline(always)]
    fn from(variant: TOVS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOVS` reader - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub struct TOVS_R(crate::FieldReader<bool, TOVS_A>);
impl TOVS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOVS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOVS_A {
        match self.bits {
            false => TOVS_A::B_0X0,
            true => TOVS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TOVS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TOVS_A::B_0X1
    }
}
impl core::ops::Deref for TOVS_R {
    type Target = crate::FieldReader<bool, TOVS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOVS` writer - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub struct TOVS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOVS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOVS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "All oversampled conversions for a channel are done consecutively after a trigger"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TOVS_A::B_0X0)
    }
    #[doc = "Each oversampled conversion for a channel needs a trigger"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TOVS_A::B_0X1)
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
#[doc = "Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFTRIG_A {
    #[doc = "0: Low Frequency Trigger Mode disabled"]
    B_0X0 = 0,
    #[doc = "1: Low Frequency Trigger Mode enabled"]
    B_0X1 = 1,
}
impl From<LFTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: LFTRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFTRIG` reader - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub struct LFTRIG_R(crate::FieldReader<bool, LFTRIG_A>);
impl LFTRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFTRIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFTRIG_A {
        match self.bits {
            false => LFTRIG_A::B_0X0,
            true => LFTRIG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == LFTRIG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == LFTRIG_A::B_0X1
    }
}
impl core::ops::Deref for LFTRIG_R {
    type Target = crate::FieldReader<bool, LFTRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFTRIG` writer - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub struct LFTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> LFTRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFTRIG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low Frequency Trigger Mode disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LFTRIG_A::B_0X0)
    }
    #[doc = "Low Frequency Trigger Mode enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LFTRIG_A::B_0X1)
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
#[doc = "ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKMODE_A {
    #[doc = "0: ADCCLK (Asynchronous clock mode), generated at product level (refer to RCC section)"]
    B_0X0 = 0,
    #[doc = "1: PCLK/2 (Synchronous clock mode)"]
    B_0X1 = 1,
    #[doc = "2: PCLK/4 (Synchronous clock mode)"]
    B_0X2 = 2,
    #[doc = "3: PCLK (Synchronous clock mode). This configuration must be enabled only if PCLK has a 50% duty clock cycle (APB prescaler configured inside the RCC must be bypassed and the system clock must by 50% duty cycle)"]
    B_0X3 = 3,
}
impl From<CKMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CKMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CKMODE` reader - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0)."]
pub struct CKMODE_R(crate::FieldReader<u8, CKMODE_A>);
impl CKMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKMODE_A {
        match self.bits {
            0 => CKMODE_A::B_0X0,
            1 => CKMODE_A::B_0X1,
            2 => CKMODE_A::B_0X2,
            3 => CKMODE_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CKMODE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CKMODE_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == CKMODE_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == CKMODE_A::B_0X3
    }
}
impl core::ops::Deref for CKMODE_R {
    type Target = crate::FieldReader<u8, CKMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKMODE` writer - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0)."]
pub struct CKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "ADCCLK (Asynchronous clock mode), generated at product level (refer to RCC section)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CKMODE_A::B_0X0)
    }
    #[doc = "PCLK/2 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CKMODE_A::B_0X1)
    }
    #[doc = "PCLK/4 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(CKMODE_A::B_0X2)
    }
    #[doc = "PCLK (Synchronous clock mode). This configuration must be enabled only if PCLK has a 50% duty clock cycle (APB prescaler configured inside the RCC must be bypassed and the system clock must by 50% duty cycle)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(CKMODE_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovse(&self) -> OVSE_R {
        OVSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 5:8 - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn lftrig(&self) -> LFTRIG_R {
        LFTRIG_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0)."]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovse(&mut self) -> OVSE_W {
        OVSE_W { w: self }
    }
    #[doc = "Bits 2:4 - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovsr(&mut self) -> OVSR_W {
        OVSR_W { w: self }
    }
    #[doc = "Bits 5:8 - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovss(&mut self) -> OVSS_W {
        OVSS_W { w: self }
    }
    #[doc = "Bit 9 - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn tovs(&mut self) -> TOVS_W {
        TOVS_W { w: self }
    }
    #[doc = "Bit 29 - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn lftrig(&mut self) -> LFTRIG_W {
        LFTRIG_W { w: self }
    }
    #[doc = "Bits 30:31 - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0)."]
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W {
        CKMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_cfgr2](index.html) module"]
pub struct ADC_CFGR2_SPEC;
impl crate::RegisterSpec for ADC_CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_cfgr2::R](R) reader structure"]
impl crate::Readable for ADC_CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_cfgr2::W](W) writer structure"]
impl crate::Writable for ADC_CFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_CFGR2 to value 0"]
impl crate::Resettable for ADC_CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
