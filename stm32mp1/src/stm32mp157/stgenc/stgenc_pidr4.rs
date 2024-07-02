///Register `STGENC_PIDR4` reader
pub type R = crate::R<STGENC_PIDR4rs>;
///Field `DES_2` reader - DES_2
pub type DES_2_R = crate::FieldReader;
///Field `SIZE` reader - SIZE
pub type SIZE_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - DES_2
    #[inline(always)]
    pub fn des_2(&self) -> DES_2_R {
        DES_2_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - SIZE
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STGENC_PIDR4")
            .field("des_2", &self.des_2())
            .field("size", &self.size())
            .finish()
    }
}
/**STGENC peripheral ID4 register

You can [`read`](crate::Reg::read) this register and get [`stgenc_pidr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:STGENC_PIDR4)*/
pub struct STGENC_PIDR4rs;
impl crate::RegisterSpec for STGENC_PIDR4rs {
    type Ux = u32;
}
///`read()` method returns [`stgenc_pidr4::R`](R) reader structure
impl crate::Readable for STGENC_PIDR4rs {}
///`reset()` method sets STGENC_PIDR4 to value 0x04
impl crate::Resettable for STGENC_PIDR4rs {
    const RESET_VALUE: u32 = 0x04;
}
