#[doc = "Register `PTPTSCR` reader"]
pub type R = crate::R<PTPTSCRrs>;
#[doc = "Register `PTPTSCR` writer"]
pub type W = crate::W<PTPTSCRrs>;
#[doc = "Field `TSE` reader - TSE"]
pub type TSE_R = crate::BitReader;
#[doc = "Field `TSE` writer - TSE"]
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSFCU` reader - TSFCU"]
pub type TSFCU_R = crate::BitReader;
#[doc = "Field `TSFCU` writer - TSFCU"]
pub type TSFCU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSSTI` reader - TSSTI"]
pub type TSSTI_R = crate::BitReader;
#[doc = "Field `TSSTI` writer - TSSTI"]
pub type TSSTI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSSTU` reader - TSSTU"]
pub type TSSTU_R = crate::BitReader;
#[doc = "Field `TSSTU` writer - TSSTU"]
pub type TSSTU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSITE` reader - TSITE"]
pub type TSITE_R = crate::BitReader;
#[doc = "Field `TSITE` writer - TSITE"]
pub type TSITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTSARU` reader - TTSARU"]
pub type TTSARU_R = crate::BitReader;
#[doc = "Field `TTSARU` writer - TTSARU"]
pub type TTSARU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSSARFE` reader - TSSARFE"]
pub type TSSARFE_R = crate::BitReader;
#[doc = "Field `TSSARFE` writer - TSSARFE"]
pub type TSSARFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSSSR` reader - TSSSR"]
pub type TSSSR_R = crate::BitReader;
#[doc = "Field `TSSSR` writer - TSSSR"]
pub type TSSSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSPTPPSV2E` reader - TSPTPPSV2E"]
pub type TSPTPPSV2E_R = crate::BitReader;
#[doc = "Field `TSPTPPSV2E` writer - TSPTPPSV2E"]
pub type TSPTPPSV2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSSPTPOEFE` reader - TSSPTPOEFE"]
pub type TSSPTPOEFE_R = crate::BitReader;
#[doc = "Field `TSSPTPOEFE` writer - TSSPTPOEFE"]
pub type TSSPTPOEFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSSIPV6FE` reader - TSSIPV6FE"]
pub type TSSIPV6FE_R = crate::BitReader;
#[doc = "Field `TSSIPV6FE` writer - TSSIPV6FE"]
pub type TSSIPV6FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSSIPV4FE` reader - TSSIPV4FE"]
pub type TSSIPV4FE_R = crate::BitReader;
#[doc = "Field `TSSIPV4FE` writer - TSSIPV4FE"]
pub type TSSIPV4FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSSEME` reader - TSSEME"]
pub type TSSEME_R = crate::BitReader;
#[doc = "Field `TSSEME` writer - TSSEME"]
pub type TSSEME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSSMRME` reader - TSSMRME"]
pub type TSSMRME_R = crate::BitReader;
#[doc = "Field `TSSMRME` writer - TSSMRME"]
pub type TSSMRME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCNT` reader - TSCNT"]
pub type TSCNT_R = crate::FieldReader;
#[doc = "Field `TSCNT` writer - TSCNT"]
pub type TSCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSPFFMAE` reader - TSPFFMAE"]
pub type TSPFFMAE_R = crate::BitReader;
#[doc = "Field `TSPFFMAE` writer - TSPFFMAE"]
pub type TSPFFMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TSE"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TSFCU"]
    #[inline(always)]
    pub fn tsfcu(&self) -> TSFCU_R {
        TSFCU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TSSTI"]
    #[inline(always)]
    pub fn tssti(&self) -> TSSTI_R {
        TSSTI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TSSTU"]
    #[inline(always)]
    pub fn tsstu(&self) -> TSSTU_R {
        TSSTU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TSITE"]
    #[inline(always)]
    pub fn tsite(&self) -> TSITE_R {
        TSITE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TTSARU"]
    #[inline(always)]
    pub fn ttsaru(&self) -> TTSARU_R {
        TTSARU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - TSSARFE"]
    #[inline(always)]
    pub fn tssarfe(&self) -> TSSARFE_R {
        TSSARFE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TSSSR"]
    #[inline(always)]
    pub fn tsssr(&self) -> TSSSR_R {
        TSSSR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TSPTPPSV2E"]
    #[inline(always)]
    pub fn tsptppsv2e(&self) -> TSPTPPSV2E_R {
        TSPTPPSV2E_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TSSPTPOEFE"]
    #[inline(always)]
    pub fn tssptpoefe(&self) -> TSSPTPOEFE_R {
        TSSPTPOEFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TSSIPV6FE"]
    #[inline(always)]
    pub fn tssipv6fe(&self) -> TSSIPV6FE_R {
        TSSIPV6FE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TSSIPV4FE"]
    #[inline(always)]
    pub fn tssipv4fe(&self) -> TSSIPV4FE_R {
        TSSIPV4FE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TSSEME"]
    #[inline(always)]
    pub fn tsseme(&self) -> TSSEME_R {
        TSSEME_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TSSMRME"]
    #[inline(always)]
    pub fn tssmrme(&self) -> TSSMRME_R {
        TSSMRME_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - TSCNT"]
    #[inline(always)]
    pub fn tscnt(&self) -> TSCNT_R {
        TSCNT_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - TSPFFMAE"]
    #[inline(always)]
    pub fn tspffmae(&self) -> TSPFFMAE_R {
        TSPFFMAE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TSE"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<PTPTSCRrs> {
        TSE_W::new(self, 0)
    }
    #[doc = "Bit 1 - TSFCU"]
    #[inline(always)]
    #[must_use]
    pub fn tsfcu(&mut self) -> TSFCU_W<PTPTSCRrs> {
        TSFCU_W::new(self, 1)
    }
    #[doc = "Bit 2 - TSSTI"]
    #[inline(always)]
    #[must_use]
    pub fn tssti(&mut self) -> TSSTI_W<PTPTSCRrs> {
        TSSTI_W::new(self, 2)
    }
    #[doc = "Bit 3 - TSSTU"]
    #[inline(always)]
    #[must_use]
    pub fn tsstu(&mut self) -> TSSTU_W<PTPTSCRrs> {
        TSSTU_W::new(self, 3)
    }
    #[doc = "Bit 4 - TSITE"]
    #[inline(always)]
    #[must_use]
    pub fn tsite(&mut self) -> TSITE_W<PTPTSCRrs> {
        TSITE_W::new(self, 4)
    }
    #[doc = "Bit 5 - TTSARU"]
    #[inline(always)]
    #[must_use]
    pub fn ttsaru(&mut self) -> TTSARU_W<PTPTSCRrs> {
        TTSARU_W::new(self, 5)
    }
    #[doc = "Bit 8 - TSSARFE"]
    #[inline(always)]
    #[must_use]
    pub fn tssarfe(&mut self) -> TSSARFE_W<PTPTSCRrs> {
        TSSARFE_W::new(self, 8)
    }
    #[doc = "Bit 9 - TSSSR"]
    #[inline(always)]
    #[must_use]
    pub fn tsssr(&mut self) -> TSSSR_W<PTPTSCRrs> {
        TSSSR_W::new(self, 9)
    }
    #[doc = "Bit 10 - TSPTPPSV2E"]
    #[inline(always)]
    #[must_use]
    pub fn tsptppsv2e(&mut self) -> TSPTPPSV2E_W<PTPTSCRrs> {
        TSPTPPSV2E_W::new(self, 10)
    }
    #[doc = "Bit 11 - TSSPTPOEFE"]
    #[inline(always)]
    #[must_use]
    pub fn tssptpoefe(&mut self) -> TSSPTPOEFE_W<PTPTSCRrs> {
        TSSPTPOEFE_W::new(self, 11)
    }
    #[doc = "Bit 12 - TSSIPV6FE"]
    #[inline(always)]
    #[must_use]
    pub fn tssipv6fe(&mut self) -> TSSIPV6FE_W<PTPTSCRrs> {
        TSSIPV6FE_W::new(self, 12)
    }
    #[doc = "Bit 13 - TSSIPV4FE"]
    #[inline(always)]
    #[must_use]
    pub fn tssipv4fe(&mut self) -> TSSIPV4FE_W<PTPTSCRrs> {
        TSSIPV4FE_W::new(self, 13)
    }
    #[doc = "Bit 14 - TSSEME"]
    #[inline(always)]
    #[must_use]
    pub fn tsseme(&mut self) -> TSSEME_W<PTPTSCRrs> {
        TSSEME_W::new(self, 14)
    }
    #[doc = "Bit 15 - TSSMRME"]
    #[inline(always)]
    #[must_use]
    pub fn tssmrme(&mut self) -> TSSMRME_W<PTPTSCRrs> {
        TSSMRME_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - TSCNT"]
    #[inline(always)]
    #[must_use]
    pub fn tscnt(&mut self) -> TSCNT_W<PTPTSCRrs> {
        TSCNT_W::new(self, 16)
    }
    #[doc = "Bit 18 - TSPFFMAE"]
    #[inline(always)]
    #[must_use]
    pub fn tspffmae(&mut self) -> TSPFFMAE_W<PTPTSCRrs> {
        TSPFFMAE_W::new(self, 18)
    }
}
#[doc = "Ethernet PTP time stamp control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSCRrs;
impl crate::RegisterSpec for PTPTSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptscr::R`](R) reader structure"]
impl crate::Readable for PTPTSCRrs {}
#[doc = "`write(|w| ..)` method takes [`ptptscr::W`](W) writer structure"]
impl crate::Writable for PTPTSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTPTSCR to value 0x2000"]
impl crate::Resettable for PTPTSCRrs {
    const RESET_VALUE: u32 = 0x2000;
}
