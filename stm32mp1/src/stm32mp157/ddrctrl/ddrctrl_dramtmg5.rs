#[doc = "Register `DDRCTRL_DRAMTMG5` reader"]
pub struct R(crate::R<DDRCTRL_DRAMTMG5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DRAMTMG5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DRAMTMG5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DRAMTMG5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DRAMTMG5` writer"]
pub struct W(crate::W<DDRCTRL_DRAMTMG5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DRAMTMG5_SPEC>;
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
impl From<crate::W<DDRCTRL_DRAMTMG5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DRAMTMG5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_CKE` reader - T_CKE"]
pub struct T_CKE_R(crate::FieldReader<u8, u8>);
impl T_CKE_R {
    pub(crate) fn new(bits: u8) -> Self {
        T_CKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_CKE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_CKE` writer - T_CKE"]
pub struct T_CKE_W<'a> {
    w: &'a mut W,
}
impl<'a> T_CKE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `T_CKESR` reader - T_CKESR"]
pub struct T_CKESR_R(crate::FieldReader<u8, u8>);
impl T_CKESR_R {
    pub(crate) fn new(bits: u8) -> Self {
        T_CKESR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_CKESR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_CKESR` writer - T_CKESR"]
pub struct T_CKESR_W<'a> {
    w: &'a mut W,
}
impl<'a> T_CKESR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `T_CKSRE` reader - T_CKSRE"]
pub struct T_CKSRE_R(crate::FieldReader<u8, u8>);
impl T_CKSRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        T_CKSRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_CKSRE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_CKSRE` writer - T_CKSRE"]
pub struct T_CKSRE_W<'a> {
    w: &'a mut W,
}
impl<'a> T_CKSRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `T_CKSRX` reader - T_CKSRX"]
pub struct T_CKSRX_R(crate::FieldReader<u8, u8>);
impl T_CKSRX_R {
    pub(crate) fn new(bits: u8) -> Self {
        T_CKSRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_CKSRX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_CKSRX` writer - T_CKSRX"]
pub struct T_CKSRX_W<'a> {
    w: &'a mut W,
}
impl<'a> T_CKSRX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - T_CKE"]
    #[inline(always)]
    pub fn t_cke(&self) -> T_CKE_R {
        T_CKE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - T_CKESR"]
    #[inline(always)]
    pub fn t_ckesr(&self) -> T_CKESR_R {
        T_CKESR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - T_CKSRE"]
    #[inline(always)]
    pub fn t_cksre(&self) -> T_CKSRE_R {
        T_CKSRE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - T_CKSRX"]
    #[inline(always)]
    pub fn t_cksrx(&self) -> T_CKSRX_R {
        T_CKSRX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - T_CKE"]
    #[inline(always)]
    pub fn t_cke(&mut self) -> T_CKE_W {
        T_CKE_W { w: self }
    }
    #[doc = "Bits 8:13 - T_CKESR"]
    #[inline(always)]
    pub fn t_ckesr(&mut self) -> T_CKESR_W {
        T_CKESR_W { w: self }
    }
    #[doc = "Bits 16:19 - T_CKSRE"]
    #[inline(always)]
    pub fn t_cksre(&mut self) -> T_CKSRE_W {
        T_CKSRE_W { w: self }
    }
    #[doc = "Bits 24:27 - T_CKSRX"]
    #[inline(always)]
    pub fn t_cksrx(&mut self) -> T_CKSRX_W {
        T_CKSRX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM timing register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg5](index.html) module"]
pub struct DDRCTRL_DRAMTMG5_SPEC;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dramtmg5::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg5::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG5 to value 0x0505_0403"]
impl crate::Resettable for DDRCTRL_DRAMTMG5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0505_0403
    }
}
