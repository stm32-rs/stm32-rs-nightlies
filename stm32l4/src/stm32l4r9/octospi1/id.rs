///Register `ID` reader
pub type R = crate::R<IDrs>;
///Field `ID` reader - Identification
pub type ID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Identification
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID").field("id", &self.id()).finish()
    }
}
/**identification

You can [`read`](crate::Reg::read) this register and get [`id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#OCTOSPI1:ID)*/
pub struct IDrs;
impl crate::RegisterSpec for IDrs {
    type Ux = u32;
}
///`read()` method returns [`id::R`](R) reader structure
impl crate::Readable for IDrs {}
///`reset()` method sets ID to value 0x0014_0041
impl crate::Resettable for IDrs {
    const RESET_VALUE: u32 = 0x0014_0041;
}
