///Register `ITARGETSR36` reader
pub type R = crate::R<ITARGETSR36rs>;
///Register `ITARGETSR36` writer
pub type W = crate::W<ITARGETSR36rs>;
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
        f.debug_struct("ITARGETSR36")
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
    pub fn cpu_targets0(&mut self) -> CPU_TARGETS0_W<ITARGETSR36rs> {
        CPU_TARGETS0_W::new(self, 0)
    }
    ///Bits 8:9 - CPU_TARGETS1
    #[inline(always)]
    pub fn cpu_targets1(&mut self) -> CPU_TARGETS1_W<ITARGETSR36rs> {
        CPU_TARGETS1_W::new(self, 8)
    }
    ///Bits 16:17 - CPU_TARGETS2
    #[inline(always)]
    pub fn cpu_targets2(&mut self) -> CPU_TARGETS2_W<ITARGETSR36rs> {
        CPU_TARGETS2_W::new(self, 16)
    }
    ///Bits 24:25 - CPU_TARGETS3
    #[inline(always)]
    pub fn cpu_targets3(&mut self) -> CPU_TARGETS3_W<ITARGETSR36rs> {
        CPU_TARGETS3_W::new(self, 24)
    }
}
/**GICD interrupt processor target register 36

You can [`read`](crate::Reg::read) this register and get [`itargetsr36::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr36::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ITARGETSR36)*/
pub struct ITARGETSR36rs;
impl crate::RegisterSpec for ITARGETSR36rs {
    type Ux = u32;
}
///`read()` method returns [`itargetsr36::R`](R) reader structure
impl crate::Readable for ITARGETSR36rs {}
///`write(|w| ..)` method takes [`itargetsr36::W`](W) writer structure
impl crate::Writable for ITARGETSR36rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ITARGETSR36 to value 0
impl crate::Resettable for ITARGETSR36rs {
    const RESET_VALUE: u32 = 0;
}