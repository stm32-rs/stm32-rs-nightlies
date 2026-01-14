///Register `HASH_CSR49` reader
pub type R = crate::R<HASH_CSR49rs>;
///Register `HASH_CSR49` writer
pub type W = crate::W<HASH_CSR49rs>;
///Field `CS49` reader - Context swap x
pub type CS49_R = crate::FieldReader<u32>;
///Field `CS49` writer - Context swap x
pub type CS49_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs49(&self) -> CS49_R {
        CS49_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR49")
            .field("cs49", &self.cs49())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs49(&mut self) -> CS49_W<'_, HASH_CSR49rs> {
        CS49_W::new(self, 0)
    }
}
/**HASH context swap register 49

You can [`read`](crate::Reg::read) this register and get [`hash_csr49::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr49::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#HASH:HASH_CSR49)*/
pub struct HASH_CSR49rs;
impl crate::RegisterSpec for HASH_CSR49rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr49::R`](R) reader structure
impl crate::Readable for HASH_CSR49rs {}
///`write(|w| ..)` method takes [`hash_csr49::W`](W) writer structure
impl crate::Writable for HASH_CSR49rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR49 to value 0
impl crate::Resettable for HASH_CSR49rs {}
