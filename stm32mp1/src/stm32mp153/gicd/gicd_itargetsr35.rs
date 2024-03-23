#[doc = "Register `GICD_ITARGETSR35` reader"]
pub type R = crate::R<GICD_ITARGETSR35rs>;
#[doc = "Register `GICD_ITARGETSR35` writer"]
pub type W = crate::W<GICD_ITARGETSR35rs>;
#[doc = "Field `CPU_TARGETS0` reader - CPU_TARGETS0"]
pub type CPU_TARGETS0_R = crate::FieldReader;
#[doc = "Field `CPU_TARGETS0` writer - CPU_TARGETS0"]
pub type CPU_TARGETS0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CPU_TARGETS1` reader - CPU_TARGETS1"]
pub type CPU_TARGETS1_R = crate::FieldReader;
#[doc = "Field `CPU_TARGETS1` writer - CPU_TARGETS1"]
pub type CPU_TARGETS1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CPU_TARGETS2` reader - CPU_TARGETS2"]
pub type CPU_TARGETS2_R = crate::FieldReader;
#[doc = "Field `CPU_TARGETS2` writer - CPU_TARGETS2"]
pub type CPU_TARGETS2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CPU_TARGETS3` reader - CPU_TARGETS3"]
pub type CPU_TARGETS3_R = crate::FieldReader;
#[doc = "Field `CPU_TARGETS3` writer - CPU_TARGETS3"]
pub type CPU_TARGETS3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
impl W {
    #[doc = "Bits 0:1 - CPU_TARGETS0"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_targets0(&mut self) -> CPU_TARGETS0_W<GICD_ITARGETSR35rs> {
        CPU_TARGETS0_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - CPU_TARGETS1"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_targets1(&mut self) -> CPU_TARGETS1_W<GICD_ITARGETSR35rs> {
        CPU_TARGETS1_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - CPU_TARGETS2"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_targets2(&mut self) -> CPU_TARGETS2_W<GICD_ITARGETSR35rs> {
        CPU_TARGETS2_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - CPU_TARGETS3"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_targets3(&mut self) -> CPU_TARGETS3_W<GICD_ITARGETSR35rs> {
        CPU_TARGETS3_W::new(self, 24)
    }
}
#[doc = "GICD interrupt processor target register 35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr35::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr35::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR35rs;
impl crate::RegisterSpec for GICD_ITARGETSR35rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr35::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR35rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr35::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR35rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR35 to value 0"]
impl crate::Resettable for GICD_ITARGETSR35rs {
    const RESET_VALUE: u32 = 0;
}
