///Register `HASH_CSR18` reader
pub type R = crate::R<HASH_CSR18rs>;
///Register `HASH_CSR18` writer
pub type W = crate::W<HASH_CSR18rs>;
///Field `CS18` reader - Context swap x
pub type CS18_R = crate::FieldReader<u32>;
///Field `CS18` writer - Context swap x
pub type CS18_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs18(&self) -> CS18_R {
        CS18_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR18")
            .field("cs18", &self.cs18())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs18(&mut self) -> CS18_W<'_, HASH_CSR18rs> {
        CS18_W::new(self, 0)
    }
}
/**HASH context swap register 18

You can [`read`](crate::Reg::read) this register and get [`hash_csr18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#HASH:HASH_CSR18)*/
pub struct HASH_CSR18rs;
impl crate::RegisterSpec for HASH_CSR18rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr18::R`](R) reader structure
impl crate::Readable for HASH_CSR18rs {}
///`write(|w| ..)` method takes [`hash_csr18::W`](W) writer structure
impl crate::Writable for HASH_CSR18rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR18 to value 0
impl crate::Resettable for HASH_CSR18rs {}
