///Register `PTPTSCR` reader
pub type R = crate::R<PTPTSCRrs>;
///Register `PTPTSCR` writer
pub type W = crate::W<PTPTSCRrs>;
///Field `TSE` reader - Time stamp enable
pub type TSE_R = crate::BitReader;
///Field `TSE` writer - Time stamp enable
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSFCU` reader - Time stamp fine or coarse update
pub type TSFCU_R = crate::BitReader;
///Field `TSFCU` writer - Time stamp fine or coarse update
pub type TSFCU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSTI` reader - Time stamp system time initialize
pub type TSSTI_R = crate::BitReader;
///Field `TSSTI` writer - Time stamp system time initialize
pub type TSSTI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSTU` reader - Time stamp system time update
pub type TSSTU_R = crate::BitReader;
///Field `TSSTU` writer - Time stamp system time update
pub type TSSTU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSITE` reader - Time stamp interrupt trigger enable
pub type TSITE_R = crate::BitReader;
///Field `TSITE` writer - Time stamp interrupt trigger enable
pub type TSITE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TTSARU` reader - Time stamp addend register update
pub type TTSARU_R = crate::BitReader;
///Field `TTSARU` writer - Time stamp addend register update
pub type TTSARU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSARFE` reader - Time stamp snapshot for all received frames enable
pub type TSSARFE_R = crate::BitReader;
///Field `TSSARFE` writer - Time stamp snapshot for all received frames enable
pub type TSSARFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSSR` reader - Time stamp subsecond rollover: digital or binary rollover control
pub type TSSSR_R = crate::BitReader;
///Field `TSSSR` writer - Time stamp subsecond rollover: digital or binary rollover control
pub type TSSSR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSPTPPSV2E` reader - Time stamp PTP packet snooping for version2 format enable
pub type TSPTPPSV2E_R = crate::BitReader;
///Field `TSPTPPSV2E` writer - Time stamp PTP packet snooping for version2 format enable
pub type TSPTPPSV2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSPTPOEFE` reader - Time stamp snapshot for PTP over ethernet frames enable
pub type TSSPTPOEFE_R = crate::BitReader;
///Field `TSSPTPOEFE` writer - Time stamp snapshot for PTP over ethernet frames enable
pub type TSSPTPOEFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSIPV6FE` reader - Time stamp snapshot for IPv6 frames enable
pub type TSSIPV6FE_R = crate::BitReader;
///Field `TSSIPV6FE` writer - Time stamp snapshot for IPv6 frames enable
pub type TSSIPV6FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSIPV4FE` reader - Time stamp snapshot for IPv4 frames enable
pub type TSSIPV4FE_R = crate::BitReader;
///Field `TSSIPV4FE` writer - Time stamp snapshot for IPv4 frames enable
pub type TSSIPV4FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSEME` reader - Time stamp snapshot for event message enable
pub type TSSEME_R = crate::BitReader;
///Field `TSSEME` writer - Time stamp snapshot for event message enable
pub type TSSEME_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSMRME` reader - Time stamp snapshot for message relevant to master enable
pub type TSSMRME_R = crate::BitReader;
///Field `TSSMRME` writer - Time stamp snapshot for message relevant to master enable
pub type TSSMRME_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSCNT` reader - Time stamp clock node type
pub type TSCNT_R = crate::FieldReader;
///Field `TSCNT` writer - Time stamp clock node type
pub type TSCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TSPFFMAE` reader - Time stamp PTP frame filtering MAC address enable
pub type TSPFFMAE_R = crate::BitReader;
///Field `TSPFFMAE` writer - Time stamp PTP frame filtering MAC address enable
pub type TSPFFMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Time stamp enable
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Time stamp fine or coarse update
    #[inline(always)]
    pub fn tsfcu(&self) -> TSFCU_R {
        TSFCU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Time stamp system time initialize
    #[inline(always)]
    pub fn tssti(&self) -> TSSTI_R {
        TSSTI_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Time stamp system time update
    #[inline(always)]
    pub fn tsstu(&self) -> TSSTU_R {
        TSSTU_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Time stamp interrupt trigger enable
    #[inline(always)]
    pub fn tsite(&self) -> TSITE_R {
        TSITE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Time stamp addend register update
    #[inline(always)]
    pub fn ttsaru(&self) -> TTSARU_R {
        TTSARU_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Time stamp snapshot for all received frames enable
    #[inline(always)]
    pub fn tssarfe(&self) -> TSSARFE_R {
        TSSARFE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Time stamp subsecond rollover: digital or binary rollover control
    #[inline(always)]
    pub fn tsssr(&self) -> TSSSR_R {
        TSSSR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Time stamp PTP packet snooping for version2 format enable
    #[inline(always)]
    pub fn tsptppsv2e(&self) -> TSPTPPSV2E_R {
        TSPTPPSV2E_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Time stamp snapshot for PTP over ethernet frames enable
    #[inline(always)]
    pub fn tssptpoefe(&self) -> TSSPTPOEFE_R {
        TSSPTPOEFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Time stamp snapshot for IPv6 frames enable
    #[inline(always)]
    pub fn tssipv6fe(&self) -> TSSIPV6FE_R {
        TSSIPV6FE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Time stamp snapshot for IPv4 frames enable
    #[inline(always)]
    pub fn tssipv4fe(&self) -> TSSIPV4FE_R {
        TSSIPV4FE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Time stamp snapshot for event message enable
    #[inline(always)]
    pub fn tsseme(&self) -> TSSEME_R {
        TSSEME_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Time stamp snapshot for message relevant to master enable
    #[inline(always)]
    pub fn tssmrme(&self) -> TSSMRME_R {
        TSSMRME_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Time stamp clock node type
    #[inline(always)]
    pub fn tscnt(&self) -> TSCNT_R {
        TSCNT_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - Time stamp PTP frame filtering MAC address enable
    #[inline(always)]
    pub fn tspffmae(&self) -> TSPFFMAE_R {
        TSPFFMAE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSCR")
            .field("tse", &self.tse())
            .field("tsfcu", &self.tsfcu())
            .field("tssti", &self.tssti())
            .field("tsstu", &self.tsstu())
            .field("tsite", &self.tsite())
            .field("ttsaru", &self.ttsaru())
            .field("tssarfe", &self.tssarfe())
            .field("tsssr", &self.tsssr())
            .field("tsptppsv2e", &self.tsptppsv2e())
            .field("tssptpoefe", &self.tssptpoefe())
            .field("tssipv6fe", &self.tssipv6fe())
            .field("tssipv4fe", &self.tssipv4fe())
            .field("tsseme", &self.tsseme())
            .field("tssmrme", &self.tssmrme())
            .field("tscnt", &self.tscnt())
            .field("tspffmae", &self.tspffmae())
            .finish()
    }
}
impl W {
    ///Bit 0 - Time stamp enable
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W<'_, PTPTSCRrs> {
        TSE_W::new(self, 0)
    }
    ///Bit 1 - Time stamp fine or coarse update
    #[inline(always)]
    pub fn tsfcu(&mut self) -> TSFCU_W<'_, PTPTSCRrs> {
        TSFCU_W::new(self, 1)
    }
    ///Bit 2 - Time stamp system time initialize
    #[inline(always)]
    pub fn tssti(&mut self) -> TSSTI_W<'_, PTPTSCRrs> {
        TSSTI_W::new(self, 2)
    }
    ///Bit 3 - Time stamp system time update
    #[inline(always)]
    pub fn tsstu(&mut self) -> TSSTU_W<'_, PTPTSCRrs> {
        TSSTU_W::new(self, 3)
    }
    ///Bit 4 - Time stamp interrupt trigger enable
    #[inline(always)]
    pub fn tsite(&mut self) -> TSITE_W<'_, PTPTSCRrs> {
        TSITE_W::new(self, 4)
    }
    ///Bit 5 - Time stamp addend register update
    #[inline(always)]
    pub fn ttsaru(&mut self) -> TTSARU_W<'_, PTPTSCRrs> {
        TTSARU_W::new(self, 5)
    }
    ///Bit 8 - Time stamp snapshot for all received frames enable
    #[inline(always)]
    pub fn tssarfe(&mut self) -> TSSARFE_W<'_, PTPTSCRrs> {
        TSSARFE_W::new(self, 8)
    }
    ///Bit 9 - Time stamp subsecond rollover: digital or binary rollover control
    #[inline(always)]
    pub fn tsssr(&mut self) -> TSSSR_W<'_, PTPTSCRrs> {
        TSSSR_W::new(self, 9)
    }
    ///Bit 10 - Time stamp PTP packet snooping for version2 format enable
    #[inline(always)]
    pub fn tsptppsv2e(&mut self) -> TSPTPPSV2E_W<'_, PTPTSCRrs> {
        TSPTPPSV2E_W::new(self, 10)
    }
    ///Bit 11 - Time stamp snapshot for PTP over ethernet frames enable
    #[inline(always)]
    pub fn tssptpoefe(&mut self) -> TSSPTPOEFE_W<'_, PTPTSCRrs> {
        TSSPTPOEFE_W::new(self, 11)
    }
    ///Bit 12 - Time stamp snapshot for IPv6 frames enable
    #[inline(always)]
    pub fn tssipv6fe(&mut self) -> TSSIPV6FE_W<'_, PTPTSCRrs> {
        TSSIPV6FE_W::new(self, 12)
    }
    ///Bit 13 - Time stamp snapshot for IPv4 frames enable
    #[inline(always)]
    pub fn tssipv4fe(&mut self) -> TSSIPV4FE_W<'_, PTPTSCRrs> {
        TSSIPV4FE_W::new(self, 13)
    }
    ///Bit 14 - Time stamp snapshot for event message enable
    #[inline(always)]
    pub fn tsseme(&mut self) -> TSSEME_W<'_, PTPTSCRrs> {
        TSSEME_W::new(self, 14)
    }
    ///Bit 15 - Time stamp snapshot for message relevant to master enable
    #[inline(always)]
    pub fn tssmrme(&mut self) -> TSSMRME_W<'_, PTPTSCRrs> {
        TSSMRME_W::new(self, 15)
    }
    ///Bits 16:17 - Time stamp clock node type
    #[inline(always)]
    pub fn tscnt(&mut self) -> TSCNT_W<'_, PTPTSCRrs> {
        TSCNT_W::new(self, 16)
    }
    ///Bit 18 - Time stamp PTP frame filtering MAC address enable
    #[inline(always)]
    pub fn tspffmae(&mut self) -> TSPFFMAE_W<'_, PTPTSCRrs> {
        TSPFFMAE_W::new(self, 18)
    }
}
/**Ethernet PTP time stamp control register

You can [`read`](crate::Reg::read) this register and get [`ptptscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#Ethernet_PTP:PTPTSCR)*/
pub struct PTPTSCRrs;
impl crate::RegisterSpec for PTPTSCRrs {
    type Ux = u32;
}
///`read()` method returns [`ptptscr::R`](R) reader structure
impl crate::Readable for PTPTSCRrs {}
///`write(|w| ..)` method takes [`ptptscr::W`](W) writer structure
impl crate::Writable for PTPTSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PTPTSCR to value 0x2000
impl crate::Resettable for PTPTSCRrs {
    const RESET_VALUE: u32 = 0x2000;
}
