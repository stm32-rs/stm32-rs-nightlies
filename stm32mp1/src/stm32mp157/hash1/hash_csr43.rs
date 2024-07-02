///Register `HASH_CSR43` reader
pub type R = crate::R<HASH_CSR43rs>;
///Register `HASH_CSR43` writer
pub type W = crate::W<HASH_CSR43rs>;
///Field `CS43` reader - CS43
pub type CS43_R = crate::FieldReader<u32>;
///Field `CS43` writer - CS43
pub type CS43_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS43
    #[inline(always)]
    pub fn cs43(&self) -> CS43_R {
        CS43_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR43")
            .field("cs43", &self.cs43())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CS43
    #[inline(always)]
    #[must_use]
    pub fn cs43(&mut self) -> CS43_W<HASH_CSR43rs> {
        CS43_W::new(self, 0)
    }
}
/**HASH context swap registers

You can [`read`](crate::Reg::read) this register and get [`hash_csr43::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr43::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HASH1:HASH_CSR43)*/
pub struct HASH_CSR43rs;
impl crate::RegisterSpec for HASH_CSR43rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr43::R`](R) reader structure
impl crate::Readable for HASH_CSR43rs {}
///`write(|w| ..)` method takes [`hash_csr43::W`](W) writer structure
impl crate::Writable for HASH_CSR43rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HASH_CSR43 to value 0
impl crate::Resettable for HASH_CSR43rs {
    const RESET_VALUE: u32 = 0;
}
