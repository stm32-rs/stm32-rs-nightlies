///Register `FMC_BCHPBR3` reader
pub type R = crate::R<FMC_BCHPBR3rs>;
///Field `BCHPB` reader - BCHPB
pub type BCHPB_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - BCHPB
    #[inline(always)]
    pub fn bchpb(&self) -> BCHPB_R {
        BCHPB_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMC_BCHPBR3")
            .field("bchpb", &self.bchpb())
            .finish()
    }
}
/**FMC BCH Parity Bits Register 3

You can [`read`](crate::Reg::read) this register and get [`fmc_bchpbr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:FMC_BCHPBR3)*/
pub struct FMC_BCHPBR3rs;
impl crate::RegisterSpec for FMC_BCHPBR3rs {
    type Ux = u32;
}
///`read()` method returns [`fmc_bchpbr3::R`](R) reader structure
impl crate::Readable for FMC_BCHPBR3rs {}
///`reset()` method sets FMC_BCHPBR3 to value 0
impl crate::Resettable for FMC_BCHPBR3rs {
    const RESET_VALUE: u32 = 0;
}
