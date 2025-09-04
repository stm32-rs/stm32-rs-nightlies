///Register `NSEPOCHR_CUR` reader
pub type R = crate::R<NSEPOCHR_CURrs>;
///Field `NS_EPOCH` reader - Non-volatile non-secure EPOCH counter
pub type NS_EPOCH_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:23 - Non-volatile non-secure EPOCH counter
    #[inline(always)]
    pub fn ns_epoch(&self) -> NS_EPOCH_R {
        NS_EPOCH_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NSEPOCHR_CUR")
            .field("ns_epoch", &self.ns_epoch())
            .finish()
    }
}
/**FLASH non-secure EPOCH register

You can [`read`](crate::Reg::read) this register and get [`nsepochr_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:NSEPOCHR_CUR)*/
pub struct NSEPOCHR_CURrs;
impl crate::RegisterSpec for NSEPOCHR_CURrs {
    type Ux = u32;
}
///`read()` method returns [`nsepochr_cur::R`](R) reader structure
impl crate::Readable for NSEPOCHR_CURrs {}
///`reset()` method sets NSEPOCHR_CUR to value 0
impl crate::Resettable for NSEPOCHR_CURrs {}
