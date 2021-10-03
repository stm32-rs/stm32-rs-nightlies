#[doc = "Register `OPTCR` reader"]
pub struct R(crate::R<OPTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTCR` writer"]
pub struct W(crate::W<OPTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTCR_SPEC>;
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
impl From<crate::W<OPTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPTLOCK` reader - FLASH_OPTCR lock option configuration bit"]
pub struct OPTLOCK_R(crate::FieldReader<bool, bool>);
impl OPTLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPTLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPTLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPTLOCK` writer - FLASH_OPTCR lock option configuration bit"]
pub struct OPTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTLOCK_W<'a> {
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
#[doc = "Field `OPTSTART` reader - Option byte start change option configuration bit"]
pub struct OPTSTART_R(crate::FieldReader<bool, bool>);
impl OPTSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPTSTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPTSTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPTSTART` writer - Option byte start change option configuration bit"]
pub struct OPTSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTSTART_W<'a> {
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
#[doc = "Field `MER` reader - Flash mass erase enable bit"]
pub struct MER_R(crate::FieldReader<bool, bool>);
impl MER_R {
    pub(crate) fn new(bits: bool) -> Self {
        MER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MER` writer - Flash mass erase enable bit"]
pub struct MER_W<'a> {
    w: &'a mut W,
}
impl<'a> MER_W<'a> {
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
#[doc = "Field `OPTCHANGEERRIE` reader - Option byte change error interrupt enable bit"]
pub struct OPTCHANGEERRIE_R(crate::FieldReader<bool, bool>);
impl OPTCHANGEERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPTCHANGEERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPTCHANGEERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPTCHANGEERRIE` writer - Option byte change error interrupt enable bit"]
pub struct OPTCHANGEERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTCHANGEERRIE_W<'a> {
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
#[doc = "Field `SWAP_BANK` reader - Bank swapping configuration bit"]
pub struct SWAP_BANK_R(crate::FieldReader<bool, bool>);
impl SWAP_BANK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWAP_BANK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWAP_BANK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWAP_BANK` writer - Bank swapping configuration bit"]
pub struct SWAP_BANK_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP_BANK_W<'a> {
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
    #[doc = "Bit 0 - FLASH_OPTCR lock option configuration bit"]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Option byte start change option configuration bit"]
    #[inline(always)]
    pub fn optstart(&self) -> OPTSTART_R {
        OPTSTART_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flash mass erase enable bit"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Option byte change error interrupt enable bit"]
    #[inline(always)]
    pub fn optchangeerrie(&self) -> OPTCHANGEERRIE_R {
        OPTCHANGEERRIE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Bank swapping configuration bit"]
    #[inline(always)]
    pub fn swap_bank(&self) -> SWAP_BANK_R {
        SWAP_BANK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH_OPTCR lock option configuration bit"]
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W {
        OPTLOCK_W { w: self }
    }
    #[doc = "Bit 1 - Option byte start change option configuration bit"]
    #[inline(always)]
    pub fn optstart(&mut self) -> OPTSTART_W {
        OPTSTART_W { w: self }
    }
    #[doc = "Bit 4 - Flash mass erase enable bit"]
    #[inline(always)]
    pub fn mer(&mut self) -> MER_W {
        MER_W { w: self }
    }
    #[doc = "Bit 30 - Option byte change error interrupt enable bit"]
    #[inline(always)]
    pub fn optchangeerrie(&mut self) -> OPTCHANGEERRIE_W {
        OPTCHANGEERRIE_W { w: self }
    }
    #[doc = "Bit 31 - Bank swapping configuration bit"]
    #[inline(always)]
    pub fn swap_bank(&mut self) -> SWAP_BANK_W {
        SWAP_BANK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH option control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optcr](index.html) module"]
pub struct OPTCR_SPEC;
impl crate::RegisterSpec for OPTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optcr::R](R) reader structure"]
impl crate::Readable for OPTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optcr::W](W) writer structure"]
impl crate::Writable for OPTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPTCR to value 0"]
impl crate::Resettable for OPTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
