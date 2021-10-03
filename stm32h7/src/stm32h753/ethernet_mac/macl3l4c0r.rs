#[doc = "Register `MACL3L4C0R` reader"]
pub struct R(crate::R<MACL3L4C0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACL3L4C0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACL3L4C0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACL3L4C0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACL3L4C0R` writer"]
pub struct W(crate::W<MACL3L4C0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACL3L4C0R_SPEC>;
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
impl From<crate::W<MACL3L4C0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACL3L4C0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L3PEN0` reader - Layer 3 Protocol Enable"]
pub struct L3PEN0_R(crate::FieldReader<bool, bool>);
impl L3PEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        L3PEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L3PEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L3PEN0` writer - Layer 3 Protocol Enable"]
pub struct L3PEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> L3PEN0_W<'a> {
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
#[doc = "Field `L3SAM0` reader - Layer 3 IP SA Match Enable"]
pub struct L3SAM0_R(crate::FieldReader<bool, bool>);
impl L3SAM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        L3SAM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L3SAM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L3SAM0` writer - Layer 3 IP SA Match Enable"]
pub struct L3SAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L3SAM0_W<'a> {
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
#[doc = "Field `L3SAIM0` reader - Layer 3 IP SA Inverse Match Enable"]
pub struct L3SAIM0_R(crate::FieldReader<bool, bool>);
impl L3SAIM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        L3SAIM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L3SAIM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L3SAIM0` writer - Layer 3 IP SA Inverse Match Enable"]
pub struct L3SAIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L3SAIM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `L3DAM0` reader - Layer 3 IP DA Match Enable"]
pub struct L3DAM0_R(crate::FieldReader<bool, bool>);
impl L3DAM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        L3DAM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L3DAM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L3DAM0` writer - Layer 3 IP DA Match Enable"]
pub struct L3DAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L3DAM0_W<'a> {
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
#[doc = "Field `L3DAIM0` reader - Layer 3 IP DA Inverse Match Enable"]
pub struct L3DAIM0_R(crate::FieldReader<bool, bool>);
impl L3DAIM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        L3DAIM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L3DAIM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L3DAIM0` writer - Layer 3 IP DA Inverse Match Enable"]
pub struct L3DAIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L3DAIM0_W<'a> {
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
#[doc = "Field `L3HSBM0` reader - Layer 3 IP SA Higher Bits Match"]
pub struct L3HSBM0_R(crate::FieldReader<u8, u8>);
impl L3HSBM0_R {
    pub(crate) fn new(bits: u8) -> Self {
        L3HSBM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L3HSBM0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L3HSBM0` writer - Layer 3 IP SA Higher Bits Match"]
pub struct L3HSBM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L3HSBM0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | ((value as u32 & 0x1f) << 6);
        self.w
    }
}
#[doc = "Field `L3HDBM0` reader - Layer 3 IP DA Higher Bits Match"]
pub struct L3HDBM0_R(crate::FieldReader<u8, u8>);
impl L3HDBM0_R {
    pub(crate) fn new(bits: u8) -> Self {
        L3HDBM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L3HDBM0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L3HDBM0` writer - Layer 3 IP DA Higher Bits Match"]
pub struct L3HDBM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L3HDBM0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
#[doc = "Field `L4PEN0` reader - Layer 4 Protocol Enable"]
pub struct L4PEN0_R(crate::FieldReader<bool, bool>);
impl L4PEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        L4PEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L4PEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L4PEN0` writer - Layer 4 Protocol Enable"]
pub struct L4PEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> L4PEN0_W<'a> {
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
#[doc = "Field `L4SPM0` reader - Layer 4 Source Port Match Enable"]
pub struct L4SPM0_R(crate::FieldReader<bool, bool>);
impl L4SPM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        L4SPM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L4SPM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L4SPM0` writer - Layer 4 Source Port Match Enable"]
pub struct L4SPM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L4SPM0_W<'a> {
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
#[doc = "Field `L4SPIM0` reader - Layer 4 Source Port Inverse Match Enable"]
pub struct L4SPIM0_R(crate::FieldReader<bool, bool>);
impl L4SPIM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        L4SPIM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L4SPIM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L4SPIM0` writer - Layer 4 Source Port Inverse Match Enable"]
pub struct L4SPIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L4SPIM0_W<'a> {
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
#[doc = "Field `L4DPM0` reader - Layer 4 Destination Port Match Enable"]
pub struct L4DPM0_R(crate::FieldReader<bool, bool>);
impl L4DPM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        L4DPM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L4DPM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L4DPM0` writer - Layer 4 Destination Port Match Enable"]
pub struct L4DPM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L4DPM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `L4DPIM0` reader - Layer 4 Destination Port Inverse Match Enable"]
pub struct L4DPIM0_R(crate::FieldReader<bool, bool>);
impl L4DPIM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        L4DPIM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L4DPIM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L4DPIM0` writer - Layer 4 Destination Port Inverse Match Enable"]
pub struct L4DPIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L4DPIM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Layer 3 Protocol Enable"]
    #[inline(always)]
    pub fn l3pen0(&self) -> L3PEN0_R {
        L3PEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Layer 3 IP SA Match Enable"]
    #[inline(always)]
    pub fn l3sam0(&self) -> L3SAM0_R {
        L3SAM0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Layer 3 IP SA Inverse Match Enable"]
    #[inline(always)]
    pub fn l3saim0(&self) -> L3SAIM0_R {
        L3SAIM0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Layer 3 IP DA Match Enable"]
    #[inline(always)]
    pub fn l3dam0(&self) -> L3DAM0_R {
        L3DAM0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Layer 3 IP DA Inverse Match Enable"]
    #[inline(always)]
    pub fn l3daim0(&self) -> L3DAIM0_R {
        L3DAIM0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:10 - Layer 3 IP SA Higher Bits Match"]
    #[inline(always)]
    pub fn l3hsbm0(&self) -> L3HSBM0_R {
        L3HSBM0_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Layer 3 IP DA Higher Bits Match"]
    #[inline(always)]
    pub fn l3hdbm0(&self) -> L3HDBM0_R {
        L3HDBM0_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Layer 4 Protocol Enable"]
    #[inline(always)]
    pub fn l4pen0(&self) -> L4PEN0_R {
        L4PEN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Layer 4 Source Port Match Enable"]
    #[inline(always)]
    pub fn l4spm0(&self) -> L4SPM0_R {
        L4SPM0_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Layer 4 Source Port Inverse Match Enable"]
    #[inline(always)]
    pub fn l4spim0(&self) -> L4SPIM0_R {
        L4SPIM0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Layer 4 Destination Port Match Enable"]
    #[inline(always)]
    pub fn l4dpm0(&self) -> L4DPM0_R {
        L4DPM0_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Layer 4 Destination Port Inverse Match Enable"]
    #[inline(always)]
    pub fn l4dpim0(&self) -> L4DPIM0_R {
        L4DPIM0_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Layer 3 Protocol Enable"]
    #[inline(always)]
    pub fn l3pen0(&mut self) -> L3PEN0_W {
        L3PEN0_W { w: self }
    }
    #[doc = "Bit 2 - Layer 3 IP SA Match Enable"]
    #[inline(always)]
    pub fn l3sam0(&mut self) -> L3SAM0_W {
        L3SAM0_W { w: self }
    }
    #[doc = "Bit 3 - Layer 3 IP SA Inverse Match Enable"]
    #[inline(always)]
    pub fn l3saim0(&mut self) -> L3SAIM0_W {
        L3SAIM0_W { w: self }
    }
    #[doc = "Bit 4 - Layer 3 IP DA Match Enable"]
    #[inline(always)]
    pub fn l3dam0(&mut self) -> L3DAM0_W {
        L3DAM0_W { w: self }
    }
    #[doc = "Bit 5 - Layer 3 IP DA Inverse Match Enable"]
    #[inline(always)]
    pub fn l3daim0(&mut self) -> L3DAIM0_W {
        L3DAIM0_W { w: self }
    }
    #[doc = "Bits 6:10 - Layer 3 IP SA Higher Bits Match"]
    #[inline(always)]
    pub fn l3hsbm0(&mut self) -> L3HSBM0_W {
        L3HSBM0_W { w: self }
    }
    #[doc = "Bits 11:15 - Layer 3 IP DA Higher Bits Match"]
    #[inline(always)]
    pub fn l3hdbm0(&mut self) -> L3HDBM0_W {
        L3HDBM0_W { w: self }
    }
    #[doc = "Bit 16 - Layer 4 Protocol Enable"]
    #[inline(always)]
    pub fn l4pen0(&mut self) -> L4PEN0_W {
        L4PEN0_W { w: self }
    }
    #[doc = "Bit 18 - Layer 4 Source Port Match Enable"]
    #[inline(always)]
    pub fn l4spm0(&mut self) -> L4SPM0_W {
        L4SPM0_W { w: self }
    }
    #[doc = "Bit 19 - Layer 4 Source Port Inverse Match Enable"]
    #[inline(always)]
    pub fn l4spim0(&mut self) -> L4SPIM0_W {
        L4SPIM0_W { w: self }
    }
    #[doc = "Bit 20 - Layer 4 Destination Port Match Enable"]
    #[inline(always)]
    pub fn l4dpm0(&mut self) -> L4DPM0_W {
        L4DPM0_W { w: self }
    }
    #[doc = "Bit 21 - Layer 4 Destination Port Inverse Match Enable"]
    #[inline(always)]
    pub fn l4dpim0(&mut self) -> L4DPIM0_W {
        L4DPIM0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "L3 and L4 control 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macl3l4c0r](index.html) module"]
pub struct MACL3L4C0R_SPEC;
impl crate::RegisterSpec for MACL3L4C0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macl3l4c0r::R](R) reader structure"]
impl crate::Readable for MACL3L4C0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macl3l4c0r::W](W) writer structure"]
impl crate::Writable for MACL3L4C0R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACL3L4C0R to value 0"]
impl crate::Resettable for MACL3L4C0R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
