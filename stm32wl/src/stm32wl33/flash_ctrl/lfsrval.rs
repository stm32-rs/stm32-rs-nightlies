///Register `LFSRVAL` reader
pub type R = crate::R<LFSRVALrs>;
///Field `LFSRVAL` reader - Flash read data CRC signature
pub type LFSRVAL_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Flash read data CRC signature
    #[inline(always)]
    pub fn lfsrval(&self) -> LFSRVAL_R {
        LFSRVAL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LFSRVAL")
            .field("lfsrval", &self.lfsrval())
            .finish()
    }
}
/**LFSRVAL register

You can [`read`](crate::Reg::read) this register and get [`lfsrval::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#FLASH_CTRL:LFSRVAL)*/
pub struct LFSRVALrs;
impl crate::RegisterSpec for LFSRVALrs {
    type Ux = u32;
}
///`read()` method returns [`lfsrval::R`](R) reader structure
impl crate::Readable for LFSRVALrs {}
///`reset()` method sets LFSRVAL to value 0xffff_ffff
impl crate::Resettable for LFSRVALrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
