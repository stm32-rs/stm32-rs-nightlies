///Register `DSI_FIR1` writer
pub type W = crate::W<DSI_FIR1rs>;
///Field `FTOHSTX` writer - Force timeout high-speed transmission Writing one to this bit forces a timeout high-speed transmission.
pub type FTOHSTX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTOLPRX` writer - Force timeout low-power reception Writing one to this bit forces a timeout low-power reception.
pub type FTOLPRX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FECCSE` writer - Force ECC single-bit error Writing one to this bit forces a ECC single-bit error.
pub type FECCSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FECCME` writer - Force ECC multi-bit error Writing one to this bit forces a ECC multi-bit error.
pub type FECCME_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FCRCE` writer - Force CRC error Writing one to this bit forces a CRC error.
pub type FCRCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPSE` writer - Force packet size error Writing one to this bit forces a packet size error.
pub type FPSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FEOTPE` writer - Force EoTp error Writing one to this bit forces a EoTp error.
pub type FEOTPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLPWRE` writer - Force LTDC payload write error Writing one to this bit forces a LTDC payload write error.
pub type FLPWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FGCWRE` writer - Force generic command write error Writing one to this bit forces a generic command write error.
pub type FGCWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FGPWRE` writer - Force generic payload write error Writing one to this bit forces a generic payload write error.
pub type FGPWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FGPTXE` writer - Force generic payload transmit error Writing one to this bit forces a generic payload transmit error.
pub type FGPTXE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FGPRDE` writer - Force generic payload read error Writing one to this bit forces a generic payload read error.
pub type FGPRDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FGPRXE` writer - Force generic payload receive error Writing one to this bit forces a generic payload receive error.
pub type FGPRXE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPBUE` writer - Force payload buffer underflow error Writing one to this bit forces a payload undrflow error.
pub type FPBUE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<DSI_FIR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Force timeout high-speed transmission Writing one to this bit forces a timeout high-speed transmission.
    #[inline(always)]
    #[must_use]
    pub fn ftohstx(&mut self) -> FTOHSTX_W<DSI_FIR1rs> {
        FTOHSTX_W::new(self, 0)
    }
    ///Bit 1 - Force timeout low-power reception Writing one to this bit forces a timeout low-power reception.
    #[inline(always)]
    #[must_use]
    pub fn ftolprx(&mut self) -> FTOLPRX_W<DSI_FIR1rs> {
        FTOLPRX_W::new(self, 1)
    }
    ///Bit 2 - Force ECC single-bit error Writing one to this bit forces a ECC single-bit error.
    #[inline(always)]
    #[must_use]
    pub fn feccse(&mut self) -> FECCSE_W<DSI_FIR1rs> {
        FECCSE_W::new(self, 2)
    }
    ///Bit 3 - Force ECC multi-bit error Writing one to this bit forces a ECC multi-bit error.
    #[inline(always)]
    #[must_use]
    pub fn feccme(&mut self) -> FECCME_W<DSI_FIR1rs> {
        FECCME_W::new(self, 3)
    }
    ///Bit 4 - Force CRC error Writing one to this bit forces a CRC error.
    #[inline(always)]
    #[must_use]
    pub fn fcrce(&mut self) -> FCRCE_W<DSI_FIR1rs> {
        FCRCE_W::new(self, 4)
    }
    ///Bit 5 - Force packet size error Writing one to this bit forces a packet size error.
    #[inline(always)]
    #[must_use]
    pub fn fpse(&mut self) -> FPSE_W<DSI_FIR1rs> {
        FPSE_W::new(self, 5)
    }
    ///Bit 6 - Force EoTp error Writing one to this bit forces a EoTp error.
    #[inline(always)]
    #[must_use]
    pub fn feotpe(&mut self) -> FEOTPE_W<DSI_FIR1rs> {
        FEOTPE_W::new(self, 6)
    }
    ///Bit 7 - Force LTDC payload write error Writing one to this bit forces a LTDC payload write error.
    #[inline(always)]
    #[must_use]
    pub fn flpwre(&mut self) -> FLPWRE_W<DSI_FIR1rs> {
        FLPWRE_W::new(self, 7)
    }
    ///Bit 8 - Force generic command write error Writing one to this bit forces a generic command write error.
    #[inline(always)]
    #[must_use]
    pub fn fgcwre(&mut self) -> FGCWRE_W<DSI_FIR1rs> {
        FGCWRE_W::new(self, 8)
    }
    ///Bit 9 - Force generic payload write error Writing one to this bit forces a generic payload write error.
    #[inline(always)]
    #[must_use]
    pub fn fgpwre(&mut self) -> FGPWRE_W<DSI_FIR1rs> {
        FGPWRE_W::new(self, 9)
    }
    ///Bit 10 - Force generic payload transmit error Writing one to this bit forces a generic payload transmit error.
    #[inline(always)]
    #[must_use]
    pub fn fgptxe(&mut self) -> FGPTXE_W<DSI_FIR1rs> {
        FGPTXE_W::new(self, 10)
    }
    ///Bit 11 - Force generic payload read error Writing one to this bit forces a generic payload read error.
    #[inline(always)]
    #[must_use]
    pub fn fgprde(&mut self) -> FGPRDE_W<DSI_FIR1rs> {
        FGPRDE_W::new(self, 11)
    }
    ///Bit 12 - Force generic payload receive error Writing one to this bit forces a generic payload receive error.
    #[inline(always)]
    #[must_use]
    pub fn fgprxe(&mut self) -> FGPRXE_W<DSI_FIR1rs> {
        FGPRXE_W::new(self, 12)
    }
    ///Bit 19 - Force payload buffer underflow error Writing one to this bit forces a payload undrflow error.
    #[inline(always)]
    #[must_use]
    pub fn fpbue(&mut self) -> FPBUE_W<DSI_FIR1rs> {
        FPBUE_W::new(self, 19)
    }
}
/**DSI Host force interrupt register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_fir1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:DSI_FIR1)*/
pub struct DSI_FIR1rs;
impl crate::RegisterSpec for DSI_FIR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`dsi_fir1::W`](W) writer structure
impl crate::Writable for DSI_FIR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_FIR1 to value 0
impl crate::Resettable for DSI_FIR1rs {
    const RESET_VALUE: u32 = 0;
}
