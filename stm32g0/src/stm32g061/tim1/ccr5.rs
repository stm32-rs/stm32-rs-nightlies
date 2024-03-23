#[doc = "Register `CCR5` reader"]
pub type R = crate::R<CCR5rs>;
#[doc = "Register `CCR5` writer"]
pub type W = crate::W<CCR5rs>;
#[doc = "Field `CCR` reader - Capture/Compare value"]
pub type CCR_R = crate::FieldReader<u16>;
#[doc = "Field `CCR` writer - Capture/Compare value"]
pub type CCR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `GC5C1` reader - Group Channel 5 and Channel 1 Distortion on Channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C1_R = crate::BitReader;
#[doc = "Field `GC5C1` writer - Group Channel 5 and Channel 1 Distortion on Channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC5C2` reader - Group Channel 5 and Channel 2 Distortion on Channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C2_R = crate::BitReader;
#[doc = "Field `GC5C2` writer - Group Channel 5 and Channel 2 Distortion on Channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC5C3` reader - Group Channel 5 and Channel 3 Distortion on Channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C3_R = crate::BitReader;
#[doc = "Field `GC5C3` writer - Group Channel 5 and Channel 3 Distortion on Channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 29 - Group Channel 5 and Channel 1 Distortion on Channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn gc5c1(&self) -> GC5C1_R {
        GC5C1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Group Channel 5 and Channel 2 Distortion on Channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn gc5c2(&self) -> GC5C2_R {
        GC5C2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Group Channel 5 and Channel 3 Distortion on Channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn gc5c3(&self) -> GC5C3_R {
        GC5C3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr(&mut self) -> CCR_W<CCR5rs> {
        CCR_W::new(self, 0)
    }
    #[doc = "Bit 29 - Group Channel 5 and Channel 1 Distortion on Channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    #[must_use]
    pub fn gc5c1(&mut self) -> GC5C1_W<CCR5rs> {
        GC5C1_W::new(self, 29)
    }
    #[doc = "Bit 30 - Group Channel 5 and Channel 2 Distortion on Channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    #[must_use]
    pub fn gc5c2(&mut self) -> GC5C2_W<CCR5rs> {
        GC5C2_W::new(self, 30)
    }
    #[doc = "Bit 31 - Group Channel 5 and Channel 3 Distortion on Channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    #[must_use]
    pub fn gc5c3(&mut self) -> GC5C3_W<CCR5rs> {
        GC5C3_W::new(self, 31)
    }
}
#[doc = "capture/compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR5rs;
impl crate::RegisterSpec for CCR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr5::R`](R) reader structure"]
impl crate::Readable for CCR5rs {}
#[doc = "`write(|w| ..)` method takes [`ccr5::W`](W) writer structure"]
impl crate::Writable for CCR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR5 to value 0"]
impl crate::Resettable for CCR5rs {
    const RESET_VALUE: u32 = 0;
}
