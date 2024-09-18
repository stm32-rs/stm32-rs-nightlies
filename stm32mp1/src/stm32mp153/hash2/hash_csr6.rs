///Register `HASH_CSR6` reader
pub type R = crate::R<HASH_CSR6rs>;
///Register `HASH_CSR6` writer
pub type W = crate::W<HASH_CSR6rs>;
///Field `CS6` reader - CS6
pub type CS6_R = crate::FieldReader<u32>;
///Field `CS6` writer - CS6
pub type CS6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS6
    #[inline(always)]
    pub fn cs6(&self) -> CS6_R {
        CS6_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR6")
            .field("cs6", &self.cs6())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CS6
    #[inline(always)]
    #[must_use]
    pub fn cs6(&mut self) -> CS6_W<HASH_CSR6rs> {
        CS6_W::new(self, 0)
    }
}
/**HASH context swap registers

You can [`read`](crate::Reg::read) this register and get [`hash_csr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HASH2:HASH_CSR6)*/
pub struct HASH_CSR6rs;
impl crate::RegisterSpec for HASH_CSR6rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr6::R`](R) reader structure
impl crate::Readable for HASH_CSR6rs {}
///`write(|w| ..)` method takes [`hash_csr6::W`](W) writer structure
impl crate::Writable for HASH_CSR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HASH_CSR6 to value 0
impl crate::Resettable for HASH_CSR6rs {
    const RESET_VALUE: u32 = 0;
}
