#[doc = "Register `CSR23` reader"]
pub type R = crate::R<CSR23rs>;
#[doc = "Register `CSR23` writer"]
pub type W = crate::W<CSR23rs>;
#[doc = "Field `CS23` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS23_R = crate::FieldReader<u32>;
#[doc = "Field `CS23` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS23_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn cs23(&self) -> CS23_R {
        CS23_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    #[must_use]
    pub fn cs23(&mut self) -> CS23_W<CSR23rs> {
        CS23_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR23rs;
impl crate::RegisterSpec for CSR23rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr23::R`](R) reader structure"]
impl crate::Readable for CSR23rs {}
#[doc = "`write(|w| ..)` method takes [`csr23::W`](W) writer structure"]
impl crate::Writable for CSR23rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR23 to value 0"]
impl crate::Resettable for CSR23rs {
    const RESET_VALUE: u32 = 0;
}
