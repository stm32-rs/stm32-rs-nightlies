///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `SFT` reader - Signal free time SFT bits are set by software. In the SFT = 0x0 configuration, the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. 0x0 2.5 data-bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST = 1, TXERR = 1, TXUDR = 1 or TXACKE = 1) 4 data-bit periods if CEC is the new bus initiator 6 data-bit periods if CEC is the last bus initiator with successful transmission (TXEOM = 1)
pub type SFT_R = crate::FieldReader;
///Field `SFT` writer - Signal free time SFT bits are set by software. In the SFT = 0x0 configuration, the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. 0x0 2.5 data-bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST = 1, TXERR = 1, TXUDR = 1 or TXACKE = 1) 4 data-bit periods if CEC is the new bus initiator 6 data-bit periods if CEC is the last bus initiator with successful transmission (TXEOM = 1)
pub type SFT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RXTOL` reader - Rx-tolerance The RXTOL bit is set and cleared by software. Start-bit, +/- 200 s rise, +/- 200 s fall Data-bit: +/- 200 s rise. +/- 350 s fall Start-bit: +/- 400 s rise, +/- 400 s fall Data-bit: +/-300 s rise, +/- 500 s fall
pub type RXTOL_R = crate::BitReader;
///Field `RXTOL` writer - Rx-tolerance The RXTOL bit is set and cleared by software. Start-bit, +/- 200 s rise, +/- 200 s fall Data-bit: +/- 200 s rise. +/- 350 s fall Start-bit: +/- 400 s rise, +/- 400 s fall Data-bit: +/-300 s rise, +/- 500 s fall
pub type RXTOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRESTP` reader - Rx-stop on bit rising error The BRESTP bit is set and cleared by software.
pub type BRESTP_R = crate::BitReader;
///Field `BRESTP` writer - Rx-stop on bit rising error The BRESTP bit is set and cleared by software.
pub type BRESTP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BREGEN` reader - Generate error-bit on bit rising error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon BRE detection with BRESTP = 1 in broadcast even if BREGEN = 0.
pub type BREGEN_R = crate::BitReader;
///Field `BREGEN` writer - Generate error-bit on bit rising error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon BRE detection with BRESTP = 1 in broadcast even if BREGEN = 0.
pub type BREGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LBPEGEN` reader - Generate error-bit on long bit period error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon LBPE detection in broadcast even if LBPEGEN = 0.
pub type LBPEGEN_R = crate::BitReader;
///Field `LBPEGEN` writer - Generate error-bit on long bit period error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon LBPE detection in broadcast even if LBPEGEN = 0.
pub type LBPEGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRDNOGEN` reader - Avoid error-bit generation in broadcast The BRDNOGEN bit is set and cleared by software. error-bit on the CEC line. LBPE detection with LBPEGEN = 0 on a broadcast message generates an error-bit on the CEC line.
pub type BRDNOGEN_R = crate::BitReader;
///Field `BRDNOGEN` writer - Avoid error-bit generation in broadcast The BRDNOGEN bit is set and cleared by software. error-bit on the CEC line. LBPE detection with LBPEGEN = 0 on a broadcast message generates an error-bit on the CEC line.
pub type BRDNOGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SFTOP` reader - SFT option bit The SFTOPT bit is set and cleared by software.
pub type SFTOP_R = crate::BitReader;
///Field `SFTOP` writer - SFT option bit The SFTOPT bit is set and cleared by software.
pub type SFTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OAR` reader - Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN = 1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received.
pub type OAR_R = crate::FieldReader<u16>;
///Field `OAR` writer - Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN = 1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received.
pub type OAR_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `LSTN` reader - Listen mode LSTN bit is set and cleared by software.
pub type LSTN_R = crate::BitReader;
///Field `LSTN` writer - Listen mode LSTN bit is set and cleared by software.
pub type LSTN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Signal free time SFT bits are set by software. In the SFT = 0x0 configuration, the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. 0x0 2.5 data-bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST = 1, TXERR = 1, TXUDR = 1 or TXACKE = 1) 4 data-bit periods if CEC is the new bus initiator 6 data-bit periods if CEC is the last bus initiator with successful transmission (TXEOM = 1)
    #[inline(always)]
    pub fn sft(&self) -> SFT_R {
        SFT_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Rx-tolerance The RXTOL bit is set and cleared by software. Start-bit, +/- 200 s rise, +/- 200 s fall Data-bit: +/- 200 s rise. +/- 350 s fall Start-bit: +/- 400 s rise, +/- 400 s fall Data-bit: +/-300 s rise, +/- 500 s fall
    #[inline(always)]
    pub fn rxtol(&self) -> RXTOL_R {
        RXTOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rx-stop on bit rising error The BRESTP bit is set and cleared by software.
    #[inline(always)]
    pub fn brestp(&self) -> BRESTP_R {
        BRESTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Generate error-bit on bit rising error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon BRE detection with BRESTP = 1 in broadcast even if BREGEN = 0.
    #[inline(always)]
    pub fn bregen(&self) -> BREGEN_R {
        BREGEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Generate error-bit on long bit period error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon LBPE detection in broadcast even if LBPEGEN = 0.
    #[inline(always)]
    pub fn lbpegen(&self) -> LBPEGEN_R {
        LBPEGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Avoid error-bit generation in broadcast The BRDNOGEN bit is set and cleared by software. error-bit on the CEC line. LBPE detection with LBPEGEN = 0 on a broadcast message generates an error-bit on the CEC line.
    #[inline(always)]
    pub fn brdnogen(&self) -> BRDNOGEN_R {
        BRDNOGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SFT option bit The SFTOPT bit is set and cleared by software.
    #[inline(always)]
    pub fn sftop(&self) -> SFTOP_R {
        SFTOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 16:30 - Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN = 1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received.
    #[inline(always)]
    pub fn oar(&self) -> OAR_R {
        OAR_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    ///Bit 31 - Listen mode LSTN bit is set and cleared by software.
    #[inline(always)]
    pub fn lstn(&self) -> LSTN_R {
        LSTN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("sft", &self.sft())
            .field("rxtol", &self.rxtol())
            .field("brestp", &self.brestp())
            .field("bregen", &self.bregen())
            .field("lbpegen", &self.lbpegen())
            .field("brdnogen", &self.brdnogen())
            .field("sftop", &self.sftop())
            .field("oar", &self.oar())
            .field("lstn", &self.lstn())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Signal free time SFT bits are set by software. In the SFT = 0x0 configuration, the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. 0x0 2.5 data-bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST = 1, TXERR = 1, TXUDR = 1 or TXACKE = 1) 4 data-bit periods if CEC is the new bus initiator 6 data-bit periods if CEC is the last bus initiator with successful transmission (TXEOM = 1)
    #[inline(always)]
    pub fn sft(&mut self) -> SFT_W<'_, CFGRrs> {
        SFT_W::new(self, 0)
    }
    ///Bit 3 - Rx-tolerance The RXTOL bit is set and cleared by software. Start-bit, +/- 200 s rise, +/- 200 s fall Data-bit: +/- 200 s rise. +/- 350 s fall Start-bit: +/- 400 s rise, +/- 400 s fall Data-bit: +/-300 s rise, +/- 500 s fall
    #[inline(always)]
    pub fn rxtol(&mut self) -> RXTOL_W<'_, CFGRrs> {
        RXTOL_W::new(self, 3)
    }
    ///Bit 4 - Rx-stop on bit rising error The BRESTP bit is set and cleared by software.
    #[inline(always)]
    pub fn brestp(&mut self) -> BRESTP_W<'_, CFGRrs> {
        BRESTP_W::new(self, 4)
    }
    ///Bit 5 - Generate error-bit on bit rising error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon BRE detection with BRESTP = 1 in broadcast even if BREGEN = 0.
    #[inline(always)]
    pub fn bregen(&mut self) -> BREGEN_W<'_, CFGRrs> {
        BREGEN_W::new(self, 5)
    }
    ///Bit 6 - Generate error-bit on long bit period error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon LBPE detection in broadcast even if LBPEGEN = 0.
    #[inline(always)]
    pub fn lbpegen(&mut self) -> LBPEGEN_W<'_, CFGRrs> {
        LBPEGEN_W::new(self, 6)
    }
    ///Bit 7 - Avoid error-bit generation in broadcast The BRDNOGEN bit is set and cleared by software. error-bit on the CEC line. LBPE detection with LBPEGEN = 0 on a broadcast message generates an error-bit on the CEC line.
    #[inline(always)]
    pub fn brdnogen(&mut self) -> BRDNOGEN_W<'_, CFGRrs> {
        BRDNOGEN_W::new(self, 7)
    }
    ///Bit 8 - SFT option bit The SFTOPT bit is set and cleared by software.
    #[inline(always)]
    pub fn sftop(&mut self) -> SFTOP_W<'_, CFGRrs> {
        SFTOP_W::new(self, 8)
    }
    ///Bits 16:30 - Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN = 1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received.
    #[inline(always)]
    pub fn oar(&mut self) -> OAR_W<'_, CFGRrs> {
        OAR_W::new(self, 16)
    }
    ///Bit 31 - Listen mode LSTN bit is set and cleared by software.
    #[inline(always)]
    pub fn lstn(&mut self) -> LSTN_W<'_, CFGRrs> {
        LSTN_W::new(self, 31)
    }
}
/**CEC configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#CEC:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}
