#[doc = "Register `MACCR` reader"]
pub type R = crate::R<MACCRrs>;
#[doc = "Register `MACCR` writer"]
pub type W = crate::W<MACCRrs>;
#[doc = "Field `RE` reader - Receiver enable"]
pub type RE_R = crate::BitReader;
#[doc = "Field `RE` writer - Receiver enable"]
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - Transmitter enable"]
pub type TE_R = crate::BitReader;
#[doc = "Field `TE` writer - Transmitter enable"]
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC` reader - Deferral check"]
pub type DC_R = crate::BitReader;
#[doc = "Field `DC` writer - Deferral check"]
pub type DC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL` reader - Back-off limit"]
pub type BL_R = crate::FieldReader;
#[doc = "Field `BL` writer - Back-off limit"]
pub type BL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `APCS` reader - Automatic pad/CRC stripping"]
pub type APCS_R = crate::BitReader;
#[doc = "Field `APCS` writer - Automatic pad/CRC stripping"]
pub type APCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD` reader - Retry disable"]
pub type RD_R = crate::BitReader;
#[doc = "Field `RD` writer - Retry disable"]
pub type RD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPCO` reader - IPv4 checksum offload"]
pub type IPCO_R = crate::BitReader;
#[doc = "Field `IPCO` writer - IPv4 checksum offload"]
pub type IPCO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM` reader - Duplex mode"]
pub type DM_R = crate::BitReader;
#[doc = "Field `DM` writer - Duplex mode"]
pub type DM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LM` reader - Loopback mode"]
pub type LM_R = crate::BitReader;
#[doc = "Field `LM` writer - Loopback mode"]
pub type LM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROD` reader - Receive own disable"]
pub type ROD_R = crate::BitReader;
#[doc = "Field `ROD` writer - Receive own disable"]
pub type ROD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FES` reader - Fast Ethernet speed"]
pub type FES_R = crate::BitReader;
#[doc = "Field `FES` writer - Fast Ethernet speed"]
pub type FES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSD` reader - Carrier sense disable"]
pub type CSD_R = crate::BitReader;
#[doc = "Field `CSD` writer - Carrier sense disable"]
pub type CSD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFG` reader - Interframe gap"]
pub type IFG_R = crate::FieldReader;
#[doc = "Field `IFG` writer - Interframe gap"]
pub type IFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JD` reader - Jabber disable"]
pub type JD_R = crate::BitReader;
#[doc = "Field `JD` writer - Jabber disable"]
pub type JD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WD` reader - Watchdog disable"]
pub type WD_R = crate::BitReader;
#[doc = "Field `WD` writer - Watchdog disable"]
pub type WD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    pub fn apcs(&self) -> APCS_R {
        APCS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Retry disable"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IPv4 checksum offload"]
    #[inline(always)]
    pub fn ipco(&self) -> IPCO_R {
        IPCO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive own disable"]
    #[inline(always)]
    pub fn rod(&self) -> ROD_R {
        ROD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast Ethernet speed"]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Carrier sense disable"]
    #[inline(always)]
    pub fn csd(&self) -> CSD_R {
        CSD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Interframe gap"]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<MACCRrs> {
        RE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<MACCRrs> {
        TE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<MACCRrs> {
        DC_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<MACCRrs> {
        BL_W::new(self, 5)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    #[must_use]
    pub fn apcs(&mut self) -> APCS_W<MACCRrs> {
        APCS_W::new(self, 7)
    }
    #[doc = "Bit 9 - Retry disable"]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RD_W<MACCRrs> {
        RD_W::new(self, 9)
    }
    #[doc = "Bit 10 - IPv4 checksum offload"]
    #[inline(always)]
    #[must_use]
    pub fn ipco(&mut self) -> IPCO_W<MACCRrs> {
        IPCO_W::new(self, 10)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<MACCRrs> {
        DM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<MACCRrs> {
        LM_W::new(self, 12)
    }
    #[doc = "Bit 13 - Receive own disable"]
    #[inline(always)]
    #[must_use]
    pub fn rod(&mut self) -> ROD_W<MACCRrs> {
        ROD_W::new(self, 13)
    }
    #[doc = "Bit 14 - Fast Ethernet speed"]
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FES_W<MACCRrs> {
        FES_W::new(self, 14)
    }
    #[doc = "Bit 16 - Carrier sense disable"]
    #[inline(always)]
    #[must_use]
    pub fn csd(&mut self) -> CSD_W<MACCRrs> {
        CSD_W::new(self, 16)
    }
    #[doc = "Bits 17:19 - Interframe gap"]
    #[inline(always)]
    #[must_use]
    pub fn ifg(&mut self) -> IFG_W<MACCRrs> {
        IFG_W::new(self, 17)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    #[must_use]
    pub fn jd(&mut self) -> JD_W<MACCRrs> {
        JD_W::new(self, 22)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    #[must_use]
    pub fn wd(&mut self) -> WD_W<MACCRrs> {
        WD_W::new(self, 23)
    }
}
#[doc = "Ethernet MAC configuration register (ETH_MACCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets MACCR to value 0x8000"]
impl crate::Resettable for MACCRrs {
    const RESET_VALUE: u32 = 0x8000;
}
