#[doc = "Register `SECCR` reader"]
pub struct R(crate::R<SECCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECCR` writer"]
pub struct W(crate::W<SECCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCR_SPEC>;
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
impl From<crate::W<SECCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECPG` reader - SECPG"]
pub struct SECPG_R(crate::FieldReader<bool, bool>);
impl SECPG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECPG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECPG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECPG` writer - SECPG"]
pub struct SECPG_W<'a> {
    w: &'a mut W,
}
impl<'a> SECPG_W<'a> {
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
#[doc = "Field `SECPER` reader - SECPER"]
pub struct SECPER_R(crate::FieldReader<bool, bool>);
impl SECPER_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECPER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECPER` writer - SECPER"]
pub struct SECPER_W<'a> {
    w: &'a mut W,
}
impl<'a> SECPER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SECMER1` reader - SECMER1"]
pub struct SECMER1_R(crate::FieldReader<bool, bool>);
impl SECMER1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECMER1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECMER1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECMER1` writer - SECMER1"]
pub struct SECMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> SECMER1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SECPNB` reader - SECPNB"]
pub struct SECPNB_R(crate::FieldReader<u8, u8>);
impl SECPNB_R {
    pub(crate) fn new(bits: u8) -> Self {
        SECPNB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECPNB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECPNB` writer - SECPNB"]
pub struct SECPNB_W<'a> {
    w: &'a mut W,
}
impl<'a> SECPNB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 3)) | ((value as u32 & 0x7f) << 3);
        self.w
    }
}
#[doc = "Field `SECBKER` reader - SECBKER"]
pub struct SECBKER_R(crate::FieldReader<bool, bool>);
impl SECBKER_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECBKER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECBKER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECBKER` writer - SECBKER"]
pub struct SECBKER_W<'a> {
    w: &'a mut W,
}
impl<'a> SECBKER_W<'a> {
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
#[doc = "Field `SECMER2` reader - SECMER2"]
pub struct SECMER2_R(crate::FieldReader<bool, bool>);
impl SECMER2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECMER2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECMER2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECMER2` writer - SECMER2"]
pub struct SECMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> SECMER2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `SECSTRT` reader - SECSTRT"]
pub struct SECSTRT_R(crate::FieldReader<bool, bool>);
impl SECSTRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECSTRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECSTRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECSTRT` writer - SECSTRT"]
pub struct SECSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> SECSTRT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `SECEOPIE` reader - SECEOPIE"]
pub struct SECEOPIE_R(crate::FieldReader<bool, bool>);
impl SECEOPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECEOPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECEOPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECEOPIE` writer - SECEOPIE"]
pub struct SECEOPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SECEOPIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `SECERRIE` reader - SECERRIE"]
pub struct SECERRIE_R(crate::FieldReader<bool, bool>);
impl SECERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECERRIE` writer - SECERRIE"]
pub struct SECERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SECERRIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `SECRDERRIE` reader - SECRDERRIE"]
pub struct SECRDERRIE_R(crate::FieldReader<bool, bool>);
impl SECRDERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECRDERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECRDERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECRDERRIE` writer - SECRDERRIE"]
pub struct SECRDERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SECRDERRIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `SECINV` reader - SECINV"]
pub struct SECINV_R(crate::FieldReader<bool, bool>);
impl SECINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECINV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECINV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECINV` writer - SECINV"]
pub struct SECINV_W<'a> {
    w: &'a mut W,
}
impl<'a> SECINV_W<'a> {
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
#[doc = "Field `SECLOCK` reader - SECLOCK"]
pub struct SECLOCK_R(crate::FieldReader<bool, bool>);
impl SECLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECLOCK` writer - SECLOCK"]
pub struct SECLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SECLOCK_W<'a> {
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
    #[doc = "Bit 0 - SECPG"]
    #[inline(always)]
    pub fn secpg(&self) -> SECPG_R {
        SECPG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SECPER"]
    #[inline(always)]
    pub fn secper(&self) -> SECPER_R {
        SECPER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SECMER1"]
    #[inline(always)]
    pub fn secmer1(&self) -> SECMER1_R {
        SECMER1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:9 - SECPNB"]
    #[inline(always)]
    pub fn secpnb(&self) -> SECPNB_R {
        SECPNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bit 11 - SECBKER"]
    #[inline(always)]
    pub fn secbker(&self) -> SECBKER_R {
        SECBKER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SECMER2"]
    #[inline(always)]
    pub fn secmer2(&self) -> SECMER2_R {
        SECMER2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SECSTRT"]
    #[inline(always)]
    pub fn secstrt(&self) -> SECSTRT_R {
        SECSTRT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - SECEOPIE"]
    #[inline(always)]
    pub fn seceopie(&self) -> SECEOPIE_R {
        SECEOPIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SECERRIE"]
    #[inline(always)]
    pub fn secerrie(&self) -> SECERRIE_R {
        SECERRIE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SECRDERRIE"]
    #[inline(always)]
    pub fn secrderrie(&self) -> SECRDERRIE_R {
        SECRDERRIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 29 - SECINV"]
    #[inline(always)]
    pub fn secinv(&self) -> SECINV_R {
        SECINV_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SECLOCK"]
    #[inline(always)]
    pub fn seclock(&self) -> SECLOCK_R {
        SECLOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SECPG"]
    #[inline(always)]
    pub fn secpg(&mut self) -> SECPG_W {
        SECPG_W { w: self }
    }
    #[doc = "Bit 1 - SECPER"]
    #[inline(always)]
    pub fn secper(&mut self) -> SECPER_W {
        SECPER_W { w: self }
    }
    #[doc = "Bit 2 - SECMER1"]
    #[inline(always)]
    pub fn secmer1(&mut self) -> SECMER1_W {
        SECMER1_W { w: self }
    }
    #[doc = "Bits 3:9 - SECPNB"]
    #[inline(always)]
    pub fn secpnb(&mut self) -> SECPNB_W {
        SECPNB_W { w: self }
    }
    #[doc = "Bit 11 - SECBKER"]
    #[inline(always)]
    pub fn secbker(&mut self) -> SECBKER_W {
        SECBKER_W { w: self }
    }
    #[doc = "Bit 15 - SECMER2"]
    #[inline(always)]
    pub fn secmer2(&mut self) -> SECMER2_W {
        SECMER2_W { w: self }
    }
    #[doc = "Bit 16 - SECSTRT"]
    #[inline(always)]
    pub fn secstrt(&mut self) -> SECSTRT_W {
        SECSTRT_W { w: self }
    }
    #[doc = "Bit 24 - SECEOPIE"]
    #[inline(always)]
    pub fn seceopie(&mut self) -> SECEOPIE_W {
        SECEOPIE_W { w: self }
    }
    #[doc = "Bit 25 - SECERRIE"]
    #[inline(always)]
    pub fn secerrie(&mut self) -> SECERRIE_W {
        SECERRIE_W { w: self }
    }
    #[doc = "Bit 26 - SECRDERRIE"]
    #[inline(always)]
    pub fn secrderrie(&mut self) -> SECRDERRIE_W {
        SECRDERRIE_W { w: self }
    }
    #[doc = "Bit 29 - SECINV"]
    #[inline(always)]
    pub fn secinv(&mut self) -> SECINV_W {
        SECINV_W { w: self }
    }
    #[doc = "Bit 31 - SECLOCK"]
    #[inline(always)]
    pub fn seclock(&mut self) -> SECLOCK_W {
        SECLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash secure control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccr](index.html) module"]
pub struct SECCR_SPEC;
impl crate::RegisterSpec for SECCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seccr::R](R) reader structure"]
impl crate::Readable for SECCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seccr::W](W) writer structure"]
impl crate::Writable for SECCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECCR to value 0x8000_0000"]
impl crate::Resettable for SECCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
