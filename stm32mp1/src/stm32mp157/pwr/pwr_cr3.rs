#[doc = "Register `PWR_CR3` reader"]
pub struct R(crate::R<PWR_CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CR3` writer"]
pub struct W(crate::W<PWR_CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CR3_SPEC>;
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
impl From<crate::W<PWR_CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBE` reader - VBE"]
pub struct VBE_R(crate::FieldReader<bool, bool>);
impl VBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBE` writer - VBE"]
pub struct VBE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBE_W<'a> {
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
#[doc = "Field `VBRS` reader - VBRS"]
pub struct VBRS_R(crate::FieldReader<bool, bool>);
impl VBRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBRS` writer - VBRS"]
pub struct VBRS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBRS_W<'a> {
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
#[doc = "Field `DDRSREN` reader - DDRSREN"]
pub struct DDRSREN_R(crate::FieldReader<bool, bool>);
impl DDRSREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRSREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRSREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRSREN` writer - DDRSREN"]
pub struct DDRSREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRSREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `DDRSRDIS` reader - DDRSRDIS"]
pub struct DDRSRDIS_R(crate::FieldReader<bool, bool>);
impl DDRSRDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRSRDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRSRDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRSRDIS` writer - DDRSRDIS"]
pub struct DDRSRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRSRDIS_W<'a> {
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
#[doc = "Field `DDRRETEN` reader - DDRRETEN"]
pub struct DDRRETEN_R(crate::FieldReader<bool, bool>);
impl DDRRETEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRRETEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRRETEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRRETEN` writer - DDRRETEN"]
pub struct DDRRETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRRETEN_W<'a> {
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
#[doc = "Field `POPL` reader - POPL"]
pub struct POPL_R(crate::FieldReader<u8, u8>);
impl POPL_R {
    pub(crate) fn new(bits: u8) -> Self {
        POPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POPL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POPL` writer - POPL"]
pub struct POPL_W<'a> {
    w: &'a mut W,
}
impl<'a> POPL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | ((value as u32 & 0x1f) << 17);
        self.w
    }
}
#[doc = "Field `USB33DEN` reader - USB33DEN"]
pub struct USB33DEN_R(crate::FieldReader<bool, bool>);
impl USB33DEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB33DEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB33DEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB33DEN` writer - USB33DEN"]
pub struct USB33DEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB33DEN_W<'a> {
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
#[doc = "Field `USB33RDY` reader - USB33RDY"]
pub struct USB33RDY_R(crate::FieldReader<bool, bool>);
impl USB33RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB33RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB33RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG18EN` reader - REG18EN"]
pub struct REG18EN_R(crate::FieldReader<bool, bool>);
impl REG18EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG18EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG18EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG18EN` writer - REG18EN"]
pub struct REG18EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG18EN_W<'a> {
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
#[doc = "Field `REG18RDY` reader - REG18RDY"]
pub struct REG18RDY_R(crate::FieldReader<bool, bool>);
impl REG18RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG18RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG18RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG11EN` reader - REG11EN"]
pub struct REG11EN_R(crate::FieldReader<bool, bool>);
impl REG11EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG11EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG11EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG11EN` writer - REG11EN"]
pub struct REG11EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG11EN_W<'a> {
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
#[doc = "Field `REG11RDY` reader - REG11RDY"]
pub struct REG11RDY_R(crate::FieldReader<bool, bool>);
impl REG11RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG11RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG11RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 8 - VBE"]
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VBRS"]
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DDRSREN"]
    #[inline(always)]
    pub fn ddrsren(&self) -> DDRSREN_R {
        DDRSREN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DDRSRDIS"]
    #[inline(always)]
    pub fn ddrsrdis(&self) -> DDRSRDIS_R {
        DDRSRDIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DDRRETEN"]
    #[inline(always)]
    pub fn ddrreten(&self) -> DDRRETEN_R {
        DDRRETEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 17:21 - POPL"]
    #[inline(always)]
    pub fn popl(&self) -> POPL_R {
        POPL_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - USB33DEN"]
    #[inline(always)]
    pub fn usb33den(&self) -> USB33DEN_R {
        USB33DEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - USB33RDY"]
    #[inline(always)]
    pub fn usb33rdy(&self) -> USB33RDY_R {
        USB33RDY_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - REG18EN"]
    #[inline(always)]
    pub fn reg18en(&self) -> REG18EN_R {
        REG18EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - REG18RDY"]
    #[inline(always)]
    pub fn reg18rdy(&self) -> REG18RDY_R {
        REG18RDY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - REG11EN"]
    #[inline(always)]
    pub fn reg11en(&self) -> REG11EN_R {
        REG11EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - REG11RDY"]
    #[inline(always)]
    pub fn reg11rdy(&self) -> REG11RDY_R {
        REG11RDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - VBE"]
    #[inline(always)]
    pub fn vbe(&mut self) -> VBE_W {
        VBE_W { w: self }
    }
    #[doc = "Bit 9 - VBRS"]
    #[inline(always)]
    pub fn vbrs(&mut self) -> VBRS_W {
        VBRS_W { w: self }
    }
    #[doc = "Bit 10 - DDRSREN"]
    #[inline(always)]
    pub fn ddrsren(&mut self) -> DDRSREN_W {
        DDRSREN_W { w: self }
    }
    #[doc = "Bit 11 - DDRSRDIS"]
    #[inline(always)]
    pub fn ddrsrdis(&mut self) -> DDRSRDIS_W {
        DDRSRDIS_W { w: self }
    }
    #[doc = "Bit 12 - DDRRETEN"]
    #[inline(always)]
    pub fn ddrreten(&mut self) -> DDRRETEN_W {
        DDRRETEN_W { w: self }
    }
    #[doc = "Bits 17:21 - POPL"]
    #[inline(always)]
    pub fn popl(&mut self) -> POPL_W {
        POPL_W { w: self }
    }
    #[doc = "Bit 24 - USB33DEN"]
    #[inline(always)]
    pub fn usb33den(&mut self) -> USB33DEN_W {
        USB33DEN_W { w: self }
    }
    #[doc = "Bit 28 - REG18EN"]
    #[inline(always)]
    pub fn reg18en(&mut self) -> REG18EN_W {
        REG18EN_W { w: self }
    }
    #[doc = "Bit 30 - REG11EN"]
    #[inline(always)]
    pub fn reg11en(&mut self) -> REG11EN_W {
        REG11EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Not reset by wakeup from Standby mode and Application reset (such as NRST, IWDG) but only reset by VDD POR. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_cr3](index.html) module"]
pub struct PWR_CR3_SPEC;
impl crate::RegisterSpec for PWR_CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_cr3::R](R) reader structure"]
impl crate::Readable for PWR_CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_cr3::W](W) writer structure"]
impl crate::Writable for PWR_CR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_CR3 to value 0x5000_0000"]
impl crate::Resettable for PWR_CR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5000_0000
    }
}
