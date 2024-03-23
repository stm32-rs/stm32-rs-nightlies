#[doc = "Register `WCFGR` reader"]
pub type R = crate::R<WCFGRrs>;
#[doc = "Register `WCFGR` writer"]
pub type W = crate::W<WCFGRrs>;
#[doc = "Field `DSIM` reader - DSI Mode"]
pub type DSIM_R = crate::BitReader;
#[doc = "Field `DSIM` writer - DSI Mode"]
pub type DSIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLMUX` reader - Color Multiplexing"]
pub type COLMUX_R = crate::FieldReader;
#[doc = "Field `COLMUX` writer - Color Multiplexing"]
pub type COLMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TESRC` reader - TE Source"]
pub type TESRC_R = crate::BitReader;
#[doc = "Field `TESRC` writer - TE Source"]
pub type TESRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEPOL` reader - TE Polarity"]
pub type TEPOL_R = crate::BitReader;
#[doc = "Field `TEPOL` writer - TE Polarity"]
pub type TEPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AR` reader - Automatic Refresh"]
pub type AR_R = crate::BitReader;
#[doc = "Field `AR` writer - Automatic Refresh"]
pub type AR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSPOL` reader - VSync Polarity"]
pub type VSPOL_R = crate::BitReader;
#[doc = "Field `VSPOL` writer - VSync Polarity"]
pub type VSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DSI Mode"]
    #[inline(always)]
    pub fn dsim(&self) -> DSIM_R {
        DSIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Color Multiplexing"]
    #[inline(always)]
    pub fn colmux(&self) -> COLMUX_R {
        COLMUX_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - TE Source"]
    #[inline(always)]
    pub fn tesrc(&self) -> TESRC_R {
        TESRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TE Polarity"]
    #[inline(always)]
    pub fn tepol(&self) -> TEPOL_R {
        TEPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic Refresh"]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VSync Polarity"]
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DSI Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dsim(&mut self) -> DSIM_W<WCFGRrs> {
        DSIM_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - Color Multiplexing"]
    #[inline(always)]
    #[must_use]
    pub fn colmux(&mut self) -> COLMUX_W<WCFGRrs> {
        COLMUX_W::new(self, 1)
    }
    #[doc = "Bit 4 - TE Source"]
    #[inline(always)]
    #[must_use]
    pub fn tesrc(&mut self) -> TESRC_W<WCFGRrs> {
        TESRC_W::new(self, 4)
    }
    #[doc = "Bit 5 - TE Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn tepol(&mut self) -> TEPOL_W<WCFGRrs> {
        TEPOL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Automatic Refresh"]
    #[inline(always)]
    #[must_use]
    pub fn ar(&mut self) -> AR_W<WCFGRrs> {
        AR_W::new(self, 6)
    }
    #[doc = "Bit 7 - VSync Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vspol(&mut self) -> VSPOL_W<WCFGRrs> {
        VSPOL_W::new(self, 7)
    }
}
#[doc = "DSI Wrapper Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WCFGRrs;
impl crate::RegisterSpec for WCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wcfgr::R`](R) reader structure"]
impl crate::Readable for WCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`wcfgr::W`](W) writer structure"]
impl crate::Writable for WCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WCFGR to value 0"]
impl crate::Resettable for WCFGRrs {
    const RESET_VALUE: u32 = 0;
}
