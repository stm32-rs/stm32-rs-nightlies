#[doc = "Register `CSR41` reader"]
pub type R = crate::R<CSR41rs>;
#[doc = "Register `CSR41` writer"]
pub type W = crate::W<CSR41rs>;
#[doc = "Field `CS41` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS41_R = crate::FieldReader<u32>;
#[doc = "Field `CS41` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS41_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn cs41(&self) -> CS41_R {
        CS41_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    #[must_use]
    pub fn cs41(&mut self) -> CS41_W<CSR41rs> {
        CS41_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr41::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr41::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR41rs;
impl crate::RegisterSpec for CSR41rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr41::R`](R) reader structure"]
impl crate::Readable for CSR41rs {}
#[doc = "`write(|w| ..)` method takes [`csr41::W`](W) writer structure"]
impl crate::Writable for CSR41rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR41 to value 0"]
impl crate::Resettable for CSR41rs {
    const RESET_VALUE: u32 = 0;
}
