///Register `GINTMSK_ALTERNATE` reader
pub type R = crate::R<GINTMSK_ALTERNATErs>;
///Register `GINTMSK_ALTERNATE` writer
pub type W = crate::W<GINTMSK_ALTERNATErs>;
///Field `MMISM` reader - Mode mismatch interrupt mask
pub type MMISM_R = crate::BitReader;
///Field `MMISM` writer - Mode mismatch interrupt mask
pub type MMISM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGINT` reader - OTG interrupt mask
pub type OTGINT_R = crate::BitReader;
///Field `OTGINT` writer - OTG interrupt mask
pub type OTGINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOFM` reader - Start of frame mask
pub type SOFM_R = crate::BitReader;
///Field `SOFM` writer - Start of frame mask
pub type SOFM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFLVLM` reader - Receive FIFO non-empty mask
pub type RXFLVLM_R = crate::BitReader;
///Field `RXFLVLM` writer - Receive FIFO non-empty mask
pub type RXFLVLM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GINAKEFFM` reader - Global non-periodic IN NAK effective mask
pub type GINAKEFFM_R = crate::BitReader;
///Field `GINAKEFFM` writer - Global non-periodic IN NAK effective mask
pub type GINAKEFFM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GONAKEFFM` reader - Global OUT NAK effective mask
pub type GONAKEFFM_R = crate::BitReader;
///Field `GONAKEFFM` writer - Global OUT NAK effective mask
pub type GONAKEFFM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ESUSPM` reader - Early suspend mask
pub type ESUSPM_R = crate::BitReader;
///Field `ESUSPM` writer - Early suspend mask
pub type ESUSPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBSUSPM` reader - USB suspend mask
pub type USBSUSPM_R = crate::BitReader;
///Field `USBSUSPM` writer - USB suspend mask
pub type USBSUSPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBRST` reader - USB reset mask
pub type USBRST_R = crate::BitReader;
///Field `USBRST` writer - USB reset mask
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENUMDNEM` reader - Enumeration done mask
pub type ENUMDNEM_R = crate::BitReader;
///Field `ENUMDNEM` writer - Enumeration done mask
pub type ENUMDNEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ISOODRPM` reader - Isochronous OUT packet dropped interrupt mask
pub type ISOODRPM_R = crate::BitReader;
///Field `ISOODRPM` writer - Isochronous OUT packet dropped interrupt mask
pub type ISOODRPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOPFM` reader - End of periodic frame interrupt mask
pub type EOPFM_R = crate::BitReader;
///Field `EOPFM` writer - End of periodic frame interrupt mask
pub type EOPFM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IEPINT` reader - IN endpoints interrupt mask
pub type IEPINT_R = crate::BitReader;
///Field `IEPINT` writer - IN endpoints interrupt mask
pub type IEPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OEPINT` reader - OUT endpoints interrupt mask
pub type OEPINT_R = crate::BitReader;
///Field `OEPINT` writer - OUT endpoints interrupt mask
pub type OEPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IISOIXFRM` reader - Incomplete isochronous IN transfer mask
pub type IISOIXFRM_R = crate::BitReader;
///Field `IISOIXFRM` writer - Incomplete isochronous IN transfer mask
pub type IISOIXFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IISOOXFRM` reader - Incomplete isochronous OUT transfer mask
pub type IISOOXFRM_R = crate::BitReader;
///Field `IISOOXFRM` writer - Incomplete isochronous OUT transfer mask
pub type IISOOXFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSUSPM` reader - Data fetch suspended mask
pub type FSUSPM_R = crate::BitReader;
///Field `FSUSPM` writer - Data fetch suspended mask
pub type FSUSPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTDETM` reader - Reset detected interrupt mask
pub type RSTDETM_R = crate::BitReader;
///Field `RSTDETM` writer - Reset detected interrupt mask
pub type RSTDETM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPMINTM` reader - LPM interrupt mask
pub type LPMINTM_R = crate::BitReader;
///Field `LPMINTM` writer - LPM interrupt mask
pub type LPMINTM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIDSCHGM` reader - Connector ID status change mask
pub type CIDSCHGM_R = crate::BitReader;
///Field `CIDSCHGM` writer - Connector ID status change mask
pub type CIDSCHGM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRQIM` reader - Session request/new session detected interrupt mask
pub type SRQIM_R = crate::BitReader;
///Field `SRQIM` writer - Session request/new session detected interrupt mask
pub type SRQIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUIM` reader - Resume/remote wake-up detected interrupt mask
pub type WUIM_R = crate::BitReader;
///Field `WUIM` writer - Resume/remote wake-up detected interrupt mask
pub type WUIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Mode mismatch interrupt mask
    #[inline(always)]
    pub fn mmism(&self) -> MMISM_R {
        MMISM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - OTG interrupt mask
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Start of frame mask
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Receive FIFO non-empty mask
    #[inline(always)]
    pub fn rxflvlm(&self) -> RXFLVLM_R {
        RXFLVLM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Global non-periodic IN NAK effective mask
    #[inline(always)]
    pub fn ginakeffm(&self) -> GINAKEFFM_R {
        GINAKEFFM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Global OUT NAK effective mask
    #[inline(always)]
    pub fn gonakeffm(&self) -> GONAKEFFM_R {
        GONAKEFFM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 10 - Early suspend mask
    #[inline(always)]
    pub fn esuspm(&self) -> ESUSPM_R {
        ESUSPM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - USB suspend mask
    #[inline(always)]
    pub fn usbsuspm(&self) -> USBSUSPM_R {
        USBSUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - USB reset mask
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Enumeration done mask
    #[inline(always)]
    pub fn enumdnem(&self) -> ENUMDNEM_R {
        ENUMDNEM_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Isochronous OUT packet dropped interrupt mask
    #[inline(always)]
    pub fn isoodrpm(&self) -> ISOODRPM_R {
        ISOODRPM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - End of periodic frame interrupt mask
    #[inline(always)]
    pub fn eopfm(&self) -> EOPFM_R {
        EOPFM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 18 - IN endpoints interrupt mask
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - OUT endpoints interrupt mask
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Incomplete isochronous IN transfer mask
    #[inline(always)]
    pub fn iisoixfrm(&self) -> IISOIXFRM_R {
        IISOIXFRM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Incomplete isochronous OUT transfer mask
    #[inline(always)]
    pub fn iisooxfrm(&self) -> IISOOXFRM_R {
        IISOOXFRM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Data fetch suspended mask
    #[inline(always)]
    pub fn fsuspm(&self) -> FSUSPM_R {
        FSUSPM_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Reset detected interrupt mask
    #[inline(always)]
    pub fn rstdetm(&self) -> RSTDETM_R {
        RSTDETM_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 27 - LPM interrupt mask
    #[inline(always)]
    pub fn lpmintm(&self) -> LPMINTM_R {
        LPMINTM_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Connector ID status change mask
    #[inline(always)]
    pub fn cidschgm(&self) -> CIDSCHGM_R {
        CIDSCHGM_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - Session request/new session detected interrupt mask
    #[inline(always)]
    pub fn srqim(&self) -> SRQIM_R {
        SRQIM_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Resume/remote wake-up detected interrupt mask
    #[inline(always)]
    pub fn wuim(&self) -> WUIM_R {
        WUIM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GINTMSK_ALTERNATE")
            .field("mmism", &self.mmism())
            .field("otgint", &self.otgint())
            .field("sofm", &self.sofm())
            .field("rxflvlm", &self.rxflvlm())
            .field("ginakeffm", &self.ginakeffm())
            .field("gonakeffm", &self.gonakeffm())
            .field("esuspm", &self.esuspm())
            .field("usbsuspm", &self.usbsuspm())
            .field("usbrst", &self.usbrst())
            .field("enumdnem", &self.enumdnem())
            .field("isoodrpm", &self.isoodrpm())
            .field("eopfm", &self.eopfm())
            .field("iepint", &self.iepint())
            .field("oepint", &self.oepint())
            .field("iisoixfrm", &self.iisoixfrm())
            .field("iisooxfrm", &self.iisooxfrm())
            .field("fsuspm", &self.fsuspm())
            .field("rstdetm", &self.rstdetm())
            .field("lpmintm", &self.lpmintm())
            .field("cidschgm", &self.cidschgm())
            .field("srqim", &self.srqim())
            .field("wuim", &self.wuim())
            .finish()
    }
}
impl W {
    ///Bit 1 - Mode mismatch interrupt mask
    #[inline(always)]
    pub fn mmism(&mut self) -> MMISM_W<GINTMSK_ALTERNATErs> {
        MMISM_W::new(self, 1)
    }
    ///Bit 2 - OTG interrupt mask
    #[inline(always)]
    pub fn otgint(&mut self) -> OTGINT_W<GINTMSK_ALTERNATErs> {
        OTGINT_W::new(self, 2)
    }
    ///Bit 3 - Start of frame mask
    #[inline(always)]
    pub fn sofm(&mut self) -> SOFM_W<GINTMSK_ALTERNATErs> {
        SOFM_W::new(self, 3)
    }
    ///Bit 4 - Receive FIFO non-empty mask
    #[inline(always)]
    pub fn rxflvlm(&mut self) -> RXFLVLM_W<GINTMSK_ALTERNATErs> {
        RXFLVLM_W::new(self, 4)
    }
    ///Bit 6 - Global non-periodic IN NAK effective mask
    #[inline(always)]
    pub fn ginakeffm(&mut self) -> GINAKEFFM_W<GINTMSK_ALTERNATErs> {
        GINAKEFFM_W::new(self, 6)
    }
    ///Bit 7 - Global OUT NAK effective mask
    #[inline(always)]
    pub fn gonakeffm(&mut self) -> GONAKEFFM_W<GINTMSK_ALTERNATErs> {
        GONAKEFFM_W::new(self, 7)
    }
    ///Bit 10 - Early suspend mask
    #[inline(always)]
    pub fn esuspm(&mut self) -> ESUSPM_W<GINTMSK_ALTERNATErs> {
        ESUSPM_W::new(self, 10)
    }
    ///Bit 11 - USB suspend mask
    #[inline(always)]
    pub fn usbsuspm(&mut self) -> USBSUSPM_W<GINTMSK_ALTERNATErs> {
        USBSUSPM_W::new(self, 11)
    }
    ///Bit 12 - USB reset mask
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W<GINTMSK_ALTERNATErs> {
        USBRST_W::new(self, 12)
    }
    ///Bit 13 - Enumeration done mask
    #[inline(always)]
    pub fn enumdnem(&mut self) -> ENUMDNEM_W<GINTMSK_ALTERNATErs> {
        ENUMDNEM_W::new(self, 13)
    }
    ///Bit 14 - Isochronous OUT packet dropped interrupt mask
    #[inline(always)]
    pub fn isoodrpm(&mut self) -> ISOODRPM_W<GINTMSK_ALTERNATErs> {
        ISOODRPM_W::new(self, 14)
    }
    ///Bit 15 - End of periodic frame interrupt mask
    #[inline(always)]
    pub fn eopfm(&mut self) -> EOPFM_W<GINTMSK_ALTERNATErs> {
        EOPFM_W::new(self, 15)
    }
    ///Bit 18 - IN endpoints interrupt mask
    #[inline(always)]
    pub fn iepint(&mut self) -> IEPINT_W<GINTMSK_ALTERNATErs> {
        IEPINT_W::new(self, 18)
    }
    ///Bit 19 - OUT endpoints interrupt mask
    #[inline(always)]
    pub fn oepint(&mut self) -> OEPINT_W<GINTMSK_ALTERNATErs> {
        OEPINT_W::new(self, 19)
    }
    ///Bit 20 - Incomplete isochronous IN transfer mask
    #[inline(always)]
    pub fn iisoixfrm(&mut self) -> IISOIXFRM_W<GINTMSK_ALTERNATErs> {
        IISOIXFRM_W::new(self, 20)
    }
    ///Bit 21 - Incomplete isochronous OUT transfer mask
    #[inline(always)]
    pub fn iisooxfrm(&mut self) -> IISOOXFRM_W<GINTMSK_ALTERNATErs> {
        IISOOXFRM_W::new(self, 21)
    }
    ///Bit 22 - Data fetch suspended mask
    #[inline(always)]
    pub fn fsuspm(&mut self) -> FSUSPM_W<GINTMSK_ALTERNATErs> {
        FSUSPM_W::new(self, 22)
    }
    ///Bit 23 - Reset detected interrupt mask
    #[inline(always)]
    pub fn rstdetm(&mut self) -> RSTDETM_W<GINTMSK_ALTERNATErs> {
        RSTDETM_W::new(self, 23)
    }
    ///Bit 27 - LPM interrupt mask
    #[inline(always)]
    pub fn lpmintm(&mut self) -> LPMINTM_W<GINTMSK_ALTERNATErs> {
        LPMINTM_W::new(self, 27)
    }
    ///Bit 28 - Connector ID status change mask
    #[inline(always)]
    pub fn cidschgm(&mut self) -> CIDSCHGM_W<GINTMSK_ALTERNATErs> {
        CIDSCHGM_W::new(self, 28)
    }
    ///Bit 30 - Session request/new session detected interrupt mask
    #[inline(always)]
    pub fn srqim(&mut self) -> SRQIM_W<GINTMSK_ALTERNATErs> {
        SRQIM_W::new(self, 30)
    }
    ///Bit 31 - Resume/remote wake-up detected interrupt mask
    #[inline(always)]
    pub fn wuim(&mut self) -> WUIM_W<GINTMSK_ALTERNATErs> {
        WUIM_W::new(self, 31)
    }
}
/**OTG interrupt mask register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`gintmsk_alternate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk_alternate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#OTG1:GINTMSK_ALTERNATE)*/
pub struct GINTMSK_ALTERNATErs;
impl crate::RegisterSpec for GINTMSK_ALTERNATErs {
    type Ux = u32;
}
///`read()` method returns [`gintmsk_alternate::R`](R) reader structure
impl crate::Readable for GINTMSK_ALTERNATErs {}
///`write(|w| ..)` method takes [`gintmsk_alternate::W`](W) writer structure
impl crate::Writable for GINTMSK_ALTERNATErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GINTMSK_ALTERNATE to value 0
impl crate::Resettable for GINTMSK_ALTERNATErs {}
