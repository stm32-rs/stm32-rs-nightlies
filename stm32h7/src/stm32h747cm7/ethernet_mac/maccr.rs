#[doc = "Register `MACCR` reader"]
pub type R = crate::R<MACCRrs>;
#[doc = "Register `MACCR` writer"]
pub type W = crate::W<MACCRrs>;
#[doc = "Field `RE` reader - Receiver Enable"]
pub type RE_R = crate::BitReader;
#[doc = "Field `RE` writer - Receiver Enable"]
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - Transmitter Enable"]
pub type TE_R = crate::BitReader;
#[doc = "Field `TE` writer - Transmitter Enable"]
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRELEN` reader - Preamble Length for Transmit Packets"]
pub type PRELEN_R = crate::FieldReader;
#[doc = "Field `PRELEN` writer - Preamble Length for Transmit Packets"]
pub type PRELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DC` reader - Deferral Check"]
pub type DC_R = crate::BitReader;
#[doc = "Field `DC` writer - Deferral Check"]
pub type DC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL` reader - Back-Off Limit"]
pub type BL_R = crate::FieldReader;
#[doc = "Field `BL` writer - Back-Off Limit"]
pub type BL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DR` reader - Disable Retry"]
pub type DR_R = crate::BitReader;
#[doc = "Field `DR` writer - Disable Retry"]
pub type DR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRS` reader - Disable Carrier Sense During Transmission"]
pub type DCRS_R = crate::BitReader;
#[doc = "Field `DCRS` writer - Disable Carrier Sense During Transmission"]
pub type DCRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DO` reader - Disable Receive Own"]
pub type DO_R = crate::BitReader;
#[doc = "Field `DO` writer - Disable Receive Own"]
pub type DO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECRSFD` reader - Enable Carrier Sense Before Transmission in Full-Duplex Mode"]
pub type ECRSFD_R = crate::BitReader;
#[doc = "Field `ECRSFD` writer - Enable Carrier Sense Before Transmission in Full-Duplex Mode"]
pub type ECRSFD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LM` reader - Loopback Mode"]
pub type LM_R = crate::BitReader;
#[doc = "Field `LM` writer - Loopback Mode"]
pub type LM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM` reader - Duplex Mode"]
pub type DM_R = crate::BitReader;
#[doc = "Field `DM` writer - Duplex Mode"]
pub type DM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FES` reader - MAC Speed"]
pub type FES_R = crate::BitReader;
#[doc = "Field `FES` writer - MAC Speed"]
pub type FES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JE` reader - Jumbo Packet Enable"]
pub type JE_R = crate::BitReader;
#[doc = "Field `JE` writer - Jumbo Packet Enable"]
pub type JE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JD` reader - Jabber Disable"]
pub type JD_R = crate::BitReader;
#[doc = "Field `JD` writer - Jabber Disable"]
pub type JD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WD` reader - Watchdog Disable"]
pub type WD_R = crate::BitReader;
#[doc = "Field `WD` writer - Watchdog Disable"]
pub type WD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACS` reader - Automatic Pad or CRC Stripping"]
pub type ACS_R = crate::BitReader;
#[doc = "Field `ACS` writer - Automatic Pad or CRC Stripping"]
pub type ACS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CST` reader - CRC stripping for Type packets"]
pub type CST_R = crate::BitReader;
#[doc = "Field `CST` writer - CRC stripping for Type packets"]
pub type CST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2KP` reader - IEEE 802.3as Support for 2K Packets"]
pub type S2KP_R = crate::BitReader;
#[doc = "Field `S2KP` writer - IEEE 802.3as Support for 2K Packets"]
pub type S2KP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPSLCE` reader - Giant Packet Size Limit Control Enable"]
pub type GPSLCE_R = crate::BitReader;
#[doc = "Field `GPSLCE` writer - Giant Packet Size Limit Control Enable"]
pub type GPSLCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPG` reader - Inter-Packet Gap"]
pub type IPG_R = crate::FieldReader;
#[doc = "Field `IPG` writer - Inter-Packet Gap"]
pub type IPG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IPC` reader - Checksum Offload"]
pub type IPC_R = crate::BitReader;
#[doc = "Field `IPC` writer - Checksum Offload"]
pub type IPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARC` reader - Source Address Insertion or Replacement Control"]
pub type SARC_R = crate::FieldReader;
#[doc = "Field `SARC` writer - Source Address Insertion or Replacement Control"]
pub type SARC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ARPEN` reader - ARP Offload Enable"]
pub type ARPEN_R = crate::BitReader;
#[doc = "Field `ARPEN` writer - ARP Offload Enable"]
pub type ARPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Preamble Length for Transmit Packets"]
    #[inline(always)]
    pub fn prelen(&self) -> PRELEN_R {
        PRELEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - Disable Retry"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    pub fn dcrs(&self) -> DCRS_R {
        DCRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Disable Receive Own"]
    #[inline(always)]
    pub fn do_(&self) -> DO_R {
        DO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Carrier Sense Before Transmission in Full-Duplex Mode"]
    #[inline(always)]
    pub fn ecrsfd(&self) -> ECRSFD_R {
        ECRSFD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Duplex Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MAC Speed"]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Jumbo Packet Enable"]
    #[inline(always)]
    pub fn je(&self) -> JE_R {
        JE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Jabber Disable"]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Watchdog Disable"]
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Automatic Pad or CRC Stripping"]
    #[inline(always)]
    pub fn acs(&self) -> ACS_R {
        ACS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CRC stripping for Type packets"]
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - IEEE 802.3as Support for 2K Packets"]
    #[inline(always)]
    pub fn s2kp(&self) -> S2KP_R {
        S2KP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Giant Packet Size Limit Control Enable"]
    #[inline(always)]
    pub fn gpslce(&self) -> GPSLCE_R {
        GPSLCE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Inter-Packet Gap"]
    #[inline(always)]
    pub fn ipg(&self) -> IPG_R {
        IPG_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Checksum Offload"]
    #[inline(always)]
    pub fn ipc(&self) -> IPC_R {
        IPC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Source Address Insertion or Replacement Control"]
    #[inline(always)]
    pub fn sarc(&self) -> SARC_R {
        SARC_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - ARP Offload Enable"]
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<MACCRrs> {
        RE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<MACCRrs> {
        TE_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Preamble Length for Transmit Packets"]
    #[inline(always)]
    #[must_use]
    pub fn prelen(&mut self) -> PRELEN_W<MACCRrs> {
        PRELEN_W::new(self, 2)
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<MACCRrs> {
        DC_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<MACCRrs> {
        BL_W::new(self, 5)
    }
    #[doc = "Bit 8 - Disable Retry"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DR_W<MACCRrs> {
        DR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn dcrs(&mut self) -> DCRS_W<MACCRrs> {
        DCRS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Disable Receive Own"]
    #[inline(always)]
    #[must_use]
    pub fn do_(&mut self) -> DO_W<MACCRrs> {
        DO_W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Carrier Sense Before Transmission in Full-Duplex Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ecrsfd(&mut self) -> ECRSFD_W<MACCRrs> {
        ECRSFD_W::new(self, 11)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<MACCRrs> {
        LM_W::new(self, 12)
    }
    #[doc = "Bit 13 - Duplex Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<MACCRrs> {
        DM_W::new(self, 13)
    }
    #[doc = "Bit 14 - MAC Speed"]
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FES_W<MACCRrs> {
        FES_W::new(self, 14)
    }
    #[doc = "Bit 16 - Jumbo Packet Enable"]
    #[inline(always)]
    #[must_use]
    pub fn je(&mut self) -> JE_W<MACCRrs> {
        JE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Jabber Disable"]
    #[inline(always)]
    #[must_use]
    pub fn jd(&mut self) -> JD_W<MACCRrs> {
        JD_W::new(self, 17)
    }
    #[doc = "Bit 19 - Watchdog Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wd(&mut self) -> WD_W<MACCRrs> {
        WD_W::new(self, 19)
    }
    #[doc = "Bit 20 - Automatic Pad or CRC Stripping"]
    #[inline(always)]
    #[must_use]
    pub fn acs(&mut self) -> ACS_W<MACCRrs> {
        ACS_W::new(self, 20)
    }
    #[doc = "Bit 21 - CRC stripping for Type packets"]
    #[inline(always)]
    #[must_use]
    pub fn cst(&mut self) -> CST_W<MACCRrs> {
        CST_W::new(self, 21)
    }
    #[doc = "Bit 22 - IEEE 802.3as Support for 2K Packets"]
    #[inline(always)]
    #[must_use]
    pub fn s2kp(&mut self) -> S2KP_W<MACCRrs> {
        S2KP_W::new(self, 22)
    }
    #[doc = "Bit 23 - Giant Packet Size Limit Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpslce(&mut self) -> GPSLCE_W<MACCRrs> {
        GPSLCE_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - Inter-Packet Gap"]
    #[inline(always)]
    #[must_use]
    pub fn ipg(&mut self) -> IPG_W<MACCRrs> {
        IPG_W::new(self, 24)
    }
    #[doc = "Bit 27 - Checksum Offload"]
    #[inline(always)]
    #[must_use]
    pub fn ipc(&mut self) -> IPC_W<MACCRrs> {
        IPC_W::new(self, 27)
    }
    #[doc = "Bits 28:30 - Source Address Insertion or Replacement Control"]
    #[inline(always)]
    #[must_use]
    pub fn sarc(&mut self) -> SARC_W<MACCRrs> {
        SARC_W::new(self, 28)
    }
    #[doc = "Bit 31 - ARP Offload Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arpen(&mut self) -> ARPEN_W<MACCRrs> {
        ARPEN_W::new(self, 31)
    }
}
#[doc = "Operating mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACCRrs;
impl crate::RegisterSpec for MACCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maccr::R`](R) reader structure"]
impl crate::Readable for MACCRrs {}
#[doc = "`write(|w| ..)` method takes [`maccr::W`](W) writer structure"]
impl crate::Writable for MACCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACCR to value 0"]
impl crate::Resettable for MACCRrs {
    const RESET_VALUE: u32 = 0;
}
