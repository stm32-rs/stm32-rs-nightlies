#[doc = "Register `R1ENDADDR` reader"]
pub struct R(crate::R<R1ENDADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R1ENDADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R1ENDADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R1ENDADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R1ENDADDR` writer"]
pub struct W(crate::W<R1ENDADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R1ENDADDR_SPEC>;
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
impl From<crate::W<R1ENDADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R1ENDADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGx_END_ADDR` reader - Region AXI end address"]
pub struct REGX_END_ADDR_R(crate::FieldReader<u32, u32>);
impl REGX_END_ADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        REGX_END_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGX_END_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGx_END_ADDR` writer - Region AXI end address"]
pub struct REGX_END_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> REGX_END_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Region AXI end address"]
    #[inline(always)]
    pub fn regx_end_addr(&self) -> REGX_END_ADDR_R {
        REGX_END_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region AXI end address"]
    #[inline(always)]
    pub fn regx_end_addr(&mut self) -> REGX_END_ADDR_W {
        REGX_END_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTFDEC region x end address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r1endaddr](index.html) module"]
pub struct R1ENDADDR_SPEC;
impl crate::RegisterSpec for R1ENDADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r1endaddr::R](R) reader structure"]
impl crate::Readable for R1ENDADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r1endaddr::W](W) writer structure"]
impl crate::Writable for R1ENDADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R1ENDADDR to value 0x0fff"]
impl crate::Resettable for R1ENDADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff
    }
}
