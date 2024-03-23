#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Field `SFT` reader - Signal Free Time SFT bits are set by software. In the SFT=0x0 configuration the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. * 0x0 ** 2.5 Data-Bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST=1, TXERR=1, TXUDR=1 or TXACKE= 1) ** 4 Data-Bit periods if CEC is the new bus initiator ** 6 Data-Bit periods if CEC is the last bus initiator with successful transmission (TXEOM=1) * 0x1: 0.5 nominal data bit periods * 0x2: 1.5 nominal data bit periods * 0x3: 2.5 nominal data bit periods * 0x4: 3.5 nominal data bit periods * 0x5: 4.5 nominal data bit periods * 0x6: 5.5 nominal data bit periods * 0x7: 6.5 nominal data bit periods"]
pub type SFT_R = crate::FieldReader;
#[doc = "Field `SFT` writer - Signal Free Time SFT bits are set by software. In the SFT=0x0 configuration the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. * 0x0 ** 2.5 Data-Bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST=1, TXERR=1, TXUDR=1 or TXACKE= 1) ** 4 Data-Bit periods if CEC is the new bus initiator ** 6 Data-Bit periods if CEC is the last bus initiator with successful transmission (TXEOM=1) * 0x1: 0.5 nominal data bit periods * 0x2: 1.5 nominal data bit periods * 0x3: 2.5 nominal data bit periods * 0x4: 3.5 nominal data bit periods * 0x5: 4.5 nominal data bit periods * 0x6: 5.5 nominal data bit periods * 0x7: 6.5 nominal data bit periods"]
pub type SFT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Field `RXTOL` reader - Rx-Tolerance The RXTOL bit is set and cleared by software. ** Start-Bit, +/- 200 s rise, +/- 200 s fall. ** Data-Bit: +/- 200 s rise. +/- 350 s fall. ** Start-Bit: +/- 400 s rise, +/- 400 s fall ** Data-Bit: +/-300 s rise, +/- 500 s fall"]
pub type RXTOL_R = crate::BitReader;
#[doc = "Field `RXTOL` writer - Rx-Tolerance The RXTOL bit is set and cleared by software. ** Start-Bit, +/- 200 s rise, +/- 200 s fall. ** Data-Bit: +/- 200 s rise. +/- 350 s fall. ** Start-Bit: +/- 400 s rise, +/- 400 s fall ** Data-Bit: +/-300 s rise, +/- 500 s fall"]
pub type RXTOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRESTP` reader - Rx-Stop on Bit Rising Error The BRESTP bit is set and cleared by software."]
pub type BRESTP_R = crate::BitReader;
#[doc = "Field `BRESTP` writer - Rx-Stop on Bit Rising Error The BRESTP bit is set and cleared by software."]
pub type BRESTP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BREGEN` reader - Generate Error-Bit on Bit Rising Error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN=0, an Error-bit is generated upon BRE detection with BRESTP=1 in broadcast even if BREGEN=0"]
pub type BREGEN_R = crate::BitReader;
#[doc = "Field `BREGEN` writer - Generate Error-Bit on Bit Rising Error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN=0, an Error-bit is generated upon BRE detection with BRESTP=1 in broadcast even if BREGEN=0"]
pub type BREGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBPEGEN` reader - Generate Error-Bit on Long Bit Period Error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN=0, an Error-bit is generated upon LBPE detection in broadcast even if LBPEGEN=0"]
pub type LBPEGEN_R = crate::BitReader;
#[doc = "Field `LBPEGEN` writer - Generate Error-Bit on Long Bit Period Error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN=0, an Error-bit is generated upon LBPE detection in broadcast even if LBPEGEN=0"]
pub type LBPEGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRDNOGEN` reader - Avoid Error-Bit Generation in Broadcast The BRDNOGEN bit is set and cleared by software."]
pub type BRDNOGEN_R = crate::BitReader;
#[doc = "Field `BRDNOGEN` writer - Avoid Error-Bit Generation in Broadcast The BRDNOGEN bit is set and cleared by software."]
pub type BRDNOGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTOPT` reader - SFT Option Bit The SFTOPT bit is set and cleared by software."]
pub type SFTOPT_R = crate::BitReader;
#[doc = "Field `SFTOPT` writer - SFT Option Bit The SFTOPT bit is set and cleared by software."]
pub type SFTOPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OAR` reader - Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN=1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received."]
pub type OAR_R = crate::FieldReader<u16>;
#[doc = "Field `OAR` writer - Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN=1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received."]
pub type OAR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 15, u16>;
#[doc = "Field `LSTN` reader - Listen mode LSTN bit is set and cleared by software."]
pub type LSTN_R = crate::BitReader;
#[doc = "Field `LSTN` writer - Listen mode LSTN bit is set and cleared by software."]
pub type LSTN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Signal Free Time SFT bits are set by software. In the SFT=0x0 configuration the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. * 0x0 ** 2.5 Data-Bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST=1, TXERR=1, TXUDR=1 or TXACKE= 1) ** 4 Data-Bit periods if CEC is the new bus initiator ** 6 Data-Bit periods if CEC is the last bus initiator with successful transmission (TXEOM=1) * 0x1: 0.5 nominal data bit periods * 0x2: 1.5 nominal data bit periods * 0x3: 2.5 nominal data bit periods * 0x4: 3.5 nominal data bit periods * 0x5: 4.5 nominal data bit periods * 0x6: 5.5 nominal data bit periods * 0x7: 6.5 nominal data bit periods"]
    #[inline(always)]
    pub fn sft(&self) -> SFT_R {
        SFT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Rx-Tolerance The RXTOL bit is set and cleared by software. ** Start-Bit, +/- 200 s rise, +/- 200 s fall. ** Data-Bit: +/- 200 s rise. +/- 350 s fall. ** Start-Bit: +/- 400 s rise, +/- 400 s fall ** Data-Bit: +/-300 s rise, +/- 500 s fall"]
    #[inline(always)]
    pub fn rxtol(&self) -> RXTOL_R {
        RXTOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx-Stop on Bit Rising Error The BRESTP bit is set and cleared by software."]
    #[inline(always)]
    pub fn brestp(&self) -> BRESTP_R {
        BRESTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generate Error-Bit on Bit Rising Error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN=0, an Error-bit is generated upon BRE detection with BRESTP=1 in broadcast even if BREGEN=0"]
    #[inline(always)]
    pub fn bregen(&self) -> BREGEN_R {
        BREGEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Generate Error-Bit on Long Bit Period Error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN=0, an Error-bit is generated upon LBPE detection in broadcast even if LBPEGEN=0"]
    #[inline(always)]
    pub fn lbpegen(&self) -> LBPEGEN_R {
        LBPEGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Avoid Error-Bit Generation in Broadcast The BRDNOGEN bit is set and cleared by software."]
    #[inline(always)]
    pub fn brdnogen(&self) -> BRDNOGEN_R {
        BRDNOGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SFT Option Bit The SFTOPT bit is set and cleared by software."]
    #[inline(always)]
    pub fn sftopt(&self) -> SFTOPT_R {
        SFTOPT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:30 - Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN=1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received."]
    #[inline(always)]
    pub fn oar(&self) -> OAR_R {
        OAR_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Listen mode LSTN bit is set and cleared by software."]
    #[inline(always)]
    pub fn lstn(&self) -> LSTN_R {
        LSTN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal Free Time SFT bits are set by software. In the SFT=0x0 configuration the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. * 0x0 ** 2.5 Data-Bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST=1, TXERR=1, TXUDR=1 or TXACKE= 1) ** 4 Data-Bit periods if CEC is the new bus initiator ** 6 Data-Bit periods if CEC is the last bus initiator with successful transmission (TXEOM=1) * 0x1: 0.5 nominal data bit periods * 0x2: 1.5 nominal data bit periods * 0x3: 2.5 nominal data bit periods * 0x4: 3.5 nominal data bit periods * 0x5: 4.5 nominal data bit periods * 0x6: 5.5 nominal data bit periods * 0x7: 6.5 nominal data bit periods"]
    #[inline(always)]
    #[must_use]
    pub fn sft(&mut self) -> SFT_W<CFGRrs> {
        SFT_W::new(self, 0)
    }
    #[doc = "Bit 3 - Rx-Tolerance The RXTOL bit is set and cleared by software. ** Start-Bit, +/- 200 s rise, +/- 200 s fall. ** Data-Bit: +/- 200 s rise. +/- 350 s fall. ** Start-Bit: +/- 400 s rise, +/- 400 s fall ** Data-Bit: +/-300 s rise, +/- 500 s fall"]
    #[inline(always)]
    #[must_use]
    pub fn rxtol(&mut self) -> RXTOL_W<CFGRrs> {
        RXTOL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Rx-Stop on Bit Rising Error The BRESTP bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn brestp(&mut self) -> BRESTP_W<CFGRrs> {
        BRESTP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Generate Error-Bit on Bit Rising Error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN=0, an Error-bit is generated upon BRE detection with BRESTP=1 in broadcast even if BREGEN=0"]
    #[inline(always)]
    #[must_use]
    pub fn bregen(&mut self) -> BREGEN_W<CFGRrs> {
        BREGEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Generate Error-Bit on Long Bit Period Error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN=0, an Error-bit is generated upon LBPE detection in broadcast even if LBPEGEN=0"]
    #[inline(always)]
    #[must_use]
    pub fn lbpegen(&mut self) -> LBPEGEN_W<CFGRrs> {
        LBPEGEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Avoid Error-Bit Generation in Broadcast The BRDNOGEN bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn brdnogen(&mut self) -> BRDNOGEN_W<CFGRrs> {
        BRDNOGEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - SFT Option Bit The SFTOPT bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sftopt(&mut self) -> SFTOPT_W<CFGRrs> {
        SFTOPT_W::new(self, 8)
    }
    #[doc = "Bits 16:30 - Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN=1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received."]
    #[inline(always)]
    #[must_use]
    pub fn oar(&mut self) -> OAR_W<CFGRrs> {
        OAR_W::new(self, 16)
    }
    #[doc = "Bit 31 - Listen mode LSTN bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lstn(&mut self) -> LSTN_W<CFGRrs> {
        LSTN_W::new(self, 31)
    }
}
#[doc = "This register is used to configure the HDMI-CEC controller. It is mandatory to write CEC_CFGR only when CECEN=0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
