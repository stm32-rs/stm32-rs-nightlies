#[doc = "Register `QMEM1_11` reader"]
pub struct R(crate::R<QMEM1_11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QMEM1_11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QMEM1_11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QMEM1_11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QMEM1_11` writer"]
pub struct W(crate::W<QMEM1_11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QMEM1_11_SPEC>;
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
impl From<crate::W<QMEM1_11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QMEM1_11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QMem_RAM` reader - QMem RAM"]
pub struct QMEM_RAM_R(crate::FieldReader<u32, u32>);
impl QMEM_RAM_R {
    pub(crate) fn new(bits: u32) -> Self {
        QMEM_RAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QMEM_RAM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QMem_RAM` writer - QMem RAM"]
pub struct QMEM_RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> QMEM_RAM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - QMem RAM"]
    #[inline(always)]
    pub fn qmem_ram(&self) -> QMEM_RAM_R {
        QMEM_RAM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - QMem RAM"]
    #[inline(always)]
    pub fn qmem_ram(&mut self) -> QMEM_RAM_W {
        QMEM_RAM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem1_11](index.html) module"]
pub struct QMEM1_11_SPEC;
impl crate::RegisterSpec for QMEM1_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qmem1_11::R](R) reader structure"]
impl crate::Readable for QMEM1_11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qmem1_11::W](W) writer structure"]
impl crate::Writable for QMEM1_11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QMEM1_11 to value 0"]
impl crate::Resettable for QMEM1_11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
