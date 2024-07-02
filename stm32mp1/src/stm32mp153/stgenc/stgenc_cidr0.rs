///Register `STGENC_CIDR0` reader
pub type R = crate::R<STGENC_CIDR0rs>;
///Field `PRMBL_0` reader - PRMBL_0
pub type PRMBL_0_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PRMBL_0
    #[inline(always)]
    pub fn prmbl_0(&self) -> PRMBL_0_R {
        PRMBL_0_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STGENC_CIDR0")
            .field("prmbl_0", &self.prmbl_0())
            .finish()
    }
}
/**STGENC component ID0 register

You can [`read`](crate::Reg::read) this register and get [`stgenc_cidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_CIDR0)*/
pub struct STGENC_CIDR0rs;
impl crate::RegisterSpec for STGENC_CIDR0rs {
    type Ux = u32;
}
///`read()` method returns [`stgenc_cidr0::R`](R) reader structure
impl crate::Readable for STGENC_CIDR0rs {}
///`reset()` method sets STGENC_CIDR0 to value 0x0d
impl crate::Resettable for STGENC_CIDR0rs {
    const RESET_VALUE: u32 = 0x0d;
}
