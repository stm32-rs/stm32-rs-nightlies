///Register `ITARGETSR6` reader
pub type R = crate::R<ITARGETSR6rs>;
///Field `CPU_TARGETS0` reader - CPU_TARGETS0
pub type CPU_TARGETS0_R = crate::FieldReader;
///Field `CPU_TARGETS1` reader - CPU_TARGETS1
pub type CPU_TARGETS1_R = crate::FieldReader;
///Field `CPU_TARGETS2` reader - CPU_TARGETS2
pub type CPU_TARGETS2_R = crate::FieldReader;
///Field `CPU_TARGETS3` reader - CPU_TARGETS3
pub type CPU_TARGETS3_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - CPU_TARGETS0
    #[inline(always)]
    pub fn cpu_targets0(&self) -> CPU_TARGETS0_R {
        CPU_TARGETS0_R::new((self.bits & 3) as u8)
    }
    ///Bits 8:9 - CPU_TARGETS1
    #[inline(always)]
    pub fn cpu_targets1(&self) -> CPU_TARGETS1_R {
        CPU_TARGETS1_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:17 - CPU_TARGETS2
    #[inline(always)]
    pub fn cpu_targets2(&self) -> CPU_TARGETS2_R {
        CPU_TARGETS2_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 24:25 - CPU_TARGETS3
    #[inline(always)]
    pub fn cpu_targets3(&self) -> CPU_TARGETS3_R {
        CPU_TARGETS3_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITARGETSR6")
            .field("cpu_targets0", &self.cpu_targets0())
            .field("cpu_targets1", &self.cpu_targets1())
            .field("cpu_targets2", &self.cpu_targets2())
            .field("cpu_targets3", &self.cpu_targets3())
            .finish()
    }
}
/**For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.

You can [`read`](crate::Reg::read) this register and get [`itargetsr6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR6)*/
pub struct ITARGETSR6rs;
impl crate::RegisterSpec for ITARGETSR6rs {
    type Ux = u32;
}
///`read()` method returns [`itargetsr6::R`](R) reader structure
impl crate::Readable for ITARGETSR6rs {}
///`reset()` method sets ITARGETSR6 to value 0
impl crate::Resettable for ITARGETSR6rs {}
