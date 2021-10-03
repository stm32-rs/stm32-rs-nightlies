#[doc = "Register `DDRPHYC_DDR3_MR3` reader"]
pub struct R(crate::R<DDRPHYC_DDR3_MR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DDR3_MR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DDR3_MR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DDR3_MR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DDR3_MR3` writer"]
pub struct W(crate::W<DDRPHYC_DDR3_MR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DDR3_MR3_SPEC>;
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
impl From<crate::W<DDRPHYC_DDR3_MR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DDR3_MR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPRLOC` reader - MPRLOC"]
pub struct MPRLOC_R(crate::FieldReader<u8, u8>);
impl MPRLOC_R {
    pub(crate) fn new(bits: u8) -> Self {
        MPRLOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPRLOC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPRLOC` writer - MPRLOC"]
pub struct MPRLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPRLOC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
#[doc = "Field `MPR` reader - MPR"]
pub struct MPR_R(crate::FieldReader<bool, bool>);
impl MPR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPR` writer - MPR"]
pub struct MPR_W<'a> {
    w: &'a mut W,
}
impl<'a> MPR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - MPRLOC"]
    #[inline(always)]
    pub fn mprloc(&self) -> MPRLOC_R {
        MPRLOC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - MPR"]
    #[inline(always)]
    pub fn mpr(&self) -> MPR_R {
        MPR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - MPRLOC"]
    #[inline(always)]
    pub fn mprloc(&mut self) -> MPRLOC_W {
        MPRLOC_W { w: self }
    }
    #[doc = "Bit 2 - MPR"]
    #[inline(always)]
    pub fn mpr(&mut self) -> MPR_W {
        MPR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC MR3 register for DDR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ddr3_mr3](index.html) module"]
pub struct DDRPHYC_DDR3_MR3_SPEC;
impl crate::RegisterSpec for DDRPHYC_DDR3_MR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ddrphyc_ddr3_mr3::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DDR3_MR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ddr3_mr3::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DDR3_MR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_DDR3_MR3 to value 0"]
impl crate::Resettable for DDRPHYC_DDR3_MR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
