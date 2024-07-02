///Register `GICV_AHPPIR` reader
pub type R = crate::R<GICV_AHPPIRrs>;
///Field `PENDINTID` reader - PENDINTID
pub type PENDINTID_R = crate::FieldReader<u16>;
///Field `CPUID` reader - CPUID
pub type CPUID_R = crate::BitReader;
impl R {
    ///Bits 0:9 - PENDINTID
    #[inline(always)]
    pub fn pendintid(&self) -> PENDINTID_R {
        PENDINTID_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 10 - CPUID
    #[inline(always)]
    pub fn cpuid(&self) -> CPUID_R {
        CPUID_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICV_AHPPIR")
            .field("pendintid", &self.pendintid())
            .field("cpuid", &self.cpuid())
            .finish()
    }
}
/**GICV VM aliased highest priority pending interrupt register

You can [`read`](crate::Reg::read) this register and get [`gicv_ahppir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICV:GICV_AHPPIR)*/
pub struct GICV_AHPPIRrs;
impl crate::RegisterSpec for GICV_AHPPIRrs {
    type Ux = u32;
}
///`read()` method returns [`gicv_ahppir::R`](R) reader structure
impl crate::Readable for GICV_AHPPIRrs {}
///`reset()` method sets GICV_AHPPIR to value 0x03ff
impl crate::Resettable for GICV_AHPPIRrs {
    const RESET_VALUE: u32 = 0x03ff;
}
