#[doc = "Register `OPAMP_OR` reader"]
pub type R = crate::R<OPAMP_ORrs>;
#[doc = "Register `OPAMP_OR` writer"]
pub type W = crate::W<OPAMP_ORrs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<OPAMP_ORrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "OPAMP option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp_or::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp_or::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPAMP_ORrs;
impl crate::RegisterSpec for OPAMP_ORrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp_or::R`](R) reader structure"]
impl crate::Readable for OPAMP_ORrs {}
#[doc = "`write(|w| ..)` method takes [`opamp_or::W`](W) writer structure"]
impl crate::Writable for OPAMP_ORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPAMP_OR to value 0"]
impl crate::Resettable for OPAMP_ORrs {
    const RESET_VALUE: u32 = 0;
}
