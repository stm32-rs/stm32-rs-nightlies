///Register `HASH_CSR39` reader
pub type R = crate::R<HASH_CSR39rs>;
///Register `HASH_CSR39` writer
pub type W = crate::W<HASH_CSR39rs>;
///Field `CS39` reader - CS39
pub type CS39_R = crate::FieldReader<u32>;
///Field `CS39` writer - CS39
pub type CS39_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS39
    #[inline(always)]
    pub fn cs39(&self) -> CS39_R {
        CS39_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR39")
            .field("cs39", &self.cs39())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CS39
    #[inline(always)]
    #[must_use]
    pub fn cs39(&mut self) -> CS39_W<HASH_CSR39rs> {
        CS39_W::new(self, 0)
    }
}
/**HASH context swap registers

You can [`read`](crate::Reg::read) this register and get [`hash_csr39::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr39::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HASH1:HASH_CSR39)*/
pub struct HASH_CSR39rs;
impl crate::RegisterSpec for HASH_CSR39rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr39::R`](R) reader structure
impl crate::Readable for HASH_CSR39rs {}
///`write(|w| ..)` method takes [`hash_csr39::W`](W) writer structure
impl crate::Writable for HASH_CSR39rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HASH_CSR39 to value 0
impl crate::Resettable for HASH_CSR39rs {
    const RESET_VALUE: u32 = 0;
}
