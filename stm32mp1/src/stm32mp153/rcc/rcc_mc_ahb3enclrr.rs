#[doc = "Register `RCC_MC_AHB3ENCLRR` reader"]
pub struct R(crate::R<RCC_MC_AHB3ENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_AHB3ENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_AHB3ENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_AHB3ENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MC_AHB3ENCLRR` writer"]
pub struct W(crate::W<RCC_MC_AHB3ENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_AHB3ENCLRR_SPEC>;
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
impl From<crate::W<RCC_MC_AHB3ENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_AHB3ENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCMIEN` reader - DCMIEN"]
pub struct DCMIEN_R(crate::FieldReader<bool, bool>);
impl DCMIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCMIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCMIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCMIEN` writer - DCMIEN"]
pub struct DCMIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMIEN_W<'a> {
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
#[doc = "Field `CRYP2EN` reader - CRYP2EN"]
pub struct CRYP2EN_R(crate::FieldReader<bool, bool>);
impl CRYP2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRYP2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYP2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYP2EN` writer - CRYP2EN"]
pub struct CRYP2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYP2EN_W<'a> {
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
#[doc = "Field `HASH2EN` reader - HASH2EN"]
pub struct HASH2EN_R(crate::FieldReader<bool, bool>);
impl HASH2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HASH2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HASH2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASH2EN` writer - HASH2EN"]
pub struct HASH2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH2EN_W<'a> {
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
#[doc = "Field `RNG2EN` reader - RNG2EN"]
pub struct RNG2EN_R(crate::FieldReader<bool, bool>);
impl RNG2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNG2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNG2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNG2EN` writer - RNG2EN"]
pub struct RNG2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG2EN_W<'a> {
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
#[doc = "Field `CRC2EN` reader - CRC2EN"]
pub struct CRC2EN_R(crate::FieldReader<bool, bool>);
impl CRC2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC2EN` writer - CRC2EN"]
pub struct CRC2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC2EN_W<'a> {
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
#[doc = "Field `HSEMEN` reader - HSEMEN"]
pub struct HSEMEN_R(crate::FieldReader<bool, bool>);
impl HSEMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSEMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSEMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSEMEN` writer - HSEMEN"]
pub struct HSEMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEMEN_W<'a> {
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
#[doc = "Field `IPCCEN` reader - IPCCEN"]
pub struct IPCCEN_R(crate::FieldReader<bool, bool>);
impl IPCCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPCCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPCCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPCCEN` writer - IPCCEN"]
pub struct IPCCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCCEN_W<'a> {
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
    #[doc = "Bit 0 - DCMIEN"]
    #[inline(always)]
    pub fn dcmien(&self) -> DCMIEN_R {
        DCMIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRYP2EN"]
    #[inline(always)]
    pub fn cryp2en(&self) -> CRYP2EN_R {
        CRYP2EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HASH2EN"]
    #[inline(always)]
    pub fn hash2en(&self) -> HASH2EN_R {
        HASH2EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RNG2EN"]
    #[inline(always)]
    pub fn rng2en(&self) -> RNG2EN_R {
        RNG2EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CRC2EN"]
    #[inline(always)]
    pub fn crc2en(&self) -> CRC2EN_R {
        CRC2EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HSEMEN"]
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - IPCCEN"]
    #[inline(always)]
    pub fn ipccen(&self) -> IPCCEN_R {
        IPCCEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCMIEN"]
    #[inline(always)]
    pub fn dcmien(&mut self) -> DCMIEN_W {
        DCMIEN_W { w: self }
    }
    #[doc = "Bit 4 - CRYP2EN"]
    #[inline(always)]
    pub fn cryp2en(&mut self) -> CRYP2EN_W {
        CRYP2EN_W { w: self }
    }
    #[doc = "Bit 5 - HASH2EN"]
    #[inline(always)]
    pub fn hash2en(&mut self) -> HASH2EN_W {
        HASH2EN_W { w: self }
    }
    #[doc = "Bit 6 - RNG2EN"]
    #[inline(always)]
    pub fn rng2en(&mut self) -> RNG2EN_W {
        RNG2EN_W { w: self }
    }
    #[doc = "Bit 7 - CRC2EN"]
    #[inline(always)]
    pub fn crc2en(&mut self) -> CRC2EN_W {
        CRC2EN_W { w: self }
    }
    #[doc = "Bit 11 - HSEMEN"]
    #[inline(always)]
    pub fn hsemen(&mut self) -> HSEMEN_W {
        HSEMEN_W { w: self }
    }
    #[doc = "Bit 12 - IPCCEN"]
    #[inline(always)]
    pub fn ipccen(&mut self) -> IPCCEN_W {
        IPCCEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to clear the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb3enclrr](index.html) module"]
pub struct RCC_MC_AHB3ENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MC_AHB3ENCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mc_ahb3enclrr::R](R) reader structure"]
impl crate::Readable for RCC_MC_AHB3ENCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb3enclrr::W](W) writer structure"]
impl crate::Writable for RCC_MC_AHB3ENCLRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MC_AHB3ENCLRR to value 0"]
impl crate::Resettable for RCC_MC_AHB3ENCLRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
