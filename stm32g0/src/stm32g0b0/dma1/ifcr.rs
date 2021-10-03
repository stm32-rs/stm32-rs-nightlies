#[doc = "Register `IFCR` reader"]
pub struct R(crate::R<IFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CGIF1` reader - Clear channel 1 global interrupt flag"]
pub struct CGIF1_R(crate::FieldReader<bool, bool>);
impl CGIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CGIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CGIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTCIF1` reader - Clear channel 1 transfer complete flag"]
pub struct CTCIF1_R(crate::FieldReader<bool, bool>);
impl CTCIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTCIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTCIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHTIF1` reader - Clear channel 1 half transfer flag"]
pub struct CHTIF1_R(crate::FieldReader<bool, bool>);
impl CHTIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHTIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHTIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTEIF1` reader - Clear channel 1 transfer error flag"]
pub struct CTEIF1_R(crate::FieldReader<bool, bool>);
impl CTEIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTEIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTEIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CGIF2` reader - Clear channel 2 global interrupt flag"]
pub struct CGIF2_R(crate::FieldReader<bool, bool>);
impl CGIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CGIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CGIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTCIF2` reader - Clear channel 2 transfer complete flag"]
pub struct CTCIF2_R(crate::FieldReader<bool, bool>);
impl CTCIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTCIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTCIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHTIF2` reader - Clear channel 2 half transfer flag"]
pub struct CHTIF2_R(crate::FieldReader<bool, bool>);
impl CHTIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHTIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHTIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTEIF2` reader - Clear channel 2 transfer error flag"]
pub struct CTEIF2_R(crate::FieldReader<bool, bool>);
impl CTEIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTEIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTEIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CGIF3` reader - Clear channel 3 global interrupt flag"]
pub struct CGIF3_R(crate::FieldReader<bool, bool>);
impl CGIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CGIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CGIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTCIF3` reader - Clear channel 3 transfer complete flag"]
pub struct CTCIF3_R(crate::FieldReader<bool, bool>);
impl CTCIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTCIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTCIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHTIF3` reader - Clear channel 3 half transfer flag"]
pub struct CHTIF3_R(crate::FieldReader<bool, bool>);
impl CHTIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHTIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHTIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTEIF3` reader - Clear channel 3 transfer error flag"]
pub struct CTEIF3_R(crate::FieldReader<bool, bool>);
impl CTEIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTEIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTEIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CGIF4` reader - Clear channel 4 global interrupt flag"]
pub struct CGIF4_R(crate::FieldReader<bool, bool>);
impl CGIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CGIF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CGIF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTCIF4` reader - Clear channel 4 transfer complete flag"]
pub struct CTCIF4_R(crate::FieldReader<bool, bool>);
impl CTCIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTCIF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTCIF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHTIF4` reader - Clear channel 4 half transfer flag"]
pub struct CHTIF4_R(crate::FieldReader<bool, bool>);
impl CHTIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHTIF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHTIF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTEIF15` reader - Clear channel 4 transfer error flag"]
pub struct CTEIF15_R(crate::FieldReader<bool, bool>);
impl CTEIF15_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTEIF15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTEIF15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CGIF5` reader - Clear channel 5 global interrupt flag"]
pub struct CGIF5_R(crate::FieldReader<bool, bool>);
impl CGIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CGIF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CGIF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTCIF5` reader - Clear channel 5 transfer complete flag"]
pub struct CTCIF5_R(crate::FieldReader<bool, bool>);
impl CTCIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTCIF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTCIF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHTIF5` reader - Clear channel 5 half transfer flag"]
pub struct CHTIF5_R(crate::FieldReader<bool, bool>);
impl CHTIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHTIF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHTIF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTEIF5` reader - Clear channel 5 transfer error flag"]
pub struct CTEIF5_R(crate::FieldReader<bool, bool>);
impl CTEIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTEIF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTEIF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CGIF6` reader - Clear channel 6 global interrupt flag"]
pub struct CGIF6_R(crate::FieldReader<bool, bool>);
impl CGIF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CGIF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CGIF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTCIF6` reader - Clear channel 6 transfer complete flag"]
pub struct CTCIF6_R(crate::FieldReader<bool, bool>);
impl CTCIF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTCIF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTCIF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHTIF6` reader - Clear channel 6 half transfer flag"]
pub struct CHTIF6_R(crate::FieldReader<bool, bool>);
impl CHTIF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHTIF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHTIF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTEIF6` reader - Clear channel 6 transfer error flag"]
pub struct CTEIF6_R(crate::FieldReader<bool, bool>);
impl CTEIF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTEIF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTEIF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CGIF7` reader - Clear channel 7 global interrupt flag"]
pub struct CGIF7_R(crate::FieldReader<bool, bool>);
impl CGIF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CGIF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CGIF7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTCIF7` reader - Clear channel 7 transfer complete flag"]
pub struct CTCIF7_R(crate::FieldReader<bool, bool>);
impl CTCIF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTCIF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTCIF7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHTIF7` reader - Clear channel 7 half transfer flag"]
pub struct CHTIF7_R(crate::FieldReader<bool, bool>);
impl CHTIF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHTIF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHTIF7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTEIF7` reader - Clear channel 7 transfer error flag"]
pub struct CTEIF7_R(crate::FieldReader<bool, bool>);
impl CTEIF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTEIF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTEIF7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Clear channel 1 global interrupt flag"]
    #[inline(always)]
    pub fn cgif1(&self) -> CGIF1_R {
        CGIF1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clear channel 1 transfer complete flag"]
    #[inline(always)]
    pub fn ctcif1(&self) -> CTCIF1_R {
        CTCIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clear channel 1 half transfer flag"]
    #[inline(always)]
    pub fn chtif1(&self) -> CHTIF1_R {
        CHTIF1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear channel 1 transfer error flag"]
    #[inline(always)]
    pub fn cteif1(&self) -> CTEIF1_R {
        CTEIF1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clear channel 2 global interrupt flag"]
    #[inline(always)]
    pub fn cgif2(&self) -> CGIF2_R {
        CGIF2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clear channel 2 transfer complete flag"]
    #[inline(always)]
    pub fn ctcif2(&self) -> CTCIF2_R {
        CTCIF2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Clear channel 2 half transfer flag"]
    #[inline(always)]
    pub fn chtif2(&self) -> CHTIF2_R {
        CHTIF2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Clear channel 2 transfer error flag"]
    #[inline(always)]
    pub fn cteif2(&self) -> CTEIF2_R {
        CTEIF2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Clear channel 3 global interrupt flag"]
    #[inline(always)]
    pub fn cgif3(&self) -> CGIF3_R {
        CGIF3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clear channel 3 transfer complete flag"]
    #[inline(always)]
    pub fn ctcif3(&self) -> CTCIF3_R {
        CTCIF3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Clear channel 3 half transfer flag"]
    #[inline(always)]
    pub fn chtif3(&self) -> CHTIF3_R {
        CHTIF3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Clear channel 3 transfer error flag"]
    #[inline(always)]
    pub fn cteif3(&self) -> CTEIF3_R {
        CTEIF3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Clear channel 4 global interrupt flag"]
    #[inline(always)]
    pub fn cgif4(&self) -> CGIF4_R {
        CGIF4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Clear channel 4 transfer complete flag"]
    #[inline(always)]
    pub fn ctcif4(&self) -> CTCIF4_R {
        CTCIF4_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Clear channel 4 half transfer flag"]
    #[inline(always)]
    pub fn chtif4(&self) -> CHTIF4_R {
        CHTIF4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Clear channel 4 transfer error flag"]
    #[inline(always)]
    pub fn cteif15(&self) -> CTEIF15_R {
        CTEIF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Clear channel 5 global interrupt flag"]
    #[inline(always)]
    pub fn cgif5(&self) -> CGIF5_R {
        CGIF5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Clear channel 5 transfer complete flag"]
    #[inline(always)]
    pub fn ctcif5(&self) -> CTCIF5_R {
        CTCIF5_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Clear channel 5 half transfer flag"]
    #[inline(always)]
    pub fn chtif5(&self) -> CHTIF5_R {
        CHTIF5_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Clear channel 5 transfer error flag"]
    #[inline(always)]
    pub fn cteif5(&self) -> CTEIF5_R {
        CTEIF5_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Clear channel 6 global interrupt flag"]
    #[inline(always)]
    pub fn cgif6(&self) -> CGIF6_R {
        CGIF6_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Clear channel 6 transfer complete flag"]
    #[inline(always)]
    pub fn ctcif6(&self) -> CTCIF6_R {
        CTCIF6_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Clear channel 6 half transfer flag"]
    #[inline(always)]
    pub fn chtif6(&self) -> CHTIF6_R {
        CHTIF6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Clear channel 6 transfer error flag"]
    #[inline(always)]
    pub fn cteif6(&self) -> CTEIF6_R {
        CTEIF6_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Clear channel 7 global interrupt flag"]
    #[inline(always)]
    pub fn cgif7(&self) -> CGIF7_R {
        CGIF7_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Clear channel 7 transfer complete flag"]
    #[inline(always)]
    pub fn ctcif7(&self) -> CTCIF7_R {
        CTCIF7_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Clear channel 7 half transfer flag"]
    #[inline(always)]
    pub fn chtif7(&self) -> CHTIF7_R {
        CHTIF7_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Clear channel 7 transfer error flag"]
    #[inline(always)]
    pub fn cteif7(&self) -> CTEIF7_R {
        CTEIF7_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
#[doc = "high interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](index.html) module"]
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifcr::R](R) reader structure"]
impl crate::Readable for IFCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
