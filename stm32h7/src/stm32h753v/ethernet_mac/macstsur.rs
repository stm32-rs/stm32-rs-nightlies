#[doc = "Register `MACSTSUR` reader"]
pub struct R(crate::R<MACSTSUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACSTSUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACSTSUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACSTSUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACSTSUR` writer"]
pub struct W(crate::W<MACSTSUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACSTSUR_SPEC>;
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
impl From<crate::W<MACSTSUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACSTSUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSS` reader - Timestamp Seconds"]
pub struct TSS_R(crate::FieldReader<u32, u32>);
impl TSS_R {
    pub(crate) fn new(bits: u32) -> Self {
        TSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSS` writer - Timestamp Seconds"]
pub struct TSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Timestamp Seconds"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Seconds"]
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W {
        TSS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System time seconds update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macstsur](index.html) module"]
pub struct MACSTSUR_SPEC;
impl crate::RegisterSpec for MACSTSUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macstsur::R](R) reader structure"]
impl crate::Readable for MACSTSUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macstsur::W](W) writer structure"]
impl crate::Writable for MACSTSUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACSTSUR to value 0"]
impl crate::Resettable for MACSTSUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
