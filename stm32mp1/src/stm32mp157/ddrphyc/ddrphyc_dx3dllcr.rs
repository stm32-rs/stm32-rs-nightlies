#[doc = "Register `DDRPHYC_DX3DLLCR` reader"]
pub struct R(crate::R<DDRPHYC_DX3DLLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DX3DLLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DX3DLLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DX3DLLCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DX3DLLCR` writer"]
pub struct W(crate::W<DDRPHYC_DX3DLLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DX3DLLCR_SPEC>;
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
impl From<crate::W<DDRPHYC_DX3DLLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DX3DLLCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFBDLY` reader - SFBDLY"]
pub struct SFBDLY_R(crate::FieldReader<u8, u8>);
impl SFBDLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        SFBDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFBDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFBDLY` writer - SFBDLY"]
pub struct SFBDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> SFBDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `SFWDLY` reader - SFWDLY"]
pub struct SFWDLY_R(crate::FieldReader<u8, u8>);
impl SFWDLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        SFWDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFWDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFWDLY` writer - SFWDLY"]
pub struct SFWDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> SFWDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `MFBDLY` reader - MFBDLY"]
pub struct MFBDLY_R(crate::FieldReader<u8, u8>);
impl MFBDLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        MFBDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFBDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MFBDLY` writer - MFBDLY"]
pub struct MFBDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> MFBDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Field `MFWDLY` reader - MFWDLY"]
pub struct MFWDLY_R(crate::FieldReader<u8, u8>);
impl MFWDLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        MFWDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFWDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MFWDLY` writer - MFWDLY"]
pub struct MFWDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> MFWDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `SSTART` reader - SSTART"]
pub struct SSTART_R(crate::FieldReader<u8, u8>);
impl SSTART_R {
    pub(crate) fn new(bits: u8) -> Self {
        SSTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSTART_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSTART` writer - SSTART"]
pub struct SSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> SSTART_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `SDPHASE` reader - SDPHASE"]
pub struct SDPHASE_R(crate::FieldReader<u8, u8>);
impl SDPHASE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDPHASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDPHASE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDPHASE` writer - SDPHASE"]
pub struct SDPHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDPHASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 14)) | ((value as u32 & 0x0f) << 14);
        self.w
    }
}
#[doc = "Field `ATESTEN` reader - ATESTEN"]
pub struct ATESTEN_R(crate::FieldReader<bool, bool>);
impl ATESTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATESTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATESTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATESTEN` writer - ATESTEN"]
pub struct ATESTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ATESTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `SDLBMODE` reader - SDLBMODE"]
pub struct SDLBMODE_R(crate::FieldReader<bool, bool>);
impl SDLBMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDLBMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDLBMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDLBMODE` writer - SDLBMODE"]
pub struct SDLBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDLBMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `DLLSRST` reader - DLLSRST"]
pub struct DLLSRST_R(crate::FieldReader<bool, bool>);
impl DLLSRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLLSRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLLSRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLLSRST` writer - DLLSRST"]
pub struct DLLSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLSRST_W<'a> {
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
#[doc = "Field `DLLDIS` reader - DLLDIS"]
pub struct DLLDIS_R(crate::FieldReader<bool, bool>);
impl DLLDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLLDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLLDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLLDIS` writer - DLLDIS"]
pub struct DLLDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLDIS_W<'a> {
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
    #[doc = "Bits 0:2 - SFBDLY"]
    #[inline(always)]
    pub fn sfbdly(&self) -> SFBDLY_R {
        SFBDLY_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - SFWDLY"]
    #[inline(always)]
    pub fn sfwdly(&self) -> SFWDLY_R {
        SFWDLY_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - MFBDLY"]
    #[inline(always)]
    pub fn mfbdly(&self) -> MFBDLY_R {
        MFBDLY_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - MFWDLY"]
    #[inline(always)]
    pub fn mfwdly(&self) -> MFWDLY_R {
        MFWDLY_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:13 - SSTART"]
    #[inline(always)]
    pub fn sstart(&self) -> SSTART_R {
        SSTART_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:17 - SDPHASE"]
    #[inline(always)]
    pub fn sdphase(&self) -> SDPHASE_R {
        SDPHASE_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - ATESTEN"]
    #[inline(always)]
    pub fn atesten(&self) -> ATESTEN_R {
        ATESTEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SDLBMODE"]
    #[inline(always)]
    pub fn sdlbmode(&self) -> SDLBMODE_R {
        SDLBMODE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DLLSRST"]
    #[inline(always)]
    pub fn dllsrst(&self) -> DLLSRST_R {
        DLLSRST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DLLDIS"]
    #[inline(always)]
    pub fn dlldis(&self) -> DLLDIS_R {
        DLLDIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SFBDLY"]
    #[inline(always)]
    pub fn sfbdly(&mut self) -> SFBDLY_W {
        SFBDLY_W { w: self }
    }
    #[doc = "Bits 3:5 - SFWDLY"]
    #[inline(always)]
    pub fn sfwdly(&mut self) -> SFWDLY_W {
        SFWDLY_W { w: self }
    }
    #[doc = "Bits 6:8 - MFBDLY"]
    #[inline(always)]
    pub fn mfbdly(&mut self) -> MFBDLY_W {
        MFBDLY_W { w: self }
    }
    #[doc = "Bits 9:11 - MFWDLY"]
    #[inline(always)]
    pub fn mfwdly(&mut self) -> MFWDLY_W {
        MFWDLY_W { w: self }
    }
    #[doc = "Bits 12:13 - SSTART"]
    #[inline(always)]
    pub fn sstart(&mut self) -> SSTART_W {
        SSTART_W { w: self }
    }
    #[doc = "Bits 14:17 - SDPHASE"]
    #[inline(always)]
    pub fn sdphase(&mut self) -> SDPHASE_W {
        SDPHASE_W { w: self }
    }
    #[doc = "Bit 18 - ATESTEN"]
    #[inline(always)]
    pub fn atesten(&mut self) -> ATESTEN_W {
        ATESTEN_W { w: self }
    }
    #[doc = "Bit 19 - SDLBMODE"]
    #[inline(always)]
    pub fn sdlbmode(&mut self) -> SDLBMODE_W {
        SDLBMODE_W { w: self }
    }
    #[doc = "Bit 30 - DLLSRST"]
    #[inline(always)]
    pub fn dllsrst(&mut self) -> DLLSRST_W {
        DLLSRST_W { w: self }
    }
    #[doc = "Bit 31 - DLLDIS"]
    #[inline(always)]
    pub fn dlldis(&mut self) -> DLLDIS_W {
        DLLDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC byte lane 3 DLLC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx3dllcr](index.html) module"]
pub struct DDRPHYC_DX3DLLCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_DX3DLLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dx3dllcr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DX3DLLCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx3dllcr::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DX3DLLCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_DX3DLLCR to value 0x4000_0000"]
impl crate::Resettable for DDRPHYC_DX3DLLCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
