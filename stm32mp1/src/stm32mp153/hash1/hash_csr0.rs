#[doc = "Register `HASH_CSR0` reader"]
pub type R = crate::R<HASH_CSR0rs>;
#[doc = "Register `HASH_CSR0` writer"]
pub type W = crate::W<HASH_CSR0rs>;
#[doc = "Field `CS0` reader - CS0"]
pub type CS0_R = crate::FieldReader<u32>;
#[doc = "Field `CS0` writer - CS0"]
pub type CS0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS0"]
    #[inline(always)]
    pub fn cs0(&self) -> CS0_R {
        CS0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS0"]
    #[inline(always)]
    #[must_use]
    pub fn cs0(&mut self) -> CS0_W<HASH_CSR0rs> {
        CS0_W::new(self, 0)
    }
}
#[doc = "These registers contain the complete internal register states of the hash processor. They are useful when a context swap has to be done because a high-priority task needs to use the hash processor while it is already used by another task. When such an event occurs, the HASH_CSRx registers have to be read and the read values have to be saved in the system memory space. Then the hash processor can be used by the preemptive task, and when the hash computation is complete, the saved context can be read from memory and written back into the HASH_CSRx registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR0rs;
impl crate::RegisterSpec for HASH_CSR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr0::R`](R) reader structure"]
impl crate::Readable for HASH_CSR0rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr0::W`](W) writer structure"]
impl crate::Writable for HASH_CSR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR0 to value 0x02"]
impl crate::Resettable for HASH_CSR0rs {
    const RESET_VALUE: u32 = 0x02;
}
