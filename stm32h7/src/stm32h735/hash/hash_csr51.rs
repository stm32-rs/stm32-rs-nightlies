///Register `HASH_CSR51` reader
pub type R = crate::R<HASH_CSR51rs>;
///Register `HASH_CSR51` writer
pub type W = crate::W<HASH_CSR51rs>;
///Field `CS51` reader - Context swap x
pub type CS51_R = crate::FieldReader<u32>;
///Field `CS51` writer - Context swap x
pub type CS51_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs51(&self) -> CS51_R {
        CS51_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR51")
            .field("cs51", &self.cs51())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs51(&mut self) -> CS51_W<'_, HASH_CSR51rs> {
        CS51_W::new(self, 0)
    }
}
/**HASH context swap register 51

You can [`read`](crate::Reg::read) this register and get [`hash_csr51::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr51::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#HASH:HASH_CSR51)*/
pub struct HASH_CSR51rs;
impl crate::RegisterSpec for HASH_CSR51rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr51::R`](R) reader structure
impl crate::Readable for HASH_CSR51rs {}
///`write(|w| ..)` method takes [`hash_csr51::W`](W) writer structure
impl crate::Writable for HASH_CSR51rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR51 to value 0
impl crate::Resettable for HASH_CSR51rs {}
