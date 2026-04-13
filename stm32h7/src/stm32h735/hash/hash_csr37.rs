///Register `HASH_CSR37` reader
pub type R = crate::R<HASH_CSR37rs>;
///Register `HASH_CSR37` writer
pub type W = crate::W<HASH_CSR37rs>;
///Field `CS37` reader - Context swap x
pub type CS37_R = crate::FieldReader<u32>;
///Field `CS37` writer - Context swap x
pub type CS37_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs37(&self) -> CS37_R {
        CS37_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR37")
            .field("cs37", &self.cs37())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs37(&mut self) -> CS37_W<'_, HASH_CSR37rs> {
        CS37_W::new(self, 0)
    }
}
/**HASH context swap register 37

You can [`read`](crate::Reg::read) this register and get [`hash_csr37::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr37::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#HASH:HASH_CSR37)*/
pub struct HASH_CSR37rs;
impl crate::RegisterSpec for HASH_CSR37rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr37::R`](R) reader structure
impl crate::Readable for HASH_CSR37rs {}
///`write(|w| ..)` method takes [`hash_csr37::W`](W) writer structure
impl crate::Writable for HASH_CSR37rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR37 to value 0
impl crate::Resettable for HASH_CSR37rs {}
