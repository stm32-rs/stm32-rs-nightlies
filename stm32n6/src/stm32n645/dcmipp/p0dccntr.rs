///Register `P0DCCNTR` reader
pub type R = crate::R<P0DCCNTRrs>;
///Field `CNT` reader - Number of data dumped during the frame.
pub type CNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:25 - Number of data dumped during the frame.
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0DCCNTR")
            .field("cnt", &self.cnt())
            .finish()
    }
}
/**DCMIPP Pipe0 dump counter register

You can [`read`](crate::Reg::read) this register and get [`p0dccntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P0DCCNTR)*/
pub struct P0DCCNTRrs;
impl crate::RegisterSpec for P0DCCNTRrs {
    type Ux = u32;
}
///`read()` method returns [`p0dccntr::R`](R) reader structure
impl crate::Readable for P0DCCNTRrs {}
///`reset()` method sets P0DCCNTR to value 0
impl crate::Resettable for P0DCCNTRrs {}
