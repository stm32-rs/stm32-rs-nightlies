///Register `PTPTSCR` reader
pub type R = crate::R<PTPTSCRrs>;
///Register `PTPTSCR` writer
pub type W = crate::W<PTPTSCRrs>;
///Field `TSE` reader - TSE
pub type TSE_R = crate::BitReader;
///Field `TSE` writer - TSE
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSFCU` reader - TSFCU
pub type TSFCU_R = crate::BitReader;
///Field `TSFCU` writer - TSFCU
pub type TSFCU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSTI` reader - TSSTI
pub type TSSTI_R = crate::BitReader;
///Field `TSSTI` writer - TSSTI
pub type TSSTI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSTU` reader - TSSTU
pub type TSSTU_R = crate::BitReader;
///Field `TSSTU` writer - TSSTU
pub type TSSTU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSITE` reader - TSITE
pub type TSITE_R = crate::BitReader;
///Field `TSITE` writer - TSITE
pub type TSITE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TTSARU` reader - TTSARU
pub type TTSARU_R = crate::BitReader;
///Field `TTSARU` writer - TTSARU
pub type TTSARU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSARFE` reader - TSSARFE
pub type TSSARFE_R = crate::BitReader;
///Field `TSSARFE` writer - TSSARFE
pub type TSSARFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSSR` reader - TSSSR
pub type TSSSR_R = crate::BitReader;
///Field `TSSSR` writer - TSSSR
pub type TSSSR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSPTPPSV2E` reader - TSPTPPSV2E
pub type TSPTPPSV2E_R = crate::BitReader;
///Field `TSPTPPSV2E` writer - TSPTPPSV2E
pub type TSPTPPSV2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSPTPOEFE` reader - TSSPTPOEFE
pub type TSSPTPOEFE_R = crate::BitReader;
///Field `TSSPTPOEFE` writer - TSSPTPOEFE
pub type TSSPTPOEFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSIPV6FE` reader - TSSIPV6FE
pub type TSSIPV6FE_R = crate::BitReader;
///Field `TSSIPV6FE` writer - TSSIPV6FE
pub type TSSIPV6FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSIPV4FE` reader - TSSIPV4FE
pub type TSSIPV4FE_R = crate::BitReader;
///Field `TSSIPV4FE` writer - TSSIPV4FE
pub type TSSIPV4FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSEME` reader - TSSEME
pub type TSSEME_R = crate::BitReader;
///Field `TSSEME` writer - TSSEME
pub type TSSEME_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSMRME` reader - TSSMRME
pub type TSSMRME_R = crate::BitReader;
///Field `TSSMRME` writer - TSSMRME
pub type TSSMRME_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSCNT` reader - TSCNT
pub type TSCNT_R = crate::FieldReader;
///Field `TSCNT` writer - TSCNT
pub type TSCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TSPFFMAE` reader - TSPFFMAE
pub type TSPFFMAE_R = crate::BitReader;
///Field `TSPFFMAE` writer - TSPFFMAE
pub type TSPFFMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TSE
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TSFCU
    #[inline(always)]
    pub fn tsfcu(&self) -> TSFCU_R {
        TSFCU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TSSTI
    #[inline(always)]
    pub fn tssti(&self) -> TSSTI_R {
        TSSTI_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TSSTU
    #[inline(always)]
    pub fn tsstu(&self) -> TSSTU_R {
        TSSTU_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TSITE
    #[inline(always)]
    pub fn tsite(&self) -> TSITE_R {
        TSITE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TTSARU
    #[inline(always)]
    pub fn ttsaru(&self) -> TTSARU_R {
        TTSARU_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - TSSARFE
    #[inline(always)]
    pub fn tssarfe(&self) -> TSSARFE_R {
        TSSARFE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TSSSR
    #[inline(always)]
    pub fn tsssr(&self) -> TSSSR_R {
        TSSSR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - TSPTPPSV2E
    #[inline(always)]
    pub fn tsptppsv2e(&self) -> TSPTPPSV2E_R {
        TSPTPPSV2E_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TSSPTPOEFE
    #[inline(always)]
    pub fn tssptpoefe(&self) -> TSSPTPOEFE_R {
        TSSPTPOEFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TSSIPV6FE
    #[inline(always)]
    pub fn tssipv6fe(&self) -> TSSIPV6FE_R {
        TSSIPV6FE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TSSIPV4FE
    #[inline(always)]
    pub fn tssipv4fe(&self) -> TSSIPV4FE_R {
        TSSIPV4FE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - TSSEME
    #[inline(always)]
    pub fn tsseme(&self) -> TSSEME_R {
        TSSEME_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TSSMRME
    #[inline(always)]
    pub fn tssmrme(&self) -> TSSMRME_R {
        TSSMRME_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - TSCNT
    #[inline(always)]
    pub fn tscnt(&self) -> TSCNT_R {
        TSCNT_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - TSPFFMAE
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
            .field("tsptppsv2e", &self.tsptppsv2e())
            .field("tssptpoefe", &self.tssptpoefe())
            .field("tssipv6fe", &self.tssipv6fe())
            .field("tssipv4fe", &self.tssipv4fe())
            .field("tsseme", &self.tsseme())
            .field("tssmrme", &self.tssmrme())
            .field("tscnt", &self.tscnt())
            .field("tspffmae", &self.tspffmae())
            .field("tssti", &self.tssti())
            .field("tsstu", &self.tsstu())
            .field("tsite", &self.tsite())
            .field("ttsaru", &self.ttsaru())
            .field("tssarfe", &self.tssarfe())
            .field("tsssr", &self.tsssr())
            .finish()
    }
}
impl W {
    ///Bit 0 - TSE
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W<'_, PTPTSCRrs> {
        TSE_W::new(self, 0)
    }
    ///Bit 1 - TSFCU
    #[inline(always)]
    pub fn tsfcu(&mut self) -> TSFCU_W<'_, PTPTSCRrs> {
        TSFCU_W::new(self, 1)
    }
    ///Bit 2 - TSSTI
    #[inline(always)]
    pub fn tssti(&mut self) -> TSSTI_W<'_, PTPTSCRrs> {
        TSSTI_W::new(self, 2)
    }
    ///Bit 3 - TSSTU
    #[inline(always)]
    pub fn tsstu(&mut self) -> TSSTU_W<'_, PTPTSCRrs> {
        TSSTU_W::new(self, 3)
    }
    ///Bit 4 - TSITE
    #[inline(always)]
    pub fn tsite(&mut self) -> TSITE_W<'_, PTPTSCRrs> {
        TSITE_W::new(self, 4)
    }
    ///Bit 5 - TTSARU
    #[inline(always)]
    pub fn ttsaru(&mut self) -> TTSARU_W<'_, PTPTSCRrs> {
        TTSARU_W::new(self, 5)
    }
    ///Bit 8 - TSSARFE
    #[inline(always)]
    pub fn tssarfe(&mut self) -> TSSARFE_W<'_, PTPTSCRrs> {
        TSSARFE_W::new(self, 8)
    }
    ///Bit 9 - TSSSR
    #[inline(always)]
    pub fn tsssr(&mut self) -> TSSSR_W<'_, PTPTSCRrs> {
        TSSSR_W::new(self, 9)
    }
    ///Bit 10 - TSPTPPSV2E
    #[inline(always)]
    pub fn tsptppsv2e(&mut self) -> TSPTPPSV2E_W<'_, PTPTSCRrs> {
        TSPTPPSV2E_W::new(self, 10)
    }
    ///Bit 11 - TSSPTPOEFE
    #[inline(always)]
    pub fn tssptpoefe(&mut self) -> TSSPTPOEFE_W<'_, PTPTSCRrs> {
        TSSPTPOEFE_W::new(self, 11)
    }
    ///Bit 12 - TSSIPV6FE
    #[inline(always)]
    pub fn tssipv6fe(&mut self) -> TSSIPV6FE_W<'_, PTPTSCRrs> {
        TSSIPV6FE_W::new(self, 12)
    }
    ///Bit 13 - TSSIPV4FE
    #[inline(always)]
    pub fn tssipv4fe(&mut self) -> TSSIPV4FE_W<'_, PTPTSCRrs> {
        TSSIPV4FE_W::new(self, 13)
    }
    ///Bit 14 - TSSEME
    #[inline(always)]
    pub fn tsseme(&mut self) -> TSSEME_W<'_, PTPTSCRrs> {
        TSSEME_W::new(self, 14)
    }
    ///Bit 15 - TSSMRME
    #[inline(always)]
    pub fn tssmrme(&mut self) -> TSSMRME_W<'_, PTPTSCRrs> {
        TSSMRME_W::new(self, 15)
    }
    ///Bits 16:17 - TSCNT
    #[inline(always)]
    pub fn tscnt(&mut self) -> TSCNT_W<'_, PTPTSCRrs> {
        TSCNT_W::new(self, 16)
    }
    ///Bit 18 - TSPFFMAE
    #[inline(always)]
    pub fn tspffmae(&mut self) -> TSPFFMAE_W<'_, PTPTSCRrs> {
        TSPFFMAE_W::new(self, 18)
    }
}
/**Ethernet PTP time stamp control register

You can [`read`](crate::Reg::read) this register and get [`ptptscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F429.html#Ethernet_PTP:PTPTSCR)*/
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
