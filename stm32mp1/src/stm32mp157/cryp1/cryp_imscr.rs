#[doc = "Register `CRYP_IMSCR` reader"]
pub struct R(crate::R<CRYP_IMSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_IMSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_IMSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_IMSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYP_IMSCR` writer"]
pub struct W(crate::W<CRYP_IMSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYP_IMSCR_SPEC>;
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
impl From<crate::W<CRYP_IMSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYP_IMSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIM` reader - INIM"]
pub struct INIM_R(crate::FieldReader<bool, bool>);
impl INIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        INIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INIM` writer - INIM"]
pub struct INIM_W<'a> {
    w: &'a mut W,
}
impl<'a> INIM_W<'a> {
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
#[doc = "Field `OUTIM` reader - OUTIM"]
pub struct OUTIM_R(crate::FieldReader<bool, bool>);
impl OUTIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTIM` writer - OUTIM"]
pub struct OUTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTIM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - INIM"]
    #[inline(always)]
    pub fn inim(&self) -> INIM_R {
        INIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OUTIM"]
    #[inline(always)]
    pub fn outim(&self) -> OUTIM_R {
        OUTIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INIM"]
    #[inline(always)]
    pub fn inim(&mut self) -> INIM_W {
        INIM_W { w: self }
    }
    #[doc = "Bit 1 - OUTIM"]
    #[inline(always)]
    pub fn outim(&mut self) -> OUTIM_W {
        OUTIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The CRYP_IMSCR register is the interrupt mask set or clear register. It is a read/write register. When a read operation is performed, this register gives the current value of the mask applied to the relevant interrupt. Writing 1 to the particular bit sets the mask, thus enabling the interrupt to be read. Writing 0 to this bit clears the corresponding mask. All the bits are cleared to 0 when the peripheral is reset.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_imscr](index.html) module"]
pub struct CRYP_IMSCR_SPEC;
impl crate::RegisterSpec for CRYP_IMSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cryp_imscr::R](R) reader structure"]
impl crate::Readable for CRYP_IMSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cryp_imscr::W](W) writer structure"]
impl crate::Writable for CRYP_IMSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRYP_IMSCR to value 0"]
impl crate::Resettable for CRYP_IMSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
