#[doc = "Register `RCC_PLL4CFGR1` reader"]
pub struct R(crate::R<RCC_PLL4CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PLL4CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PLL4CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PLL4CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_PLL4CFGR1` writer"]
pub struct W(crate::W<RCC_PLL4CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PLL4CFGR1_SPEC>;
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
impl From<crate::W<RCC_PLL4CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PLL4CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVN` reader - DIVN"]
pub struct DIVN_R(crate::FieldReader<u16, u16>);
impl DIVN_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIVN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVN` writer - DIVN"]
pub struct DIVN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `DIVM4` reader - DIVM4"]
pub struct DIVM4_R(crate::FieldReader<u8, u8>);
impl DIVM4_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVM4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVM4` writer - DIVM4"]
pub struct DIVM4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVM4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `IFRGE` reader - IFRGE"]
pub struct IFRGE_R(crate::FieldReader<u8, u8>);
impl IFRGE_R {
    pub(crate) fn new(bits: u8) -> Self {
        IFRGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFRGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFRGE` writer - IFRGE"]
pub struct IFRGE_W<'a> {
    w: &'a mut W,
}
impl<'a> IFRGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - DIVN"]
    #[inline(always)]
    pub fn divn(&self) -> DIVN_R {
        DIVN_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:21 - DIVM4"]
    #[inline(always)]
    pub fn divm4(&self) -> DIVM4_R {
        DIVM4_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - IFRGE"]
    #[inline(always)]
    pub fn ifrge(&self) -> IFRGE_R {
        IFRGE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - DIVN"]
    #[inline(always)]
    pub fn divn(&mut self) -> DIVN_W {
        DIVN_W { w: self }
    }
    #[doc = "Bits 16:21 - DIVM4"]
    #[inline(always)]
    pub fn divm4(&mut self) -> DIVM4_W {
        DIVM4_W { w: self }
    }
    #[doc = "Bits 24:25 - IFRGE"]
    #[inline(always)]
    pub fn ifrge(&mut self) -> IFRGE_W {
        IFRGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to configure the PLL4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll4cfgr1](index.html) module"]
pub struct RCC_PLL4CFGR1_SPEC;
impl crate::RegisterSpec for RCC_PLL4CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_pll4cfgr1::R](R) reader structure"]
impl crate::Readable for RCC_PLL4CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_pll4cfgr1::W](W) writer structure"]
impl crate::Writable for RCC_PLL4CFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_PLL4CFGR1 to value 0x0001_0031"]
impl crate::Resettable for RCC_PLL4CFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0031
    }
}
