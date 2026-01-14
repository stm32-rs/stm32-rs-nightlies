///Register `HCINT15` reader
pub type R = crate::R<HCINT15rs>;
///Register `HCINT15` writer
pub type W = crate::W<HCINT15rs>;
///Field `XFRC` reader - Transfer completed. Transfer completed normally without any errors.
pub type XFRC_R = crate::BitReader;
///Field `XFRC` writer - Transfer completed. Transfer completed normally without any errors.
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHH` reader - Channel halted. Indicates the transfer completed abnormally either because of any USB transaction error or in response to disable request by the application.
pub type CHH_R = crate::BitReader;
///Field `CHH` writer - Channel halted. Indicates the transfer completed abnormally either because of any USB transaction error or in response to disable request by the application.
pub type CHH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBERR` reader - AHB error This error is generated only in Internal DMA mode when an AHB error occurs during an AHB read/write operation. The application can read the corresponding DMA channel address register to get the error address.
pub type AHBERR_R = crate::BitReader;
///Field `AHBERR` writer - AHB error This error is generated only in Internal DMA mode when an AHB error occurs during an AHB read/write operation. The application can read the corresponding DMA channel address register to get the error address.
pub type AHBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STALL` reader - STALL response received interrupt.
pub type STALL_R = crate::BitReader;
///Field `STALL` writer - STALL response received interrupt.
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NAK` reader - NAK response received interrupt.
pub type NAK_R = crate::BitReader;
///Field `NAK` writer - NAK response received interrupt.
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACK` reader - ACK response received/transmitted interrupt.
pub type ACK_R = crate::BitReader;
///Field `ACK` writer - ACK response received/transmitted interrupt.
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NYET` reader - Not yet ready response received interrupt.
pub type NYET_R = crate::BitReader;
///Field `NYET` writer - Not yet ready response received interrupt.
pub type NYET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXERR` reader - Transaction error. Indicates one of the following errors occurred on the USB. CRC check failure Timeout Bit stuff error False EOP
pub type TXERR_R = crate::BitReader;
///Field `TXERR` writer - Transaction error. Indicates one of the following errors occurred on the USB. CRC check failure Timeout Bit stuff error False EOP
pub type TXERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BBERR` reader - Babble error.
pub type BBERR_R = crate::BitReader;
///Field `BBERR` writer - Babble error.
pub type BBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRMOR` reader - Frame overrun.
pub type FRMOR_R = crate::BitReader;
///Field `FRMOR` writer - Frame overrun.
pub type FRMOR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTERR` reader - Data toggle error.
pub type DTERR_R = crate::BitReader;
///Field `DTERR` writer - Data toggle error.
pub type DTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Transfer completed. Transfer completed normally without any errors.
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel halted. Indicates the transfer completed abnormally either because of any USB transaction error or in response to disable request by the application.
    #[inline(always)]
    pub fn chh(&self) -> CHH_R {
        CHH_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AHB error This error is generated only in Internal DMA mode when an AHB error occurs during an AHB read/write operation. The application can read the corresponding DMA channel address register to get the error address.
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - STALL response received interrupt.
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NAK response received interrupt.
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ACK response received/transmitted interrupt.
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Not yet ready response received interrupt.
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transaction error. Indicates one of the following errors occurred on the USB. CRC check failure Timeout Bit stuff error False EOP
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Babble error.
    #[inline(always)]
    pub fn bberr(&self) -> BBERR_R {
        BBERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Frame overrun.
    #[inline(always)]
    pub fn frmor(&self) -> FRMOR_R {
        FRMOR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Data toggle error.
    #[inline(always)]
    pub fn dterr(&self) -> DTERR_R {
        DTERR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINT15")
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
            .finish()
    }
}
impl W {
    ///Bit 0 - Transfer completed. Transfer completed normally without any errors.
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W<'_, HCINT15rs> {
        XFRC_W::new(self, 0)
    }
    ///Bit 1 - Channel halted. Indicates the transfer completed abnormally either because of any USB transaction error or in response to disable request by the application.
    #[inline(always)]
    pub fn chh(&mut self) -> CHH_W<'_, HCINT15rs> {
        CHH_W::new(self, 1)
    }
    ///Bit 2 - AHB error This error is generated only in Internal DMA mode when an AHB error occurs during an AHB read/write operation. The application can read the corresponding DMA channel address register to get the error address.
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W<'_, HCINT15rs> {
        AHBERR_W::new(self, 2)
    }
    ///Bit 3 - STALL response received interrupt.
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<'_, HCINT15rs> {
        STALL_W::new(self, 3)
    }
    ///Bit 4 - NAK response received interrupt.
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W<'_, HCINT15rs> {
        NAK_W::new(self, 4)
    }
    ///Bit 5 - ACK response received/transmitted interrupt.
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W<'_, HCINT15rs> {
        ACK_W::new(self, 5)
    }
    ///Bit 6 - Not yet ready response received interrupt.
    #[inline(always)]
    pub fn nyet(&mut self) -> NYET_W<'_, HCINT15rs> {
        NYET_W::new(self, 6)
    }
    ///Bit 7 - Transaction error. Indicates one of the following errors occurred on the USB. CRC check failure Timeout Bit stuff error False EOP
    #[inline(always)]
    pub fn txerr(&mut self) -> TXERR_W<'_, HCINT15rs> {
        TXERR_W::new(self, 7)
    }
    ///Bit 8 - Babble error.
    #[inline(always)]
    pub fn bberr(&mut self) -> BBERR_W<'_, HCINT15rs> {
        BBERR_W::new(self, 8)
    }
    ///Bit 9 - Frame overrun.
    #[inline(always)]
    pub fn frmor(&mut self) -> FRMOR_W<'_, HCINT15rs> {
        FRMOR_W::new(self, 9)
    }
    ///Bit 10 - Data toggle error.
    #[inline(always)]
    pub fn dterr(&mut self) -> DTERR_W<'_, HCINT15rs> {
        DTERR_W::new(self, 10)
    }
}
/**OTG host channel 15 interrupt register

You can [`read`](crate::Reg::read) this register and get [`hcint15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#OTG_HS:HCINT15)*/
pub struct HCINT15rs;
impl crate::RegisterSpec for HCINT15rs {
    type Ux = u32;
}
///`read()` method returns [`hcint15::R`](R) reader structure
impl crate::Readable for HCINT15rs {}
///`write(|w| ..)` method takes [`hcint15::W`](W) writer structure
impl crate::Writable for HCINT15rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCINT15 to value 0
impl crate::Resettable for HCINT15rs {}
