///Register `DMAOMR` reader
pub type R = crate::R<DMAOMRrs>;
///Register `DMAOMR` writer
pub type W = crate::W<DMAOMRrs>;
///Field `SR` reader - SR
pub type SR_R = crate::BitReader;
///Field `SR` writer - SR
pub type SR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSF` reader - OSF
pub type OSF_R = crate::BitReader;
///Field `OSF` writer - OSF
pub type OSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTC` reader - RTC
pub type RTC_R = crate::FieldReader;
///Field `RTC` writer - RTC
pub type RTC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FUGF` reader - FUGF
pub type FUGF_R = crate::BitReader;
///Field `FUGF` writer - FUGF
pub type FUGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FEF` reader - FEF
pub type FEF_R = crate::BitReader;
///Field `FEF` writer - FEF
pub type FEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ST` reader - ST
pub type ST_R = crate::BitReader;
///Field `ST` writer - ST
pub type ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TTC` reader - TTC
pub type TTC_R = crate::FieldReader;
///Field `TTC` writer - TTC
pub type TTC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FTF` reader - FTF
pub type FTF_R = crate::BitReader;
///Field `FTF` writer - FTF
pub type FTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSF` reader - TSF
pub type TSF_R = crate::BitReader;
///Field `TSF` writer - TSF
pub type TSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFRF` reader - DFRF
pub type DFRF_R = crate::BitReader;
///Field `DFRF` writer - DFRF
pub type DFRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSF` reader - RSF
pub type RSF_R = crate::BitReader;
///Field `RSF` writer - RSF
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTCEFD` reader - DTCEFD
pub type DTCEFD_R = crate::BitReader;
///Field `DTCEFD` writer - DTCEFD
pub type DTCEFD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - SR
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - OSF
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - RTC
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 6 - FUGF
    #[inline(always)]
    pub fn fugf(&self) -> FUGF_R {
        FUGF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - FEF
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 13 - ST
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:16 - TTC
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 14) & 7) as u8)
    }
    ///Bit 20 - FTF
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TSF
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - DFRF
    #[inline(always)]
    pub fn dfrf(&self) -> DFRF_R {
        DFRF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - RSF
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - DTCEFD
    #[inline(always)]
    pub fn dtcefd(&self) -> DTCEFD_R {
        DTCEFD_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAOMR")
            .field("sr", &self.sr())
            .field("osf", &self.osf())
            .field("rtc", &self.rtc())
            .field("fugf", &self.fugf())
            .field("fef", &self.fef())
            .field("st", &self.st())
            .field("ttc", &self.ttc())
            .field("ftf", &self.ftf())
            .field("tsf", &self.tsf())
            .field("dfrf", &self.dfrf())
            .field("rsf", &self.rsf())
            .field("dtcefd", &self.dtcefd())
            .finish()
    }
}
impl W {
    ///Bit 1 - SR
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W<'_, DMAOMRrs> {
        SR_W::new(self, 1)
    }
    ///Bit 2 - OSF
    #[inline(always)]
    pub fn osf(&mut self) -> OSF_W<'_, DMAOMRrs> {
        OSF_W::new(self, 2)
    }
    ///Bits 3:4 - RTC
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<'_, DMAOMRrs> {
        RTC_W::new(self, 3)
    }
    ///Bit 6 - FUGF
    #[inline(always)]
    pub fn fugf(&mut self) -> FUGF_W<'_, DMAOMRrs> {
        FUGF_W::new(self, 6)
    }
    ///Bit 7 - FEF
    #[inline(always)]
    pub fn fef(&mut self) -> FEF_W<'_, DMAOMRrs> {
        FEF_W::new(self, 7)
    }
    ///Bit 13 - ST
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<'_, DMAOMRrs> {
        ST_W::new(self, 13)
    }
    ///Bits 14:16 - TTC
    #[inline(always)]
    pub fn ttc(&mut self) -> TTC_W<'_, DMAOMRrs> {
        TTC_W::new(self, 14)
    }
    ///Bit 20 - FTF
    #[inline(always)]
    pub fn ftf(&mut self) -> FTF_W<'_, DMAOMRrs> {
        FTF_W::new(self, 20)
    }
    ///Bit 21 - TSF
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W<'_, DMAOMRrs> {
        TSF_W::new(self, 21)
    }
    ///Bit 24 - DFRF
    #[inline(always)]
    pub fn dfrf(&mut self) -> DFRF_W<'_, DMAOMRrs> {
        DFRF_W::new(self, 24)
    }
    ///Bit 25 - RSF
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<'_, DMAOMRrs> {
        RSF_W::new(self, 25)
    }
    ///Bit 26 - DTCEFD
    #[inline(always)]
    pub fn dtcefd(&mut self) -> DTCEFD_W<'_, DMAOMRrs> {
        DTCEFD_W::new(self, 26)
    }
}
/**Ethernet DMA operation mode register

You can [`read`](crate::Reg::read) this register and get [`dmaomr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaomr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#Ethernet_DMA:DMAOMR)*/
pub struct DMAOMRrs;
impl crate::RegisterSpec for DMAOMRrs {
    type Ux = u32;
}
///`read()` method returns [`dmaomr::R`](R) reader structure
impl crate::Readable for DMAOMRrs {}
///`write(|w| ..)` method takes [`dmaomr::W`](W) writer structure
impl crate::Writable for DMAOMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAOMR to value 0
impl crate::Resettable for DMAOMRrs {}
