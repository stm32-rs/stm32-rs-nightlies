#[doc = "Register `DDRCTRL_INIT0` reader"]
pub struct R(crate::R<DDRCTRL_INIT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_INIT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_INIT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_INIT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_INIT0` writer"]
pub struct W(crate::W<DDRCTRL_INIT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_INIT0_SPEC>;
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
impl From<crate::W<DDRCTRL_INIT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_INIT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRE_CKE_X1024` reader - PRE_CKE_X1024"]
pub struct PRE_CKE_X1024_R(crate::FieldReader<u16, u16>);
impl PRE_CKE_X1024_R {
    pub(crate) fn new(bits: u16) -> Self {
        PRE_CKE_X1024_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRE_CKE_X1024_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRE_CKE_X1024` writer - PRE_CKE_X1024"]
pub struct PRE_CKE_X1024_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_CKE_X1024_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `POST_CKE_X1024` reader - POST_CKE_X1024"]
pub struct POST_CKE_X1024_R(crate::FieldReader<u16, u16>);
impl POST_CKE_X1024_R {
    pub(crate) fn new(bits: u16) -> Self {
        POST_CKE_X1024_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POST_CKE_X1024_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POST_CKE_X1024` writer - POST_CKE_X1024"]
pub struct POST_CKE_X1024_W<'a> {
    w: &'a mut W,
}
impl<'a> POST_CKE_X1024_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `SKIP_DRAM_INIT` reader - SKIP_DRAM_INIT"]
pub struct SKIP_DRAM_INIT_R(crate::FieldReader<u8, u8>);
impl SKIP_DRAM_INIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SKIP_DRAM_INIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SKIP_DRAM_INIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SKIP_DRAM_INIT` writer - SKIP_DRAM_INIT"]
pub struct SKIP_DRAM_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SKIP_DRAM_INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - PRE_CKE_X1024"]
    #[inline(always)]
    pub fn pre_cke_x1024(&self) -> PRE_CKE_X1024_R {
        PRE_CKE_X1024_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:25 - POST_CKE_X1024"]
    #[inline(always)]
    pub fn post_cke_x1024(&self) -> POST_CKE_X1024_R {
        POST_CKE_X1024_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 30:31 - SKIP_DRAM_INIT"]
    #[inline(always)]
    pub fn skip_dram_init(&self) -> SKIP_DRAM_INIT_R {
        SKIP_DRAM_INIT_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - PRE_CKE_X1024"]
    #[inline(always)]
    pub fn pre_cke_x1024(&mut self) -> PRE_CKE_X1024_W {
        PRE_CKE_X1024_W { w: self }
    }
    #[doc = "Bits 16:25 - POST_CKE_X1024"]
    #[inline(always)]
    pub fn post_cke_x1024(&mut self) -> POST_CKE_X1024_W {
        POST_CKE_X1024_W { w: self }
    }
    #[doc = "Bits 30:31 - SKIP_DRAM_INIT"]
    #[inline(always)]
    pub fn skip_dram_init(&mut self) -> SKIP_DRAM_INIT_W {
        SKIP_DRAM_INIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM initialization register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_init0](index.html) module"]
pub struct DDRCTRL_INIT0_SPEC;
impl crate::RegisterSpec for DDRCTRL_INIT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_init0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_INIT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_init0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_INIT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_INIT0 to value 0x0002_004e"]
impl crate::Resettable for DDRCTRL_INIT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_004e
    }
}
