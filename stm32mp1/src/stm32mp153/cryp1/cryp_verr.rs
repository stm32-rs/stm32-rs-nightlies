///Register `CRYP_VERR` reader
pub type R = crate::R<CRYP_VERRrs>;
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
        f.debug_struct("CRYP_VERR")
            .field("ver", &self.ver())
            .finish()
    }
}
/**CRYP HW Version Register

You can [`read`](crate::Reg::read) this register and get [`cryp_verr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#CRYP1:CRYP_VERR)*/
pub struct CRYP_VERRrs;
impl crate::RegisterSpec for CRYP_VERRrs {
    type Ux = u32;
}
///`read()` method returns [`cryp_verr::R`](R) reader structure
impl crate::Readable for CRYP_VERRrs {}
///`reset()` method sets CRYP_VERR to value 0x22
impl crate::Resettable for CRYP_VERRrs {
    const RESET_VALUE: u32 = 0x22;
}
