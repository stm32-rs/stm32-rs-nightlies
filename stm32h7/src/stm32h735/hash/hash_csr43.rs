///Register `HASH_CSR43` reader
pub type R = crate::R<HASH_CSR43rs>;
///Register `HASH_CSR43` writer
pub type W = crate::W<HASH_CSR43rs>;
///Field `CS43` reader - Context swap x
pub type CS43_R = crate::FieldReader<u32>;
///Field `CS43` writer - Context swap x
pub type CS43_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs43(&self) -> CS43_R {
        CS43_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR43")
            .field("cs43", &self.cs43())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs43(&mut self) -> CS43_W<'_, HASH_CSR43rs> {
        CS43_W::new(self, 0)
    }
}
/**HASH context swap register 43

You can [`read`](crate::Reg::read) this register and get [`hash_csr43::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr43::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#HASH:HASH_CSR43)*/
pub struct HASH_CSR43rs;
impl crate::RegisterSpec for HASH_CSR43rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr43::R`](R) reader structure
impl crate::Readable for HASH_CSR43rs {}
///`write(|w| ..)` method takes [`hash_csr43::W`](W) writer structure
impl crate::Writable for HASH_CSR43rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR43 to value 0
impl crate::Resettable for HASH_CSR43rs {}
