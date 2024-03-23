#[doc = "Register `HWCFGR6` reader"]
pub type R = crate::R<HWCFGR6rs>;
#[doc = "Register `HWCFGR6` writer"]
pub type W = crate::W<HWCFGR6rs>;
#[doc = "Field `CPUEVENT` reader - HW configuration CPU event generation"]
pub type CPUEVENT_R = crate::FieldReader<u32>;
#[doc = "Field `CPUEVENT` writer - HW configuration CPU event generation"]
pub type CPUEVENT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - HW configuration CPU event generation"]
    #[inline(always)]
    pub fn cpuevent(&self) -> CPUEVENT_R {
        CPUEVENT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HW configuration CPU event generation"]
    #[inline(always)]
    #[must_use]
    pub fn cpuevent(&mut self) -> CPUEVENT_W<HWCFGR6rs> {
        CPUEVENT_W::new(self, 0)
    }
}
#[doc = "Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR6rs;
impl crate::RegisterSpec for HWCFGR6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr6::R`](R) reader structure"]
impl crate::Readable for HWCFGR6rs {}
#[doc = "`write(|w| ..)` method takes [`hwcfgr6::W`](W) writer structure"]
impl crate::Writable for HWCFGR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWCFGR6 to value 0x03"]
impl crate::Resettable for HWCFGR6rs {
    const RESET_VALUE: u32 = 0x03;
}
