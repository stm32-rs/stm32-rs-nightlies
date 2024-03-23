#[doc = "Register `GCCFG` reader"]
pub type R = crate::R<GCCFGrs>;
#[doc = "Register `GCCFG` writer"]
pub type W = crate::W<GCCFGrs>;
#[doc = "Field `PWRDWN` reader - Power down"]
pub type PWRDWN_R = crate::BitReader;
#[doc = "Field `PWRDWN` writer - Power down"]
pub type PWRDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSASEN` reader - Enable the VBUS sensing device"]
pub type VBUSASEN_R = crate::BitReader;
#[doc = "Field `VBUSASEN` writer - Enable the VBUS sensing device"]
pub type VBUSASEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSBSEN` reader - Enable the VBUS sensing device"]
pub type VBUSBSEN_R = crate::BitReader;
#[doc = "Field `VBUSBSEN` writer - Enable the VBUS sensing device"]
pub type VBUSBSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFOUTEN` reader - SOF output enable"]
pub type SOFOUTEN_R = crate::BitReader;
#[doc = "Field `SOFOUTEN` writer - SOF output enable"]
pub type SOFOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOVBUSSENS` reader - Vbus sensing disable option"]
pub type NOVBUSSENS_R = crate::BitReader;
#[doc = "Field `NOVBUSSENS` writer - Vbus sensing disable option"]
pub type NOVBUSSENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable the VBUS sensing device"]
    #[inline(always)]
    pub fn vbusasen(&self) -> VBUSASEN_R {
        VBUSASEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable the VBUS sensing device"]
    #[inline(always)]
    pub fn vbusbsen(&self) -> VBUSBSEN_R {
        VBUSBSEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    pub fn sofouten(&self) -> SOFOUTEN_R {
        SOFOUTEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Vbus sensing disable option"]
    #[inline(always)]
    pub fn novbussens(&self) -> NOVBUSSENS_R {
        NOVBUSSENS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<GCCFGrs> {
        PWRDWN_W::new(self, 16)
    }
    #[doc = "Bit 18 - Enable the VBUS sensing device"]
    #[inline(always)]
    #[must_use]
    pub fn vbusasen(&mut self) -> VBUSASEN_W<GCCFGrs> {
        VBUSASEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable the VBUS sensing device"]
    #[inline(always)]
    #[must_use]
    pub fn vbusbsen(&mut self) -> VBUSBSEN_W<GCCFGrs> {
        VBUSBSEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofouten(&mut self) -> SOFOUTEN_W<GCCFGrs> {
        SOFOUTEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Vbus sensing disable option"]
    #[inline(always)]
    #[must_use]
    pub fn novbussens(&mut self) -> NOVBUSSENS_W<GCCFGrs> {
        NOVBUSSENS_W::new(self, 21)
    }
}
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
