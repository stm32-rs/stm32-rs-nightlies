#[doc = "Register `LPTIM_CFGR2` reader"]
pub struct R(crate::R<LPTIM_CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPTIM_CFGR2` writer"]
pub struct W(crate::W<LPTIM_CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTIM_CFGR2_SPEC>;
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
impl From<crate::W<LPTIM_CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTIM_CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN1SEL` reader - IN1SEL"]
pub struct IN1SEL_R(crate::FieldReader<u8, u8>);
impl IN1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        IN1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN1SEL` writer - IN1SEL"]
pub struct IN1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IN1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `IN2SEL` reader - IN2SEL"]
pub struct IN2SEL_R(crate::FieldReader<u8, u8>);
impl IN2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        IN2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN2SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN2SEL` writer - IN2SEL"]
pub struct IN2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IN2SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - IN1SEL"]
    #[inline(always)]
    pub fn in1sel(&self) -> IN1SEL_R {
        IN1SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - IN2SEL"]
    #[inline(always)]
    pub fn in2sel(&self) -> IN2SEL_R {
        IN2SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - IN1SEL"]
    #[inline(always)]
    pub fn in1sel(&mut self) -> IN1SEL_W {
        IN1SEL_W { w: self }
    }
    #[doc = "Bits 4:5 - IN2SEL"]
    #[inline(always)]
    pub fn in2sel(&mut self) -> IN2SEL_W {
        IN2SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPTIM configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_cfgr2](index.html) module"]
pub struct LPTIM_CFGR2_SPEC;
impl crate::RegisterSpec for LPTIM_CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_cfgr2::R](R) reader structure"]
impl crate::Readable for LPTIM_CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lptim_cfgr2::W](W) writer structure"]
impl crate::Writable for LPTIM_CFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPTIM_CFGR2 to value 0"]
impl crate::Resettable for LPTIM_CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
