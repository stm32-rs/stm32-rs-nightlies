///Register `HASH_CSR5` reader
pub type R = crate::R<HASH_CSR5rs>;
///Register `HASH_CSR5` writer
pub type W = crate::W<HASH_CSR5rs>;
///Field `CS5` reader - CS5
pub type CS5_R = crate::FieldReader<u32>;
///Field `CS5` writer - CS5
pub type CS5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS5
    #[inline(always)]
    pub fn cs5(&self) -> CS5_R {
        CS5_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR5")
            .field("cs5", &self.cs5())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CS5
    #[inline(always)]
    #[must_use]
    pub fn cs5(&mut self) -> CS5_W<HASH_CSR5rs> {
        CS5_W::new(self, 0)
    }
}
/**HASH context swap registers

You can [`read`](crate::Reg::read) this register and get [`hash_csr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HASH2:HASH_CSR5)*/
pub struct HASH_CSR5rs;
impl crate::RegisterSpec for HASH_CSR5rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr5::R`](R) reader structure
impl crate::Readable for HASH_CSR5rs {}
///`write(|w| ..)` method takes [`hash_csr5::W`](W) writer structure
impl crate::Writable for HASH_CSR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HASH_CSR5 to value 0
impl crate::Resettable for HASH_CSR5rs {
    const RESET_VALUE: u32 = 0;
}
