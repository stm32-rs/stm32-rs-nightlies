///Register `IIDR` reader
pub type R = crate::R<IIDRrs>;
///Field `IIDR` reader - IIDR
pub type IIDR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - IIDR
    #[inline(always)]
    pub fn iidr(&self) -> IIDR_R {
        IIDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IIDR").field("iidr", &self.iidr()).finish()
    }
}
/**The GICV_IIDR is an alias of GICC_IIDR.

You can [`read`](crate::Reg::read) this register and get [`iidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICV:IIDR)*/
pub struct IIDRrs;
impl crate::RegisterSpec for IIDRrs {
    type Ux = u32;
}
///`read()` method returns [`iidr::R`](R) reader structure
impl crate::Readable for IIDRrs {}
///`reset()` method sets IIDR to value 0x0102_143b
impl crate::Resettable for IIDRrs {
    const RESET_VALUE: u32 = 0x0102_143b;
}
