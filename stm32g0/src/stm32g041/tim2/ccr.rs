#[doc = "Register `CCR%s` reader"]
pub type R = crate::R<CCRrs>;
#[doc = "Register `CCR%s` writer"]
pub type W = crate::W<CCRrs>;
#[doc = "Field `CCR1_L` reader - Low Capture/Compare 1 value"]
pub type CCR1_L_R = crate::FieldReader<u16>;
#[doc = "Field `CCR1_L` writer - Low Capture/Compare 1 value"]
pub type CCR1_L_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CCR1_H` reader - High Capture/Compare 1 value (TIM2 only)"]
pub type CCR1_H_R = crate::FieldReader<u16>;
#[doc = "Field `CCR1_H` writer - High Capture/Compare 1 value (TIM2 only)"]
pub type CCR1_H_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low Capture/Compare 1 value"]
    #[inline(always)]
    pub fn ccr1_l(&self) -> CCR1_L_R {
        CCR1_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Capture/Compare 1 value (TIM2 only)"]
    #[inline(always)]
    pub fn ccr1_h(&self) -> CCR1_H_R {
        CCR1_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Capture/Compare 1 value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr1_l(&mut self) -> CCR1_L_W<CCRrs> {
        CCR1_L_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - High Capture/Compare 1 value (TIM2 only)"]
    #[inline(always)]
    #[must_use]
    pub fn ccr1_h(&mut self) -> CCR1_H_W<CCRrs> {
        CCR1_H_W::new(self, 16)
    }
}
#[doc = "capture/compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCRrs {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR%s to value 0"]
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0;
}
