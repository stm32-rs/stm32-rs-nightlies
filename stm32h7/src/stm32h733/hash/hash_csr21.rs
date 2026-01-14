///Register `HASH_CSR21` reader
pub type R = crate::R<HASH_CSR21rs>;
///Register `HASH_CSR21` writer
pub type W = crate::W<HASH_CSR21rs>;
///Field `CS21` reader - Context swap x
pub type CS21_R = crate::FieldReader<u32>;
///Field `CS21` writer - Context swap x
pub type CS21_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs21(&self) -> CS21_R {
        CS21_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR21")
            .field("cs21", &self.cs21())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs21(&mut self) -> CS21_W<'_, HASH_CSR21rs> {
        CS21_W::new(self, 0)
    }
}
/**HASH context swap register 21

You can [`read`](crate::Reg::read) this register and get [`hash_csr21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#HASH:HASH_CSR21)*/
pub struct HASH_CSR21rs;
impl crate::RegisterSpec for HASH_CSR21rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr21::R`](R) reader structure
impl crate::Readable for HASH_CSR21rs {}
///`write(|w| ..)` method takes [`hash_csr21::W`](W) writer structure
impl crate::Writable for HASH_CSR21rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR21 to value 0
impl crate::Resettable for HASH_CSR21rs {}
