///Register `HASH_CSR25` reader
pub type R = crate::R<HASH_CSR25rs>;
///Register `HASH_CSR25` writer
pub type W = crate::W<HASH_CSR25rs>;
///Field `CS25` reader - Context swap x
pub type CS25_R = crate::FieldReader<u32>;
///Field `CS25` writer - Context swap x
pub type CS25_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs25(&self) -> CS25_R {
        CS25_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR25")
            .field("cs25", &self.cs25())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs25(&mut self) -> CS25_W<'_, HASH_CSR25rs> {
        CS25_W::new(self, 0)
    }
}
/**HASH context swap register 25

You can [`read`](crate::Reg::read) this register and get [`hash_csr25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#HASH:HASH_CSR25)*/
pub struct HASH_CSR25rs;
impl crate::RegisterSpec for HASH_CSR25rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr25::R`](R) reader structure
impl crate::Readable for HASH_CSR25rs {}
///`write(|w| ..)` method takes [`hash_csr25::W`](W) writer structure
impl crate::Writable for HASH_CSR25rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR25 to value 0
impl crate::Resettable for HASH_CSR25rs {}
