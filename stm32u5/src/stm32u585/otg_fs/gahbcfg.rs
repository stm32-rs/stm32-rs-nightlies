#[doc = "Register `GAHBCFG` reader"]
pub type R = crate::R<GAHBCFGrs>;
#[doc = "Register `GAHBCFG` writer"]
pub type W = crate::W<GAHBCFGrs>;
#[doc = "Field `GINTMSK` reader - GINTMSK"]
pub type GINTMSK_R = crate::BitReader;
#[doc = "Field `GINTMSK` writer - GINTMSK"]
pub type GINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFELVL` reader - TXFELVL"]
pub type TXFELVL_R = crate::BitReader;
#[doc = "Field `TXFELVL` writer - TXFELVL"]
pub type TXFELVL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFELVL` reader - PTXFELVL"]
pub type PTXFELVL_R = crate::BitReader;
#[doc = "Field `PTXFELVL` writer - PTXFELVL"]
pub type PTXFELVL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GINTMSK"]
    #[inline(always)]
    pub fn gintmsk(&self) -> GINTMSK_R {
        GINTMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - TXFELVL"]
    #[inline(always)]
    pub fn txfelvl(&self) -> TXFELVL_R {
        TXFELVL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PTXFELVL"]
    #[inline(always)]
    pub fn ptxfelvl(&self) -> PTXFELVL_R {
        PTXFELVL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GINTMSK"]
    #[inline(always)]
    #[must_use]
    pub fn gintmsk(&mut self) -> GINTMSK_W<GAHBCFGrs> {
        GINTMSK_W::new(self, 0)
    }
    #[doc = "Bit 7 - TXFELVL"]
    #[inline(always)]
    #[must_use]
    pub fn txfelvl(&mut self) -> TXFELVL_W<GAHBCFGrs> {
        TXFELVL_W::new(self, 7)
    }
    #[doc = "Bit 8 - PTXFELVL"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfelvl(&mut self) -> PTXFELVL_W<GAHBCFGrs> {
        PTXFELVL_W::new(self, 8)
    }
}
#[doc = "This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAHBCFGrs;
impl crate::RegisterSpec for GAHBCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gahbcfg::R`](R) reader structure"]
impl crate::Readable for GAHBCFGrs {}
#[doc = "`write(|w| ..)` method takes [`gahbcfg::W`](W) writer structure"]
impl crate::Writable for GAHBCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GAHBCFGrs {
    const RESET_VALUE: u32 = 0;
}
