#[doc = "Register `CR3` reader"]
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR3` writer"]
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ITAMP3NOER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITAMP3NOER_A {
    #[doc = "0: Internal tamper x event erases the backup registers"]
    ERASE = 0,
    #[doc = "1: Internal tamper x event does not erase the backup registers"]
    NOTERASE = 1,
}
impl From<ITAMP3NOER_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3NOER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP3NOER` reader - ITAMP3NOER"]
pub struct ITAMP3NOER_R(crate::FieldReader<bool, ITAMP3NOER_A>);
impl ITAMP3NOER_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP3NOER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITAMP3NOER_A {
        match self.bits {
            false => ITAMP3NOER_A::ERASE,
            true => ITAMP3NOER_A::NOTERASE,
        }
    }
    #[doc = "Checks if the value of the field is `ERASE`"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        **self == ITAMP3NOER_A::ERASE
    }
    #[doc = "Checks if the value of the field is `NOTERASE`"]
    #[inline(always)]
    pub fn is_not_erase(&self) -> bool {
        **self == ITAMP3NOER_A::NOTERASE
    }
}
impl core::ops::Deref for ITAMP3NOER_R {
    type Target = crate::FieldReader<bool, ITAMP3NOER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP3NOER` writer - ITAMP3NOER"]
pub struct ITAMP3NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP3NOER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITAMP3NOER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal tamper x event erases the backup registers"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(ITAMP3NOER_A::ERASE)
    }
    #[doc = "Internal tamper x event does not erase the backup registers"]
    #[inline(always)]
    pub fn not_erase(self) -> &'a mut W {
        self.variant(ITAMP3NOER_A::NOTERASE)
    }
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
#[doc = "ITAMP5NOER"]
pub type ITAMP5NOER_A = ITAMP3NOER_A;
#[doc = "Field `ITAMP5NOER` reader - ITAMP5NOER"]
pub type ITAMP5NOER_R = ITAMP3NOER_R;
#[doc = "Field `ITAMP5NOER` writer - ITAMP5NOER"]
pub struct ITAMP5NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP5NOER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITAMP5NOER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal tamper x event erases the backup registers"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(ITAMP5NOER_A::ERASE)
    }
    #[doc = "Internal tamper x event does not erase the backup registers"]
    #[inline(always)]
    pub fn not_erase(self) -> &'a mut W {
        self.variant(ITAMP5NOER_A::NOTERASE)
    }
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
#[doc = "ITAMP6NOER"]
pub type ITAMP6NOER_A = ITAMP3NOER_A;
#[doc = "Field `ITAMP6NOER` reader - ITAMP6NOER"]
pub type ITAMP6NOER_R = ITAMP3NOER_R;
#[doc = "Field `ITAMP6NOER` writer - ITAMP6NOER"]
pub struct ITAMP6NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP6NOER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITAMP6NOER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal tamper x event erases the backup registers"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(ITAMP6NOER_A::ERASE)
    }
    #[doc = "Internal tamper x event does not erase the backup registers"]
    #[inline(always)]
    pub fn not_erase(self) -> &'a mut W {
        self.variant(ITAMP6NOER_A::NOTERASE)
    }
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
#[doc = "ITAMP8NOER"]
pub type ITAMP8NOER_A = ITAMP3NOER_A;
#[doc = "Field `ITAMP8NOER` reader - ITAMP8NOER"]
pub type ITAMP8NOER_R = ITAMP3NOER_R;
#[doc = "Field `ITAMP8NOER` writer - ITAMP8NOER"]
pub struct ITAMP8NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP8NOER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITAMP8NOER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal tamper x event erases the backup registers"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(ITAMP8NOER_A::ERASE)
    }
    #[doc = "Internal tamper x event does not erase the backup registers"]
    #[inline(always)]
    pub fn not_erase(self) -> &'a mut W {
        self.variant(ITAMP8NOER_A::NOTERASE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - ITAMP3NOER"]
    #[inline(always)]
    pub fn itamp3noer(&self) -> ITAMP3NOER_R {
        ITAMP3NOER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ITAMP5NOER"]
    #[inline(always)]
    pub fn itamp5noer(&self) -> ITAMP5NOER_R {
        ITAMP5NOER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ITAMP6NOER"]
    #[inline(always)]
    pub fn itamp6noer(&self) -> ITAMP6NOER_R {
        ITAMP6NOER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ITAMP8NOER"]
    #[inline(always)]
    pub fn itamp8noer(&self) -> ITAMP8NOER_R {
        ITAMP8NOER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - ITAMP3NOER"]
    #[inline(always)]
    pub fn itamp3noer(&mut self) -> ITAMP3NOER_W {
        ITAMP3NOER_W { w: self }
    }
    #[doc = "Bit 4 - ITAMP5NOER"]
    #[inline(always)]
    pub fn itamp5noer(&mut self) -> ITAMP5NOER_W {
        ITAMP5NOER_W { w: self }
    }
    #[doc = "Bit 5 - ITAMP6NOER"]
    #[inline(always)]
    pub fn itamp6noer(&mut self) -> ITAMP6NOER_W {
        ITAMP6NOER_W { w: self }
    }
    #[doc = "Bit 7 - ITAMP8NOER"]
    #[inline(always)]
    pub fn itamp8noer(&mut self) -> ITAMP8NOER_W {
        ITAMP8NOER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](index.html) module"]
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr3::R](R) reader structure"]
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr3::W](W) writer structure"]
impl crate::Writable for CR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
