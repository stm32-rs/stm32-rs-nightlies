///Register `HASH_CSR13` reader
pub type R = crate::R<HASH_CSR13rs>;
///Register `HASH_CSR13` writer
pub type W = crate::W<HASH_CSR13rs>;
///Field `CS13` reader - CS13
pub type CS13_R = crate::FieldReader<u32>;
///Field `CS13` writer - CS13
pub type CS13_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS13
    #[inline(always)]
    pub fn cs13(&self) -> CS13_R {
        CS13_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR13")
            .field("cs13", &self.cs13())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CS13
    #[inline(always)]
    #[must_use]
    pub fn cs13(&mut self) -> CS13_W<HASH_CSR13rs> {
        CS13_W::new(self, 0)
    }
}
/**HASH context swap registers

You can [`read`](crate::Reg::read) this register and get [`hash_csr13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HASH1:HASH_CSR13)*/
pub struct HASH_CSR13rs;
impl crate::RegisterSpec for HASH_CSR13rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr13::R`](R) reader structure
impl crate::Readable for HASH_CSR13rs {}
///`write(|w| ..)` method takes [`hash_csr13::W`](W) writer structure
impl crate::Writable for HASH_CSR13rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HASH_CSR13 to value 0
impl crate::Resettable for HASH_CSR13rs {
    const RESET_VALUE: u32 = 0;
}
