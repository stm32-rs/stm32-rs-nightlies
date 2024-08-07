///Register `GICV_RPR` reader
pub type R = crate::R<GICV_RPRrs>;
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
        f.debug_struct("GICV_RPR")
            .field("priority", &self.priority())
            .finish()
    }
}
/**GICV VM running priority register

You can [`read`](crate::Reg::read) this register and get [`gicv_rpr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICV:GICV_RPR)*/
pub struct GICV_RPRrs;
impl crate::RegisterSpec for GICV_RPRrs {
    type Ux = u32;
}
///`read()` method returns [`gicv_rpr::R`](R) reader structure
impl crate::Readable for GICV_RPRrs {}
///`reset()` method sets GICV_RPR to value 0xff
impl crate::Resettable for GICV_RPRrs {
    const RESET_VALUE: u32 = 0xff;
}
