#[doc = "Register `BDMUPR` reader"]
pub struct R(crate::R<BDMUPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDMUPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDMUPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDMUPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDMUPR` writer"]
pub struct W(crate::W<BDMUPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDMUPR_SPEC>;
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
impl From<crate::W<BDMUPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDMUPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "MCMP4"]
pub type MCMP4_A = MCR_A;
#[doc = "Field `MCMP4` reader - MCMP4"]
pub type MCMP4_R = MCR_R;
#[doc = "Field `MCMP4` writer - MCMP4"]
pub struct MCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MCMP4_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MCMP4_A::UPDATED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "MCMP3"]
pub type MCMP3_A = MCR_A;
#[doc = "Field `MCMP3` reader - MCMP3"]
pub type MCMP3_R = MCR_R;
#[doc = "Field `MCMP3` writer - MCMP3"]
pub struct MCMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MCMP3_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MCMP3_A::UPDATED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "MCMP2"]
pub type MCMP2_A = MCR_A;
#[doc = "Field `MCMP2` reader - MCMP2"]
pub type MCMP2_R = MCR_R;
#[doc = "Field `MCMP2` writer - MCMP2"]
pub struct MCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MCMP2_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MCMP2_A::UPDATED)
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
#[doc = "MCMP1"]
pub type MCMP1_A = MCR_A;
#[doc = "Field `MCMP1` reader - MCMP1"]
pub type MCMP1_R = MCR_R;
#[doc = "Field `MCMP1` writer - MCMP1"]
pub struct MCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MCMP1_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MCMP1_A::UPDATED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "MREP"]
pub type MREP_A = MCR_A;
#[doc = "Field `MREP` reader - MREP"]
pub type MREP_R = MCR_R;
#[doc = "Field `MREP` writer - MREP"]
pub struct MREP_W<'a> {
    w: &'a mut W,
}
impl<'a> MREP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MREP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MREP_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MREP_A::UPDATED)
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
#[doc = "MPER"]
pub type MPER_A = MCR_A;
#[doc = "Field `MPER` reader - MPER"]
pub type MPER_R = MCR_R;
#[doc = "Field `MPER` writer - MPER"]
pub struct MPER_W<'a> {
    w: &'a mut W,
}
impl<'a> MPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MPER_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MPER_A::UPDATED)
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
#[doc = "MCNT"]
pub type MCNT_A = MCR_A;
#[doc = "Field `MCNT` reader - MCNT"]
pub type MCNT_R = MCR_R;
#[doc = "Field `MCNT` writer - MCNT"]
pub struct MCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCNT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MCNT_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MCNT_A::UPDATED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "MDIER"]
pub type MDIER_A = MCR_A;
#[doc = "Field `MDIER` reader - MDIER"]
pub type MDIER_R = MCR_R;
#[doc = "Field `MDIER` writer - MDIER"]
pub struct MDIER_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MDIER_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MDIER_A::UPDATED)
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
#[doc = "MICR"]
pub type MICR_A = MCR_A;
#[doc = "Field `MICR` reader - MICR"]
pub type MICR_R = MCR_R;
#[doc = "Field `MICR` writer - MICR"]
pub struct MICR_W<'a> {
    w: &'a mut W,
}
impl<'a> MICR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MICR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MICR_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MICR_A::UPDATED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "MCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCR_A {
    #[doc = "0: Register not updated by burst DMA access"]
    NOTUPDATED = 0,
    #[doc = "1: Register updated by burst DMA access"]
    UPDATED = 1,
}
impl From<MCR_A> for bool {
    #[inline(always)]
    fn from(variant: MCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCR` reader - MCR"]
pub struct MCR_R(crate::FieldReader<bool, MCR_A>);
impl MCR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCR_A {
        match self.bits {
            false => MCR_A::NOTUPDATED,
            true => MCR_A::UPDATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTUPDATED`"]
    #[inline(always)]
    pub fn is_not_updated(&self) -> bool {
        **self == MCR_A::NOTUPDATED
    }
    #[doc = "Checks if the value of the field is `UPDATED`"]
    #[inline(always)]
    pub fn is_updated(&self) -> bool {
        **self == MCR_A::UPDATED
    }
}
impl core::ops::Deref for MCR_R {
    type Target = crate::FieldReader<bool, MCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCR` writer - MCR"]
pub struct MCR_W<'a> {
    w: &'a mut W,
}
impl<'a> MCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MCR_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MCR_A::UPDATED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 9 - MCMP4"]
    #[inline(always)]
    pub fn mcmp4(&self) -> MCMP4_R {
        MCMP4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MCMP3"]
    #[inline(always)]
    pub fn mcmp3(&self) -> MCMP3_R {
        MCMP3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MCMP2"]
    #[inline(always)]
    pub fn mcmp2(&self) -> MCMP2_R {
        MCMP2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MCMP1"]
    #[inline(always)]
    pub fn mcmp1(&self) -> MCMP1_R {
        MCMP1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MREP"]
    #[inline(always)]
    pub fn mrep(&self) -> MREP_R {
        MREP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPER"]
    #[inline(always)]
    pub fn mper(&self) -> MPER_R {
        MPER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MCNT"]
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MDIER"]
    #[inline(always)]
    pub fn mdier(&self) -> MDIER_R {
        MDIER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - MICR"]
    #[inline(always)]
    pub fn micr(&self) -> MICR_R {
        MICR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - MCR"]
    #[inline(always)]
    pub fn mcr(&self) -> MCR_R {
        MCR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - MCMP4"]
    #[inline(always)]
    pub fn mcmp4(&mut self) -> MCMP4_W {
        MCMP4_W { w: self }
    }
    #[doc = "Bit 8 - MCMP3"]
    #[inline(always)]
    pub fn mcmp3(&mut self) -> MCMP3_W {
        MCMP3_W { w: self }
    }
    #[doc = "Bit 7 - MCMP2"]
    #[inline(always)]
    pub fn mcmp2(&mut self) -> MCMP2_W {
        MCMP2_W { w: self }
    }
    #[doc = "Bit 6 - MCMP1"]
    #[inline(always)]
    pub fn mcmp1(&mut self) -> MCMP1_W {
        MCMP1_W { w: self }
    }
    #[doc = "Bit 5 - MREP"]
    #[inline(always)]
    pub fn mrep(&mut self) -> MREP_W {
        MREP_W { w: self }
    }
    #[doc = "Bit 4 - MPER"]
    #[inline(always)]
    pub fn mper(&mut self) -> MPER_W {
        MPER_W { w: self }
    }
    #[doc = "Bit 3 - MCNT"]
    #[inline(always)]
    pub fn mcnt(&mut self) -> MCNT_W {
        MCNT_W { w: self }
    }
    #[doc = "Bit 2 - MDIER"]
    #[inline(always)]
    pub fn mdier(&mut self) -> MDIER_W {
        MDIER_W { w: self }
    }
    #[doc = "Bit 1 - MICR"]
    #[inline(always)]
    pub fn micr(&mut self) -> MICR_W {
        MICR_W { w: self }
    }
    #[doc = "Bit 0 - MCR"]
    #[inline(always)]
    pub fn mcr(&mut self) -> MCR_W {
        MCR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BDMUPDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdmupr](index.html) module"]
pub struct BDMUPR_SPEC;
impl crate::RegisterSpec for BDMUPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdmupr::R](R) reader structure"]
impl crate::Readable for BDMUPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdmupr::W](W) writer structure"]
impl crate::Writable for BDMUPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BDMUPR to value 0"]
impl crate::Resettable for BDMUPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
