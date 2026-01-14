///Register `HASH_CSR28` reader
pub type R = crate::R<HASH_CSR28rs>;
///Register `HASH_CSR28` writer
pub type W = crate::W<HASH_CSR28rs>;
///Field `CS28` reader - Context swap x
pub type CS28_R = crate::FieldReader<u32>;
///Field `CS28` writer - Context swap x
pub type CS28_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs28(&self) -> CS28_R {
        CS28_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR28")
            .field("cs28", &self.cs28())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs28(&mut self) -> CS28_W<'_, HASH_CSR28rs> {
        CS28_W::new(self, 0)
    }
}
/**HASH context swap register 28

You can [`read`](crate::Reg::read) this register and get [`hash_csr28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#HASH:HASH_CSR28)*/
pub struct HASH_CSR28rs;
impl crate::RegisterSpec for HASH_CSR28rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr28::R`](R) reader structure
impl crate::Readable for HASH_CSR28rs {}
///`write(|w| ..)` method takes [`hash_csr28::W`](W) writer structure
impl crate::Writable for HASH_CSR28rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR28 to value 0
impl crate::Resettable for HASH_CSR28rs {}
