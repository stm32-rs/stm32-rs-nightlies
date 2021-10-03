#[doc = "Register `PRIVCR` reader"]
pub struct R(crate::R<PRIVCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIVCR` writer"]
pub struct W(crate::W<PRIVCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVCR_SPEC>;
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
impl From<crate::W<PRIVCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKPRWPRIV` reader - Backup registers zone 1 privilege protection"]
pub struct BKPRWPRIV_R(crate::FieldReader<bool, bool>);
impl BKPRWPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKPRWPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKPRWPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKPRWPRIV` writer - Backup registers zone 1 privilege protection"]
pub struct BKPRWPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPRWPRIV_W<'a> {
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
#[doc = "Field `BKPWPRIV` reader - Backup registers zone 2 privilege protection"]
pub struct BKPWPRIV_R(crate::FieldReader<bool, bool>);
impl BKPWPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKPWPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKPWPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKPWPRIV` writer - Backup registers zone 2 privilege protection"]
pub struct BKPWPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPWPRIV_W<'a> {
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
#[doc = "Field `TAMPPRIV` reader - Tamper privilege protection"]
pub struct TAMPPRIV_R(crate::FieldReader<bool, bool>);
impl TAMPPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPPRIV` writer - Tamper privilege protection"]
pub struct TAMPPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPPRIV_W<'a> {
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
    #[doc = "Bit 29 - Backup registers zone 1 privilege protection"]
    #[inline(always)]
    pub fn bkprwpriv(&self) -> BKPRWPRIV_R {
        BKPRWPRIV_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Backup registers zone 2 privilege protection"]
    #[inline(always)]
    pub fn bkpwpriv(&self) -> BKPWPRIV_R {
        BKPWPRIV_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Tamper privilege protection"]
    #[inline(always)]
    pub fn tamppriv(&self) -> TAMPPRIV_R {
        TAMPPRIV_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - Backup registers zone 1 privilege protection"]
    #[inline(always)]
    pub fn bkprwpriv(&mut self) -> BKPRWPRIV_W {
        BKPRWPRIV_W { w: self }
    }
    #[doc = "Bit 30 - Backup registers zone 2 privilege protection"]
    #[inline(always)]
    pub fn bkpwpriv(&mut self) -> BKPWPRIV_W {
        BKPWPRIV_W { w: self }
    }
    #[doc = "Bit 31 - Tamper privilege protection"]
    #[inline(always)]
    pub fn tamppriv(&mut self) -> TAMPPRIV_W {
        TAMPPRIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP privilege mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [privcr](index.html) module"]
pub struct PRIVCR_SPEC;
impl crate::RegisterSpec for PRIVCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [privcr::R](R) reader structure"]
impl crate::Readable for PRIVCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [privcr::W](W) writer structure"]
impl crate::Writable for PRIVCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRIVCR to value 0"]
impl crate::Resettable for PRIVCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
