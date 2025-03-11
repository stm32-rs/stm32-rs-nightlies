///Register `RPR` reader
pub type R = crate::R<RPRrs>;
///Field `PRIORITY` reader - PRIORITY
pub type PRIORITY_R = crate::FieldReader;
impl R {
    ///Bits 3:7 - PRIORITY
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPR")
            .field("priority", &self.priority())
            .finish()
    }
}
/**GICV VM running priority register

You can [`read`](crate::Reg::read) this register and get [`rpr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:RPR)*/
pub struct RPRrs;
impl crate::RegisterSpec for RPRrs {
    type Ux = u32;
}
///`read()` method returns [`rpr::R`](R) reader structure
impl crate::Readable for RPRrs {}
///`reset()` method sets RPR to value 0xff
impl crate::Resettable for RPRrs {
    const RESET_VALUE: u32 = 0xff;
}
