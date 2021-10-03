#[doc = "Register `DMA_HISR` reader"]
pub struct R(crate::R<DMA_HISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_HISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_HISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_HISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FEIF4` reader - FEIF4"]
pub struct FEIF4_R(crate::FieldReader<bool, bool>);
impl FEIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEIF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEIF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMEIF4` reader - DMEIF4"]
pub struct DMEIF4_R(crate::FieldReader<bool, bool>);
impl DMEIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMEIF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMEIF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIF4` reader - TEIF4"]
pub struct TEIF4_R(crate::FieldReader<bool, bool>);
impl TEIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTIF4` reader - HTIF4"]
pub struct HTIF4_R(crate::FieldReader<bool, bool>);
impl HTIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTIF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTIF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIF4` reader - TCIF4"]
pub struct TCIF4_R(crate::FieldReader<bool, bool>);
impl TCIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIF5` reader - FEIF5"]
pub struct FEIF5_R(crate::FieldReader<bool, bool>);
impl FEIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEIF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEIF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMEIF5` reader - DMEIF5"]
pub struct DMEIF5_R(crate::FieldReader<bool, bool>);
impl DMEIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMEIF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMEIF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIF5` reader - TEIF5"]
pub struct TEIF5_R(crate::FieldReader<bool, bool>);
impl TEIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTIF5` reader - HTIF5"]
pub struct HTIF5_R(crate::FieldReader<bool, bool>);
impl HTIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTIF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTIF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIF5` reader - TCIF5"]
pub struct TCIF5_R(crate::FieldReader<bool, bool>);
impl TCIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIF6` reader - FEIF6"]
pub struct FEIF6_R(crate::FieldReader<bool, bool>);
impl FEIF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEIF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEIF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMEIF6` reader - DMEIF6"]
pub struct DMEIF6_R(crate::FieldReader<bool, bool>);
impl DMEIF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMEIF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMEIF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIF6` reader - TEIF6"]
pub struct TEIF6_R(crate::FieldReader<bool, bool>);
impl TEIF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTIF6` reader - HTIF6"]
pub struct HTIF6_R(crate::FieldReader<bool, bool>);
impl HTIF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTIF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTIF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIF6` reader - TCIF6"]
pub struct TCIF6_R(crate::FieldReader<bool, bool>);
impl TCIF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIF7` reader - FEIF7"]
pub struct FEIF7_R(crate::FieldReader<bool, bool>);
impl FEIF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEIF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEIF7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMEIF7` reader - DMEIF7"]
pub struct DMEIF7_R(crate::FieldReader<bool, bool>);
impl DMEIF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMEIF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMEIF7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIF7` reader - TEIF7"]
pub struct TEIF7_R(crate::FieldReader<bool, bool>);
impl TEIF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTIF7` reader - HTIF7"]
pub struct HTIF7_R(crate::FieldReader<bool, bool>);
impl HTIF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTIF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTIF7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIF7` reader - TCIF7"]
pub struct TCIF7_R(crate::FieldReader<bool, bool>);
impl TCIF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIF7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - FEIF4"]
    #[inline(always)]
    pub fn feif4(&self) -> FEIF4_R {
        FEIF4_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMEIF4"]
    #[inline(always)]
    pub fn dmeif4(&self) -> DMEIF4_R {
        DMEIF4_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TEIF4"]
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HTIF4"]
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TCIF4"]
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FEIF5"]
    #[inline(always)]
    pub fn feif5(&self) -> FEIF5_R {
        FEIF5_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DMEIF5"]
    #[inline(always)]
    pub fn dmeif5(&self) -> DMEIF5_R {
        DMEIF5_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TEIF5"]
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HTIF5"]
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TCIF5"]
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FEIF6"]
    #[inline(always)]
    pub fn feif6(&self) -> FEIF6_R {
        FEIF6_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DMEIF6"]
    #[inline(always)]
    pub fn dmeif6(&self) -> DMEIF6_R {
        DMEIF6_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TEIF6"]
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - HTIF6"]
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TCIF6"]
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - FEIF7"]
    #[inline(always)]
    pub fn feif7(&self) -> FEIF7_R {
        FEIF7_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DMEIF7"]
    #[inline(always)]
    pub fn dmeif7(&self) -> DMEIF7_R {
        DMEIF7_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TEIF7"]
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - HTIF7"]
    #[inline(always)]
    pub fn htif7(&self) -> HTIF7_R {
        HTIF7_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TCIF7"]
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
#[doc = "DMA high interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_hisr](index.html) module"]
pub struct DMA_HISR_SPEC;
impl crate::RegisterSpec for DMA_HISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_hisr::R](R) reader structure"]
impl crate::Readable for DMA_HISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_HISR to value 0"]
impl crate::Resettable for DMA_HISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
