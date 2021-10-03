#[doc = "Register `EGR` reader"]
pub struct R(crate::R<EGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EGR` writer"]
pub struct W(crate::W<EGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EGR_SPEC>;
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
impl From<crate::W<EGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Trigger generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TG_A {
    #[doc = "1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled."]
    TRIGGER = 1,
}
impl From<TG_A> for bool {
    #[inline(always)]
    fn from(variant: TG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TG` reader - Trigger generation"]
pub struct TG_R(crate::FieldReader<bool, TG_A>);
impl TG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TG_A> {
        match self.bits {
            true => Some(TG_A::TRIGGER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        **self == TG_A::TRIGGER
    }
}
impl core::ops::Deref for TG_R {
    type Target = crate::FieldReader<bool, TG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TG` writer - Trigger generation"]
pub struct TG_W<'a> {
    w: &'a mut W,
}
impl<'a> TG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled."]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TG_A::TRIGGER)
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
#[doc = "Capture/compare 4 generation"]
pub type CC4G_A = CC1G_A;
#[doc = "Field `CC4G` reader - Capture/compare 4 generation"]
pub type CC4G_R = CC1G_R;
#[doc = "Field `CC4G` writer - Capture/compare 4 generation"]
pub struct CC4G_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4G_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC4G_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register."]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(CC4G_A::TRIGGER)
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
#[doc = "Capture/compare 3 generation"]
pub type CC3G_A = CC1G_A;
#[doc = "Field `CC3G` reader - Capture/compare 3 generation"]
pub type CC3G_R = CC1G_R;
#[doc = "Field `CC3G` writer - Capture/compare 3 generation"]
pub struct CC3G_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3G_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC3G_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register."]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(CC3G_A::TRIGGER)
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
#[doc = "Capture/compare 2 generation"]
pub type CC2G_A = CC1G_A;
#[doc = "Field `CC2G` reader - Capture/compare 2 generation"]
pub type CC2G_R = CC1G_R;
#[doc = "Field `CC2G` writer - Capture/compare 2 generation"]
pub struct CC2G_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2G_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC2G_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register."]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(CC2G_A::TRIGGER)
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
#[doc = "Capture/compare 1 generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1G_A {
    #[doc = "1: If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register."]
    TRIGGER = 1,
}
impl From<CC1G_A> for bool {
    #[inline(always)]
    fn from(variant: CC1G_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1G` reader - Capture/compare 1 generation"]
pub struct CC1G_R(crate::FieldReader<bool, CC1G_A>);
impl CC1G_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1G_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CC1G_A> {
        match self.bits {
            true => Some(CC1G_A::TRIGGER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        **self == CC1G_A::TRIGGER
    }
}
impl core::ops::Deref for CC1G_R {
    type Target = crate::FieldReader<bool, CC1G_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC1G` writer - Capture/compare 1 generation"]
pub struct CC1G_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1G_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC1G_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register."]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(CC1G_A::TRIGGER)
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
#[doc = "Update generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UG_A {
    #[doc = "1: Re-initializes the timer counter and generates an update of the registers."]
    UPDATE = 1,
}
impl From<UG_A> for bool {
    #[inline(always)]
    fn from(variant: UG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UG` reader - Update generation"]
pub struct UG_R(crate::FieldReader<bool, UG_A>);
impl UG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UG_A> {
        match self.bits {
            true => Some(UG_A::UPDATE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        **self == UG_A::UPDATE
    }
}
impl core::ops::Deref for UG_R {
    type Target = crate::FieldReader<bool, UG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UG` writer - Update generation"]
pub struct UG_W<'a> {
    w: &'a mut W,
}
impl<'a> UG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Re-initializes the timer counter and generates an update of the registers."]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(UG_A::UPDATE)
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
impl R {
    #[doc = "Bit 6 - Trigger generation"]
    #[inline(always)]
    pub fn tg(&self) -> TG_R {
        TG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture/compare 4 generation"]
    #[inline(always)]
    pub fn cc4g(&self) -> CC4G_R {
        CC4G_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture/compare 3 generation"]
    #[inline(always)]
    pub fn cc3g(&self) -> CC3G_R {
        CC3G_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Capture/compare 2 generation"]
    #[inline(always)]
    pub fn cc2g(&self) -> CC2G_R {
        CC2G_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 generation"]
    #[inline(always)]
    pub fn cc1g(&self) -> CC1G_R {
        CC1G_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn ug(&self) -> UG_R {
        UG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Trigger generation"]
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W {
        TG_W { w: self }
    }
    #[doc = "Bit 4 - Capture/compare 4 generation"]
    #[inline(always)]
    pub fn cc4g(&mut self) -> CC4G_W {
        CC4G_W { w: self }
    }
    #[doc = "Bit 3 - Capture/compare 3 generation"]
    #[inline(always)]
    pub fn cc3g(&mut self) -> CC3G_W {
        CC3G_W { w: self }
    }
    #[doc = "Bit 2 - Capture/compare 2 generation"]
    #[inline(always)]
    pub fn cc2g(&mut self) -> CC2G_W {
        CC2G_W { w: self }
    }
    #[doc = "Bit 1 - Capture/compare 1 generation"]
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W {
        CC1G_W { w: self }
    }
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W {
        UG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "event generation register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [egr](index.html) module"]
pub struct EGR_SPEC;
impl crate::RegisterSpec for EGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [egr::R](R) reader structure"]
impl crate::Readable for EGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [egr::W](W) writer structure"]
impl crate::Writable for EGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
