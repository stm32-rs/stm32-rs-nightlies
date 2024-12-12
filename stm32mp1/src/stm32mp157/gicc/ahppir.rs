///Register `AHPPIR` reader
pub type R = crate::R<AHPPIRrs>;
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
        f.debug_struct("AHPPIR")
            .field("pendintid", &self.pendintid())
            .field("cpuid", &self.cpuid())
            .finish()
    }
}
/**ICC_AHPPIR is an alias of the non-secure GICC_HPPIR. A secure access to this register is equivalent to a non-secure access to GICC_HPPIR.

You can [`read`](crate::Reg::read) this register and get [`ahppir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICC:AHPPIR)*/
pub struct AHPPIRrs;
impl crate::RegisterSpec for AHPPIRrs {
    type Ux = u32;
}
///`read()` method returns [`ahppir::R`](R) reader structure
impl crate::Readable for AHPPIRrs {}
///`reset()` method sets AHPPIR to value 0x03ff
impl crate::Resettable for AHPPIRrs {
    const RESET_VALUE: u32 = 0x03ff;
}
