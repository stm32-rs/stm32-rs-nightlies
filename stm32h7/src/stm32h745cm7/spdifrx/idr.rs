///Register `IDR` reader
pub type R = crate::R<IDRrs>;
///Field `ID` reader - SPDIFRX identifier
pub type ID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - SPDIFRX identifier
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDR").field("id", &self.id()).finish()
    }
}
/**SPDIFRX identification register

You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#SPDIFRX:IDR)*/
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
///`read()` method returns [`idr::R`](R) reader structure
impl crate::Readable for IDRrs {}
///`reset()` method sets IDR to value 0x0013_0041
impl crate::Resettable for IDRrs {
    const RESET_VALUE: u32 = 0x0013_0041;
}
