///Register `HASH_CSR26` reader
pub type R = crate::R<HASH_CSR26rs>;
///Register `HASH_CSR26` writer
pub type W = crate::W<HASH_CSR26rs>;
///Field `CS26` reader - Context swap x
pub type CS26_R = crate::FieldReader<u32>;
///Field `CS26` writer - Context swap x
pub type CS26_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs26(&self) -> CS26_R {
        CS26_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR26")
            .field("cs26", &self.cs26())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs26(&mut self) -> CS26_W<'_, HASH_CSR26rs> {
        CS26_W::new(self, 0)
    }
}
/**HASH context swap register 26

You can [`read`](crate::Reg::read) this register and get [`hash_csr26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#HASH:HASH_CSR26)*/
pub struct HASH_CSR26rs;
impl crate::RegisterSpec for HASH_CSR26rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr26::R`](R) reader structure
impl crate::Readable for HASH_CSR26rs {}
///`write(|w| ..)` method takes [`hash_csr26::W`](W) writer structure
impl crate::Writable for HASH_CSR26rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR26 to value 0
impl crate::Resettable for HASH_CSR26rs {}
