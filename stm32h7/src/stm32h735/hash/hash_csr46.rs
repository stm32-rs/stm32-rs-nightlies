///Register `HASH_CSR46` reader
pub type R = crate::R<HASH_CSR46rs>;
///Register `HASH_CSR46` writer
pub type W = crate::W<HASH_CSR46rs>;
///Field `CS46` reader - Context swap x
pub type CS46_R = crate::FieldReader<u32>;
///Field `CS46` writer - Context swap x
pub type CS46_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs46(&self) -> CS46_R {
        CS46_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR46")
            .field("cs46", &self.cs46())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs46(&mut self) -> CS46_W<'_, HASH_CSR46rs> {
        CS46_W::new(self, 0)
    }
}
/**HASH context swap register 46

You can [`read`](crate::Reg::read) this register and get [`hash_csr46::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr46::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#HASH:HASH_CSR46)*/
pub struct HASH_CSR46rs;
impl crate::RegisterSpec for HASH_CSR46rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr46::R`](R) reader structure
impl crate::Readable for HASH_CSR46rs {}
///`write(|w| ..)` method takes [`hash_csr46::W`](W) writer structure
impl crate::Writable for HASH_CSR46rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR46 to value 0
impl crate::Resettable for HASH_CSR46rs {}
