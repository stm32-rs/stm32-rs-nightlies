#[doc = "Register `CSR33` reader"]
pub type R = crate::R<CSR33rs>;
#[doc = "Register `CSR33` writer"]
pub type W = crate::W<CSR33rs>;
#[doc = "Field `CS33` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS33_R = crate::FieldReader<u32>;
#[doc = "Field `CS33` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS33_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn cs33(&self) -> CS33_R {
        CS33_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    #[must_use]
    pub fn cs33(&mut self) -> CS33_W<CSR33rs> {
        CS33_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr33::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr33::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR33rs;
impl crate::RegisterSpec for CSR33rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr33::R`](R) reader structure"]
impl crate::Readable for CSR33rs {}
#[doc = "`write(|w| ..)` method takes [`csr33::W`](W) writer structure"]
impl crate::Writable for CSR33rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR33 to value 0"]
impl crate::Resettable for CSR33rs {
    const RESET_VALUE: u32 = 0;
}
