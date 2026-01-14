///Register `WRP1R_CUR` reader
pub type R = crate::R<WRP1R_CURrs>;
///Field `WRPSG1` reader - Bank1 sector group protection option status byte
pub type WRPSG1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Bank1 sector group protection option status byte
    #[inline(always)]
    pub fn wrpsg1(&self) -> WRPSG1_R {
        WRPSG1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRP1R_CUR")
            .field("wrpsg1", &self.wrpsg1())
            .finish()
    }
}
/**FLASH write sector group protection for Bank1

You can [`read`](crate::Reg::read) this register and get [`wrp1r_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#FLASH:WRP1R_CUR)*/
pub struct WRP1R_CURrs;
impl crate::RegisterSpec for WRP1R_CURrs {
    type Ux = u32;
}
///`read()` method returns [`wrp1r_cur::R`](R) reader structure
impl crate::Readable for WRP1R_CURrs {}
///`reset()` method sets WRP1R_CUR to value 0
impl crate::Resettable for WRP1R_CURrs {}
