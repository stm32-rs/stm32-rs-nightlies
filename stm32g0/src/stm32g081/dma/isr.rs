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
#[doc = "Field `GIF0` reader - Channel global interrupt flag"]
pub struct GIF0_R(crate::FieldReader<bool, bool>);
impl GIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        GIF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GIF0_R {
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
#[doc = "Field `HTIF6` reader - Channel 6 half transfer flag"]
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
#[doc = "Field `TEIF7` reader - Channel 7 transfer error flag"]
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
#[doc = "Field `GIF8` reader - Channel global interrupt flag"]
pub struct GIF8_R(crate::FieldReader<bool, bool>);
impl GIF8_R {
    pub(crate) fn new(bits: bool) -> Self {
        GIF8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GIF8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIF9` reader - Channel transfer complete flag"]
pub struct TCIF9_R(crate::FieldReader<bool, bool>);
impl TCIF9_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIF9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIF9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTIF10` reader - Channel half transfer flag"]
pub struct HTIF10_R(crate::FieldReader<bool, bool>);
impl HTIF10_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTIF10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTIF10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIF11` reader - Channel transfer error flag"]
pub struct TEIF11_R(crate::FieldReader<bool, bool>);
impl TEIF11_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GIF12` reader - Channel global interrupt flag"]
pub struct GIF12_R(crate::FieldReader<bool, bool>);
impl GIF12_R {
    pub(crate) fn new(bits: bool) -> Self {
        GIF12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GIF12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIF13` reader - Channel transfer complete flag"]
pub struct TCIF13_R(crate::FieldReader<bool, bool>);
impl TCIF13_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIF13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIF13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTIF14` reader - Channel half transfer flag"]
pub struct HTIF14_R(crate::FieldReader<bool, bool>);
impl HTIF14_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTIF14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTIF14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIF15` reader - Channel transfer error flag"]
pub struct TEIF15_R(crate::FieldReader<bool, bool>);
impl TEIF15_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GIF16` reader - Channel global interrupt flag"]
pub struct GIF16_R(crate::FieldReader<bool, bool>);
impl GIF16_R {
    pub(crate) fn new(bits: bool) -> Self {
        GIF16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GIF16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIF17` reader - Channel transfer complete flag"]
pub struct TCIF17_R(crate::FieldReader<bool, bool>);
impl TCIF17_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIF17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIF17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTIF18` reader - Channel half transfer flag"]
pub struct HTIF18_R(crate::FieldReader<bool, bool>);
impl HTIF18_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTIF18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTIF18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIF19` reader - Channel transfer error flag"]
pub struct TEIF19_R(crate::FieldReader<bool, bool>);
impl TEIF19_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GIF20` reader - Channel global interrupt flag"]
pub struct GIF20_R(crate::FieldReader<bool, bool>);
impl GIF20_R {
    pub(crate) fn new(bits: bool) -> Self {
        GIF20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GIF20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIF21` reader - Channel transfer complete flag"]
pub struct TCIF21_R(crate::FieldReader<bool, bool>);
impl TCIF21_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIF21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIF21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTIF22` reader - Channel half transfer flag"]
pub struct HTIF22_R(crate::FieldReader<bool, bool>);
impl HTIF22_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTIF22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTIF22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIF23` reader - Channel transfer error flag"]
pub struct TEIF23_R(crate::FieldReader<bool, bool>);
impl TEIF23_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GIF24` reader - Channel global interrupt flag"]
pub struct GIF24_R(crate::FieldReader<bool, bool>);
impl GIF24_R {
    pub(crate) fn new(bits: bool) -> Self {
        GIF24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GIF24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIF25` reader - Channel transfer complete flag"]
pub struct TCIF25_R(crate::FieldReader<bool, bool>);
impl TCIF25_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIF25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIF25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTIF26` reader - Channel half transfer flag"]
pub struct HTIF26_R(crate::FieldReader<bool, bool>);
impl HTIF26_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTIF26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTIF26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIF27` reader - Channel transfer error flag"]
pub struct TEIF27_R(crate::FieldReader<bool, bool>);
impl TEIF27_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Channel global interrupt flag"]
    #[inline(always)]
    pub fn gif0(&self) -> GIF0_R {
        GIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 transfer complete flag"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 half transfer flag"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 transfer error flag"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 global interrupt flag"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 transfer complete flag"]
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 half transfer flag"]
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 transfer error flag"]
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel global interrupt flag"]
    #[inline(always)]
    pub fn gif8(&self) -> GIF8_R {
        GIF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel transfer complete flag"]
    #[inline(always)]
    pub fn tcif9(&self) -> TCIF9_R {
        TCIF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel half transfer flag"]
    #[inline(always)]
    pub fn htif10(&self) -> HTIF10_R {
        HTIF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel transfer error flag"]
    #[inline(always)]
    pub fn teif11(&self) -> TEIF11_R {
        TEIF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel global interrupt flag"]
    #[inline(always)]
    pub fn gif12(&self) -> GIF12_R {
        GIF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel transfer complete flag"]
    #[inline(always)]
    pub fn tcif13(&self) -> TCIF13_R {
        TCIF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel half transfer flag"]
    #[inline(always)]
    pub fn htif14(&self) -> HTIF14_R {
        HTIF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel transfer error flag"]
    #[inline(always)]
    pub fn teif15(&self) -> TEIF15_R {
        TEIF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel global interrupt flag"]
    #[inline(always)]
    pub fn gif16(&self) -> GIF16_R {
        GIF16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel transfer complete flag"]
    #[inline(always)]
    pub fn tcif17(&self) -> TCIF17_R {
        TCIF17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel half transfer flag"]
    #[inline(always)]
    pub fn htif18(&self) -> HTIF18_R {
        HTIF18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel transfer error flag"]
    #[inline(always)]
    pub fn teif19(&self) -> TEIF19_R {
        TEIF19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel global interrupt flag"]
    #[inline(always)]
    pub fn gif20(&self) -> GIF20_R {
        GIF20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel transfer complete flag"]
    #[inline(always)]
    pub fn tcif21(&self) -> TCIF21_R {
        TCIF21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel half transfer flag"]
    #[inline(always)]
    pub fn htif22(&self) -> HTIF22_R {
        HTIF22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel transfer error flag"]
    #[inline(always)]
    pub fn teif23(&self) -> TEIF23_R {
        TEIF23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Channel global interrupt flag"]
    #[inline(always)]
    pub fn gif24(&self) -> GIF24_R {
        GIF24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Channel transfer complete flag"]
    #[inline(always)]
    pub fn tcif25(&self) -> TCIF25_R {
        TCIF25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Channel half transfer flag"]
    #[inline(always)]
    pub fn htif26(&self) -> HTIF26_R {
        HTIF26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Channel transfer error flag"]
    #[inline(always)]
    pub fn teif27(&self) -> TEIF27_R {
        TEIF27_R::new(((self.bits >> 27) & 0x01) != 0)
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
