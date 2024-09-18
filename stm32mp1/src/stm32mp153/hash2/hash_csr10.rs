///Register `HASH_CSR10` reader
pub type R = crate::R<HASH_CSR10rs>;
///Register `HASH_CSR10` writer
pub type W = crate::W<HASH_CSR10rs>;
///Field `CS10` reader - CS10
pub type CS10_R = crate::FieldReader<u32>;
///Field `CS10` writer - CS10
pub type CS10_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS10
    #[inline(always)]
    pub fn cs10(&self) -> CS10_R {
        CS10_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR10")
            .field("cs10", &self.cs10())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CS10
    #[inline(always)]
    #[must_use]
    pub fn cs10(&mut self) -> CS10_W<HASH_CSR10rs> {
        CS10_W::new(self, 0)
    }
}
/**HASH context swap registers

You can [`read`](crate::Reg::read) this register and get [`hash_csr10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HASH2:HASH_CSR10)*/
pub struct HASH_CSR10rs;
impl crate::RegisterSpec for HASH_CSR10rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr10::R`](R) reader structure
impl crate::Readable for HASH_CSR10rs {}
///`write(|w| ..)` method takes [`hash_csr10::W`](W) writer structure
impl crate::Writable for HASH_CSR10rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HASH_CSR10 to value 0
impl crate::Resettable for HASH_CSR10rs {
    const RESET_VALUE: u32 = 0;
}
