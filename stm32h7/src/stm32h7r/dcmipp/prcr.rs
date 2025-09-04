///Register `PRCR` reader
pub type R = crate::R<PRCRrs>;
///Register `PRCR` writer
pub type W = crate::W<PRCRrs>;
///Field `ESS` reader - Embedded synchronization select Valid only for 8-bit parallel data. HSPOL/VSPOL are ignored when this bit is set.
pub type ESS_R = crate::BitReader;
///Field `ESS` writer - Embedded synchronization select Valid only for 8-bit parallel data. HSPOL/VSPOL are ignored when this bit is set.
pub type ESS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PCKPOL` reader - Pixel clock polarity This bit configures the capture edge of the pixel clock
pub type PCKPOL_R = crate::BitReader;
///Field `PCKPOL` writer - Pixel clock polarity This bit configures the capture edge of the pixel clock
pub type PCKPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSPOL` reader - Horizontal synchronization polarity This bit indicates the level on the HSYNC pin when the data are not valid on the parallel interface.
pub type HSPOL_R = crate::BitReader;
///Field `HSPOL` writer - Horizontal synchronization polarity This bit indicates the level on the HSYNC pin when the data are not valid on the parallel interface.
pub type HSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSPOL` reader - Vertical synchronization polarity This bit indicates the level on the VSYNC pin when the data are not valid on the parallel interface.
pub type VSPOL_R = crate::BitReader;
///Field `VSPOL` writer - Vertical synchronization polarity This bit indicates the level on the VSYNC pin when the data are not valid on the parallel interface.
pub type VSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EDM` reader - Extended data mode Other values: Reserved.
pub type EDM_R = crate::FieldReader;
///Field `EDM` writer - Extended data mode Other values: Reserved.
pub type EDM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ENABLE` reader - Parallel interface enable The parallel interface configuration registers must be correctly programmed before enabling this bit.
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - Parallel interface enable The parallel interface configuration registers must be correctly programmed before enabling this bit.
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORMAT` reader - Other values: data are captured and output as-is only through the data/dump pipeline (for example JPEG or byte input format). The monochrome Y input is inserted in the pipe as YUV pixels, with the U and V components set to neutral, to represent a grey color.
pub type FORMAT_R = crate::FieldReader;
///Field `FORMAT` writer - Other values: data are captured and output as-is only through the data/dump pipeline (for example JPEG or byte input format). The monochrome Y input is inserted in the pipe as YUV pixels, with the U and V components set to neutral, to represent a grey color.
pub type FORMAT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SWAPCYCLES` reader - Swap data (cycle 0 vs. cycle 1) for pixels received on two cycles The swap must not be activated by software for pixels received in one or three cycles.
pub type SWAPCYCLES_R = crate::BitReader;
///Field `SWAPCYCLES` writer - Swap data (cycle 0 vs. cycle 1) for pixels received on two cycles The swap must not be activated by software for pixels received in one or three cycles.
pub type SWAPCYCLES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWAPBITS` reader - Swap LSB vs. MSB within each received component
pub type SWAPBITS_R = crate::BitReader;
///Field `SWAPBITS` writer - Swap LSB vs. MSB within each received component
pub type SWAPBITS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 4 - Embedded synchronization select Valid only for 8-bit parallel data. HSPOL/VSPOL are ignored when this bit is set.
    #[inline(always)]
    pub fn ess(&self) -> ESS_R {
        ESS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pixel clock polarity This bit configures the capture edge of the pixel clock
    #[inline(always)]
    pub fn pckpol(&self) -> PCKPOL_R {
        PCKPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Horizontal synchronization polarity This bit indicates the level on the HSYNC pin when the data are not valid on the parallel interface.
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Vertical synchronization polarity This bit indicates the level on the VSYNC pin when the data are not valid on the parallel interface.
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 10:12 - Extended data mode Other values: Reserved.
    #[inline(always)]
    pub fn edm(&self) -> EDM_R {
        EDM_R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bit 14 - Parallel interface enable The parallel interface configuration registers must be correctly programmed before enabling this bit.
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:23 - Other values: data are captured and output as-is only through the data/dump pipeline (for example JPEG or byte input format). The monochrome Y input is inserted in the pipe as YUV pixels, with the U and V components set to neutral, to represent a grey color.
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 25 - Swap data (cycle 0 vs. cycle 1) for pixels received on two cycles The swap must not be activated by software for pixels received in one or three cycles.
    #[inline(always)]
    pub fn swapcycles(&self) -> SWAPCYCLES_R {
        SWAPCYCLES_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Swap LSB vs. MSB within each received component
    #[inline(always)]
    pub fn swapbits(&self) -> SWAPBITS_R {
        SWAPBITS_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRCR")
            .field("ess", &self.ess())
            .field("pckpol", &self.pckpol())
            .field("hspol", &self.hspol())
            .field("vspol", &self.vspol())
            .field("edm", &self.edm())
            .field("enable", &self.enable())
            .field("format", &self.format())
            .field("swapcycles", &self.swapcycles())
            .field("swapbits", &self.swapbits())
            .finish()
    }
}
impl W {
    ///Bit 4 - Embedded synchronization select Valid only for 8-bit parallel data. HSPOL/VSPOL are ignored when this bit is set.
    #[inline(always)]
    pub fn ess(&mut self) -> ESS_W<PRCRrs> {
        ESS_W::new(self, 4)
    }
    ///Bit 5 - Pixel clock polarity This bit configures the capture edge of the pixel clock
    #[inline(always)]
    pub fn pckpol(&mut self) -> PCKPOL_W<PRCRrs> {
        PCKPOL_W::new(self, 5)
    }
    ///Bit 6 - Horizontal synchronization polarity This bit indicates the level on the HSYNC pin when the data are not valid on the parallel interface.
    #[inline(always)]
    pub fn hspol(&mut self) -> HSPOL_W<PRCRrs> {
        HSPOL_W::new(self, 6)
    }
    ///Bit 7 - Vertical synchronization polarity This bit indicates the level on the VSYNC pin when the data are not valid on the parallel interface.
    #[inline(always)]
    pub fn vspol(&mut self) -> VSPOL_W<PRCRrs> {
        VSPOL_W::new(self, 7)
    }
    ///Bits 10:12 - Extended data mode Other values: Reserved.
    #[inline(always)]
    pub fn edm(&mut self) -> EDM_W<PRCRrs> {
        EDM_W::new(self, 10)
    }
    ///Bit 14 - Parallel interface enable The parallel interface configuration registers must be correctly programmed before enabling this bit.
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<PRCRrs> {
        ENABLE_W::new(self, 14)
    }
    ///Bits 16:23 - Other values: data are captured and output as-is only through the data/dump pipeline (for example JPEG or byte input format). The monochrome Y input is inserted in the pipe as YUV pixels, with the U and V components set to neutral, to represent a grey color.
    #[inline(always)]
    pub fn format(&mut self) -> FORMAT_W<PRCRrs> {
        FORMAT_W::new(self, 16)
    }
    ///Bit 25 - Swap data (cycle 0 vs. cycle 1) for pixels received on two cycles The swap must not be activated by software for pixels received in one or three cycles.
    #[inline(always)]
    pub fn swapcycles(&mut self) -> SWAPCYCLES_W<PRCRrs> {
        SWAPCYCLES_W::new(self, 25)
    }
    ///Bit 26 - Swap LSB vs. MSB within each received component
    #[inline(always)]
    pub fn swapbits(&mut self) -> SWAPBITS_W<PRCRrs> {
        SWAPBITS_W::new(self, 26)
    }
}
/**DCMIPP parallel interface control register

You can [`read`](crate::Reg::read) this register and get [`prcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:PRCR)*/
pub struct PRCRrs;
impl crate::RegisterSpec for PRCRrs {
    type Ux = u32;
}
///`read()` method returns [`prcr::R`](R) reader structure
impl crate::Readable for PRCRrs {}
///`write(|w| ..)` method takes [`prcr::W`](W) writer structure
impl crate::Writable for PRCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRCR to value 0
impl crate::Resettable for PRCRrs {}
