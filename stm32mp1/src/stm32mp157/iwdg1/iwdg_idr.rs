///Register `IWDG_IDR` reader
pub type R = crate::R<IWDG_IDRrs>;
///Field `ID` reader - ID
pub type ID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ID
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG_IDR").field("id", &self.id()).finish()
    }
}
/**IWDG identification register

You can [`read`](crate::Reg::read) this register and get [`iwdg_idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IWDG1:IWDG_IDR)*/
pub struct IWDG_IDRrs;
impl crate::RegisterSpec for IWDG_IDRrs {
    type Ux = u32;
}
///`read()` method returns [`iwdg_idr::R`](R) reader structure
impl crate::Readable for IWDG_IDRrs {}
///`reset()` method sets IWDG_IDR to value 0x0012_0041
impl crate::Resettable for IWDG_IDRrs {
    const RESET_VALUE: u32 = 0x0012_0041;
}
