///Register `WRP2R_CUR` reader
pub type R = crate::R<WRP2R_CURrs>;
///Field `WRPSG2` reader - Bank2 sector group protection option status byte
pub type WRPSG2_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Bank2 sector group protection option status byte
    #[inline(always)]
    pub fn wrpsg2(&self) -> WRPSG2_R {
        WRPSG2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRP2R_CUR")
            .field("wrpsg2", &self.wrpsg2())
            .finish()
    }
}
/**FLASH write sector group protection for Bank2

You can [`read`](crate::Reg::read) this register and get [`wrp2r_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:WRP2R_CUR)*/
pub struct WRP2R_CURrs;
impl crate::RegisterSpec for WRP2R_CURrs {
    type Ux = u32;
}
///`read()` method returns [`wrp2r_cur::R`](R) reader structure
impl crate::Readable for WRP2R_CURrs {}
///`reset()` method sets WRP2R_CUR to value 0
impl crate::Resettable for WRP2R_CURrs {}
