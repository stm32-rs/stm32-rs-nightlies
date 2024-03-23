#[doc = "Register `GCCFG` reader"]
pub type R = crate::R<GCCFGrs>;
#[doc = "Register `GCCFG` writer"]
pub type W = crate::W<GCCFGrs>;
#[doc = "Field `PWRDWN` reader - Power down"]
pub type PWRDWN_R = crate::BitReader;
#[doc = "Field `PWRDWN` writer - Power down"]
pub type PWRDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBDEN` reader - USB VBUS detection enable"]
pub type VBDEN_R = crate::BitReader;
#[doc = "Field `VBDEN` writer - USB VBUS detection enable"]
pub type VBDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - USB VBUS detection enable"]
    #[inline(always)]
    pub fn vbden(&self) -> VBDEN_R {
        VBDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<GCCFGrs> {
        PWRDWN_W::new(self, 16)
    }
    #[doc = "Bit 21 - USB VBUS detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbden(&mut self) -> VBDEN_W<GCCFGrs> {
        VBDEN_W::new(self, 21)
    }
}
#[doc = "OTG_HS general core configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCCFGrs;
impl crate::RegisterSpec for GCCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gccfg::R`](R) reader structure"]
impl crate::Readable for GCCFGrs {}
#[doc = "`write(|w| ..)` method takes [`gccfg::W`](W) writer structure"]
impl crate::Writable for GCCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCCFG to value 0"]
impl crate::Resettable for GCCFGrs {
    const RESET_VALUE: u32 = 0;
}
