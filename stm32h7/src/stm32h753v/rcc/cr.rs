#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Internal high-speed clock enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSION_A {
    #[doc = "0: Clock Off"]
    OFF = 0,
    #[doc = "1: Clock On"]
    ON = 1,
}
impl From<HSION_A> for bool {
    #[inline(always)]
    fn from(variant: HSION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSION` reader - Internal high-speed clock enable"]
pub struct HSION_R(crate::FieldReader<bool, HSION_A>);
impl HSION_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSION_A {
        match self.bits {
            false => HSION_A::OFF,
            true => HSION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == HSION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == HSION_A::ON
    }
}
impl core::ops::Deref for HSION_R {
    type Target = crate::FieldReader<bool, HSION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSION` writer - Internal high-speed clock enable"]
pub struct HSION_W<'a> {
    w: &'a mut W,
}
impl<'a> HSION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSION_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(HSION_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(HSION_A::ON)
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
#[doc = "High Speed Internal clock enable in Stop mode"]
pub type HSIKERON_A = HSION_A;
#[doc = "Field `HSIKERON` reader - High Speed Internal clock enable in Stop mode"]
pub type HSIKERON_R = HSION_R;
#[doc = "Field `HSIKERON` writer - High Speed Internal clock enable in Stop mode"]
pub struct HSIKERON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIKERON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSIKERON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(HSIKERON_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(HSIKERON_A::ON)
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
#[doc = "HSI clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIRDY_A {
    #[doc = "0: Clock not ready"]
    NOTREADY = 0,
    #[doc = "1: Clock ready"]
    READY = 1,
}
impl From<HSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDY` reader - HSI clock ready flag"]
pub struct HSIRDY_R(crate::FieldReader<bool, HSIRDY_A>);
impl HSIRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIRDY_A {
        match self.bits {
            false => HSIRDY_A::NOTREADY,
            true => HSIRDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == HSIRDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == HSIRDY_A::READY
    }
}
impl core::ops::Deref for HSIRDY_R {
    type Target = crate::FieldReader<bool, HSIRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSIRDY` writer - HSI clock ready flag"]
pub struct HSIRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSIRDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(HSIRDY_A::NOTREADY)
    }
    #[doc = "Clock ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(HSIRDY_A::READY)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "HSI clock divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HSIDIV_A {
    #[doc = "0: No division"]
    DIV1 = 0,
    #[doc = "1: Division by 2"]
    DIV2 = 1,
    #[doc = "2: Division by 4"]
    DIV4 = 2,
    #[doc = "3: Division by 8"]
    DIV8 = 3,
}
impl From<HSIDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: HSIDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HSIDIV` reader - HSI clock divider"]
pub struct HSIDIV_R(crate::FieldReader<u8, HSIDIV_A>);
impl HSIDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSIDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIDIV_A {
        match self.bits {
            0 => HSIDIV_A::DIV1,
            1 => HSIDIV_A::DIV2,
            2 => HSIDIV_A::DIV4,
            3 => HSIDIV_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == HSIDIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == HSIDIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == HSIDIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == HSIDIV_A::DIV8
    }
}
impl core::ops::Deref for HSIDIV_R {
    type Target = crate::FieldReader<u8, HSIDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSIDIV` writer - HSI clock divider"]
