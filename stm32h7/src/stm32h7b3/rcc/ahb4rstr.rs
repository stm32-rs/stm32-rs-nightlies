#[doc = "Register `AHB4RSTR` reader"]
pub struct R(crate::R<AHB4RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB4RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB4RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB4RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB4RSTR` writer"]
pub struct W(crate::W<AHB4RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB4RSTR_SPEC>;
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
impl From<crate::W<AHB4RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB4RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIOA block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOARST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<GPIOARST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOARST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOARST` reader - GPIOA block reset Set and reset by software."]
pub struct GPIOARST_R(crate::FieldReader<bool, GPIOARST_A>);
impl GPIOARST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOARST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIOARST_A> {
        match self.bits {
            true => Some(GPIOARST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == GPIOARST_A::RESET
    }
}
impl core::ops::Deref for GPIOARST_R {
    type Target = crate::FieldReader<bool, GPIOARST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOARST` writer - GPIOA block reset Set and reset by software."]
pub struct GPIOARST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOARST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOARST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIOARST_A::RESET)
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
#[doc = "GPIOB block reset Set and reset by software."]
pub type GPIOBRST_A = GPIOARST_A;
#[doc = "Field `GPIOBRST` reader - GPIOB block reset Set and reset by software."]
pub type GPIOBRST_R = GPIOARST_R;
#[doc = "Field `GPIOBRST` writer - GPIOB block reset Set and reset by software."]
pub struct GPIOBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOBRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOBRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIOBRST_A::RESET)
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
#[doc = "GPIOC block reset Set and reset by software."]
pub type GPIOCRST_A = GPIOARST_A;
#[doc = "Field `GPIOCRST` reader - GPIOC block reset Set and reset by software."]
pub type GPIOCRST_R = GPIOARST_R;
#[doc = "Field `GPIOCRST` writer - GPIOC block reset Set and reset by software."]
pub struct GPIOCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOCRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIOCRST_A::RESET)
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
#[doc = "GPIOD block reset Set and reset by software."]
pub type GPIODRST_A = GPIOARST_A;
#[doc = "Field `GPIODRST` reader - GPIOD block reset Set and reset by software."]
pub type GPIODRST_R = GPIOARST_R;
#[doc = "Field `GPIODRST` writer - GPIOD block reset Set and reset by software."]
pub struct GPIODRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIODRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIODRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIODRST_A::RESET)
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
#[doc = "GPIOE block reset Set and reset by software."]
pub type GPIOERST_A = GPIOARST_A;
#[doc = "Field `GPIOERST` reader - GPIOE block reset Set and reset by software."]
pub type GPIOERST_R = GPIOARST_R;
#[doc = "Field `GPIOERST` writer - GPIOE block reset Set and reset by software."]
pub struct GPIOERST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOERST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOERST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIOERST_A::RESET)
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
#[doc = "GPIOF block reset Set and reset by software."]
pub type GPIOFRST_A = GPIOARST_A;
#[doc = "Field `GPIOFRST` reader - GPIOF block reset Set and reset by software."]
pub type GPIOFRST_R = GPIOARST_R;
#[doc = "Field `GPIOFRST` writer - GPIOF block reset Set and reset by software."]
pub struct GPIOFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOFRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOFRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIOFRST_A::RESET)
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
#[doc = "GPIOG block reset Set and reset by software."]
pub type GPIOGRST_A = GPIOARST_A;
#[doc = "Field `GPIOGRST` reader - GPIOG block reset Set and reset by software."]
pub type GPIOGRST_R = GPIOARST_R;
#[doc = "Field `GPIOGRST` writer - GPIOG block reset Set and reset by software."]
pub struct GPIOGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOGRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOGRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIOGRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "GPIOH block reset Set and reset by software."]
pub type GPIOHRST_A = GPIOARST_A;
#[doc = "Field `GPIOHRST` reader - GPIOH block reset Set and reset by software."]
pub type GPIOHRST_R = GPIOARST_R;
#[doc = "Field `GPIOHRST` writer - GPIOH block reset Set and reset by software."]
pub struct GPIOHRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOHRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOHRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIOHRST_A::RESET)
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
#[doc = "GPIOI block reset Set and reset by software."]
pub type GPIOIRST_A = GPIOARST_A;
#[doc = "Field `GPIOIRST` reader - GPIOI block reset Set and reset by software."]
pub type GPIOIRST_R = GPIOARST_R;
#[doc = "Field `GPIOIRST` writer - GPIOI block reset Set and reset by software."]
pub struct GPIOIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOIRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOIRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIOIRST_A::RESET)
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
#[doc = "GPIOJ block reset Set and reset by software."]
pub type GPIOJRST_A = GPIOARST_A;
#[doc = "Field `GPIOJRST` reader - GPIOJ block reset Set and reset by software."]
pub type GPIOJRST_R = GPIOARST_R;
#[doc = "Field `GPIOJRST` writer - GPIOJ block reset Set and reset by software."]
pub struct GPIOJRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOJRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOJRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIOJRST_A::RESET)
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
#[doc = "GPIOK block reset Set and reset by software."]
pub type GPIOKRST_A = GPIOARST_A;
#[doc = "Field `GPIOKRST` reader - GPIOK block reset Set and reset by software."]
pub type GPIOKRST_R = GPIOARST_R;
#[doc = "Field `GPIOKRST` writer - GPIOK block reset Set and reset by software."]
pub struct GPIOKRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOKRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOKRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIOKRST_A::RESET)
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
#[doc = "SmartRun domain DMA and DMAMUX blocks reset Set and reset by software."]
pub type BDMA2RST_A = GPIOARST_A;
#[doc = "Field `BDMA2RST` reader - SmartRun domain DMA and DMAMUX blocks reset Set and reset by software."]
pub type BDMA2RST_R = GPIOARST_R;
#[doc = "Field `BDMA2RST` writer - SmartRun domain DMA and DMAMUX blocks reset Set and reset by software."]
pub struct BDMA2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> BDMA2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BDMA2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BDMA2RST_A::RESET)
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
impl R {
    #[doc = "Bit 0 - GPIOA block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIOB block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIOC block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIOD block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIOE block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIOF block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIOG block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIOH block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPIOI block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPIOJ block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiojrst(&self) -> GPIOJRST_R {
        GPIOJRST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPIOK block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiokrst(&self) -> GPIOKRST_R {
        GPIOKRST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SmartRun domain DMA and DMAMUX blocks reset Set and reset by software."]
    #[inline(always)]
    pub fn bdma2rst(&self) -> BDMA2RST_R {
        BDMA2RST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOA block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W {
        GPIOARST_W { w: self }
    }
    #[doc = "Bit 1 - GPIOB block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W {
        GPIOBRST_W { w: self }
    }
    #[doc = "Bit 2 - GPIOC block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W {
        GPIOCRST_W { w: self }
    }
    #[doc = "Bit 3 - GPIOD block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GPIODRST_W {
        GPIODRST_W { w: self }
    }
    #[doc = "Bit 4 - GPIOE block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GPIOERST_W {
        GPIOERST_W { w: self }
    }
    #[doc = "Bit 5 - GPIOF block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W {
        GPIOFRST_W { w: self }
    }
    #[doc = "Bit 6 - GPIOG block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W {
        GPIOGRST_W { w: self }
    }
    #[doc = "Bit 7 - GPIOH block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W {
        GPIOHRST_W { w: self }
    }
    #[doc = "Bit 8 - GPIOI block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpioirst(&mut self) -> GPIOIRST_W {
        GPIOIRST_W { w: self }
    }
    #[doc = "Bit 9 - GPIOJ block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiojrst(&mut self) -> GPIOJRST_W {
        GPIOJRST_W { w: self }
    }
    #[doc = "Bit 10 - GPIOK block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiokrst(&mut self) -> GPIOKRST_W {
        GPIOKRST_W { w: self }
    }
    #[doc = "Bit 21 - SmartRun domain DMA and DMAMUX blocks reset Set and reset by software."]
    #[inline(always)]
    pub fn bdma2rst(&mut self) -> BDMA2RST_W {
        BDMA2RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb4rstr](index.html) module"]
pub struct AHB4RSTR_SPEC;
impl crate::RegisterSpec for AHB4RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb4rstr::R](R) reader structure"]
impl crate::Readable for AHB4RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb4rstr::W](W) writer structure"]
impl crate::Writable for AHB4RSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB4RSTR to value 0"]
impl crate::Resettable for AHB4RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
