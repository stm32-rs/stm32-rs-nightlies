///Register `FIR1` writer
pub type W = crate::W<FIR1rs>;
///Field `FTOHSTX` writer - FTOHSTX
pub type FTOHSTX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTOLPRX` writer - FTOLPRX
pub type FTOLPRX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FECCSE` writer - FECCSE
pub type FECCSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FECCME` writer - FECCME
pub type FECCME_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FCRCE` writer - FCRCE
pub type FCRCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPSE` writer - FPSE
pub type FPSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FEOTPE` writer - FEOTPE
pub type FEOTPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLPWRE` writer - FLPWRE
pub type FLPWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FGCWRE` writer - FGCWRE
pub type FGCWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FGPWRE` writer - FGPWRE
pub type FGPWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FGPTXE` writer - FGPTXE
pub type FGPTXE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FGPRDE` writer - FGPRDE
pub type FGPRDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FGPRXE` writer - FGPRXE
pub type FGPRXE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FIR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - FTOHSTX
    #[inline(always)]
    pub fn ftohstx(&mut self) -> FTOHSTX_W<'_, FIR1rs> {
        FTOHSTX_W::new(self, 0)
    }
    ///Bit 1 - FTOLPRX
    #[inline(always)]
    pub fn ftolprx(&mut self) -> FTOLPRX_W<'_, FIR1rs> {
        FTOLPRX_W::new(self, 1)
    }
    ///Bit 2 - FECCSE
    #[inline(always)]
    pub fn feccse(&mut self) -> FECCSE_W<'_, FIR1rs> {
        FECCSE_W::new(self, 2)
    }
    ///Bit 3 - FECCME
    #[inline(always)]
    pub fn feccme(&mut self) -> FECCME_W<'_, FIR1rs> {
        FECCME_W::new(self, 3)
    }
    ///Bit 4 - FCRCE
    #[inline(always)]
    pub fn fcrce(&mut self) -> FCRCE_W<'_, FIR1rs> {
        FCRCE_W::new(self, 4)
    }
    ///Bit 5 - FPSE
    #[inline(always)]
    pub fn fpse(&mut self) -> FPSE_W<'_, FIR1rs> {
        FPSE_W::new(self, 5)
    }
    ///Bit 6 - FEOTPE
    #[inline(always)]
    pub fn feotpe(&mut self) -> FEOTPE_W<'_, FIR1rs> {
        FEOTPE_W::new(self, 6)
    }
    ///Bit 7 - FLPWRE
    #[inline(always)]
    pub fn flpwre(&mut self) -> FLPWRE_W<'_, FIR1rs> {
        FLPWRE_W::new(self, 7)
    }
    ///Bit 8 - FGCWRE
    #[inline(always)]
    pub fn fgcwre(&mut self) -> FGCWRE_W<'_, FIR1rs> {
        FGCWRE_W::new(self, 8)
    }
    ///Bit 9 - FGPWRE
    #[inline(always)]
    pub fn fgpwre(&mut self) -> FGPWRE_W<'_, FIR1rs> {
        FGPWRE_W::new(self, 9)
    }
    ///Bit 10 - FGPTXE
    #[inline(always)]
    pub fn fgptxe(&mut self) -> FGPTXE_W<'_, FIR1rs> {
        FGPTXE_W::new(self, 10)
    }
    ///Bit 11 - FGPRDE
    #[inline(always)]
    pub fn fgprde(&mut self) -> FGPRDE_W<'_, FIR1rs> {
        FGPRDE_W::new(self, 11)
    }
    ///Bit 12 - FGPRXE
    #[inline(always)]
    pub fn fgprxe(&mut self) -> FGPRXE_W<'_, FIR1rs> {
        FGPRXE_W::new(self, 12)
    }
}
/**DSI Host force interrupt register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fir1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DSI:FIR1)*/
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
