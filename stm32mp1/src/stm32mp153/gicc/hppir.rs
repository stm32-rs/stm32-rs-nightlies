///Register `HPPIR` reader
pub type R = crate::R<HPPIRrs>;
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
        f.debug_struct("HPPIR")
            .field("pendintid", &self.pendintid())
            .field("cpuid", &self.cpuid())
            .finish()
    }
}
/**GICC highest priority pending interrupt register

You can [`read`](crate::Reg::read) this register and get [`hppir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:HPPIR)*/
pub struct HPPIRrs;
impl crate::RegisterSpec for HPPIRrs {
    type Ux = u32;
}
///`read()` method returns [`hppir::R`](R) reader structure
impl crate::Readable for HPPIRrs {}
///`reset()` method sets HPPIR to value 0x03ff
impl crate::Resettable for HPPIRrs {
    const RESET_VALUE: u32 = 0x03ff;
}
