///Register `HASH_CSR31` reader
pub type R = crate::R<HASH_CSR31rs>;
///Register `HASH_CSR31` writer
pub type W = crate::W<HASH_CSR31rs>;
///Field `CS31` reader - Context swap x
pub type CS31_R = crate::FieldReader<u32>;
///Field `CS31` writer - Context swap x
pub type CS31_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs31(&self) -> CS31_R {
        CS31_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR31")
            .field("cs31", &self.cs31())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs31(&mut self) -> CS31_W<'_, HASH_CSR31rs> {
        CS31_W::new(self, 0)
    }
}
/**HASH context swap register 31

You can [`read`](crate::Reg::read) this register and get [`hash_csr31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#HASH:HASH_CSR31)*/
pub struct HASH_CSR31rs;
impl crate::RegisterSpec for HASH_CSR31rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr31::R`](R) reader structure
impl crate::Readable for HASH_CSR31rs {}
///`write(|w| ..)` method takes [`hash_csr31::W`](W) writer structure
impl crate::Writable for HASH_CSR31rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR31 to value 0
impl crate::Resettable for HASH_CSR31rs {}
