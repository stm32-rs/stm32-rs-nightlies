///Register `HASH_CSR12` reader
pub type R = crate::R<HASH_CSR12rs>;
///Register `HASH_CSR12` writer
pub type W = crate::W<HASH_CSR12rs>;
///Field `CS12` reader - Context swap x
pub type CS12_R = crate::FieldReader<u32>;
///Field `CS12` writer - Context swap x
pub type CS12_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs12(&self) -> CS12_R {
        CS12_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR12")
            .field("cs12", &self.cs12())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs12(&mut self) -> CS12_W<'_, HASH_CSR12rs> {
        CS12_W::new(self, 0)
    }
}
/**HASH context swap register 12

You can [`read`](crate::Reg::read) this register and get [`hash_csr12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#HASH:HASH_CSR12)*/
pub struct HASH_CSR12rs;
impl crate::RegisterSpec for HASH_CSR12rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr12::R`](R) reader structure
impl crate::Readable for HASH_CSR12rs {}
///`write(|w| ..)` method takes [`hash_csr12::W`](W) writer structure
impl crate::Writable for HASH_CSR12rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR12 to value 0
impl crate::Resettable for HASH_CSR12rs {}
