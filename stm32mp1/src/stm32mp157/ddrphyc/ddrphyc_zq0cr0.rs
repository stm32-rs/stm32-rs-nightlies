#[doc = "Register `DDRPHYC_ZQ0CR0` reader"]
pub struct R(crate::R<DDRPHYC_ZQ0CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_ZQ0CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_ZQ0CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_ZQ0CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_ZQ0CR0` writer"]
pub struct W(crate::W<DDRPHYC_ZQ0CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_ZQ0CR0_SPEC>;
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
impl From<crate::W<DDRPHYC_ZQ0CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_ZQ0CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ZDATA` reader - ZDATA"]
pub struct ZDATA_R(crate::FieldReader<u32, u32>);
impl ZDATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        ZDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZDATA` writer - ZDATA"]
pub struct ZDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> ZDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
#[doc = "Field `ZDEN` reader - ZDEN"]
pub struct ZDEN_R(crate::FieldReader<bool, bool>);
impl ZDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZDEN` writer - ZDEN"]
pub struct ZDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ZDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `ZCALBYP` reader - ZCALBYP"]
pub struct ZCALBYP_R(crate::FieldReader<bool, bool>);
impl ZCALBYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZCALBYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZCALBYP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZCALBYP` writer - ZCALBYP"]
pub struct ZCALBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> ZCALBYP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `ZCAL` reader - ZCAL"]
pub struct ZCAL_R(crate::FieldReader<bool, bool>);
impl ZCAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZCAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZCAL` writer - ZCAL"]
pub struct ZCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ZCAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `ZQPD` reader - ZQPD"]
pub struct ZQPD_R(crate::FieldReader<bool, bool>);
impl ZQPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZQPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZQPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZQPD` writer - ZQPD"]
pub struct ZQPD_W<'a> {
    w: &'a mut W,
}
impl<'a> ZQPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - ZDATA"]
    #[inline(always)]
    pub fn zdata(&self) -> ZDATA_R {
        ZDATA_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 28 - ZDEN"]
    #[inline(always)]
    pub fn zden(&self) -> ZDEN_R {
        ZDEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ZCALBYP"]
    #[inline(always)]
    pub fn zcalbyp(&self) -> ZCALBYP_R {
        ZCALBYP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ZCAL"]
    #[inline(always)]
    pub fn zcal(&self) -> ZCAL_R {
        ZCAL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ZQPD"]
    #[inline(always)]
    pub fn zqpd(&self) -> ZQPD_R {
        ZQPD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - ZDATA"]
    #[inline(always)]
    pub fn zdata(&mut self) -> ZDATA_W {
        ZDATA_W { w: self }
    }
    #[doc = "Bit 28 - ZDEN"]
    #[inline(always)]
    pub fn zden(&mut self) -> ZDEN_W {
        ZDEN_W { w: self }
    }
    #[doc = "Bit 29 - ZCALBYP"]
    #[inline(always)]
    pub fn zcalbyp(&mut self) -> ZCALBYP_W {
        ZCALBYP_W { w: self }
    }
    #[doc = "Bit 30 - ZCAL"]
    #[inline(always)]
    pub fn zcal(&mut self) -> ZCAL_W {
        ZCAL_W { w: self }
    }
    #[doc = "Bit 31 - ZQPD"]
    #[inline(always)]
    pub fn zqpd(&mut self) -> ZQPD_W {
        ZQPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC ZQ0C register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_zq0cr0](index.html) module"]
pub struct DDRPHYC_ZQ0CR0_SPEC;
impl crate::RegisterSpec for DDRPHYC_ZQ0CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_zq0cr0::R](R) reader structure"]
impl crate::Readable for DDRPHYC_ZQ0CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_zq0cr0::W](W) writer structure"]
impl crate::Writable for DDRPHYC_ZQ0CR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_ZQ0CR0 to value 0x014a"]
impl crate::Resettable for DDRPHYC_ZQ0CR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x014a
    }
}
