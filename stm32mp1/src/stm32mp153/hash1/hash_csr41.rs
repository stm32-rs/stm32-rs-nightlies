///Register `HASH_CSR41` reader
pub type R = crate::R<HASH_CSR41rs>;
///Register `HASH_CSR41` writer
pub type W = crate::W<HASH_CSR41rs>;
///Field `CS41` reader - CS41
pub type CS41_R = crate::FieldReader<u32>;
///Field `CS41` writer - CS41
pub type CS41_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS41
    #[inline(always)]
    pub fn cs41(&self) -> CS41_R {
        CS41_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR41")
            .field("cs41", &self.cs41())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CS41
    #[inline(always)]
    #[must_use]
    pub fn cs41(&mut self) -> CS41_W<HASH_CSR41rs> {
        CS41_W::new(self, 0)
    }
}
/**HASH context swap registers

You can [`read`](crate::Reg::read) this register and get [`hash_csr41::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr41::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HASH1:HASH_CSR41)*/
pub struct HASH_CSR41rs;
impl crate::RegisterSpec for HASH_CSR41rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr41::R`](R) reader structure
impl crate::Readable for HASH_CSR41rs {}
///`write(|w| ..)` method takes [`hash_csr41::W`](W) writer structure
impl crate::Writable for HASH_CSR41rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HASH_CSR41 to value 0
impl crate::Resettable for HASH_CSR41rs {
    const RESET_VALUE: u32 = 0;
}
