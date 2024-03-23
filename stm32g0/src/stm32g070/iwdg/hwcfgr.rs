#[doc = "Register `HWCFGR` reader"]
pub type R = crate::R<HWCFGRrs>;
#[doc = "Register `HWCFGR` writer"]
pub type W = crate::W<HWCFGRrs>;
#[doc = "Field `WINDOW` reader - Support of Window function"]
pub type WINDOW_R = crate::FieldReader;
#[doc = "Field `WINDOW` writer - Support of Window function"]
pub type WINDOW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PR_DEFAULT` reader - Prescaler default value"]
pub type PR_DEFAULT_R = crate::FieldReader;
#[doc = "Field `PR_DEFAULT` writer - Prescaler default value"]
pub type PR_DEFAULT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Support of Window function"]
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Prescaler default value"]
    #[inline(always)]
    pub fn pr_default(&self) -> PR_DEFAULT_R {
        PR_DEFAULT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Support of Window function"]
    #[inline(always)]
    #[must_use]
    pub fn window(&mut self) -> WINDOW_W<HWCFGRrs> {
        WINDOW_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Prescaler default value"]
    #[inline(always)]
    #[must_use]
    pub fn pr_default(&mut self) -> PR_DEFAULT_W<HWCFGRrs> {
        PR_DEFAULT_W::new(self, 4)
    }
}
#[doc = "hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGRrs;
impl crate::RegisterSpec for HWCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr::R`](R) reader structure"]
impl crate::Readable for HWCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`hwcfgr::W`](W) writer structure"]
impl crate::Writable for HWCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWCFGR to value 0x71"]
impl crate::Resettable for HWCFGRrs {
    const RESET_VALUE: u32 = 0x71;
}
