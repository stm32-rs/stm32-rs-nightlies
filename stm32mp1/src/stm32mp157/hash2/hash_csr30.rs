///Register `HASH_CSR30` reader
pub type R = crate::R<HASH_CSR30rs>;
///Register `HASH_CSR30` writer
pub type W = crate::W<HASH_CSR30rs>;
///Field `CS30` reader - CS30
pub type CS30_R = crate::FieldReader<u32>;
///Field `CS30` writer - CS30
pub type CS30_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS30
    #[inline(always)]
    pub fn cs30(&self) -> CS30_R {
        CS30_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR30")
            .field("cs30", &self.cs30())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CS30
    #[inline(always)]
    #[must_use]
    pub fn cs30(&mut self) -> CS30_W<HASH_CSR30rs> {
        CS30_W::new(self, 0)
    }
}
/**HASH context swap registers

You can [`read`](crate::Reg::read) this register and get [`hash_csr30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HASH2:HASH_CSR30)*/
pub struct HASH_CSR30rs;
impl crate::RegisterSpec for HASH_CSR30rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr30::R`](R) reader structure
impl crate::Readable for HASH_CSR30rs {}
///`write(|w| ..)` method takes [`hash_csr30::W`](W) writer structure
impl crate::Writable for HASH_CSR30rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HASH_CSR30 to value 0
impl crate::Resettable for HASH_CSR30rs {
    const RESET_VALUE: u32 = 0;
}
