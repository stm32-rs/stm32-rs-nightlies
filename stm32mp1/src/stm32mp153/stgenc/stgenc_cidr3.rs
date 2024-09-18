///Register `STGENC_CIDR3` reader
pub type R = crate::R<STGENC_CIDR3rs>;
///Field `PRMBL_3` reader - PRMBL_3
pub type PRMBL_3_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PRMBL_3
    #[inline(always)]
    pub fn prmbl_3(&self) -> PRMBL_3_R {
        PRMBL_3_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STGENC_CIDR3")
            .field("prmbl_3", &self.prmbl_3())
            .finish()
    }
}
/**STGENC component ID3 register

You can [`read`](crate::Reg::read) this register and get [`stgenc_cidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_CIDR3)*/
pub struct STGENC_CIDR3rs;
impl crate::RegisterSpec for STGENC_CIDR3rs {
    type Ux = u32;
}
///`read()` method returns [`stgenc_cidr3::R`](R) reader structure
impl crate::Readable for STGENC_CIDR3rs {}
///`reset()` method sets STGENC_CIDR3 to value 0xb1
impl crate::Resettable for STGENC_CIDR3rs {
    const RESET_VALUE: u32 = 0xb1;
}
