///Register `FIR1` writer
pub type W = crate::W<FIR1rs>;
///Field `FTOHSTX` writer - Force Timeout High-Speed Transmission
pub type FTOHSTX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTOLPRX` writer - Force Timeout Low-Power Reception
pub type FTOLPRX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FECCSE` writer - Force ECC Single-bit Error
pub type FECCSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FECCME` writer - Force ECC Multi-bit Error
pub type FECCME_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FCRCE` writer - Force CRC Error
pub type FCRCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPSE` writer - Force Packet Size Error
pub type FPSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FEOTPE` writer - Force EoTp Error
pub type FEOTPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLPWRE` writer - Force LTDC Payload Write Error
pub type FLPWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FGCWRE` writer - Force Generic Command Write Error
pub type FGCWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FGPWRE` writer - Force Generic Payload Write Error
pub type FGPWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FGPTXE` writer - Force Generic Payload Transmit Error
pub type FGPTXE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FGPRDE` writer - Force Generic Payload Read Error
pub type FGPRDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FGPRXE` writer - Force Generic Payload Receive Error
pub type FGPRXE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FIR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Force Timeout High-Speed Transmission
    #[inline(always)]
    pub fn ftohstx(&mut self) -> FTOHSTX_W<'_, FIR1rs> {
        FTOHSTX_W::new(self, 0)
    }
    ///Bit 1 - Force Timeout Low-Power Reception
    #[inline(always)]
    pub fn ftolprx(&mut self) -> FTOLPRX_W<'_, FIR1rs> {
        FTOLPRX_W::new(self, 1)
    }
    ///Bit 2 - Force ECC Single-bit Error
    #[inline(always)]
    pub fn feccse(&mut self) -> FECCSE_W<'_, FIR1rs> {
        FECCSE_W::new(self, 2)
    }
    ///Bit 3 - Force ECC Multi-bit Error
    #[inline(always)]
    pub fn feccme(&mut self) -> FECCME_W<'_, FIR1rs> {
        FECCME_W::new(self, 3)
    }
    ///Bit 4 - Force CRC Error
    #[inline(always)]
    pub fn fcrce(&mut self) -> FCRCE_W<'_, FIR1rs> {
        FCRCE_W::new(self, 4)
    }
    ///Bit 5 - Force Packet Size Error
    #[inline(always)]
    pub fn fpse(&mut self) -> FPSE_W<'_, FIR1rs> {
        FPSE_W::new(self, 5)
    }
    ///Bit 6 - Force EoTp Error
    #[inline(always)]
    pub fn feotpe(&mut self) -> FEOTPE_W<'_, FIR1rs> {
        FEOTPE_W::new(self, 6)
    }
    ///Bit 7 - Force LTDC Payload Write Error
    #[inline(always)]
    pub fn flpwre(&mut self) -> FLPWRE_W<'_, FIR1rs> {
        FLPWRE_W::new(self, 7)
    }
    ///Bit 8 - Force Generic Command Write Error
    #[inline(always)]
    pub fn fgcwre(&mut self) -> FGCWRE_W<'_, FIR1rs> {
        FGCWRE_W::new(self, 8)
    }
    ///Bit 9 - Force Generic Payload Write Error
    #[inline(always)]
    pub fn fgpwre(&mut self) -> FGPWRE_W<'_, FIR1rs> {
        FGPWRE_W::new(self, 9)
    }
    ///Bit 10 - Force Generic Payload Transmit Error
    #[inline(always)]
    pub fn fgptxe(&mut self) -> FGPTXE_W<'_, FIR1rs> {
        FGPTXE_W::new(self, 10)
    }
    ///Bit 11 - Force Generic Payload Read Error
    #[inline(always)]
    pub fn fgprde(&mut self) -> FGPRDE_W<'_, FIR1rs> {
        FGPRDE_W::new(self, 11)
    }
    ///Bit 12 - Force Generic Payload Receive Error
    #[inline(always)]
    pub fn fgprxe(&mut self) -> FGPRXE_W<'_, FIR1rs> {
        FGPRXE_W::new(self, 12)
    }
}
/**DSI Host Force Interrupt Register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fir1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:FIR1)*/
pub struct FIR1rs;
impl crate::RegisterSpec for FIR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fir1::W`](W) writer structure
impl crate::Writable for FIR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FIR1 to value 0
impl crate::Resettable for FIR1rs {}
