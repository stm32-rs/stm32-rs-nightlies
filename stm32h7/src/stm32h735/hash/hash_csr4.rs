///Register `HASH_CSR4` reader
pub type R = crate::R<HASH_CSR4rs>;
///Register `HASH_CSR4` writer
pub type W = crate::W<HASH_CSR4rs>;
///Field `CS4` reader - Context swap x
pub type CS4_R = crate::FieldReader<u32>;
///Field `CS4` writer - Context swap x
pub type CS4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs4(&self) -> CS4_R {
        CS4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR4")
            .field("cs4", &self.cs4())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs4(&mut self) -> CS4_W<'_, HASH_CSR4rs> {
        CS4_W::new(self, 0)
    }
}
/**HASH context swap register 4

You can [`read`](crate::Reg::read) this register and get [`hash_csr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#HASH:HASH_CSR4)*/
pub struct HASH_CSR4rs;
impl crate::RegisterSpec for HASH_CSR4rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr4::R`](R) reader structure
impl crate::Readable for HASH_CSR4rs {}
///`write(|w| ..)` method takes [`hash_csr4::W`](W) writer structure
impl crate::Writable for HASH_CSR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR4 to value 0
impl crate::Resettable for HASH_CSR4rs {}
