///Register `HASH_CSR11` reader
pub type R = crate::R<HASH_CSR11rs>;
///Register `HASH_CSR11` writer
pub type W = crate::W<HASH_CSR11rs>;
///Field `CS11` reader - Context swap x
pub type CS11_R = crate::FieldReader<u32>;
///Field `CS11` writer - Context swap x
pub type CS11_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs11(&self) -> CS11_R {
        CS11_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR11")
            .field("cs11", &self.cs11())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs11(&mut self) -> CS11_W<'_, HASH_CSR11rs> {
        CS11_W::new(self, 0)
    }
}
/**HASH context swap register 11

You can [`read`](crate::Reg::read) this register and get [`hash_csr11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#HASH:HASH_CSR11)*/
pub struct HASH_CSR11rs;
impl crate::RegisterSpec for HASH_CSR11rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr11::R`](R) reader structure
impl crate::Readable for HASH_CSR11rs {}
///`write(|w| ..)` method takes [`hash_csr11::W`](W) writer structure
impl crate::Writable for HASH_CSR11rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR11 to value 0
impl crate::Resettable for HASH_CSR11rs {}
