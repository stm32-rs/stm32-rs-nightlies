#[doc = "Register `CIFR` reader"]
pub struct R(crate::R<CIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LSIRDYF` reader - LSI ready Interrupt Flag"]
pub struct LSIRDYF_R(crate::FieldReader<bool, bool>);
impl LSIRDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSIRDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSIRDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSERDYF` reader - LSE ready Interrupt Flag"]
pub struct LSERDYF_R(crate::FieldReader<bool, bool>);
impl LSERDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSERDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSERDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSIRDYF` reader - HSI ready Interrupt Flag"]
pub struct HSIRDYF_R(crate::FieldReader<bool, bool>);
impl HSIRDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIRDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSIRDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSERDYF` reader - HSE ready Interrupt Flag"]
pub struct HSERDYF_R(crate::FieldReader<bool, bool>);
impl HSERDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSERDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSERDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSIRDY` reader - CSI ready Interrupt Flag"]
pub struct CSIRDY_R(crate::FieldReader<bool, bool>);
impl CSIRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSIRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSIRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI48RDYF` reader - RC48 ready Interrupt Flag"]
pub struct HSI48RDYF_R(crate::FieldReader<bool, bool>);
impl HSI48RDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSI48RDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI48RDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL1RDYF` reader - PLL1 ready Interrupt Flag"]
pub struct PLL1RDYF_R(crate::FieldReader<bool, bool>);
impl PLL1RDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL1RDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL1RDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL2RDYF` reader - PLL2 ready Interrupt Flag"]
pub struct PLL2RDYF_R(crate::FieldReader<bool, bool>);
impl PLL2RDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL2RDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL2RDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL3RDYF` reader - PLL3 ready Interrupt Flag"]
pub struct PLL3RDYF_R(crate::FieldReader<bool, bool>);
impl PLL3RDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL3RDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL3RDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSECSSF` reader - LSE clock security system Interrupt Flag"]
pub struct LSECSSF_R(crate::FieldReader<bool, bool>);
impl LSECSSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSECSSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSECSSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSECSSF` reader - HSE clock security system Interrupt Flag"]
pub struct HSECSSF_R(crate::FieldReader<bool, bool>);
impl HSECSSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSECSSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSECSSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - LSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSE ready Interrupt Flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSE ready Interrupt Flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn csirdy(&self) -> CSIRDY_R {
        CSIRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RC48 ready Interrupt Flag"]
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL1 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll1rdyf(&self) -> PLL1RDYF_R {
        PLL1RDYF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll2rdyf(&self) -> PLL2RDYF_R {
        PLL2RDYF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PLL3 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll3rdyf(&self) -> PLL3RDYF_R {
        PLL3RDYF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system Interrupt Flag"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HSE clock security system Interrupt Flag"]
    #[inline(always)]
    pub fn hsecssf(&self) -> HSECSSF_R {
        HSECSSF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
#[doc = "RCC Clock Source Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cifr](index.html) module"]
pub struct CIFR_SPEC;
impl crate::RegisterSpec for CIFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cifr::R](R) reader structure"]
impl crate::Readable for CIFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIFR to value 0"]
impl crate::Resettable for CIFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
