///Register `HCINT12` reader
pub type R = crate::R<HCINT12rs>;
///Register `HCINT12` writer
pub type W = crate::W<HCINT12rs>;
///Field `XFRC` reader - XFRC
pub type XFRC_R = crate::BitReader;
///Field `XFRC` writer - XFRC
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHH` reader - CHH
pub type CHH_R = crate::BitReader;
///Field `CHH` writer - CHH
pub type CHH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBERR` reader - AHBERR
pub type AHBERR_R = crate::BitReader;
///Field `AHBERR` writer - AHBERR
pub type AHBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STALL` reader - STALL
pub type STALL_R = crate::BitReader;
///Field `STALL` writer - STALL
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NAK` reader - NAK
pub type NAK_R = crate::BitReader;
///Field `NAK` writer - NAK
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACK` reader - ACK
pub type ACK_R = crate::BitReader;
///Field `ACK` writer - ACK
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NYET` reader - NYET
pub type NYET_R = crate::BitReader;
///Field `NYET` writer - NYET
pub type NYET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXERR` reader - TXERR
pub type TXERR_R = crate::BitReader;
///Field `TXERR` writer - TXERR
pub type TXERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BBERR` reader - BBERR
pub type BBERR_R = crate::BitReader;
///Field `BBERR` writer - BBERR
pub type BBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRMOR` reader - FRMOR
pub type FRMOR_R = crate::BitReader;
///Field `FRMOR` writer - FRMOR
pub type FRMOR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTERR` reader - DTERR
pub type DTERR_R = crate::BitReader;
///Field `DTERR` writer - DTERR
pub type DTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BNA` reader - BNA
pub type BNA_R = crate::BitReader;
///Field `BNA` writer - BNA
pub type BNA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XCSXACTERR` reader - XCSXACTERR
pub type XCSXACTERR_R = crate::BitReader;
///Field `XCSXACTERR` writer - XCSXACTERR
pub type XCSXACTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DESCLSTROLL` reader - DESCLSTROLL
pub type DESCLSTROLL_R = crate::BitReader;
///Field `DESCLSTROLL` writer - DESCLSTROLL
pub type DESCLSTROLL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - XFRC
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CHH
    #[inline(always)]
    pub fn chh(&self) -> CHH_R {
        CHH_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AHBERR
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - STALL
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NAK
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ACK
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - NYET
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXERR
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - BBERR
    #[inline(always)]
    pub fn bberr(&self) -> BBERR_R {
        BBERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - FRMOR
    #[inline(always)]
    pub fn frmor(&self) -> FRMOR_R {
        FRMOR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DTERR
    #[inline(always)]
    pub fn dterr(&self) -> DTERR_R {
        DTERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - BNA
    #[inline(always)]
    pub fn bna(&self) -> BNA_R {
        BNA_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - XCSXACTERR
    #[inline(always)]
    pub fn xcsxacterr(&self) -> XCSXACTERR_R {
        XCSXACTERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DESCLSTROLL
    #[inline(always)]
    pub fn desclstroll(&self) -> DESCLSTROLL_R {
        DESCLSTROLL_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINT12")
            .field("xfrc", &self.xfrc())
            .field("chh", &self.chh())
            .field("ahberr", &self.ahberr())
            .field("stall", &self.stall())
            .field("nak", &self.nak())
            .field("ack", &self.ack())
            .field("nyet", &self.nyet())
            .field("txerr", &self.txerr())
            .field("bberr", &self.bberr())
            .field("frmor", &self.frmor())
            .field("dterr", &self.dterr())
            .field("bna", &self.bna())
            .field("xcsxacterr", &self.xcsxacterr())
            .field("desclstroll", &self.desclstroll())
            .finish()
    }
}
impl W {
    ///Bit 0 - XFRC
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W<'_, HCINT12rs> {
        XFRC_W::new(self, 0)
    }
    ///Bit 1 - CHH
    #[inline(always)]
    pub fn chh(&mut self) -> CHH_W<'_, HCINT12rs> {
        CHH_W::new(self, 1)
    }
    ///Bit 2 - AHBERR
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W<'_, HCINT12rs> {
        AHBERR_W::new(self, 2)
    }
    ///Bit 3 - STALL
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<'_, HCINT12rs> {
        STALL_W::new(self, 3)
    }
    ///Bit 4 - NAK
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W<'_, HCINT12rs> {
        NAK_W::new(self, 4)
    }
    ///Bit 5 - ACK
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W<'_, HCINT12rs> {
        ACK_W::new(self, 5)
    }
    ///Bit 6 - NYET
    #[inline(always)]
    pub fn nyet(&mut self) -> NYET_W<'_, HCINT12rs> {
        NYET_W::new(self, 6)
    }
    ///Bit 7 - TXERR
    #[inline(always)]
    pub fn txerr(&mut self) -> TXERR_W<'_, HCINT12rs> {
        TXERR_W::new(self, 7)
    }
    ///Bit 8 - BBERR
    #[inline(always)]
    pub fn bberr(&mut self) -> BBERR_W<'_, HCINT12rs> {
        BBERR_W::new(self, 8)
    }
    ///Bit 9 - FRMOR
    #[inline(always)]
    pub fn frmor(&mut self) -> FRMOR_W<'_, HCINT12rs> {
        FRMOR_W::new(self, 9)
    }
    ///Bit 10 - DTERR
    #[inline(always)]
    pub fn dterr(&mut self) -> DTERR_W<'_, HCINT12rs> {
        DTERR_W::new(self, 10)
    }
    ///Bit 11 - BNA
    #[inline(always)]
    pub fn bna(&mut self) -> BNA_W<'_, HCINT12rs> {
        BNA_W::new(self, 11)
    }
    ///Bit 12 - XCSXACTERR
    #[inline(always)]
    pub fn xcsxacterr(&mut self) -> XCSXACTERR_W<'_, HCINT12rs> {
        XCSXACTERR_W::new(self, 12)
    }
    ///Bit 13 - DESCLSTROLL
    #[inline(always)]
    pub fn desclstroll(&mut self) -> DESCLSTROLL_W<'_, HCINT12rs> {
        DESCLSTROLL_W::new(self, 13)
    }
}
/**This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`hcint12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#OTG:HCINT12)*/
pub struct HCINT12rs;
impl crate::RegisterSpec for HCINT12rs {
    type Ux = u32;
}
///`read()` method returns [`hcint12::R`](R) reader structure
impl crate::Readable for HCINT12rs {}
///`write(|w| ..)` method takes [`hcint12::W`](W) writer structure
impl crate::Writable for HCINT12rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCINT12 to value 0
impl crate::Resettable for HCINT12rs {}
