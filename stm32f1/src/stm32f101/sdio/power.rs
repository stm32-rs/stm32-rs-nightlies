#[doc = "Register `POWER` reader"]
pub type R = crate::R<POWERrs>;
#[doc = "Register `POWER` writer"]
pub type W = crate::W<POWERrs>;
#[doc = "Field `PWRCTRL` reader - PWRCTRL"]
pub type PWRCTRL_R = crate::FieldReader;
#[doc = "Field `PWRCTRL` writer - PWRCTRL"]
pub type PWRCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PWRCTRL"]
    #[inline(always)]
    pub fn pwrctrl(&self) -> PWRCTRL_R {
        PWRCTRL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PWRCTRL"]
    #[inline(always)]
    #[must_use]
    pub fn pwrctrl(&mut self) -> PWRCTRL_W<POWERrs> {
        PWRCTRL_W::new(self, 0)
    }
}
#[doc = "Bits 1:0 = PWRCTRL: Power supply control bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWERrs;
impl crate::RegisterSpec for POWERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power::R`](R) reader structure"]
impl crate::Readable for POWERrs {}
#[doc = "`write(|w| ..)` method takes [`power::W`](W) writer structure"]
impl crate::Writable for POWERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWER to value 0"]
impl crate::Resettable for POWERrs {
    const RESET_VALUE: u32 = 0;
}
