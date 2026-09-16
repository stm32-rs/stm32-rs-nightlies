///Register `BCHPBR1` reader
pub type R = crate::R<BCHPBR1rs>;
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
        f.debug_struct("BCHPBR1")
            .field("bchpb", &self.bchpb())
            .finish()
    }
}
/**These registers contain the BCH parity bits (BCHPB). For the BCH 4-bit, only BCHPB\[51:0\] are significant and for the BCH 8-bit BCHPB\[103:0\] are significant.

You can [`read`](crate::Reg::read) this register and get [`bchpbr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCHPBR1)*/
pub struct BCHPBR1rs;
impl crate::RegisterSpec for BCHPBR1rs {
    type Ux = u32;
}
///`read()` method returns [`bchpbr1::R`](R) reader structure
impl crate::Readable for BCHPBR1rs {}
///`reset()` method sets BCHPBR1 to value 0
impl crate::Resettable for BCHPBR1rs {}
