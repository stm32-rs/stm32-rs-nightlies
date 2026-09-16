///Register `MACFFR` reader
pub type R = crate::R<MACFFRrs>;
///Register `MACFFR` writer
pub type W = crate::W<MACFFRrs>;
///Field `PM` reader - Promiscuous mode
pub type PM_R = crate::BitReader;
///Field `PM` writer - Promiscuous mode
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HU` reader - Hash unicast
pub type HU_R = crate::BitReader;
///Field `HU` writer - Hash unicast
pub type HU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HM` reader - Hash multicast
pub type HM_R = crate::BitReader;
///Field `HM` writer - Hash multicast
pub type HM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAIF` reader - Destination address inverse filtering
pub type DAIF_R = crate::BitReader;
///Field `DAIF` writer - Destination address inverse filtering
pub type DAIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PAM` reader - Pass all multicast
pub type PAM_R = crate::BitReader;
///Field `PAM` writer - Pass all multicast
pub type PAM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BFD` reader - Broadcast frames disable
pub type BFD_R = crate::BitReader;
///Field `BFD` writer - Broadcast frames disable
pub type BFD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PCF` reader - Pass control frames
pub type PCF_R = crate::FieldReader;
///Field `PCF` writer - Pass control frames
pub type PCF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SAIF` reader - Source address inverse filtering
pub type SAIF_R = crate::BitReader;
///Field `SAIF` writer - Source address inverse filtering
pub type SAIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAF` reader - Source address filter
pub type SAF_R = crate::BitReader;
///Field `SAF` writer - Source address filter
pub type SAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPF` reader - Hash or perfect filter
pub type HPF_R = crate::BitReader;
///Field `HPF` writer - Hash or perfect filter
pub type HPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RA` reader - Receive all
pub type RA_R = crate::BitReader;
///Field `RA` writer - Receive all
pub type RA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Promiscuous mode
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Hash unicast
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Hash multicast
    #[inline(always)]
    pub fn hm(&self) -> HM_R {
        HM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Destination address inverse filtering
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Pass all multicast
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Broadcast frames disable
    #[inline(always)]
    pub fn bfd(&self) -> BFD_R {
        BFD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Pass control frames
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - Source address inverse filtering
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Source address filter
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Hash or perfect filter
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 31 - Receive all
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACFFR")
            .field("pm", &self.pm())
            .field("hu", &self.hu())
            .field("hm", &self.hm())
            .field("daif", &self.daif())
            .field("pam", &self.pam())
            .field("bfd", &self.bfd())
            .field("pcf", &self.pcf())
            .field("saif", &self.saif())
            .field("saf", &self.saf())
            .field("hpf", &self.hpf())
            .field("ra", &self.ra())
            .finish()
    }
}
impl W {
    ///Bit 0 - Promiscuous mode
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<'_, MACFFRrs> {
        PM_W::new(self, 0)
    }
    ///Bit 1 - Hash unicast
    #[inline(always)]
    pub fn hu(&mut self) -> HU_W<'_, MACFFRrs> {
        HU_W::new(self, 1)
    }
    ///Bit 2 - Hash multicast
    #[inline(always)]
    pub fn hm(&mut self) -> HM_W<'_, MACFFRrs> {
        HM_W::new(self, 2)
    }
    ///Bit 3 - Destination address inverse filtering
    #[inline(always)]
    pub fn daif(&mut self) -> DAIF_W<'_, MACFFRrs> {
        DAIF_W::new(self, 3)
    }
    ///Bit 4 - Pass all multicast
    #[inline(always)]
    pub fn pam(&mut self) -> PAM_W<'_, MACFFRrs> {
        PAM_W::new(self, 4)
    }
    ///Bit 5 - Broadcast frames disable
    #[inline(always)]
    pub fn bfd(&mut self) -> BFD_W<'_, MACFFRrs> {
        BFD_W::new(self, 5)
    }
    ///Bits 6:7 - Pass control frames
    #[inline(always)]
    pub fn pcf(&mut self) -> PCF_W<'_, MACFFRrs> {
        PCF_W::new(self, 6)
    }
    ///Bit 8 - Source address inverse filtering
    #[inline(always)]
    pub fn saif(&mut self) -> SAIF_W<'_, MACFFRrs> {
        SAIF_W::new(self, 8)
    }
    ///Bit 9 - Source address filter
    #[inline(always)]
    pub fn saf(&mut self) -> SAF_W<'_, MACFFRrs> {
        SAF_W::new(self, 9)
    }
    ///Bit 10 - Hash or perfect filter
    #[inline(always)]
    pub fn hpf(&mut self) -> HPF_W<'_, MACFFRrs> {
        HPF_W::new(self, 10)
    }
    ///Bit 31 - Receive all
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W<'_, MACFFRrs> {
        RA_W::new(self, 31)
    }
}
/**Ethernet MAC frame filter register (ETH_MACCFFR)

You can [`read`](crate::Reg::read) this register and get [`macffr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macffr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#Ethernet_MAC:MACFFR)*/
pub struct MACFFRrs;
impl crate::RegisterSpec for MACFFRrs {
    type Ux = u32;
}
///`read()` method returns [`macffr::R`](R) reader structure
impl crate::Readable for MACFFRrs {}
///`write(|w| ..)` method takes [`macffr::W`](W) writer structure
impl crate::Writable for MACFFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACFFR to value 0
impl crate::Resettable for MACFFRrs {}
