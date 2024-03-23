#[doc = "Register `CCR3` reader"]
pub type R = crate::R<CCR3rs>;
#[doc = "Register `CCR3` writer"]
pub type W = crate::W<CCR3rs>;
#[doc = "Field `CCR3_L` reader - Low Capture/Compare value"]
pub type CCR3_L_R = crate::FieldReader<u16>;
#[doc = "Field `CCR3_L` writer - Low Capture/Compare value"]
pub type CCR3_L_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CCR3_H` reader - High Capture/Compare value (TIM2 only)"]
pub type CCR3_H_R = crate::FieldReader<u16>;
#[doc = "Field `CCR3_H` writer - High Capture/Compare value (TIM2 only)"]
pub type CCR3_H_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low Capture/Compare value"]
    #[inline(always)]
    pub fn ccr3_l(&self) -> CCR3_L_R {
        CCR3_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Capture/Compare value (TIM2 only)"]
    #[inline(always)]
    pub fn ccr3_h(&self) -> CCR3_H_R {
        CCR3_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Capture/Compare value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr3_l(&mut self) -> CCR3_L_W<CCR3rs> {
        CCR3_L_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - High Capture/Compare value (TIM2 only)"]
    #[inline(always)]
    #[must_use]
    pub fn ccr3_h(&mut self) -> CCR3_H_W<CCR3rs> {
        CCR3_H_W::new(self, 16)
    }
}
#[doc = "capture/compare register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR3rs;
impl crate::RegisterSpec for CCR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr3::R`](R) reader structure"]
impl crate::Readable for CCR3rs {}
#[doc = "`write(|w| ..)` method takes [`ccr3::W`](W) writer structure"]
impl crate::Writable for CCR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR3 to value 0"]
impl crate::Resettable for CCR3rs {
    const RESET_VALUE: u32 = 0;
}
