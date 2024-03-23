#[doc = "Register `CCR2` reader"]
pub type R = crate::R<CCR2rs>;
#[doc = "Register `CCR2` writer"]
pub type W = crate::W<CCR2rs>;
#[doc = "Field `CCR2_L` reader - Low Capture/Compare 2 value"]
pub type CCR2_L_R = crate::FieldReader<u16>;
#[doc = "Field `CCR2_L` writer - Low Capture/Compare 2 value"]
pub type CCR2_L_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CCR2_H` reader - High Capture/Compare 2 value (TIM2 only)"]
pub type CCR2_H_R = crate::FieldReader<u16>;
#[doc = "Field `CCR2_H` writer - High Capture/Compare 2 value (TIM2 only)"]
pub type CCR2_H_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low Capture/Compare 2 value"]
    #[inline(always)]
    pub fn ccr2_l(&self) -> CCR2_L_R {
        CCR2_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Capture/Compare 2 value (TIM2 only)"]
    #[inline(always)]
    pub fn ccr2_h(&self) -> CCR2_H_R {
        CCR2_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Capture/Compare 2 value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr2_l(&mut self) -> CCR2_L_W<CCR2rs> {
        CCR2_L_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - High Capture/Compare 2 value (TIM2 only)"]
    #[inline(always)]
    #[must_use]
    pub fn ccr2_h(&mut self) -> CCR2_H_W<CCR2rs> {
        CCR2_H_W::new(self, 16)
    }
}
#[doc = "capture/compare register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR2rs;
impl crate::RegisterSpec for CCR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr2::R`](R) reader structure"]
impl crate::Readable for CCR2rs {}
#[doc = "`write(|w| ..)` method takes [`ccr2::W`](W) writer structure"]
impl crate::Writable for CCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR2 to value 0"]
impl crate::Resettable for CCR2rs {
    const RESET_VALUE: u32 = 0;
}
