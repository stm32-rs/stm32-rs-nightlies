#[doc = "Register `GICD_ITARGETSR3` reader"]
pub type R = crate::R<GICD_ITARGETSR3rs>;
#[doc = "Field `CPU_TARGETS0` reader - CPU_TARGETS0"]
pub type CPU_TARGETS0_R = crate::FieldReader;
#[doc = "Field `CPU_TARGETS1` reader - CPU_TARGETS1"]
pub type CPU_TARGETS1_R = crate::FieldReader;
#[doc = "Field `CPU_TARGETS2` reader - CPU_TARGETS2"]
pub type CPU_TARGETS2_R = crate::FieldReader;
#[doc = "Field `CPU_TARGETS3` reader - CPU_TARGETS3"]
pub type CPU_TARGETS3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - CPU_TARGETS0"]
    #[inline(always)]
    pub fn cpu_targets0(&self) -> CPU_TARGETS0_R {
        CPU_TARGETS0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - CPU_TARGETS1"]
    #[inline(always)]
    pub fn cpu_targets1(&self) -> CPU_TARGETS1_R {
        CPU_TARGETS1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - CPU_TARGETS2"]
    #[inline(always)]
    pub fn cpu_targets2(&self) -> CPU_TARGETS2_R {
        CPU_TARGETS2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - CPU_TARGETS3"]
    #[inline(always)]
    pub fn cpu_targets3(&self) -> CPU_TARGETS3_R {
        CPU_TARGETS3_R::new(((self.bits >> 24) & 3) as u8)
    }
}
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR3rs;
impl crate::RegisterSpec for GICD_ITARGETSR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr3::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR3rs {}
#[doc = "`reset()` method sets GICD_ITARGETSR3 to value 0"]
impl crate::Resettable for GICD_ITARGETSR3rs {
    const RESET_VALUE: u32 = 0;
}