pub struct HSIDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSIDIV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HSIDIV_A::DIV1)
    }
    #[doc = "Division by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HSIDIV_A::DIV2)
    }
    #[doc = "Division by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HSIDIV_A::DIV4)
    }
    #[doc = "Division by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(HSIDIV_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "HSI divider flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIDIVF_A {
    #[doc = "0: New HSIDIV ratio has not yet propagated to hsi_ck"]
    NOTPROPAGATED = 0,
    #[doc = "1: HSIDIV ratio has propagated to hsi_ck"]
    PROPAGATED = 1,
}
impl From<HSIDIVF_A> for bool {
    #[inline(always)]
    fn from(variant: HSIDIVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIDIVF` reader - HSI divider flag"]
pub struct HSIDIVF_R(crate::FieldReader<bool, HSIDIVF_A>);
impl HSIDIVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIDIVF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIDIVF_A {
        match self.bits {
            false => HSIDIVF_A::NOTPROPAGATED,
            true => HSIDIVF_A::PROPAGATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPROPAGATED`"]
    #[inline(always)]
    pub fn is_not_propagated(&self) -> bool {
        **self == HSIDIVF_A::NOTPROPAGATED
    }
    #[doc = "Checks if the value of the field is `PROPAGATED`"]
    #[inline(always)]
    pub fn is_propagated(&self) -> bool {
        **self == HSIDIVF_A::PROPAGATED
    }
}
impl core::ops::Deref for HSIDIVF_R {
    type Target = crate::FieldReader<bool, HSIDIVF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSIDIVF` writer - HSI divider flag"]
pub struct HSIDIVF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIDIVF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSIDIVF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "New HSIDIV ratio has not yet propagated to hsi_ck"]
    #[inline(always)]
    pub fn not_propagated(self) -> &'a mut W {
        self.variant(HSIDIVF_A::NOTPROPAGATED)
    }
    #[doc = "HSIDIV ratio has propagated to hsi_ck"]
    #[inline(always)]
    pub fn propagated(self) -> &'a mut W {
        self.variant(HSIDIVF_A::PROPAGATED)
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
#[doc = "CSI clock enable"]
pub type CSION_A = HSION_A;
#[doc = "Field `CSION` reader - CSI clock enable"]
pub type CSION_R = HSION_R;
#[doc = "Field `CSION` writer - CSI clock enable"]
pub struct CSION_W<'a> {
    w: &'a mut W,
}
impl<'a> CSION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSION_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CSION_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CSION_A::ON)
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
#[doc = "CSI clock ready flag"]
pub type CSIRDY_A = HSIRDY_A;
#[doc = "Field `CSIRDY` reader - CSI clock ready flag"]
pub type CSIRDY_R = HSIRDY_R;
#[doc = "Field `CSIRDY` writer - CSI clock ready flag"]
pub struct CSIRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CSIRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSIRDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(CSIRDY_A::NOTREADY)
    }
    #[doc = "Clock ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(CSIRDY_A::READY)
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
#[doc = "CSI clock enable in Stop mode"]
pub type CSIKERON_A = HSION_A;
#[doc = "Field `CSIKERON` reader - CSI clock enable in Stop mode"]
pub type CSIKERON_R = HSION_R;
#[doc = "Field `CSIKERON` writer - CSI clock enable in Stop mode"]
pub struct CSIKERON_W<'a> {
    w: &'a mut W,
}
impl<'a> CSIKERON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSIKERON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CSIKERON_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CSIKERON_A::ON)
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
#[doc = "RC48 clock enable"]
pub type HSI48ON_A = HSION_A;
#[doc = "Field `HSI48ON` reader - RC48 clock enable"]
pub type HSI48ON_R = HSION_R;
#[doc = "Field `HSI48ON` writer - RC48 clock enable"]
pub struct HSI48ON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI48ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSI48ON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(HSI48ON_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(HSI48ON_A::ON)
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
#[doc = "RC48 clock ready flag"]
pub type HSI48RDY_A = HSIRDY_A;
#[doc = "Field `HSI48RDY` reader - RC48 clock ready flag"]
pub type HSI48RDY_R = HSIRDY_R;
#[doc = "Field `HSI48RDY` writer - RC48 clock ready flag"]
pub struct HSI48RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI48RDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSI48RDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(HSI48RDY_A::NOTREADY)
    }
    #[doc = "Clock ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(HSI48RDY_A::READY)
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
#[doc = "D1 domain clocks ready flag"]
pub type D1CKRDY_A = HSIRDY_A;
#[doc = "Field `D1CKRDY` reader - D1 domain clocks ready flag"]
pub type D1CKRDY_R = HSIRDY_R;
#[doc = "Field `D1CKRDY` writer - D1 domain clocks ready flag"]
pub struct D1CKRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> D1CKRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: D1CKRDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(D1CKRDY_A::NOTREADY)
    }
    #[doc = "Clock ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(D1CKRDY_A::READY)
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
#[doc = "D2 domain clocks ready flag"]
pub type D2CKRDY_A = HSIRDY_A;
#[doc = "Field `D2CKRDY` reader - D2 domain clocks ready flag"]
pub type D2CKRDY_R = HSIRDY_R;
#[doc = "Field `D2CKRDY` writer - D2 domain clocks ready flag"]
pub struct D2CKRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> D2CKRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: D2CKRDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(D2CKRDY_A::NOTREADY)
    }
    #[doc = "Clock ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(D2CKRDY_A::READY)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "HSE clock enable"]
pub type HSEON_A = HSION_A;
#[doc = "Field `HSEON` reader - HSE clock enable"]
pub type HSEON_R = HSION_R;
#[doc = "Field `HSEON` writer - HSE clock enable"]
pub struct HSEON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSEON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(HSEON_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(HSEON_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "HSE clock ready flag"]
pub type HSERDY_A = HSIRDY_A;
#[doc = "Field `HSERDY` reader - HSE clock ready flag"]
pub type HSERDY_R = HSIRDY_R;
#[doc = "Field `HSERDY` writer - HSE clock ready flag"]
pub struct HSERDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HSERDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSERDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(HSERDY_A::NOTREADY)
    }
    #[doc = "Clock ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(HSERDY_A::READY)
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
#[doc = "HSE clock bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSEBYP_A {
    #[doc = "0: HSE crystal oscillator not bypassed"]
    NOTBYPASSED = 0,
    #[doc = "1: HSE crystal oscillator bypassed with external clock"]
    BYPASSED = 1,
}
impl From<HSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEBYP` reader - HSE clock bypass"]
pub struct HSEBYP_R(crate::FieldReader<bool, HSEBYP_A>);
impl HSEBYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSEBYP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEBYP_A {
        match self.bits {
            false => HSEBYP_A::NOTBYPASSED,
            true => HSEBYP_A::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBYPASSED`"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        **self == HSEBYP_A::NOTBYPASSED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        **self == HSEBYP_A::BYPASSED
    }
}
impl core::ops::Deref for HSEBYP_R {
    type Target = crate::FieldReader<bool, HSEBYP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSEBYP` writer - HSE clock bypass"]
pub struct HSEBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEBYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSEBYP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HSE crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(HSEBYP_A::NOTBYPASSED)
    }
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(HSEBYP_A::BYPASSED)
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
#[doc = "HSE Clock Security System enable"]
pub type HSECSSON_A = HSION_A;
#[doc = "Field `HSECSSON` reader - HSE Clock Security System enable"]
pub type HSECSSON_R = HSION_R;
#[doc = "Field `HSECSSON` writer - HSE Clock Security System enable"]
pub struct HSECSSON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSECSSON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSECSSON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(HSECSSON_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(HSECSSON_A::ON)
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
#[doc = "PLL1 enable"]
pub type PLL1ON_A = HSION_A;
#[doc = "Field `PLL1ON` reader - PLL1 enable"]
pub type PLL1ON_R = HSION_R;
#[doc = "Field `PLL1ON` writer - PLL1 enable"]
pub struct PLL1ON_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL1ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL1ON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PLL1ON_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(PLL1ON_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "PLL1 clock ready flag"]
pub type PLL1RDY_A = HSIRDY_A;
#[doc = "Field `PLL1RDY` reader - PLL1 clock ready flag"]
pub type PLL1RDY_R = HSIRDY_R;
#[doc = "Field `PLL1RDY` writer - PLL1 clock ready flag"]
pub struct PLL1RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL1RDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL1RDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(PLL1RDY_A::NOTREADY)
    }
    #[doc = "Clock ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(PLL1RDY_A::READY)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "PLL2 enable"]
pub type PLL2ON_A = HSION_A;
#[doc = "Field `PLL2ON` reader - PLL2 enable"]
pub type PLL2ON_R = HSION_R;
#[doc = "Field `PLL2ON` writer - PLL2 enable"]
pub struct PLL2ON_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL2ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL2ON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PLL2ON_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(PLL2ON_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "PLL2 clock ready flag"]
pub type PLL2RDY_A = HSIRDY_A;
#[doc = "Field `PLL2RDY` reader - PLL2 clock ready flag"]
pub type PLL2RDY_R = HSIRDY_R;
#[doc = "Field `PLL2RDY` writer - PLL2 clock ready flag"]
pub struct PLL2RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL2RDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL2RDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(PLL2RDY_A::NOTREADY)
    }
    #[doc = "Clock ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(PLL2RDY_A::READY)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "PLL3 enable"]
pub type PLL3ON_A = HSION_A;
#[doc = "Field `PLL3ON` reader - PLL3 enable"]
pub type PLL3ON_R = HSION_R;
#[doc = "Field `PLL3ON` writer - PLL3 enable"]
pub struct PLL3ON_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL3ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL3ON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PLL3ON_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(PLL3ON_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "PLL3 clock ready flag"]
pub type PLL3RDY_A = HSIRDY_A;
#[doc = "Field `PLL3RDY` reader - PLL3 clock ready flag"]
pub type PLL3RDY_R = HSIRDY_R;
#[doc = "Field `PLL3RDY` writer - PLL3 clock ready flag"]
pub struct PLL3RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL3RDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL3RDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(PLL3RDY_A::NOTREADY)
    }
    #[doc = "Clock ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(PLL3RDY_A::READY)
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
impl R {
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - High Speed Internal clock enable in Stop mode"]
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSI clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - HSI clock divider"]
    #[inline(always)]
    pub fn hsidiv(&self) -> HSIDIV_R {
        HSIDIV_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - HSI divider flag"]
    #[inline(always)]
    pub fn hsidivf(&self) -> HSIDIVF_R {
        HSIDIVF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CSI clock enable"]
    #[inline(always)]
    pub fn csion(&self) -> CSION_R {
        CSION_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CSI clock ready flag"]
    #[inline(always)]
    pub fn csirdy(&self) -> CSIRDY_R {
        CSIRDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CSI clock enable in Stop mode"]
    #[inline(always)]
    pub fn csikeron(&self) -> CSIKERON_R {
        CSIKERON_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RC48 clock enable"]
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RC48 clock ready flag"]
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - D1 domain clocks ready flag"]
    #[inline(always)]
    pub fn d1ckrdy(&self) -> D1CKRDY_R {
        D1CKRDY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - D2 domain clocks ready flag"]
    #[inline(always)]
    pub fn d2ckrdy(&self) -> D2CKRDY_R {
        D2CKRDY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - HSE clock bypass"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - HSE Clock Security System enable"]
    #[inline(always)]
    pub fn hsecsson(&self) -> HSECSSON_R {
        HSECSSON_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - PLL1 enable"]
    #[inline(always)]
    pub fn pll1on(&self) -> PLL1ON_R {
        PLL1ON_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - PLL1 clock ready flag"]
    #[inline(always)]
    pub fn pll1rdy(&self) -> PLL1RDY_R {
        PLL1RDY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - PLL2 enable"]
    #[inline(always)]
    pub fn pll2on(&self) -> PLL2ON_R {
        PLL2ON_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PLL2 clock ready flag"]
    #[inline(always)]
    pub fn pll2rdy(&self) -> PLL2RDY_R {
        PLL2RDY_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PLL3 enable"]
    #[inline(always)]
    pub fn pll3on(&self) -> PLL3ON_R {
        PLL3ON_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - PLL3 clock ready flag"]
    #[inline(always)]
    pub fn pll3rdy(&self) -> PLL3RDY_R {
        PLL3RDY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W {
        HSION_W { w: self }
    }
    #[doc = "Bit 1 - High Speed Internal clock enable in Stop mode"]
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HSIKERON_W {
        HSIKERON_W { w: self }
    }
    #[doc = "Bit 2 - HSI clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&mut self) -> HSIRDY_W {
        HSIRDY_W { w: self }
    }
    #[doc = "Bits 3:4 - HSI clock divider"]
    #[inline(always)]
    pub fn hsidiv(&mut self) -> HSIDIV_W {
        HSIDIV_W { w: self }
    }
    #[doc = "Bit 5 - HSI divider flag"]
    #[inline(always)]
    pub fn hsidivf(&mut self) -> HSIDIVF_W {
        HSIDIVF_W { w: self }
    }
    #[doc = "Bit 7 - CSI clock enable"]
    #[inline(always)]
    pub fn csion(&mut self) -> CSION_W {
        CSION_W { w: self }
    }
    #[doc = "Bit 8 - CSI clock ready flag"]
    #[inline(always)]
    pub fn csirdy(&mut self) -> CSIRDY_W {
        CSIRDY_W { w: self }
    }
    #[doc = "Bit 9 - CSI clock enable in Stop mode"]
    #[inline(always)]
    pub fn csikeron(&mut self) -> CSIKERON_W {
        CSIKERON_W { w: self }
    }
    #[doc = "Bit 12 - RC48 clock enable"]
    #[inline(always)]
    pub fn hsi48on(&mut self) -> HSI48ON_W {
        HSI48ON_W { w: self }
    }
    #[doc = "Bit 13 - RC48 clock ready flag"]
    #[inline(always)]
    pub fn hsi48rdy(&mut self) -> HSI48RDY_W {
        HSI48RDY_W { w: self }
    }
    #[doc = "Bit 14 - D1 domain clocks ready flag"]
    #[inline(always)]
    pub fn d1ckrdy(&mut self) -> D1CKRDY_W {
        D1CKRDY_W { w: self }
    }
    #[doc = "Bit 15 - D2 domain clocks ready flag"]
    #[inline(always)]
    pub fn d2ckrdy(&mut self) -> D2CKRDY_W {
        D2CKRDY_W { w: self }
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W {
        HSEON_W { w: self }
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&mut self) -> HSERDY_W {
        HSERDY_W { w: self }
    }
    #[doc = "Bit 18 - HSE clock bypass"]
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W {
        HSEBYP_W { w: self }
    }
    #[doc = "Bit 19 - HSE Clock Security System enable"]
    #[inline(always)]
    pub fn hsecsson(&mut self) -> HSECSSON_W {
        HSECSSON_W { w: self }
    }
    #[doc = "Bit 24 - PLL1 enable"]
    #[inline(always)]
    pub fn pll1on(&mut self) -> PLL1ON_W {
        PLL1ON_W { w: self }
    }
    #[doc = "Bit 25 - PLL1 clock ready flag"]
    #[inline(always)]
    pub fn pll1rdy(&mut self) -> PLL1RDY_W {
        PLL1RDY_W { w: self }
    }
    #[doc = "Bit 26 - PLL2 enable"]
    #[inline(always)]
    pub fn pll2on(&mut self) -> PLL2ON_W {
        PLL2ON_W { w: self }
    }
    #[doc = "Bit 27 - PLL2 clock ready flag"]
    #[inline(always)]
    pub fn pll2rdy(&mut self) -> PLL2RDY_W {
        PLL2RDY_W { w: self }
    }
    #[doc = "Bit 28 - PLL3 enable"]
    #[inline(always)]
    pub fn pll3on(&mut self) -> PLL3ON_W {
        PLL3ON_W { w: self }
    }
    #[doc = "Bit 29 - PLL3 clock ready flag"]
    #[inline(always)]
    pub fn pll3rdy(&mut self) -> PLL3RDY_W {
        PLL3RDY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0x83"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x83
    }
}
