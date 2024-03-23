#[doc = "Register `CSR13` reader"]
pub type R = crate::R<CSR13rs>;
#[doc = "Register `CSR13` writer"]
pub type W = crate::W<CSR13rs>;
#[doc = "Field `CS13` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS13_R = crate::FieldReader<u32>;
#[doc = "Field `CS13` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS13_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn cs13(&self) -> CS13_R {
        CS13_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    #[must_use]
    pub fn cs13(&mut self) -> CS13_W<CSR13rs> {
        CS13_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR13rs;
impl crate::RegisterSpec for CSR13rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr13::R`](R) reader structure"]
impl crate::Readable for CSR13rs {}
#[doc = "`write(|w| ..)` method takes [`csr13::W`](W) writer structure"]
impl crate::Writable for CSR13rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR13 to value 0"]
impl crate::Resettable for CSR13rs {
    const RESET_VALUE: u32 = 0;
}
