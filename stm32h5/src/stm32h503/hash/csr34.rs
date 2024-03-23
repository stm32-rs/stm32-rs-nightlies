#[doc = "Register `CSR34` reader"]
pub type R = crate::R<CSR34rs>;
#[doc = "Register `CSR34` writer"]
pub type W = crate::W<CSR34rs>;
#[doc = "Field `CS34` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS34_R = crate::FieldReader<u32>;
#[doc = "Field `CS34` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS34_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn cs34(&self) -> CS34_R {
        CS34_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    #[must_use]
    pub fn cs34(&mut self) -> CS34_W<CSR34rs> {
        CS34_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR34rs;
impl crate::RegisterSpec for CSR34rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr34::R`](R) reader structure"]
impl crate::Readable for CSR34rs {}
#[doc = "`write(|w| ..)` method takes [`csr34::W`](W) writer structure"]
impl crate::Writable for CSR34rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR34 to value 0"]
impl crate::Resettable for CSR34rs {
    const RESET_VALUE: u32 = 0;
}
