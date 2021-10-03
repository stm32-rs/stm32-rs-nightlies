#[doc = "Register `PCROP2SR` reader"]
pub struct R(crate::R<PCROP2SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP2SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP2SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP2SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCROP2SR` writer"]
pub struct W(crate::W<PCROP2SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCROP2SR_SPEC>;
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
impl From<crate::W<PCROP2SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCROP2SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCROP2_STRT` reader - Bank 2 PCROP area start offset"]
pub struct PCROP2_STRT_R(crate::FieldReader<u16, u16>);
impl PCROP2_STRT_R {
    pub(crate) fn new(bits: u16) -> Self {
        PCROP2_STRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCROP2_STRT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCROP2_STRT` writer - Bank 2 PCROP area start offset"]
pub struct PCROP2_STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP2_STRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Bank 2 PCROP area start offset"]
    #[inline(always)]
    pub fn pcrop2_strt(&self) -> PCROP2_STRT_R {
        PCROP2_STRT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bank 2 PCROP area start offset"]
    #[inline(always)]
    pub fn pcrop2_strt(&mut self) -> PCROP2_STRT_W {
        PCROP2_STRT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Bank 2 PCROP Start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrop2sr](index.html) module"]
pub struct PCROP2SR_SPEC;
impl crate::RegisterSpec for PCROP2SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcrop2sr::R](R) reader structure"]
impl crate::Readable for PCROP2SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcrop2sr::W](W) writer structure"]
impl crate::Writable for PCROP2SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCROP2SR to value 0xffff_0000"]
impl crate::Resettable for PCROP2SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_0000
    }
}
