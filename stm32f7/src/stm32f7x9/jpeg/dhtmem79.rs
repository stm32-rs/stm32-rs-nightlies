#[doc = "Register `DHTMEM79` reader"]
pub struct R(crate::R<DHTMEM79_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DHTMEM79_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DHTMEM79_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DHTMEM79_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DHTMEM79` writer"]
pub struct W(crate::W<DHTMEM79_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DHTMEM79_SPEC>;
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
impl From<crate::W<DHTMEM79_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DHTMEM79_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DHTMem_RAM` reader - DHTMem RAM"]
pub struct DHTMEM_RAM_R(crate::FieldReader<u32, u32>);
impl DHTMEM_RAM_R {
    pub(crate) fn new(bits: u32) -> Self {
        DHTMEM_RAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DHTMEM_RAM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DHTMem_RAM` writer - DHTMem RAM"]
pub struct DHTMEM_RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> DHTMEM_RAM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DHTMem RAM"]
    #[inline(always)]
    pub fn dhtmem_ram(&self) -> DHTMEM_RAM_R {
        DHTMEM_RAM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DHTMem RAM"]
    #[inline(always)]
    pub fn dhtmem_ram(&mut self) -> DHTMEM_RAM_W {
        DHTMEM_RAM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem79](index.html) module"]
pub struct DHTMEM79_SPEC;
impl crate::RegisterSpec for DHTMEM79_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dhtmem79::R](R) reader structure"]
impl crate::Readable for DHTMEM79_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dhtmem79::W](W) writer structure"]
impl crate::Writable for DHTMEM79_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DHTMEM79 to value 0"]
impl crate::Resettable for DHTMEM79_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}