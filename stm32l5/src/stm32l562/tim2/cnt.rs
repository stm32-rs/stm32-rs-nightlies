#[doc = "Register `CNT` reader"]
pub struct R(crate::R<CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNT` writer"]
pub struct W(crate::W<CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNT_SPEC>;
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
impl From<crate::W<CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_H` reader - Most significant part counter value (on TIM2 and TIM5)"]
pub struct CNT_H_R(crate::FieldReader<u16, u16>);
impl CNT_H_R {
    pub(crate) fn new(bits: u16) -> Self {
        CNT_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_H_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT_H` writer - Most significant part counter value (on TIM2 and TIM5)"]
pub struct CNT_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | ((value as u32 & 0x7fff) << 16);
        self.w
    }
}
#[doc = "Field `CNT_L` reader - Least significant part of counter value"]
pub struct CNT_L_R(crate::FieldReader<u16, u16>);
impl CNT_L_R {
    pub(crate) fn new(bits: u16) -> Self {
        CNT_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_L_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT_L` writer - Least significant part of counter value"]
pub struct CNT_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `CNT_bit31` reader - Most significant bit of counter value (on TIM2 and TIM5)"]
pub struct CNT_BIT31_R(crate::FieldReader<bool, bool>);
impl CNT_BIT31_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNT_BIT31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_BIT31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT_bit31` writer - Most significant bit of counter value (on TIM2 and TIM5)"]
pub struct CNT_BIT31_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_BIT31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:30 - Most significant part counter value (on TIM2 and TIM5)"]
    #[inline(always)]
    pub fn cnt_h(&self) -> CNT_H_R {
        CNT_H_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bits 0:15 - Least significant part of counter value"]
    #[inline(always)]
    pub fn cnt_l(&self) -> CNT_L_R {
        CNT_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Most significant bit of counter value (on TIM2 and TIM5)"]
    #[inline(always)]
    pub fn cnt_bit31(&self) -> CNT_BIT31_R {
        CNT_BIT31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:30 - Most significant part counter value (on TIM2 and TIM5)"]
    #[inline(always)]
    pub fn cnt_h(&mut self) -> CNT_H_W {
        CNT_H_W { w: self }
    }
    #[doc = "Bits 0:15 - Least significant part of counter value"]
    #[inline(always)]
    pub fn cnt_l(&mut self) -> CNT_L_W {
        CNT_L_W { w: self }
    }
    #[doc = "Bit 31 - Most significant bit of counter value (on TIM2 and TIM5)"]
    #[inline(always)]
    pub fn cnt_bit31(&mut self) -> CNT_BIT31_W {
        CNT_BIT31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](index.html) module"]
pub struct CNT_SPEC;
impl crate::RegisterSpec for CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnt::R](R) reader structure"]
impl crate::Readable for CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnt::W](W) writer structure"]
impl crate::Writable for CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
