#[doc = "Register `DDRCTRL_ODTCFG` reader"]
pub type R = crate::R<DDRCTRL_ODTCFGrs>;
#[doc = "Register `DDRCTRL_ODTCFG` writer"]
pub type W = crate::W<DDRCTRL_ODTCFGrs>;
#[doc = "Field `RD_ODT_DELAY` reader - RD_ODT_DELAY"]
pub type RD_ODT_DELAY_R = crate::FieldReader;
#[doc = "Field `RD_ODT_DELAY` writer - RD_ODT_DELAY"]
pub type RD_ODT_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RD_ODT_HOLD` reader - RD_ODT_HOLD"]
pub type RD_ODT_HOLD_R = crate::FieldReader;
#[doc = "Field `RD_ODT_HOLD` writer - RD_ODT_HOLD"]
pub type RD_ODT_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WR_ODT_DELAY` reader - WR_ODT_DELAY"]
pub type WR_ODT_DELAY_R = crate::FieldReader;
#[doc = "Field `WR_ODT_DELAY` writer - WR_ODT_DELAY"]
pub type WR_ODT_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WR_ODT_HOLD` reader - WR_ODT_HOLD"]
pub type WR_ODT_HOLD_R = crate::FieldReader;
#[doc = "Field `WR_ODT_HOLD` writer - WR_ODT_HOLD"]
pub type WR_ODT_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 2:6 - RD_ODT_DELAY"]
    #[inline(always)]
    pub fn rd_odt_delay(&self) -> RD_ODT_DELAY_R {
        RD_ODT_DELAY_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - RD_ODT_HOLD"]
    #[inline(always)]
    pub fn rd_odt_hold(&self) -> RD_ODT_HOLD_R {
        RD_ODT_HOLD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - WR_ODT_DELAY"]
    #[inline(always)]
    pub fn wr_odt_delay(&self) -> WR_ODT_DELAY_R {
        WR_ODT_DELAY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - WR_ODT_HOLD"]
    #[inline(always)]
    pub fn wr_odt_hold(&self) -> WR_ODT_HOLD_R {
        WR_ODT_HOLD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:6 - RD_ODT_DELAY"]
    #[inline(always)]
    #[must_use]
    pub fn rd_odt_delay(&mut self) -> RD_ODT_DELAY_W<DDRCTRL_ODTCFGrs> {
        RD_ODT_DELAY_W::new(self, 2)
    }
    #[doc = "Bits 8:11 - RD_ODT_HOLD"]
    #[inline(always)]
    #[must_use]
    pub fn rd_odt_hold(&mut self) -> RD_ODT_HOLD_W<DDRCTRL_ODTCFGrs> {
        RD_ODT_HOLD_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - WR_ODT_DELAY"]
    #[inline(always)]
    #[must_use]
    pub fn wr_odt_delay(&mut self) -> WR_ODT_DELAY_W<DDRCTRL_ODTCFGrs> {
        WR_ODT_DELAY_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - WR_ODT_HOLD"]
    #[inline(always)]
    #[must_use]
    pub fn wr_odt_hold(&mut self) -> WR_ODT_HOLD_W<DDRCTRL_ODTCFGrs> {
        WR_ODT_HOLD_W::new(self, 24)
    }
}
#[doc = "DDRCTRL ODT configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_odtcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_odtcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_ODTCFGrs;
impl crate::RegisterSpec for DDRCTRL_ODTCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_odtcfg::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_ODTCFGrs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_odtcfg::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_ODTCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_ODTCFG to value 0x0400_0400"]
impl crate::Resettable for DDRCTRL_ODTCFGrs {
    const RESET_VALUE: u32 = 0x0400_0400;
}
