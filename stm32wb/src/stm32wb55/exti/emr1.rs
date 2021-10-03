#[doc = "Register `EMR1` reader"]
pub struct R(crate::R<EMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMR1` writer"]
pub struct W(crate::W<EMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR1_SPEC>;
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
impl From<crate::W<EMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM0_15` reader - CPU(m) Wakeup with event generation Mask on Event input"]
pub struct EM0_15_R(crate::FieldReader<u16, u16>);
impl EM0_15_R {
    pub(crate) fn new(bits: u16) -> Self {
        EM0_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EM0_15_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EM0_15` writer - CPU(m) Wakeup with event generation Mask on Event input"]
pub struct EM0_15_W<'a> {
    w: &'a mut W,
}
impl<'a> EM0_15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `EM17_21` reader - CPU(m) Wakeup with event generation Mask on Event input"]
pub struct EM17_21_R(crate::FieldReader<u8, u8>);
impl EM17_21_R {
    pub(crate) fn new(bits: u8) -> Self {
        EM17_21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EM17_21_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EM17_21` writer - CPU(m) Wakeup with event generation Mask on Event input"]
pub struct EM17_21_W<'a> {
    w: &'a mut W,
}
impl<'a> EM17_21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | ((value as u32 & 0x1f) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em0_15(&self) -> EM0_15_R {
        EM0_15_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 17:21 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em17_21(&self) -> EM17_21_R {
        EM17_21_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em0_15(&mut self) -> EM0_15_W {
        EM0_15_W { w: self }
    }
    #[doc = "Bits 17:21 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em17_21(&mut self) -> EM17_21_W {
        EM17_21_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPUm wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr1](index.html) module"]
pub struct EMR1_SPEC;
impl crate::RegisterSpec for EMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emr1::R](R) reader structure"]
impl crate::Readable for EMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emr1::W](W) writer structure"]
impl crate::Writable for EMR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMR1 to value 0"]
impl crate::Resettable for EMR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
