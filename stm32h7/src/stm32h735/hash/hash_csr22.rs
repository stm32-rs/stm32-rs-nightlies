///Register `HASH_CSR22` reader
pub type R = crate::R<HASH_CSR22rs>;
///Register `HASH_CSR22` writer
pub type W = crate::W<HASH_CSR22rs>;
///Field `CS22` reader - Context swap x
pub type CS22_R = crate::FieldReader<u32>;
///Field `CS22` writer - Context swap x
pub type CS22_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs22(&self) -> CS22_R {
        CS22_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR22")
            .field("cs22", &self.cs22())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs22(&mut self) -> CS22_W<'_, HASH_CSR22rs> {
        CS22_W::new(self, 0)
    }
}
/**HASH context swap register 22

You can [`read`](crate::Reg::read) this register and get [`hash_csr22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#HASH:HASH_CSR22)*/
pub struct HASH_CSR22rs;
impl crate::RegisterSpec for HASH_CSR22rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr22::R`](R) reader structure
impl crate::Readable for HASH_CSR22rs {}
///`write(|w| ..)` method takes [`hash_csr22::W`](W) writer structure
impl crate::Writable for HASH_CSR22rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR22 to value 0
impl crate::Resettable for HASH_CSR22rs {}
