///Register `HASH_CSR40` reader
pub type R = crate::R<HASH_CSR40rs>;
///Register `HASH_CSR40` writer
pub type W = crate::W<HASH_CSR40rs>;
///Field `CS40` reader - Context swap x
pub type CS40_R = crate::FieldReader<u32>;
///Field `CS40` writer - Context swap x
pub type CS40_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs40(&self) -> CS40_R {
        CS40_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR40")
            .field("cs40", &self.cs40())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs40(&mut self) -> CS40_W<'_, HASH_CSR40rs> {
        CS40_W::new(self, 0)
    }
}
/**HASH context swap register 40

You can [`read`](crate::Reg::read) this register and get [`hash_csr40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#HASH:HASH_CSR40)*/
pub struct HASH_CSR40rs;
impl crate::RegisterSpec for HASH_CSR40rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr40::R`](R) reader structure
impl crate::Readable for HASH_CSR40rs {}
///`write(|w| ..)` method takes [`hash_csr40::W`](W) writer structure
impl crate::Writable for HASH_CSR40rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR40 to value 0
impl crate::Resettable for HASH_CSR40rs {}
