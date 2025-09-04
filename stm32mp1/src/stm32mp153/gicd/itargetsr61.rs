///Register `ITARGETSR61` reader
pub type R = crate::R<ITARGETSR61rs>;
///Register `ITARGETSR61` writer
pub type W = crate::W<ITARGETSR61rs>;
///Field `CPU_TARGETS0` reader - CPU_TARGETS0
pub type CPU_TARGETS0_R = crate::FieldReader;
///Field `CPU_TARGETS0` writer - CPU_TARGETS0
pub type CPU_TARGETS0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CPU_TARGETS1` reader - CPU_TARGETS1
pub type CPU_TARGETS1_R = crate::FieldReader;
///Field `CPU_TARGETS1` writer - CPU_TARGETS1
pub type CPU_TARGETS1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CPU_TARGETS2` reader - CPU_TARGETS2
pub type CPU_TARGETS2_R = crate::FieldReader;
///Field `CPU_TARGETS2` writer - CPU_TARGETS2
pub type CPU_TARGETS2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CPU_TARGETS3` reader - CPU_TARGETS3
pub type CPU_TARGETS3_R = crate::FieldReader;
///Field `CPU_TARGETS3` writer - CPU_TARGETS3
pub type CPU_TARGETS3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
        f.debug_struct("ITARGETSR61")
            .field("cpu_targets0", &self.cpu_targets0())
            .field("cpu_targets1", &self.cpu_targets1())
            .field("cpu_targets2", &self.cpu_targets2())
            .field("cpu_targets3", &self.cpu_targets3())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - CPU_TARGETS0
    #[inline(always)]
    pub fn cpu_targets0(&mut self) -> CPU_TARGETS0_W<ITARGETSR61rs> {
        CPU_TARGETS0_W::new(self, 0)
    }
    ///Bits 8:9 - CPU_TARGETS1
    #[inline(always)]
    pub fn cpu_targets1(&mut self) -> CPU_TARGETS1_W<ITARGETSR61rs> {
        CPU_TARGETS1_W::new(self, 8)
    }
    ///Bits 16:17 - CPU_TARGETS2
    #[inline(always)]
    pub fn cpu_targets2(&mut self) -> CPU_TARGETS2_W<ITARGETSR61rs> {
        CPU_TARGETS2_W::new(self, 16)
    }
    ///Bits 24:25 - CPU_TARGETS3
    #[inline(always)]
    pub fn cpu_targets3(&mut self) -> CPU_TARGETS3_W<ITARGETSR61rs> {
        CPU_TARGETS3_W::new(self, 24)
    }
}
/**GICD interrupt processor target register 61

You can [`read`](crate::Reg::read) this register and get [`itargetsr61::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr61::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ITARGETSR61)*/
pub struct ITARGETSR61rs;
impl crate::RegisterSpec for ITARGETSR61rs {
    type Ux = u32;
}
///`read()` method returns [`itargetsr61::R`](R) reader structure
impl crate::Readable for ITARGETSR61rs {}
///`write(|w| ..)` method takes [`itargetsr61::W`](W) writer structure
impl crate::Writable for ITARGETSR61rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ITARGETSR61 to value 0
impl crate::Resettable for ITARGETSR61rs {}
