///Register `HASH_CSR20` reader
pub type R = crate::R<HASH_CSR20rs>;
///Register `HASH_CSR20` writer
pub type W = crate::W<HASH_CSR20rs>;
///Field `CS20` reader - Context swap x
pub type CS20_R = crate::FieldReader<u32>;
///Field `CS20` writer - Context swap x
pub type CS20_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs20(&self) -> CS20_R {
        CS20_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR20")
            .field("cs20", &self.cs20())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs20(&mut self) -> CS20_W<'_, HASH_CSR20rs> {
        CS20_W::new(self, 0)
    }
}
/**HASH context swap register 20

You can [`read`](crate::Reg::read) this register and get [`hash_csr20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#HASH:HASH_CSR20)*/
pub struct HASH_CSR20rs;
impl crate::RegisterSpec for HASH_CSR20rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr20::R`](R) reader structure
impl crate::Readable for HASH_CSR20rs {}
///`write(|w| ..)` method takes [`hash_csr20::W`](W) writer structure
impl crate::Writable for HASH_CSR20rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR20 to value 0
impl crate::Resettable for HASH_CSR20rs {}
