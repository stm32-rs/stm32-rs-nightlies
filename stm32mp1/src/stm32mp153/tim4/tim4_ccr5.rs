#[doc = "Register `TIM4_CCR5` reader"]
pub struct R(crate::R<TIM4_CCR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM4_CCR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM4_CCR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM4_CCR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM4_CCR5` writer"]
pub struct W(crate::W<TIM4_CCR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM4_CCR5_SPEC>;
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
impl From<crate::W<TIM4_CCR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM4_CCR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR5` reader - CCR5"]
pub struct CCR5_R(crate::FieldReader<u16, u16>);
impl CCR5_R {
    pub(crate) fn new(bits: u16) -> Self {
        CCR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCR5_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCR5` writer - CCR5"]
pub struct CCR5_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `GC5C1` reader - GC5C1"]
pub struct GC5C1_R(crate::FieldReader<bool, bool>);
impl GC5C1_R {
    pub(crate) fn new(bits: bool) -> Self {
        GC5C1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GC5C1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GC5C1` writer - GC5C1"]
pub struct GC5C1_W<'a> {
    w: &'a mut W,
}
impl<'a> GC5C1_W<'a> {
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
#[doc = "Field `GC5C2` reader - GC5C2"]
pub struct GC5C2_R(crate::FieldReader<bool, bool>);
impl GC5C2_R {
    pub(crate) fn new(bits: bool) -> Self {
        GC5C2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GC5C2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GC5C2` writer - GC5C2"]
pub struct GC5C2_W<'a> {
    w: &'a mut W,
}
impl<'a> GC5C2_W<'a> {
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
#[doc = "Field `GC5C3` reader - GC5C3"]
pub struct GC5C3_R(crate::FieldReader<bool, bool>);
impl GC5C3_R {
    pub(crate) fn new(bits: bool) -> Self {
        GC5C3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GC5C3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GC5C3` writer - GC5C3"]
pub struct GC5C3_W<'a> {
    w: &'a mut W,
}
impl<'a> GC5C3_W<'a> {
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
    #[doc = "Bits 0:15 - CCR5"]
    #[inline(always)]
    pub fn ccr5(&self) -> CCR5_R {
        CCR5_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 29 - GC5C1"]
    #[inline(always)]
    pub fn gc5c1(&self) -> GC5C1_R {
        GC5C1_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - GC5C2"]
    #[inline(always)]
    pub fn gc5c2(&self) -> GC5C2_R {
        GC5C2_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - GC5C3"]
    #[inline(always)]
    pub fn gc5c3(&self) -> GC5C3_R {
        GC5C3_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCR5"]
    #[inline(always)]
    pub fn ccr5(&mut self) -> CCR5_W {
        CCR5_W { w: self }
    }
    #[doc = "Bit 29 - GC5C1"]
    #[inline(always)]
    pub fn gc5c1(&mut self) -> GC5C1_W {
        GC5C1_W { w: self }
    }
    #[doc = "Bit 30 - GC5C2"]
    #[inline(always)]
    pub fn gc5c2(&mut self) -> GC5C2_W {
        GC5C2_W { w: self }
    }
    #[doc = "Bit 31 - GC5C3"]
    #[inline(always)]
    pub fn gc5c3(&mut self) -> GC5C3_W {
        GC5C3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM4 capture/compare register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_ccr5](index.html) module"]
pub struct TIM4_CCR5_SPEC;
impl crate::RegisterSpec for TIM4_CCR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim4_ccr5::R](R) reader structure"]
impl crate::Readable for TIM4_CCR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim4_ccr5::W](W) writer structure"]
impl crate::Writable for TIM4_CCR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM4_CCR5 to value 0"]
impl crate::Resettable for TIM4_CCR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
