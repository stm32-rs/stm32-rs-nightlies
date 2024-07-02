///Register `GICV_IAR` reader
pub type R = crate::R<GICV_IARrs>;
///Field `INTERRUPT_ID` reader - INTERRUPT_ID
pub type INTERRUPT_ID_R = crate::FieldReader<u16>;
///Field `CPUID` reader - CPUID
pub type CPUID_R = crate::BitReader;
impl R {
    ///Bits 0:9 - INTERRUPT_ID
    #[inline(always)]
    pub fn interrupt_id(&self) -> INTERRUPT_ID_R {
        INTERRUPT_ID_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 10 - CPUID
    #[inline(always)]
    pub fn cpuid(&self) -> CPUID_R {
        CPUID_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICV_IAR")
            .field("interrupt_id", &self.interrupt_id())
            .field("cpuid", &self.cpuid())
            .finish()
    }
}
/**GICV VM interrupt acknowledge register

You can [`read`](crate::Reg::read) this register and get [`gicv_iar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICV:GICV_IAR)*/
pub struct GICV_IARrs;
impl crate::RegisterSpec for GICV_IARrs {
    type Ux = u32;
}
///`read()` method returns [`gicv_iar::R`](R) reader structure
impl crate::Readable for GICV_IARrs {}
///`reset()` method sets GICV_IAR to value 0x03ff
impl crate::Resettable for GICV_IARrs {
    const RESET_VALUE: u32 = 0x03ff;
}
