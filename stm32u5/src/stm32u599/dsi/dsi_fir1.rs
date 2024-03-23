#[doc = "Register `DSI_FIR1` writer"]
pub type W = crate::W<DSI_FIR1rs>;
#[doc = "Field `FTOHSTX` writer - Force timeout high-speed transmission Writing one to this bit forces a timeout high-speed transmission."]
pub type FTOHSTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTOLPRX` writer - Force timeout low-power reception Writing one to this bit forces a timeout low-power reception."]
pub type FTOLPRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FECCSE` writer - Force ECC single-bit error Writing one to this bit forces a ECC single-bit error."]
pub type FECCSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FECCME` writer - Force ECC multi-bit error Writing one to this bit forces a ECC multi-bit error."]
pub type FECCME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCRCE` writer - Force CRC error Writing one to this bit forces a CRC error."]
pub type FCRCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPSE` writer - Force packet size error Writing one to this bit forces a packet size error."]
pub type FPSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEOTPE` writer - Force EoTp error Writing one to this bit forces a EoTp error."]
pub type FEOTPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLPWRE` writer - Force LTDC payload write error Writing one to this bit forces a LTDC payload write error."]
pub type FLPWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGCWRE` writer - Force generic command write error Writing one to this bit forces a generic command write error."]
pub type FGCWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGPWRE` writer - Force generic payload write error Writing one to this bit forces a generic payload write error."]
pub type FGPWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGPTXE` writer - Force generic payload transmit error Writing one to this bit forces a generic payload transmit error."]
pub type FGPTXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGPRDE` writer - Force generic payload read error Writing one to this bit forces a generic payload read error."]
pub type FGPRDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGPRXE` writer - Force generic payload receive error Writing one to this bit forces a generic payload receive error."]
pub type FGPRXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPBUE` writer - Force payload buffer underflow error Writing one to this bit forces a payload undrflow error."]
pub type FPBUE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Force timeout high-speed transmission Writing one to this bit forces a timeout high-speed transmission."]
    #[inline(always)]
    #[must_use]
    pub fn ftohstx(&mut self) -> FTOHSTX_W<DSI_FIR1rs> {
        FTOHSTX_W::new(self, 0)
    }
    #[doc = "Bit 1 - Force timeout low-power reception Writing one to this bit forces a timeout low-power reception."]
    #[inline(always)]
    #[must_use]
    pub fn ftolprx(&mut self) -> FTOLPRX_W<DSI_FIR1rs> {
        FTOLPRX_W::new(self, 1)
    }
    #[doc = "Bit 2 - Force ECC single-bit error Writing one to this bit forces a ECC single-bit error."]
    #[inline(always)]
    #[must_use]
    pub fn feccse(&mut self) -> FECCSE_W<DSI_FIR1rs> {
        FECCSE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Force ECC multi-bit error Writing one to this bit forces a ECC multi-bit error."]
    #[inline(always)]
    #[must_use]
    pub fn feccme(&mut self) -> FECCME_W<DSI_FIR1rs> {
        FECCME_W::new(self, 3)
    }
    #[doc = "Bit 4 - Force CRC error Writing one to this bit forces a CRC error."]
    #[inline(always)]
    #[must_use]
    pub fn fcrce(&mut self) -> FCRCE_W<DSI_FIR1rs> {
        FCRCE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Force packet size error Writing one to this bit forces a packet size error."]
    #[inline(always)]
    #[must_use]
    pub fn fpse(&mut self) -> FPSE_W<DSI_FIR1rs> {
        FPSE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Force EoTp error Writing one to this bit forces a EoTp error."]
    #[inline(always)]
    #[must_use]
    pub fn feotpe(&mut self) -> FEOTPE_W<DSI_FIR1rs> {
        FEOTPE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Force LTDC payload write error Writing one to this bit forces a LTDC payload write error."]
    #[inline(always)]
    #[must_use]
    pub fn flpwre(&mut self) -> FLPWRE_W<DSI_FIR1rs> {
        FLPWRE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Force generic command write error Writing one to this bit forces a generic command write error."]
    #[inline(always)]
    #[must_use]
    pub fn fgcwre(&mut self) -> FGCWRE_W<DSI_FIR1rs> {
        FGCWRE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Force generic payload write error Writing one to this bit forces a generic payload write error."]
    #[inline(always)]
    #[must_use]
    pub fn fgpwre(&mut self) -> FGPWRE_W<DSI_FIR1rs> {
        FGPWRE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Force generic payload transmit error Writing one to this bit forces a generic payload transmit error."]
    #[inline(always)]
    #[must_use]
    pub fn fgptxe(&mut self) -> FGPTXE_W<DSI_FIR1rs> {
        FGPTXE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Force generic payload read error Writing one to this bit forces a generic payload read error."]
    #[inline(always)]
    #[must_use]
    pub fn fgprde(&mut self) -> FGPRDE_W<DSI_FIR1rs> {
        FGPRDE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Force generic payload receive error Writing one to this bit forces a generic payload receive error."]
    #[inline(always)]
    #[must_use]
    pub fn fgprxe(&mut self) -> FGPRXE_W<DSI_FIR1rs> {
        FGPRXE_W::new(self, 12)
    }
    #[doc = "Bit 19 - Force payload buffer underflow error Writing one to this bit forces a payload undrflow error."]
    #[inline(always)]
    #[must_use]
    pub fn fpbue(&mut self) -> FPBUE_W<DSI_FIR1rs> {
        FPBUE_W::new(self, 19)
    }
}
#[doc = "DSI Host force interrupt register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_fir1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_FIR1rs;
impl crate::RegisterSpec for DSI_FIR1rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dsi_fir1::W`](W) writer structure"]
impl crate::Writable for DSI_FIR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_FIR1 to value 0"]
impl crate::Resettable for DSI_FIR1rs {
    const RESET_VALUE: u32 = 0;
}
