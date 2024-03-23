#[doc = "Register `MPCWM1_UPWWMR` reader"]
pub type R = crate::R<MPCWM1_UPWWMRrs>;
#[doc = "Register `MPCWM1_UPWWMR` writer"]
pub type W = crate::W<MPCWM1_UPWWMRrs>;
#[doc = "Field `LGTH` reader - Define the length of Flash Unprivileged Writable area, in 2"]
pub type LGTH_R = crate::FieldReader<u16>;
#[doc = "Field `LGTH` writer - Define the length of Flash Unprivileged Writable area, in 2"]
pub type LGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 16:27 - Define the length of Flash Unprivileged Writable area, in 2"]
    #[inline(always)]
    pub fn lgth(&self) -> LGTH_R {
        LGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - Define the length of Flash Unprivileged Writable area, in 2"]
    #[inline(always)]
    #[must_use]
    pub fn lgth(&mut self) -> LGTH_W<MPCWM1_UPWWMRrs> {
        LGTH_W::new(self, 16)
    }
}
#[doc = "Unprivileged Writable Water Mark 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm1_upwwmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm1_upwwmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPCWM1_UPWWMRrs;
impl crate::RegisterSpec for MPCWM1_UPWWMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpcwm1_upwwmr::R`](R) reader structure"]
impl crate::Readable for MPCWM1_UPWWMRrs {}
#[doc = "`write(|w| ..)` method takes [`mpcwm1_upwwmr::W`](W) writer structure"]
impl crate::Writable for MPCWM1_UPWWMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPCWM1_UPWWMR to value 0x0fff_0000"]
impl crate::Resettable for MPCWM1_UPWWMRrs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
