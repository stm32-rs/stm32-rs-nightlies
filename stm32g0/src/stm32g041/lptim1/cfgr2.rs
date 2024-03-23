#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "Field `IN1SEL` reader - LPTIMx Input 1 selection"]
pub type IN1SEL_R = crate::FieldReader;
#[doc = "Field `IN1SEL` writer - LPTIMx Input 1 selection"]
pub type IN1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IN2SEL` reader - LPTIM1 Input 2 selection"]
pub type IN2SEL_R = crate::FieldReader;
#[doc = "Field `IN2SEL` writer - LPTIM1 Input 2 selection"]
pub type IN2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - LPTIMx Input 1 selection"]
    #[inline(always)]
    pub fn in1sel(&self) -> IN1SEL_R {
        IN1SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - LPTIM1 Input 2 selection"]
    #[inline(always)]
    pub fn in2sel(&self) -> IN2SEL_R {
        IN2SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LPTIMx Input 1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn in1sel(&mut self) -> IN1SEL_W<CFGR2rs> {
        IN1SEL_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - LPTIM1 Input 2 selection"]
    #[inline(always)]
    #[must_use]
    pub fn in2sel(&mut self) -> IN2SEL_W<CFGR2rs> {
        IN2SEL_W::new(self, 4)
    }
}
#[doc = "LPTIM configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
