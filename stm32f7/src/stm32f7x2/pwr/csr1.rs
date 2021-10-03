#[doc = "Register `CSR1` reader"]
pub struct R(crate::R<CSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR1` writer"]
pub struct W(crate::W<CSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR1_SPEC>;
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
impl From<crate::W<CSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUIF` reader - Wakeup internal flag"]
pub struct WUIF_R(crate::FieldReader<bool, bool>);
impl WUIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBF` reader - Standby flag"]
pub struct SBF_R(crate::FieldReader<bool, bool>);
impl SBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVDO` reader - PVD output"]
pub struct PVDO_R(crate::FieldReader<bool, bool>);
impl PVDO_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVDO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVDO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRR` reader - Backup regulator ready"]
pub struct BRR_R(crate::FieldReader<bool, bool>);
impl BRR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRE` reader - Backup regulator enable"]
pub struct BRE_R(crate::FieldReader<bool, bool>);
impl BRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRE` writer - Backup regulator enable"]
pub struct BRE_W<'a> {
    w: &'a mut W,
}
impl<'a> BRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `VOSRDY` reader - Regulator voltage scaling output selection ready bit"]
pub struct VOSRDY_R(crate::FieldReader<bool, bool>);
impl VOSRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        VOSRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VOSRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VOSRDY` writer - Regulator voltage scaling output selection ready bit"]
pub struct VOSRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> VOSRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `ODRDY` reader - Over-drive mode ready"]
pub struct ODRDY_R(crate::FieldReader<bool, bool>);
impl ODRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODRDY` writer - Over-drive mode ready"]
pub struct ODRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> ODRDY_W<'a> {
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
#[doc = "Field `ODSWRDY` reader - Over-drive mode switching ready"]
pub struct ODSWRDY_R(crate::FieldReader<bool, bool>);
impl ODSWRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODSWRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODSWRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODSWRDY` writer - Over-drive mode switching ready"]
pub struct ODSWRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> ODSWRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `UDRDY` reader - Under-drive ready flag"]
pub struct UDRDY_R(crate::FieldReader<u8, u8>);
impl UDRDY_R {
    pub(crate) fn new(bits: u8) -> Self {
        UDRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UDRDY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UDRDY` writer - Under-drive ready flag"]
pub struct UDRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> UDRDY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `EIWUP` reader - Enable internal wakeup"]
pub struct EIWUP_R(crate::FieldReader<bool, bool>);
impl EIWUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EIWUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EIWUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIWUP` writer - Enable internal wakeup"]
pub struct EIWUP_W<'a> {
    w: &'a mut W,
}
impl<'a> EIWUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup internal flag"]
    #[inline(always)]
    pub fn wuif(&self) -> WUIF_R {
        WUIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Standby flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PVD output"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Backup regulator ready"]
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Backup regulator enable"]
    #[inline(always)]
    pub fn bre(&self) -> BRE_R {
        BRE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Regulator voltage scaling output selection ready bit"]
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Over-drive mode ready"]
    #[inline(always)]
    pub fn odrdy(&self) -> ODRDY_R {
        ODRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Over-drive mode switching ready"]
    #[inline(always)]
    pub fn odswrdy(&self) -> ODSWRDY_R {
        ODSWRDY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Under-drive ready flag"]
    #[inline(always)]
    pub fn udrdy(&self) -> UDRDY_R {
        UDRDY_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Enable internal wakeup"]
    #[inline(always)]
    pub fn eiwup(&self) -> EIWUP_R {
        EIWUP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Backup regulator enable"]
    #[inline(always)]
    pub fn bre(&mut self) -> BRE_W {
        BRE_W { w: self }
    }
    #[doc = "Bit 14 - Regulator voltage scaling output selection ready bit"]
    #[inline(always)]
    pub fn vosrdy(&mut self) -> VOSRDY_W {
        VOSRDY_W { w: self }
    }
    #[doc = "Bit 16 - Over-drive mode ready"]
    #[inline(always)]
    pub fn odrdy(&mut self) -> ODRDY_W {
        ODRDY_W { w: self }
    }
    #[doc = "Bit 17 - Over-drive mode switching ready"]
    #[inline(always)]
    pub fn odswrdy(&mut self) -> ODSWRDY_W {
        ODSWRDY_W { w: self }
    }
    #[doc = "Bits 18:19 - Under-drive ready flag"]
    #[inline(always)]
    pub fn udrdy(&mut self) -> UDRDY_W {
        UDRDY_W { w: self }
    }
    #[doc = "Bit 8 - Enable internal wakeup"]
    #[inline(always)]
    pub fn eiwup(&mut self) -> EIWUP_W {
        EIWUP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "power control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr1](index.html) module"]
pub struct CSR1_SPEC;
impl crate::RegisterSpec for CSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr1::R](R) reader structure"]
impl crate::Readable for CSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr1::W](W) writer structure"]
impl crate::Writable for CSR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR1 to value 0"]
impl crate::Resettable for CSR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
