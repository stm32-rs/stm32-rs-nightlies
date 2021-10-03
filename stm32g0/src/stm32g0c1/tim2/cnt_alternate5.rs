#[doc = "Register `CNT_ALTERNATE5` reader"]
pub struct R(crate::R<CNT_ALTERNATE5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT_ALTERNATE5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT_ALTERNATE5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT_ALTERNATE5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNT_ALTERNATE5` writer"]
pub struct W(crate::W<CNT_ALTERNATE5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNT_ALTERNATE5_SPEC>;
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
impl From<crate::W<CNT_ALTERNATE5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNT_ALTERNATE5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - Most significant part counter value (TIM2) nullLeast significant part of counter value"]
pub struct CNT_R(crate::FieldReader<u32, u32>);
impl CNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT` writer - Most significant part counter value (TIM2) nullLeast significant part of counter value"]
pub struct CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | (value as u32 & 0x7fff_ffff);
        self.w
    }
}
#[doc = "Field `UIFCPY` reader - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register"]
pub struct UIFCPY_R(crate::FieldReader<bool, bool>);
impl UIFCPY_R {
    pub(crate) fn new(bits: bool) -> Self {
        UIFCPY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UIFCPY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UIFCPY` writer - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register"]
pub struct UIFCPY_W<'a> {
    w: &'a mut W,
}
impl<'a> UIFCPY_W<'a> {
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
    #[doc = "Bits 0:30 - Most significant part counter value (TIM2) nullLeast significant part of counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register"]
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Most significant part counter value (TIM2) nullLeast significant part of counter value"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
    #[doc = "Bit 31 - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register"]
    #[inline(always)]
    pub fn uifcpy(&mut self) -> UIFCPY_W {
        UIFCPY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt_alternate5](index.html) module"]
pub struct CNT_ALTERNATE5_SPEC;
impl crate::RegisterSpec for CNT_ALTERNATE5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnt_alternate5::R](R) reader structure"]
impl crate::Readable for CNT_ALTERNATE5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnt_alternate5::W](W) writer structure"]
impl crate::Writable for CNT_ALTERNATE5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNT_ALTERNATE5 to value 0"]
impl crate::Resettable for CNT_ALTERNATE5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
