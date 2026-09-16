///Register `HASH_CSR7` reader
pub type R = crate::R<HASH_CSR7rs>;
///Register `HASH_CSR7` writer
pub type W = crate::W<HASH_CSR7rs>;
///Field `CS7` reader - Context swap x
pub type CS7_R = crate::FieldReader<u32>;
///Field `CS7` writer - Context swap x
pub type CS7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs7(&self) -> CS7_R {
        CS7_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR7")
            .field("cs7", &self.cs7())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs7(&mut self) -> CS7_W<'_, HASH_CSR7rs> {
        CS7_W::new(self, 0)
    }
}
/**HASH context swap register 7

You can [`read`](crate::Reg::read) this register and get [`hash_csr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#HASH:HASH_CSR7)*/
pub struct HASH_CSR7rs;
impl crate::RegisterSpec for HASH_CSR7rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr7::R`](R) reader structure
impl crate::Readable for HASH_CSR7rs {}
///`write(|w| ..)` method takes [`hash_csr7::W`](W) writer structure
impl crate::Writable for HASH_CSR7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR7 to value 0
impl crate::Resettable for HASH_CSR7rs {}
