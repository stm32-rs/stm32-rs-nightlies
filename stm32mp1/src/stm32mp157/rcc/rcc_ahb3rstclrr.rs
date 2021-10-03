#[doc = "Register `RCC_AHB3RSTCLRR` reader"]
pub struct R(crate::R<RCC_AHB3RSTCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB3RSTCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB3RSTCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB3RSTCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AHB3RSTCLRR` writer"]
pub struct W(crate::W<RCC_AHB3RSTCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB3RSTCLRR_SPEC>;
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
impl From<crate::W<RCC_AHB3RSTCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB3RSTCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCMIRST` reader - DCMIRST"]
pub struct DCMIRST_R(crate::FieldReader<bool, bool>);
impl DCMIRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCMIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCMIRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCMIRST` writer - DCMIRST"]
pub struct DCMIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMIRST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CRYP2RST` reader - CRYP2RST"]
pub struct CRYP2RST_R(crate::FieldReader<bool, bool>);
impl CRYP2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRYP2RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYP2RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYP2RST` writer - CRYP2RST"]
pub struct CRYP2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYP2RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `HASH2RST` reader - HASH2RST"]
pub struct HASH2RST_R(crate::FieldReader<bool, bool>);
impl HASH2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        HASH2RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HASH2RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASH2RST` writer - HASH2RST"]
pub struct HASH2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH2RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RNG2RST` reader - RNG2RST"]
pub struct RNG2RST_R(crate::FieldReader<bool, bool>);
impl RNG2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNG2RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNG2RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNG2RST` writer - RNG2RST"]
pub struct RNG2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG2RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CRC2RST` reader - CRC2RST"]
pub struct CRC2RST_R(crate::FieldReader<bool, bool>);
impl CRC2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC2RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC2RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC2RST` writer - CRC2RST"]
pub struct CRC2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC2RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `HSEMRST` reader - HSEMRST"]
pub struct HSEMRST_R(crate::FieldReader<bool, bool>);
impl HSEMRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSEMRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSEMRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSEMRST` writer - HSEMRST"]
pub struct HSEMRST_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEMRST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `IPCCRST` reader - IPCCRST"]
pub struct IPCCRST_R(crate::FieldReader<bool, bool>);
impl IPCCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPCCRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPCCRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPCCRST` writer - IPCCRST"]
pub struct IPCCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCCRST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DCMIRST"]
    #[inline(always)]
    pub fn dcmirst(&self) -> DCMIRST_R {
        DCMIRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRYP2RST"]
    #[inline(always)]
    pub fn cryp2rst(&self) -> CRYP2RST_R {
        CRYP2RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HASH2RST"]
    #[inline(always)]
    pub fn hash2rst(&self) -> HASH2RST_R {
        HASH2RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RNG2RST"]
    #[inline(always)]
    pub fn rng2rst(&self) -> RNG2RST_R {
        RNG2RST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CRC2RST"]
    #[inline(always)]
    pub fn crc2rst(&self) -> CRC2RST_R {
        CRC2RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HSEMRST"]
    #[inline(always)]
    pub fn hsemrst(&self) -> HSEMRST_R {
        HSEMRST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - IPCCRST"]
    #[inline(always)]
    pub fn ipccrst(&self) -> IPCCRST_R {
        IPCCRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCMIRST"]
    #[inline(always)]
    pub fn dcmirst(&mut self) -> DCMIRST_W {
        DCMIRST_W { w: self }
    }
    #[doc = "Bit 4 - CRYP2RST"]
    #[inline(always)]
    pub fn cryp2rst(&mut self) -> CRYP2RST_W {
        CRYP2RST_W { w: self }
    }
    #[doc = "Bit 5 - HASH2RST"]
    #[inline(always)]
    pub fn hash2rst(&mut self) -> HASH2RST_W {
        HASH2RST_W { w: self }
    }
    #[doc = "Bit 6 - RNG2RST"]
    #[inline(always)]
    pub fn rng2rst(&mut self) -> RNG2RST_W {
        RNG2RST_W { w: self }
    }
    #[doc = "Bit 7 - CRC2RST"]
    #[inline(always)]
    pub fn crc2rst(&mut self) -> CRC2RST_W {
        CRC2RST_W { w: self }
    }
    #[doc = "Bit 11 - HSEMRST"]
    #[inline(always)]
    pub fn hsemrst(&mut self) -> HSEMRST_W {
        HSEMRST_W { w: self }
    }
    #[doc = "Bit 12 - IPCCRST"]
    #[inline(always)]
    pub fn ipccrst(&mut self) -> IPCCRST_W {
        IPCCRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to release the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb3rstclrr](index.html) module"]
pub struct RCC_AHB3RSTCLRR_SPEC;
impl crate::RegisterSpec for RCC_AHB3RSTCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ahb3rstclrr::R](R) reader structure"]
impl crate::Readable for RCC_AHB3RSTCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ahb3rstclrr::W](W) writer structure"]
impl crate::Writable for RCC_AHB3RSTCLRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_AHB3RSTCLRR to value 0"]
impl crate::Resettable for RCC_AHB3RSTCLRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
