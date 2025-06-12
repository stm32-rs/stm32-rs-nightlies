///Register `BCHPBR3` reader
pub type R = crate::R<BCHPBR3rs>;
///Field `BCHPB` reader - BCH parity bits
pub type BCHPB_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - BCH parity bits
    #[inline(always)]
    pub fn bchpb(&self) -> BCHPB_R {
        BCHPB_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCHPBR3")
            .field("bchpb", &self.bchpb())
            .finish()
    }
}
/**FMC BCH parity bits register 3

You can [`read`](crate::Reg::read) this register and get [`bchpbr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#FMC1:BCHPBR3)*/
pub struct BCHPBR3rs;
impl crate::RegisterSpec for BCHPBR3rs {
    type Ux = u32;
}
///`read()` method returns [`bchpbr3::R`](R) reader structure
impl crate::Readable for BCHPBR3rs {}
///`reset()` method sets BCHPBR3 to value 0
impl crate::Resettable for BCHPBR3rs {}
