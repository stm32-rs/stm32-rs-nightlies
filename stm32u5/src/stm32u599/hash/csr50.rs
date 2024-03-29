#[doc = "Register `CSR50` reader"]
pub type R = crate::R<CSR50rs>;
#[doc = "Register `CSR50` writer"]
pub type W = crate::W<CSR50rs>;
#[doc = "Field `CS50` reader - CS50"]
pub type CS50_R = crate::FieldReader<u32>;
#[doc = "Field `CS50` writer - CS50"]
pub type CS50_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS50"]
    #[inline(always)]
    pub fn cs50(&self) -> CS50_R {
        CS50_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS50"]
    #[inline(always)]
    #[must_use]
    pub fn cs50(&mut self) -> CS50_W<CSR50rs> {
        CS50_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr50::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr50::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR50rs;
impl crate::RegisterSpec for CSR50rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr50::R`](R) reader structure"]
impl crate::Readable for CSR50rs {}
#[doc = "`write(|w| ..)` method takes [`csr50::W`](W) writer structure"]
impl crate::Writable for CSR50rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR50 to value 0"]
impl crate::Resettable for CSR50rs {
    const RESET_VALUE: u32 = 0;
}
