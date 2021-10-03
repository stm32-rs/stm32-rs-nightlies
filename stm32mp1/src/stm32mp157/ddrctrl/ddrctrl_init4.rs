#[doc = "Register `DDRCTRL_INIT4` reader"]
pub struct R(crate::R<DDRCTRL_INIT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_INIT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_INIT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_INIT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_INIT4` writer"]
pub struct W(crate::W<DDRCTRL_INIT4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_INIT4_SPEC>;
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
impl From<crate::W<DDRCTRL_INIT4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_INIT4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMR3` reader - EMR3"]
pub struct EMR3_R(crate::FieldReader<u16, u16>);
impl EMR3_R {
    pub(crate) fn new(bits: u16) -> Self {
        EMR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMR3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMR3` writer - EMR3"]
pub struct EMR3_W<'a> {
    w: &'a mut W,
}
impl<'a> EMR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `EMR2` reader - EMR2"]
pub struct EMR2_R(crate::FieldReader<u16, u16>);
impl EMR2_R {
    pub(crate) fn new(bits: u16) -> Self {
        EMR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMR2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMR2` writer - EMR2"]
pub struct EMR2_W<'a> {
    w: &'a mut W,
}
impl<'a> EMR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - EMR3"]
    #[inline(always)]
    pub fn emr3(&self) -> EMR3_R {
        EMR3_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - EMR2"]
    #[inline(always)]
    pub fn emr2(&self) -> EMR2_R {
        EMR2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - EMR3"]
    #[inline(always)]
    pub fn emr3(&mut self) -> EMR3_W {
        EMR3_W { w: self }
    }
    #[doc = "Bits 16:31 - EMR2"]
    #[inline(always)]
    pub fn emr2(&mut self) -> EMR2_W {
        EMR2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM initialization register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_init4](index.html) module"]
pub struct DDRCTRL_INIT4_SPEC;
impl crate::RegisterSpec for DDRCTRL_INIT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_init4::R](R) reader structure"]
impl crate::Readable for DDRCTRL_INIT4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_init4::W](W) writer structure"]
impl crate::Writable for DDRCTRL_INIT4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_INIT4 to value 0"]
impl crate::Resettable for DDRCTRL_INIT4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
