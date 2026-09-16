///Register `BCHPBR4` reader
pub type R = crate::R<BCHPBR4rs>;
///Field `BCHPB` reader - BCH parity bits
pub type BCHPB_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - BCH parity bits
    #[inline(always)]
    pub fn bchpb(&self) -> BCHPB_R {
        BCHPB_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCHPBR4")
            .field("bchpb", &self.bchpb())
            .finish()
    }
}
/**FMC BCH parity bits register 4

You can [`read`](crate::Reg::read) this register and get [`bchpbr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FMC1:BCHPBR4)*/
pub struct BCHPBR4rs;
impl crate::RegisterSpec for BCHPBR4rs {
    type Ux = u32;
}
///`read()` method returns [`bchpbr4::R`](R) reader structure
impl crate::Readable for BCHPBR4rs {}
///`reset()` method sets BCHPBR4 to value 0
impl crate::Resettable for BCHPBR4rs {}
