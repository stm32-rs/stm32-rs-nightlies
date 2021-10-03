#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPWRRSTF` reader - Low-power reset flag"]
pub struct LPWRRSTF_R(crate::FieldReader<bool, bool>);
impl LPWRRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPWRRSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPWRRSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub struct WWDGRSTF_R(crate::FieldReader<bool, bool>);
impl WWDGRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WWDGRSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WWDGRSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWDGRSTF` reader - Independent window watchdog reset flag"]
pub struct IWDGRSTF_R(crate::FieldReader<bool, bool>);
impl IWDGRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDGRSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWDGRSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub struct SFTRSTF_R(crate::FieldReader<bool, bool>);
impl SFTRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SFTRSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFTRSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BORRSTF` reader - BOR flag"]
pub struct BORRSTF_R(crate::FieldReader<bool, bool>);
impl BORRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BORRSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BORRSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINRSTF` reader - Pin reset flag"]
pub struct PINRSTF_R(crate::FieldReader<bool, bool>);
impl PINRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINRSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PINRSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OBLRSTF` reader - Option byte loader reset flag"]
pub struct OBLRSTF_R(crate::FieldReader<bool, bool>);
impl OBLRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OBLRSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OBLRSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub struct RMVF_R(crate::FieldReader<bool, bool>);
impl RMVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RMVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub struct RMVF_W<'a> {
    w: &'a mut W,
}
impl<'a> RMVF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `RFWKPSEL` reader - RF system wakeup clock source selection"]
pub struct RFWKPSEL_R(crate::FieldReader<u8, u8>);
impl RFWKPSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RFWKPSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFWKPSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFWKPSEL` writer - RF system wakeup clock source selection"]
pub struct RFWKPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RFWKPSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `LSI2BW` reader - LSI2 oscillator bias configuration"]
pub struct LSI2BW_R(crate::FieldReader<u8, u8>);
impl LSI2BW_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSI2BW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSI2BW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSI2BW` writer - LSI2 oscillator bias configuration"]
pub struct LSI2BW_W<'a> {
    w: &'a mut W,
}
impl<'a> LSI2BW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `LSI2TRIMOK` reader - LSI2 oscillator trim OK"]
pub struct LSI2TRIMOK_R(crate::FieldReader<bool, bool>);
impl LSI2TRIMOK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSI2TRIMOK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSI2TRIMOK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSI2TRIMEN` reader - LSI2 oscillator trimming enable"]
pub struct LSI2TRIMEN_R(crate::FieldReader<bool, bool>);
impl LSI2TRIMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSI2TRIMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSI2TRIMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSI2TRIMEN` writer - LSI2 oscillator trimming enable"]
pub struct LSI2TRIMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LSI2TRIMEN_W<'a> {
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
#[doc = "Field `LSI2RDY` reader - LSI2 oscillator ready"]
pub struct LSI2RDY_R(crate::FieldReader<bool, bool>);
impl LSI2RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSI2RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSI2RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSI2ON` reader - LSI2 oscillator enabled"]
pub struct LSI2ON_R(crate::FieldReader<bool, bool>);
impl LSI2ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSI2ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSI2ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSI2ON` writer - LSI2 oscillator enabled"]
pub struct LSI2ON_W<'a> {
    w: &'a mut W,
}
impl<'a> LSI2ON_W<'a> {
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
#[doc = "Field `LSI1RDY` reader - LSI1 oscillator ready"]
pub struct LSI1RDY_R(crate::FieldReader<bool, bool>);
impl LSI1RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSI1RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSI1RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSI1ON` reader - LSI1 oscillator enabled"]
pub struct LSI1ON_R(crate::FieldReader<bool, bool>);
impl LSI1ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSI1ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSI1ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSI1ON` writer - LSI1 oscillator enabled"]
pub struct LSI1ON_W<'a> {
    w: &'a mut W,
}
impl<'a> LSI1ON_W<'a> {
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
#[doc = "Field `RFRSTS` reader - Radio system BLE and 802.15.4 reset status"]
pub struct RFRSTS_R(crate::FieldReader<bool, bool>);
impl RFRSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFRSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFRSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Independent window watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - BOR flag"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Pin reset flag"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Option byte loader reset flag"]
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - RF system wakeup clock source selection"]
    #[inline(always)]
    pub fn rfwkpsel(&self) -> RFWKPSEL_R {
        RFWKPSEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - LSI2 oscillator bias configuration"]
    #[inline(always)]
    pub fn lsi2bw(&self) -> LSI2BW_R {
        LSI2BW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - LSI2 oscillator trim OK"]
    #[inline(always)]
    pub fn lsi2trimok(&self) -> LSI2TRIMOK_R {
        LSI2TRIMOK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LSI2 oscillator trimming enable"]
    #[inline(always)]
    pub fn lsi2trimen(&self) -> LSI2TRIMEN_R {
        LSI2TRIMEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LSI2 oscillator ready"]
    #[inline(always)]
    pub fn lsi2rdy(&self) -> LSI2RDY_R {
        LSI2RDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LSI2 oscillator enabled"]
    #[inline(always)]
    pub fn lsi2on(&self) -> LSI2ON_R {
        LSI2ON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSI1 oscillator ready"]
    #[inline(always)]
    pub fn lsi1rdy(&self) -> LSI1RDY_R {
        LSI1RDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LSI1 oscillator enabled"]
    #[inline(always)]
    pub fn lsi1on(&self) -> LSI1ON_R {
        LSI1ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Radio system BLE and 802.15.4 reset status"]
    #[inline(always)]
    pub fn rfrsts(&self) -> RFRSTS_R {
        RFRSTS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W {
        RMVF_W { w: self }
    }
    #[doc = "Bits 14:15 - RF system wakeup clock source selection"]
    #[inline(always)]
    pub fn rfwkpsel(&mut self) -> RFWKPSEL_W {
        RFWKPSEL_W { w: self }
    }
    #[doc = "Bits 8:11 - LSI2 oscillator bias configuration"]
    #[inline(always)]
    pub fn lsi2bw(&mut self) -> LSI2BW_W {
        LSI2BW_W { w: self }
    }
    #[doc = "Bit 4 - LSI2 oscillator trimming enable"]
    #[inline(always)]
    pub fn lsi2trimen(&mut self) -> LSI2TRIMEN_W {
        LSI2TRIMEN_W { w: self }
    }
    #[doc = "Bit 2 - LSI2 oscillator enabled"]
    #[inline(always)]
    pub fn lsi2on(&mut self) -> LSI2ON_W {
        LSI2ON_W { w: self }
    }
    #[doc = "Bit 0 - LSI1 oscillator enabled"]
    #[inline(always)]
    pub fn lsi1on(&mut self) -> LSI1ON_W {
        LSI1ON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR to value 0x0c00_0000"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c00_0000
    }
}
