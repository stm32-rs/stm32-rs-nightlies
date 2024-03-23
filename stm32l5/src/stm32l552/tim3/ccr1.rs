#[doc = "Register `CCR1` reader"]
pub type R = crate::R<CCR1rs>;
#[doc = "Register `CCR1` writer"]
pub type W = crate::W<CCR1rs>;
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
    pub fn ccr1_l(&mut self) -> CCR1_L_W<CCR1rs> {
        CCR1_L_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - High Capture/Compare 1 value (TIM2 only)"]
    #[inline(always)]
    #[must_use]
    pub fn ccr1_h(&mut self) -> CCR1_H_W<CCR1rs> {
        CCR1_H_W::new(self, 16)
    }
}
#[doc = "capture/compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR1rs;
impl crate::RegisterSpec for CCR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr1::R`](R) reader structure"]
impl crate::Readable for CCR1rs {}
#[doc = "`write(|w| ..)` method takes [`ccr1::W`](W) writer structure"]
impl crate::Writable for CCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR1 to value 0"]
impl crate::Resettable for CCR1rs {
    const RESET_VALUE: u32 = 0;
}
