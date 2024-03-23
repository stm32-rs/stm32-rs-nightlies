#[doc = "Register `CSR4` reader"]
pub type R = crate::R<CSR4rs>;
#[doc = "Register `CSR4` writer"]
pub type W = crate::W<CSR4rs>;
#[doc = "Field `CS4` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS4_R = crate::FieldReader<u32>;
#[doc = "Field `CS4` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn cs4(&self) -> CS4_R {
        CS4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    #[must_use]
    pub fn cs4(&mut self) -> CS4_W<CSR4rs> {
        CS4_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR4rs;
impl crate::RegisterSpec for CSR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr4::R`](R) reader structure"]
impl crate::Readable for CSR4rs {}
#[doc = "`write(|w| ..)` method takes [`csr4::W`](W) writer structure"]
impl crate::Writable for CSR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR4 to value 0"]
impl crate::Resettable for CSR4rs {
    const RESET_VALUE: u32 = 0;
}
