///Register `HASH_CSR47` reader
pub type R = crate::R<HASH_CSR47rs>;
///Register `HASH_CSR47` writer
pub type W = crate::W<HASH_CSR47rs>;
///Field `CS47` reader - Context swap x
pub type CS47_R = crate::FieldReader<u32>;
///Field `CS47` writer - Context swap x
pub type CS47_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs47(&self) -> CS47_R {
        CS47_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR47")
            .field("cs47", &self.cs47())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs47(&mut self) -> CS47_W<'_, HASH_CSR47rs> {
        CS47_W::new(self, 0)
    }
}
/**HASH context swap register 47

You can [`read`](crate::Reg::read) this register and get [`hash_csr47::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr47::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#HASH:HASH_CSR47)*/
pub struct HASH_CSR47rs;
impl crate::RegisterSpec for HASH_CSR47rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr47::R`](R) reader structure
impl crate::Readable for HASH_CSR47rs {}
///`write(|w| ..)` method takes [`hash_csr47::W`](W) writer structure
impl crate::Writable for HASH_CSR47rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR47 to value 0
impl crate::Resettable for HASH_CSR47rs {}
