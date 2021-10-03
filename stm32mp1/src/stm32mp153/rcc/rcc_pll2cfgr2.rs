#[doc = "Register `RCC_PLL2CFGR2` reader"]
pub struct R(crate::R<RCC_PLL2CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PLL2CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PLL2CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PLL2CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_PLL2CFGR2` writer"]
pub struct W(crate::W<RCC_PLL2CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PLL2CFGR2_SPEC>;
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
impl From<crate::W<RCC_PLL2CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PLL2CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVP` reader - DIVP"]
pub struct DIVP_R(crate::FieldReader<u8, u8>);
impl DIVP_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVP` writer - DIVP"]
pub struct DIVP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `DIVQ` reader - DIVQ"]
pub struct DIVQ_R(crate::FieldReader<u8, u8>);
impl DIVQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVQ` writer - DIVQ"]
pub struct DIVQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `DIVR` reader - DIVR"]
pub struct DIVR_R(crate::FieldReader<u8, u8>);
impl DIVR_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVR` writer - DIVR"]
pub struct DIVR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - DIVP"]
    #[inline(always)]
    pub fn divp(&self) -> DIVP_R {
        DIVP_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - DIVQ"]
    #[inline(always)]
    pub fn divq(&self) -> DIVQ_R {
        DIVQ_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - DIVR"]
    #[inline(always)]
    pub fn divr(&self) -> DIVR_R {
        DIVR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DIVP"]
    #[inline(always)]
    pub fn divp(&mut self) -> DIVP_W {
        DIVP_W { w: self }
    }
    #[doc = "Bits 8:14 - DIVQ"]
    #[inline(always)]
    pub fn divq(&mut self) -> DIVQ_W {
        DIVQ_W { w: self }
    }
    #[doc = "Bits 16:22 - DIVR"]
    #[inline(always)]
    pub fn divr(&mut self) -> DIVR_W {
        DIVR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll2cfgr2](index.html) module"]
pub struct RCC_PLL2CFGR2_SPEC;
impl crate::RegisterSpec for RCC_PLL2CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_pll2cfgr2::R](R) reader structure"]
impl crate::Readable for RCC_PLL2CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_pll2cfgr2::W](W) writer structure"]
impl crate::Writable for RCC_PLL2CFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_PLL2CFGR2 to value 0x0001_0101"]
impl crate::Resettable for RCC_PLL2CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0101
    }
}
