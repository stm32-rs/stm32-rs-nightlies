#[doc = "Register `IER1` reader"]
pub type R = crate::R<IER1rs>;
#[doc = "Register `IER1` writer"]
pub type W = crate::W<IER1rs>;
#[doc = "Field `TOHSTXIE` reader - Timeout high"]
pub type TOHSTXIE_R = crate::BitReader;
#[doc = "Field `TOHSTXIE` writer - Timeout high"]
pub type TOHSTXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOLPRXIE` reader - Timeout low"]
pub type TOLPRXIE_R = crate::BitReader;
#[doc = "Field `TOLPRXIE` writer - Timeout low"]
pub type TOLPRXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCSEIE` reader - ECC single"]
pub type ECCSEIE_R = crate::BitReader;
#[doc = "Field `ECCSEIE` writer - ECC single"]
pub type ECCSEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCMEIE` reader - ECC multi"]
pub type ECCMEIE_R = crate::BitReader;
#[doc = "Field `ECCMEIE` writer - ECC multi"]
pub type ECCMEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEIE` reader - CRC error interrupt enable"]
pub type CRCEIE_R = crate::BitReader;
#[doc = "Field `CRCEIE` writer - CRC error interrupt enable"]
pub type CRCEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSEIE` reader - Packet size error interrupt enable"]
pub type PSEIE_R = crate::BitReader;
#[doc = "Field `PSEIE` writer - Packet size error interrupt enable"]
pub type PSEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOTPEIE` reader - EoTp error interrupt enable"]
pub type EOTPEIE_R = crate::BitReader;
#[doc = "Field `EOTPEIE` writer - EoTp error interrupt enable"]
pub type EOTPEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPWREIE` reader - LTDC payload write error interrupt enable"]
pub type LPWREIE_R = crate::BitReader;
#[doc = "Field `LPWREIE` writer - LTDC payload write error interrupt enable"]
pub type LPWREIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCWREIE` reader - Generic command write error interrupt enable"]
pub type GCWREIE_R = crate::BitReader;
#[doc = "Field `GCWREIE` writer - Generic command write error interrupt enable"]
pub type GCWREIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPWREIE` reader - Generic payload write error interrupt enable"]
pub type GPWREIE_R = crate::BitReader;
#[doc = "Field `GPWREIE` writer - Generic payload write error interrupt enable"]
pub type GPWREIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPTXEIE` reader - Generic payload transmit error interrupt enable"]
pub type GPTXEIE_R = crate::BitReader;
#[doc = "Field `GPTXEIE` writer - Generic payload transmit error interrupt enable"]
pub type GPTXEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPRDEIE` reader - Generic payload read error interrupt enable"]
pub type GPRDEIE_R = crate::BitReader;
#[doc = "Field `GPRDEIE` writer - Generic payload read error interrupt enable"]
pub type GPRDEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPRXEIE` reader - Generic payload receive error interrupt enable"]
pub type GPRXEIE_R = crate::BitReader;
#[doc = "Field `GPRXEIE` writer - Generic payload receive error interrupt enable"]
pub type GPRXEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timeout high"]
    #[inline(always)]
    pub fn tohstxie(&self) -> TOHSTXIE_R {
        TOHSTXIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timeout low"]
    #[inline(always)]
    pub fn tolprxie(&self) -> TOLPRXIE_R {
        TOLPRXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ECC single"]
    #[inline(always)]
    pub fn eccseie(&self) -> ECCSEIE_R {
        ECCSEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC multi"]
    #[inline(always)]
    pub fn eccmeie(&self) -> ECCMEIE_R {
        ECCMEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC error interrupt enable"]
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Packet size error interrupt enable"]
    #[inline(always)]
    pub fn pseie(&self) -> PSEIE_R {
        PSEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EoTp error interrupt enable"]
    #[inline(always)]
    pub fn eotpeie(&self) -> EOTPEIE_R {
        EOTPEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LTDC payload write error interrupt enable"]
    #[inline(always)]
    pub fn lpwreie(&self) -> LPWREIE_R {
        LPWREIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Generic command write error interrupt enable"]
    #[inline(always)]
    pub fn gcwreie(&self) -> GCWREIE_R {
        GCWREIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Generic payload write error interrupt enable"]
    #[inline(always)]
    pub fn gpwreie(&self) -> GPWREIE_R {
        GPWREIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Generic payload transmit error interrupt enable"]
    #[inline(always)]
    pub fn gptxeie(&self) -> GPTXEIE_R {
        GPTXEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Generic payload read error interrupt enable"]
    #[inline(always)]
    pub fn gprdeie(&self) -> GPRDEIE_R {
        GPRDEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Generic payload receive error interrupt enable"]
    #[inline(always)]
    pub fn gprxeie(&self) -> GPRXEIE_R {
        GPRXEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timeout high"]
    #[inline(always)]
    #[must_use]
    pub fn tohstxie(&mut self) -> TOHSTXIE_W<IER1rs> {
        TOHSTXIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timeout low"]
    #[inline(always)]
    #[must_use]
    pub fn tolprxie(&mut self) -> TOLPRXIE_W<IER1rs> {
        TOLPRXIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - ECC single"]
    #[inline(always)]
    #[must_use]
    pub fn eccseie(&mut self) -> ECCSEIE_W<IER1rs> {
        ECCSEIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - ECC multi"]
    #[inline(always)]
    #[must_use]
    pub fn eccmeie(&mut self) -> ECCMEIE_W<IER1rs> {
        ECCMEIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - CRC error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn crceie(&mut self) -> CRCEIE_W<IER1rs> {
        CRCEIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Packet size error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pseie(&mut self) -> PSEIE_W<IER1rs> {
        PSEIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - EoTp error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eotpeie(&mut self) -> EOTPEIE_W<IER1rs> {
        EOTPEIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - LTDC payload write error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpwreie(&mut self) -> LPWREIE_W<IER1rs> {
        LPWREIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Generic command write error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcwreie(&mut self) -> GCWREIE_W<IER1rs> {
        GCWREIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Generic payload write error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwreie(&mut self) -> GPWREIE_W<IER1rs> {
        GPWREIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Generic payload transmit error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gptxeie(&mut self) -> GPTXEIE_W<IER1rs> {
        GPTXEIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Generic payload read error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gprdeie(&mut self) -> GPRDEIE_W<IER1rs> {
        GPRDEIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Generic payload receive error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gprxeie(&mut self) -> GPRXEIE_W<IER1rs> {
        GPRXEIE_W::new(self, 12)
    }
}
#[doc = "DSI Host interrupt enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER1rs;
impl crate::RegisterSpec for IER1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier1::R`](R) reader structure"]
impl crate::Readable for IER1rs {}
#[doc = "`write(|w| ..)` method takes [`ier1::W`](W) writer structure"]
impl crate::Writable for IER1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER1 to value 0"]
impl crate::Resettable for IER1rs {
    const RESET_VALUE: u32 = 0;
}
