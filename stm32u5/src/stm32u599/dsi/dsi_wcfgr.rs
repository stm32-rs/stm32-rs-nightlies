#[doc = "Register `DSI_WCFGR` reader"]
pub type R = crate::R<DSI_WCFGRrs>;
#[doc = "Register `DSI_WCFGR` writer"]
pub type W = crate::W<DSI_WCFGRrs>;
#[doc = "Field `DSIM` reader - DSI mode This bit selects the mode for the video transmission. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0)."]
pub type DSIM_R = crate::BitReader;
#[doc = "Field `DSIM` writer - DSI mode This bit selects the mode for the video transmission. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0)."]
pub type DSIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLMUX` reader - Color multiplexing This bit selects the color multiplexing used by DSI Host. This field must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.ENÂ =Â 0)."]
pub type COLMUX_R = crate::FieldReader;
#[doc = "Field `COLMUX` writer - Color multiplexing This bit selects the color multiplexing used by DSI Host. This field must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.ENÂ =Â 0)."]
pub type COLMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TESRC` reader - TE source This bit selects the tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0)."]
pub type TESRC_R = crate::BitReader;
#[doc = "Field `TESRC` writer - TE source This bit selects the tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0)."]
pub type TESRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEPOL` reader - TE polarity This bit selects the polarity of the external pin tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0)."]
pub type TEPOL_R = crate::BitReader;
#[doc = "Field `TEPOL` writer - TE polarity This bit selects the polarity of the external pin tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0)."]
pub type TEPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AR` reader - Automatic refresh This bit selects the refresh mode in DBI mode. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0)."]
pub type AR_R = crate::BitReader;
#[doc = "Field `AR` writer - Automatic refresh This bit selects the refresh mode in DBI mode. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0)."]
pub type AR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSPOL` reader - VSync polarity This bit selects the VSync edge on which the LTDC is halted. This bit must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.ENÂ =Â 0)."]
pub type VSPOL_R = crate::BitReader;
#[doc = "Field `VSPOL` writer - VSync polarity This bit selects the VSync edge on which the LTDC is halted. This bit must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.ENÂ =Â 0)."]
pub type VSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DSI mode This bit selects the mode for the video transmission. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0)."]
    #[inline(always)]
    pub fn dsim(&self) -> DSIM_R {
        DSIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Color multiplexing This bit selects the color multiplexing used by DSI Host. This field must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.ENÂ =Â 0)."]
    #[inline(always)]
    pub fn colmux(&self) -> COLMUX_R {
        COLMUX_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - TE source This bit selects the tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0)."]
    #[inline(always)]
    pub fn tesrc(&self) -> TESRC_R {
        TESRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TE polarity This bit selects the polarity of the external pin tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0)."]
    #[inline(always)]
    pub fn tepol(&self) -> TEPOL_R {
        TEPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic refresh This bit selects the refresh mode in DBI mode. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0)."]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VSync polarity This bit selects the VSync edge on which the LTDC is halted. This bit must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.ENÂ =Â 0)."]
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DSI mode This bit selects the mode for the video transmission. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn dsim(&mut self) -> DSIM_W<DSI_WCFGRrs> {
        DSIM_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - Color multiplexing This bit selects the color multiplexing used by DSI Host. This field must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.ENÂ =Â 0)."]
    #[inline(always)]
    #[must_use]
    pub fn colmux(&mut self) -> COLMUX_W<DSI_WCFGRrs> {
        COLMUX_W::new(self, 1)
    }
    #[doc = "Bit 4 - TE source This bit selects the tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn tesrc(&mut self) -> TESRC_W<DSI_WCFGRrs> {
        TESRC_W::new(self, 4)
    }
    #[doc = "Bit 5 - TE polarity This bit selects the polarity of the external pin tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn tepol(&mut self) -> TEPOL_W<DSI_WCFGRrs> {
        TEPOL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Automatic refresh This bit selects the refresh mode in DBI mode. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn ar(&mut self) -> AR_W<DSI_WCFGRrs> {
        AR_W::new(self, 6)
    }
    #[doc = "Bit 7 - VSync polarity This bit selects the VSync edge on which the LTDC is halted. This bit must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.ENÂ =Â 0)."]
    #[inline(always)]
    #[must_use]
    pub fn vspol(&mut self) -> VSPOL_W<DSI_WCFGRrs> {
        VSPOL_W::new(self, 7)
    }
}
#[doc = "DSI Wrapper configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_WCFGRrs;
impl crate::RegisterSpec for DSI_WCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wcfgr::R`](R) reader structure"]
impl crate::Readable for DSI_WCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_wcfgr::W`](W) writer structure"]
impl crate::Writable for DSI_WCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_WCFGR to value 0"]
impl crate::Resettable for DSI_WCFGRrs {
    const RESET_VALUE: u32 = 0;
}
