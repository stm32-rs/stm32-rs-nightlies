///Register `HASH_VERR` reader
pub type R = crate::R<HASH_VERRrs>;
///Field `VER` reader - VER
pub type VER_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - VER
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_VERR")
            .field("ver", &self.ver())
            .finish()
    }
}
/**HASH Version Register

You can [`read`](crate::Reg::read) this register and get [`hash_verr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HASH2:HASH_VERR)*/
pub struct HASH_VERRrs;
impl crate::RegisterSpec for HASH_VERRrs {
    type Ux = u32;
}
///`read()` method returns [`hash_verr::R`](R) reader structure
impl crate::Readable for HASH_VERRrs {}
///`reset()` method sets HASH_VERR to value 0x23
impl crate::Resettable for HASH_VERRrs {
    const RESET_VALUE: u32 = 0x23;
}
