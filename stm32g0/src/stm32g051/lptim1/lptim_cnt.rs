///Register `LPTIM_CNT` reader
pub type R = crate::R<LPTIM_CNTrs>;
///Field `CNT` reader - Counter value
pub type CNT_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_CNT")
            .field("cnt", &self.cnt())
            .finish()
    }
}
/**Counter Register

You can [`read`](crate::Reg::read) this register and get [`lptim_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#LPTIM1:LPTIM_CNT)*/
pub struct LPTIM_CNTrs;
impl crate::RegisterSpec for LPTIM_CNTrs {
    type Ux = u32;
}
///`read()` method returns [`lptim_cnt::R`](R) reader structure
impl crate::Readable for LPTIM_CNTrs {}
///`reset()` method sets LPTIM_CNT to value 0
impl crate::Resettable for LPTIM_CNTrs {
    const RESET_VALUE: u32 = 0;
}
