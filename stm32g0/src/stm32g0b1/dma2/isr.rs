#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GIF1` reader - Channel 1 global interrupt flag"]
pub struct GIF1_R(crate::FieldReader<bool, bool>);
impl GIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        GIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIF1` reader - Channel 1 transfer complete flag"]
pub struct TCIF1_R(crate::FieldReader<bool, bool>);
impl TCIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTIF1` reader - Channel 1 half transfer flag"]
pub struct HTIF1_R(crate::FieldReader<bool, bool>);
impl HTIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIF1` reader - Channel 1 transfer error flag"]
pub struct TEIF1_R(crate::FieldReader<bool, bool>);
impl TEIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GIF2` reader - Channel 2 global interrupt flag"]
pub struct GIF2_R(crate::FieldReader<bool, bool>);
impl GIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        GIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIF2` reader - Channel 2 transfer complete flag"]
pub struct TCIF2_R(crate::FieldReader<bool, bool>);
impl TCIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTIF2` reader - Channel 2 half transfer flag"]
pub struct HTIF2_R(crate::FieldReader<bool, bool>);
impl HTIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIF2` reader - Channel 2 transfer error flag"]
pub struct TEIF2_R(crate::FieldReader<bool, bool>);
impl TEIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GIF3` reader - Channel 3 global interrupt flag"]
pub struct GIF3_R(crate::FieldReader<bool, bool>);
impl GIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        GIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIF3` reader - Channel 3 transfer complete flag"]
pub struct TCIF3_R(crate::FieldReader<bool, bool>);
impl TCIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTIF3` reader - Channel 3 half transfer flag"]
pub struct HTIF3_R(crate::FieldReader<bool, bool>);
impl HTIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIF3` reader - Channel 3 transfer error flag"]
pub struct TEIF3_R(crate::FieldReader<bool, bool>);
impl TEIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GIF4` reader - Channel 4 global interrupt flag"]
pub struct GIF4_R(crate::FieldReader<bool, bool>);
impl GIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        GIF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GIF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIF4` reader - Channel 4 transfer complete flag"]
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
#[doc = "Field `HTIF4` reader - Channel 4 half transfer flag"]
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
#[doc = "Field `TEIF4` reader - Channel 4 transfer error flag"]
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
#[doc = "Field `GIF5` reader - Channel 5 global interrupt flag"]
pub struct GIF5_R(crate::FieldReader<bool, bool>);
impl GIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        GIF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GIF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIF5` reader - Channel 5 transfer complete flag"]
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
#[doc = "Field `HTIF5` reader - Channel 5 half transfer flag"]
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
#[doc = "Field `TEIF5` reader - Channel 5 transfer error flag"]
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
impl R {
    #[doc = "Bit 0 - Channel 1 global interrupt flag"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 transfer complete flag"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 1 half transfer flag"]
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 1 transfer error flag"]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 2 global interrupt flag"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 2 transfer complete flag"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 2 half transfer flag"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 2 transfer error flag"]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 3 global interrupt flag"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 3 transfer complete flag"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 3 half transfer flag"]
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 3 transfer error flag"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel 4 global interrupt flag"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel 4 transfer complete flag"]
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel 4 half transfer flag"]
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel 4 transfer error flag"]
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 5 global interrupt flag"]
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 5 transfer complete flag"]
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 5 half transfer flag"]
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 5 transfer error flag"]
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
#[doc = "low interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
