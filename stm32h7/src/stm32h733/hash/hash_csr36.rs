///Register `HASH_CSR36` reader
pub type R = crate::R<HASH_CSR36rs>;
///Register `HASH_CSR36` writer
pub type W = crate::W<HASH_CSR36rs>;
///Field `CS36` reader - Context swap x
pub type CS36_R = crate::FieldReader<u32>;
///Field `CS36` writer - Context swap x
pub type CS36_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs36(&self) -> CS36_R {
        CS36_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR36")
            .field("cs36", &self.cs36())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs36(&mut self) -> CS36_W<'_, HASH_CSR36rs> {
        CS36_W::new(self, 0)
    }
}
/**HASH context swap register 36

You can [`read`](crate::Reg::read) this register and get [`hash_csr36::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr36::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#HASH:HASH_CSR36)*/
pub struct HASH_CSR36rs;
impl crate::RegisterSpec for HASH_CSR36rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr36::R`](R) reader structure
impl crate::Readable for HASH_CSR36rs {}
///`write(|w| ..)` method takes [`hash_csr36::W`](W) writer structure
impl crate::Writable for HASH_CSR36rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR36 to value 0
impl crate::Resettable for HASH_CSR36rs {}
