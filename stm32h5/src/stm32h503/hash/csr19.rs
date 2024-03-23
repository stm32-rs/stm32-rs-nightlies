#[doc = "Register `CSR19` reader"]
pub type R = crate::R<CSR19rs>;
#[doc = "Register `CSR19` writer"]
pub type W = crate::W<CSR19rs>;
#[doc = "Field `CS19` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS19_R = crate::FieldReader<u32>;
#[doc = "Field `CS19` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS19_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn cs19(&self) -> CS19_R {
        CS19_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    #[must_use]
    pub fn cs19(&mut self) -> CS19_W<CSR19rs> {
        CS19_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR19rs;
impl crate::RegisterSpec for CSR19rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr19::R`](R) reader structure"]
impl crate::Readable for CSR19rs {}
#[doc = "`write(|w| ..)` method takes [`csr19::W`](W) writer structure"]
impl crate::Writable for CSR19rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR19 to value 0"]
impl crate::Resettable for CSR19rs {
    const RESET_VALUE: u32 = 0;
}
