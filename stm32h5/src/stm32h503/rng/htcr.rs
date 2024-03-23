#[doc = "Register `HTCR` reader"]
pub type R = crate::R<HTCRrs>;
#[doc = "Register `HTCR` writer"]
pub type W = crate::W<HTCRrs>;
#[doc = "Field `HTCFG` reader - health test configuration This configuration is used by RNG to configure the health tests. See Section 23.6: RNG entropy source validation for the recommended value. Note: The RNG behavior, including the read to this register, is not guaranteed if a different value from the recommended value is written."]
pub type HTCFG_R = crate::FieldReader<u32>;
#[doc = "Field `HTCFG` writer - health test configuration This configuration is used by RNG to configure the health tests. See Section 23.6: RNG entropy source validation for the recommended value. Note: The RNG behavior, including the read to this register, is not guaranteed if a different value from the recommended value is written."]
pub type HTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - health test configuration This configuration is used by RNG to configure the health tests. See Section 23.6: RNG entropy source validation for the recommended value. Note: The RNG behavior, including the read to this register, is not guaranteed if a different value from the recommended value is written."]
    #[inline(always)]
    pub fn htcfg(&self) -> HTCFG_R {
        HTCFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - health test configuration This configuration is used by RNG to configure the health tests. See Section 23.6: RNG entropy source validation for the recommended value. Note: The RNG behavior, including the read to this register, is not guaranteed if a different value from the recommended value is written."]
    #[inline(always)]
    #[must_use]
    pub fn htcfg(&mut self) -> HTCFG_W<HTCRrs> {
        HTCFG_W::new(self, 0)
    }
}
#[doc = "RNG health test control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HTCRrs;
impl crate::RegisterSpec for HTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`htcr::R`](R) reader structure"]
impl crate::Readable for HTCRrs {}
#[doc = "`write(|w| ..)` method takes [`htcr::W`](W) writer structure"]
impl crate::Writable for HTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HTCR to value 0x72ac"]
impl crate::Resettable for HTCRrs {
    const RESET_VALUE: u32 = 0x72ac;
}
