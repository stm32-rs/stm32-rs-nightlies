///Register `IER1` reader
pub type R = crate::R<IER1rs>;
///Register `IER1` writer
pub type W = crate::W<IER1rs>;
///Field `TOHSTXIE` reader - Timeout high
pub type TOHSTXIE_R = crate::BitReader;
///Field `TOHSTXIE` writer - Timeout high
pub type TOHSTXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOLPRXIE` reader - Timeout low
pub type TOLPRXIE_R = crate::BitReader;
///Field `TOLPRXIE` writer - Timeout low
pub type TOLPRXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCSEIE` reader - ECC single
pub type ECCSEIE_R = crate::BitReader;
///Field `ECCSEIE` writer - ECC single
pub type ECCSEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCMEIE` reader - ECC multi
pub type ECCMEIE_R = crate::BitReader;
///Field `ECCMEIE` writer - ECC multi
pub type ECCMEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCEIE` reader - CRC error interrupt enable
pub type CRCEIE_R = crate::BitReader;
///Field `CRCEIE` writer - CRC error interrupt enable
pub type CRCEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSEIE` reader - Packet size error interrupt enable
pub type PSEIE_R = crate::BitReader;
///Field `PSEIE` writer - Packet size error interrupt enable
pub type PSEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOTPEIE` reader - EoTp error interrupt enable
pub type EOTPEIE_R = crate::BitReader;
///Field `EOTPEIE` writer - EoTp error interrupt enable
pub type EOTPEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPWREIE` reader - LTDC payload write error interrupt enable
pub type LPWREIE_R = crate::BitReader;
///Field `LPWREIE` writer - LTDC payload write error interrupt enable
pub type LPWREIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GCWREIE` reader - Generic command write error interrupt enable
pub type GCWREIE_R = crate::BitReader;
///Field `GCWREIE` writer - Generic command write error interrupt enable
pub type GCWREIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPWREIE` reader - Generic payload write error interrupt enable
pub type GPWREIE_R = crate::BitReader;
///Field `GPWREIE` writer - Generic payload write error interrupt enable
pub type GPWREIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPTXEIE` reader - Generic payload transmit error interrupt enable
pub type GPTXEIE_R = crate::BitReader;
///Field `GPTXEIE` writer - Generic payload transmit error interrupt enable
pub type GPTXEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPRDEIE` reader - Generic payload read error interrupt enable
pub type GPRDEIE_R = crate::BitReader;
///Field `GPRDEIE` writer - Generic payload read error interrupt enable
pub type GPRDEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPRXEIE` reader - Generic payload receive error interrupt enable
pub type GPRXEIE_R = crate::BitReader;
///Field `GPRXEIE` writer - Generic payload receive error interrupt enable
pub type GPRXEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Timeout high
    #[inline(always)]
    pub fn tohstxie(&self) -> TOHSTXIE_R {
        TOHSTXIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timeout low
    #[inline(always)]
    pub fn tolprxie(&self) -> TOLPRXIE_R {
        TOLPRXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ECC single
    #[inline(always)]
    pub fn eccseie(&self) -> ECCSEIE_R {
        ECCSEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ECC multi
    #[inline(always)]
    pub fn eccmeie(&self) -> ECCMEIE_R {
        ECCMEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CRC error interrupt enable
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Packet size error interrupt enable
    #[inline(always)]
    pub fn pseie(&self) -> PSEIE_R {
        PSEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - EoTp error interrupt enable
    #[inline(always)]
    pub fn eotpeie(&self) -> EOTPEIE_R {
        EOTPEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LTDC payload write error interrupt enable
    #[inline(always)]
    pub fn lpwreie(&self) -> LPWREIE_R {
        LPWREIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Generic command write error interrupt enable
    #[inline(always)]
    pub fn gcwreie(&self) -> GCWREIE_R {
        GCWREIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Generic payload write error interrupt enable
    #[inline(always)]
    pub fn gpwreie(&self) -> GPWREIE_R {
        GPWREIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Generic payload transmit error interrupt enable
    #[inline(always)]
    pub fn gptxeie(&self) -> GPTXEIE_R {
        GPTXEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Generic payload read error interrupt enable
    #[inline(always)]
    pub fn gprdeie(&self) -> GPRDEIE_R {
        GPRDEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Generic payload receive error interrupt enable
    #[inline(always)]
    pub fn gprxeie(&self) -> GPRXEIE_R {
        GPRXEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER1")
            .field("tohstxie", &self.tohstxie())
            .field("tolprxie", &self.tolprxie())
            .field("eccseie", &self.eccseie())
            .field("eccmeie", &self.eccmeie())
            .field("crceie", &self.crceie())
            .field("pseie", &self.pseie())
            .field("eotpeie", &self.eotpeie())
            .field("lpwreie", &self.lpwreie())
            .field("gcwreie", &self.gcwreie())
            .field("gpwreie", &self.gpwreie())
            .field("gptxeie", &self.gptxeie())
            .field("gprdeie", &self.gprdeie())
            .field("gprxeie", &self.gprxeie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Timeout high
    #[inline(always)]
    pub fn tohstxie(&mut self) -> TOHSTXIE_W<'_, IER1rs> {
        TOHSTXIE_W::new(self, 0)
    }
    ///Bit 1 - Timeout low
    #[inline(always)]
    pub fn tolprxie(&mut self) -> TOLPRXIE_W<'_, IER1rs> {
        TOLPRXIE_W::new(self, 1)
    }
    ///Bit 2 - ECC single
    #[inline(always)]
    pub fn eccseie(&mut self) -> ECCSEIE_W<'_, IER1rs> {
        ECCSEIE_W::new(self, 2)
    }
    ///Bit 3 - ECC multi
    #[inline(always)]
    pub fn eccmeie(&mut self) -> ECCMEIE_W<'_, IER1rs> {
        ECCMEIE_W::new(self, 3)
    }
    ///Bit 4 - CRC error interrupt enable
    #[inline(always)]
    pub fn crceie(&mut self) -> CRCEIE_W<'_, IER1rs> {
        CRCEIE_W::new(self, 4)
    }
    ///Bit 5 - Packet size error interrupt enable
    #[inline(always)]
    pub fn pseie(&mut self) -> PSEIE_W<'_, IER1rs> {
        PSEIE_W::new(self, 5)
    }
    ///Bit 6 - EoTp error interrupt enable
    #[inline(always)]
    pub fn eotpeie(&mut self) -> EOTPEIE_W<'_, IER1rs> {
        EOTPEIE_W::new(self, 6)
    }
    ///Bit 7 - LTDC payload write error interrupt enable
    #[inline(always)]
    pub fn lpwreie(&mut self) -> LPWREIE_W<'_, IER1rs> {
        LPWREIE_W::new(self, 7)
    }
    ///Bit 8 - Generic command write error interrupt enable
    #[inline(always)]
    pub fn gcwreie(&mut self) -> GCWREIE_W<'_, IER1rs> {
        GCWREIE_W::new(self, 8)
    }
    ///Bit 9 - Generic payload write error interrupt enable
    #[inline(always)]
    pub fn gpwreie(&mut self) -> GPWREIE_W<'_, IER1rs> {
        GPWREIE_W::new(self, 9)
    }
    ///Bit 10 - Generic payload transmit error interrupt enable
    #[inline(always)]
    pub fn gptxeie(&mut self) -> GPTXEIE_W<'_, IER1rs> {
        GPTXEIE_W::new(self, 10)
    }
    ///Bit 11 - Generic payload read error interrupt enable
    #[inline(always)]
    pub fn gprdeie(&mut self) -> GPRDEIE_W<'_, IER1rs> {
        GPRDEIE_W::new(self, 11)
    }
    ///Bit 12 - Generic payload receive error interrupt enable
    #[inline(always)]
    pub fn gprxeie(&mut self) -> GPRXEIE_W<'_, IER1rs> {
        GPRXEIE_W::new(self, 12)
    }
}
/**DSI Host interrupt enable register 1

You can [`read`](crate::Reg::read) this register and get [`ier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#DSIHOST:IER1)*/
pub struct IER1rs;
impl crate::RegisterSpec for IER1rs {
    type Ux = u32;
}
///`read()` method returns [`ier1::R`](R) reader structure
impl crate::Readable for IER1rs {}
///`write(|w| ..)` method takes [`ier1::W`](W) writer structure
impl crate::Writable for IER1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER1 to value 0
impl crate::Resettable for IER1rs {}
