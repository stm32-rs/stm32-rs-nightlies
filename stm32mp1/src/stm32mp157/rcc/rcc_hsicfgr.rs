#[doc = "Register `RCC_HSICFGR` reader"]
pub struct R(crate::R<RCC_HSICFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_HSICFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_HSICFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_HSICFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_HSICFGR` writer"]
pub struct W(crate::W<RCC_HSICFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_HSICFGR_SPEC>;
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
impl From<crate::W<RCC_HSICFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_HSICFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSIDIV` reader - HSIDIV"]
pub struct HSIDIV_R(crate::FieldReader<u8, u8>);
impl HSIDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSIDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSIDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSIDIV` writer - HSIDIV"]
pub struct HSIDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `HSITRIM` reader - HSITRIM"]
pub struct HSITRIM_R(crate::FieldReader<u8, u8>);
impl HSITRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSITRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSITRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSITRIM` writer - HSITRIM"]
pub struct HSITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `HSICAL` reader - HSICAL"]
pub struct HSICAL_R(crate::FieldReader<u16, u16>);
impl HSICAL_R {
    pub(crate) fn new(bits: u16) -> Self {
        HSICAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSICAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - HSIDIV"]
    #[inline(always)]
    pub fn hsidiv(&self) -> HSIDIV_R {
        HSIDIV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:14 - HSITRIM"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:27 - HSICAL"]
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - HSIDIV"]
    #[inline(always)]
    pub fn hsidiv(&mut self) -> HSIDIV_W {
        HSIDIV_W { w: self }
    }
    #[doc = "Bits 8:14 - HSITRIM"]
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HSITRIM_W {
        HSITRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to configure the HSI. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_hsicfgr](index.html) module"]
pub struct RCC_HSICFGR_SPEC;
impl crate::RegisterSpec for RCC_HSICFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_hsicfgr::R](R) reader structure"]
impl crate::Readable for RCC_HSICFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_hsicfgr::W](W) writer structure"]
impl crate::Writable for RCC_HSICFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_HSICFGR to value 0"]
impl crate::Resettable for RCC_HSICFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
