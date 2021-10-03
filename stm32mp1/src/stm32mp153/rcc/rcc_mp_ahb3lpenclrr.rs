#[doc = "Register `RCC_MP_AHB3LPENCLRR` reader"]
pub struct R(crate::R<RCC_MP_AHB3LPENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_AHB3LPENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_AHB3LPENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_AHB3LPENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_AHB3LPENCLRR` writer"]
pub struct W(crate::W<RCC_MP_AHB3LPENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_AHB3LPENCLRR_SPEC>;
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
impl From<crate::W<RCC_MP_AHB3LPENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_AHB3LPENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCMILPEN` reader - DCMILPEN"]
pub struct DCMILPEN_R(crate::FieldReader<bool, bool>);
impl DCMILPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCMILPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCMILPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCMILPEN` writer - DCMILPEN"]
pub struct DCMILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMILPEN_W<'a> {
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
#[doc = "Field `CRYP2LPEN` reader - CRYP2LPEN"]
pub struct CRYP2LPEN_R(crate::FieldReader<bool, bool>);
impl CRYP2LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRYP2LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYP2LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYP2LPEN` writer - CRYP2LPEN"]
pub struct CRYP2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYP2LPEN_W<'a> {
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
#[doc = "Field `HASH2LPEN` reader - HASH2LPEN"]
pub struct HASH2LPEN_R(crate::FieldReader<bool, bool>);
impl HASH2LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HASH2LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HASH2LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASH2LPEN` writer - HASH2LPEN"]
pub struct HASH2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH2LPEN_W<'a> {
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
#[doc = "Field `RNG2LPEN` reader - RNG2LPEN"]
pub struct RNG2LPEN_R(crate::FieldReader<bool, bool>);
impl RNG2LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNG2LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNG2LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNG2LPEN` writer - RNG2LPEN"]
pub struct RNG2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG2LPEN_W<'a> {
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
#[doc = "Field `CRC2LPEN` reader - CRC2LPEN"]
pub struct CRC2LPEN_R(crate::FieldReader<bool, bool>);
impl CRC2LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC2LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC2LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC2LPEN` writer - CRC2LPEN"]
pub struct CRC2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC2LPEN_W<'a> {
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
#[doc = "Field `HSEMLPEN` reader - HSEMLPEN"]
pub struct HSEMLPEN_R(crate::FieldReader<bool, bool>);
impl HSEMLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSEMLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSEMLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSEMLPEN` writer - HSEMLPEN"]
pub struct HSEMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEMLPEN_W<'a> {
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
#[doc = "Field `IPCCLPEN` reader - IPCCLPEN"]
pub struct IPCCLPEN_R(crate::FieldReader<bool, bool>);
impl IPCCLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPCCLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPCCLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPCCLPEN` writer - IPCCLPEN"]
pub struct IPCCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCCLPEN_W<'a> {
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
    #[doc = "Bit 0 - DCMILPEN"]
    #[inline(always)]
    pub fn dcmilpen(&self) -> DCMILPEN_R {
        DCMILPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRYP2LPEN"]
    #[inline(always)]
    pub fn cryp2lpen(&self) -> CRYP2LPEN_R {
        CRYP2LPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HASH2LPEN"]
    #[inline(always)]
    pub fn hash2lpen(&self) -> HASH2LPEN_R {
        HASH2LPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RNG2LPEN"]
    #[inline(always)]
    pub fn rng2lpen(&self) -> RNG2LPEN_R {
        RNG2LPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CRC2LPEN"]
    #[inline(always)]
    pub fn crc2lpen(&self) -> CRC2LPEN_R {
        CRC2LPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HSEMLPEN"]
    #[inline(always)]
    pub fn hsemlpen(&self) -> HSEMLPEN_R {
        HSEMLPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - IPCCLPEN"]
    #[inline(always)]
    pub fn ipcclpen(&self) -> IPCCLPEN_R {
        IPCCLPEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCMILPEN"]
    #[inline(always)]
    pub fn dcmilpen(&mut self) -> DCMILPEN_W {
        DCMILPEN_W { w: self }
    }
    #[doc = "Bit 4 - CRYP2LPEN"]
    #[inline(always)]
    pub fn cryp2lpen(&mut self) -> CRYP2LPEN_W {
        CRYP2LPEN_W { w: self }
    }
    #[doc = "Bit 5 - HASH2LPEN"]
    #[inline(always)]
    pub fn hash2lpen(&mut self) -> HASH2LPEN_W {
        HASH2LPEN_W { w: self }
    }
    #[doc = "Bit 6 - RNG2LPEN"]
    #[inline(always)]
    pub fn rng2lpen(&mut self) -> RNG2LPEN_W {
        RNG2LPEN_W { w: self }
    }
    #[doc = "Bit 7 - CRC2LPEN"]
    #[inline(always)]
    pub fn crc2lpen(&mut self) -> CRC2LPEN_W {
        CRC2LPEN_W { w: self }
    }
    #[doc = "Bit 11 - HSEMLPEN"]
    #[inline(always)]
    pub fn hsemlpen(&mut self) -> HSEMLPEN_W {
        HSEMLPEN_W { w: self }
    }
    #[doc = "Bit 12 - IPCCLPEN"]
    #[inline(always)]
    pub fn ipcclpen(&mut self) -> IPCCLPEN_W {
        IPCCLPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MPU in order to clear the PERxLPEN bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb3lpenclrr](index.html) module"]
pub struct RCC_MP_AHB3LPENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MP_AHB3LPENCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_ahb3lpenclrr::R](R) reader structure"]
impl crate::Readable for RCC_MP_AHB3LPENCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb3lpenclrr::W](W) writer structure"]
impl crate::Writable for RCC_MP_AHB3LPENCLRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MP_AHB3LPENCLRR to value 0x18f1"]
impl crate::Resettable for RCC_MP_AHB3LPENCLRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x18f1
    }
}
