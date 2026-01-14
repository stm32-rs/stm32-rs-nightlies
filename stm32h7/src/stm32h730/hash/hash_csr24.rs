///Register `HASH_CSR24` reader
pub type R = crate::R<HASH_CSR24rs>;
///Register `HASH_CSR24` writer
pub type W = crate::W<HASH_CSR24rs>;
///Field `CS24` reader - Context swap x
pub type CS24_R = crate::FieldReader<u32>;
///Field `CS24` writer - Context swap x
pub type CS24_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs24(&self) -> CS24_R {
        CS24_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR24")
            .field("cs24", &self.cs24())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs24(&mut self) -> CS24_W<'_, HASH_CSR24rs> {
        CS24_W::new(self, 0)
    }
}
/**HASH context swap register 24

You can [`read`](crate::Reg::read) this register and get [`hash_csr24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#HASH:HASH_CSR24)*/
pub struct HASH_CSR24rs;
impl crate::RegisterSpec for HASH_CSR24rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr24::R`](R) reader structure
impl crate::Readable for HASH_CSR24rs {}
///`write(|w| ..)` method takes [`hash_csr24::W`](W) writer structure
impl crate::Writable for HASH_CSR24rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR24 to value 0
impl crate::Resettable for HASH_CSR24rs {}
