///Register `HASH_CSR32` reader
pub type R = crate::R<HASH_CSR32rs>;
///Register `HASH_CSR32` writer
pub type W = crate::W<HASH_CSR32rs>;
///Field `CS32` reader - Context swap x
pub type CS32_R = crate::FieldReader<u32>;
///Field `CS32` writer - Context swap x
pub type CS32_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs32(&self) -> CS32_R {
        CS32_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR32")
            .field("cs32", &self.cs32())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs32(&mut self) -> CS32_W<'_, HASH_CSR32rs> {
        CS32_W::new(self, 0)
    }
}
/**HASH context swap register 32

You can [`read`](crate::Reg::read) this register and get [`hash_csr32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#HASH:HASH_CSR32)*/
pub struct HASH_CSR32rs;
impl crate::RegisterSpec for HASH_CSR32rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr32::R`](R) reader structure
impl crate::Readable for HASH_CSR32rs {}
///`write(|w| ..)` method takes [`hash_csr32::W`](W) writer structure
impl crate::Writable for HASH_CSR32rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR32 to value 0
impl crate::Resettable for HASH_CSR32rs {}
