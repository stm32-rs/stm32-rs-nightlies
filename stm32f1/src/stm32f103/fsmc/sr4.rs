#[doc = "Register `SR4` reader"]
pub struct R(crate::R<SR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR4` writer"]
pub struct W(crate::W<SR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR4_SPEC>;
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
impl From<crate::W<SR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEMPT` reader - FEMPT"]
pub struct FEMPT_R(crate::FieldReader<bool, bool>);
impl FEMPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEMPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEMPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFEN` reader - IFEN"]
pub struct IFEN_R(crate::FieldReader<bool, bool>);
impl IFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFEN` writer - IFEN"]
pub struct IFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IFEN_W<'a> {
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
#[doc = "Field `ILEN` reader - ILEN"]
pub struct ILEN_R(crate::FieldReader<bool, bool>);
impl ILEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ILEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ILEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ILEN` writer - ILEN"]
pub struct ILEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ILEN_W<'a> {
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
#[doc = "Field `IREN` reader - IREN"]
pub struct IREN_R(crate::FieldReader<bool, bool>);
impl IREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IREN` writer - IREN"]
pub struct IREN_W<'a> {
    w: &'a mut W,
}
impl<'a> IREN_W<'a> {
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
#[doc = "Field `IFS` reader - IFS"]
pub struct IFS_R(crate::FieldReader<bool, bool>);
impl IFS_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFS` writer - IFS"]
pub struct IFS_W<'a> {
    w: &'a mut W,
}
impl<'a> IFS_W<'a> {
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
#[doc = "Field `ILS` reader - ILS"]
pub struct ILS_R(crate::FieldReader<bool, bool>);
impl ILS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ILS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ILS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ILS` writer - ILS"]
pub struct ILS_W<'a> {
    w: &'a mut W,
}
impl<'a> ILS_W<'a> {
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
#[doc = "Field `IRS` reader - IRS"]
pub struct IRS_R(crate::FieldReader<bool, bool>);
impl IRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRS` writer - IRS"]
pub struct IRS_W<'a> {
    w: &'a mut W,
}
impl<'a> IRS_W<'a> {
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
impl R {
    #[doc = "Bit 6 - FEMPT"]
    #[inline(always)]
    pub fn fempt(&self) -> FEMPT_R {
        FEMPT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IFEN"]
    #[inline(always)]
    pub fn ifen(&self) -> IFEN_R {
        IFEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ILEN"]
    #[inline(always)]
    pub fn ilen(&self) -> ILEN_R {
        ILEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IREN"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IFS"]
    #[inline(always)]
    pub fn ifs(&self) -> IFS_R {
        IFS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ILS"]
    #[inline(always)]
    pub fn ils(&self) -> ILS_R {
        ILS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - IRS"]
    #[inline(always)]
    pub fn irs(&self) -> IRS_R {
        IRS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - IFEN"]
    #[inline(always)]
    pub fn ifen(&mut self) -> IFEN_W {
        IFEN_W { w: self }
    }
    #[doc = "Bit 4 - ILEN"]
    #[inline(always)]
    pub fn ilen(&mut self) -> ILEN_W {
        ILEN_W { w: self }
    }
    #[doc = "Bit 3 - IREN"]
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W {
        IREN_W { w: self }
    }
    #[doc = "Bit 2 - IFS"]
    #[inline(always)]
    pub fn ifs(&mut self) -> IFS_W {
        IFS_W { w: self }
    }
    #[doc = "Bit 1 - ILS"]
    #[inline(always)]
    pub fn ils(&mut self) -> ILS_W {
        ILS_W { w: self }
    }
    #[doc = "Bit 0 - IRS"]
    #[inline(always)]
    pub fn irs(&mut self) -> IRS_W {
        IRS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO status and interrupt register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr4](index.html) module"]
pub struct SR4_SPEC;
impl crate::RegisterSpec for SR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr4::R](R) reader structure"]
impl crate::Readable for SR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr4::W](W) writer structure"]
impl crate::Writable for SR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR4 to value 0x40"]
impl crate::Resettable for SR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
