///Register `ISR1` reader
pub type R = crate::R<ISR1rs>;
///Register `ISR1` writer
pub type W = crate::W<ISR1rs>;
///Field `TOHSTX` reader - Timeout high
pub type TOHSTX_R = crate::BitReader;
///Field `TOHSTX` writer - Timeout high
pub type TOHSTX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOLPRX` reader - Timeout low
pub type TOLPRX_R = crate::BitReader;
///Field `TOLPRX` writer - Timeout low
pub type TOLPRX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCSE` reader - ECC single
pub type ECCSE_R = crate::BitReader;
///Field `ECCSE` writer - ECC single
pub type ECCSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCME` reader - ECC multi
pub type ECCME_R = crate::BitReader;
///Field `ECCME` writer - ECC multi
pub type ECCME_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCE` reader - CRC error
pub type CRCE_R = crate::BitReader;
///Field `CRCE` writer - CRC error
pub type CRCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSE` reader - Packet size error
pub type PSE_R = crate::BitReader;
///Field `PSE` writer - Packet size error
pub type PSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOTPE` reader - EoTp error
pub type EOTPE_R = crate::BitReader;
///Field `EOTPE` writer - EoTp error
pub type EOTPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPWRE` reader - LTDC payload write error
pub type LPWRE_R = crate::BitReader;
///Field `LPWRE` writer - LTDC payload write error
pub type LPWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GCWRE` reader - Generic command write error
pub type GCWRE_R = crate::BitReader;
///Field `GCWRE` writer - Generic command write error
pub type GCWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPWRE` reader - Generic payload write error
pub type GPWRE_R = crate::BitReader;
///Field `GPWRE` writer - Generic payload write error
pub type GPWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPTXE` reader - Generic payload transmit error
pub type GPTXE_R = crate::BitReader;
///Field `GPTXE` writer - Generic payload transmit error
pub type GPTXE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPRDE` reader - Generic payload read error
pub type GPRDE_R = crate::BitReader;
///Field `GPRDE` writer - Generic payload read error
pub type GPRDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPRXE` reader - Generic payload receive error
pub type GPRXE_R = crate::BitReader;
///Field `GPRXE` writer - Generic payload receive error
pub type GPRXE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Timeout high
    #[inline(always)]
    pub fn tohstx(&self) -> TOHSTX_R {
        TOHSTX_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timeout low
    #[inline(always)]
    pub fn tolprx(&self) -> TOLPRX_R {
        TOLPRX_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ECC single
    #[inline(always)]
    pub fn eccse(&self) -> ECCSE_R {
        ECCSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ECC multi
    #[inline(always)]
    pub fn eccme(&self) -> ECCME_R {
        ECCME_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CRC error
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Packet size error
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - EoTp error
    #[inline(always)]
    pub fn eotpe(&self) -> EOTPE_R {
        EOTPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LTDC payload write error
    #[inline(always)]
    pub fn lpwre(&self) -> LPWRE_R {
        LPWRE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Generic command write error
    #[inline(always)]
    pub fn gcwre(&self) -> GCWRE_R {
        GCWRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Generic payload write error
    #[inline(always)]
    pub fn gpwre(&self) -> GPWRE_R {
        GPWRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Generic payload transmit error
    #[inline(always)]
    pub fn gptxe(&self) -> GPTXE_R {
        GPTXE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Generic payload read error
    #[inline(always)]
    pub fn gprde(&self) -> GPRDE_R {
        GPRDE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Generic payload receive error
    #[inline(always)]
    pub fn gprxe(&self) -> GPRXE_R {
        GPRXE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR1")
            .field("gprxe", &self.gprxe())
            .field("gprde", &self.gprde())
            .field("gptxe", &self.gptxe())
            .field("gpwre", &self.gpwre())
            .field("gcwre", &self.gcwre())
            .field("lpwre", &self.lpwre())
            .field("eotpe", &self.eotpe())
            .field("pse", &self.pse())
            .field("crce", &self.crce())
            .field("eccme", &self.eccme())
            .field("eccse", &self.eccse())
            .field("tolprx", &self.tolprx())
            .field("tohstx", &self.tohstx())
            .finish()
    }
}
impl W {
    ///Bit 0 - Timeout high
    #[inline(always)]
    #[must_use]
    pub fn tohstx(&mut self) -> TOHSTX_W<ISR1rs> {
        TOHSTX_W::new(self, 0)
    }
    ///Bit 1 - Timeout low
    #[inline(always)]
    #[must_use]
    pub fn tolprx(&mut self) -> TOLPRX_W<ISR1rs> {
        TOLPRX_W::new(self, 1)
    }
    ///Bit 2 - ECC single
    #[inline(always)]
    #[must_use]
    pub fn eccse(&mut self) -> ECCSE_W<ISR1rs> {
        ECCSE_W::new(self, 2)
    }
    ///Bit 3 - ECC multi
    #[inline(always)]
    #[must_use]
    pub fn eccme(&mut self) -> ECCME_W<ISR1rs> {
        ECCME_W::new(self, 3)
    }
    ///Bit 4 - CRC error
    #[inline(always)]
    #[must_use]
    pub fn crce(&mut self) -> CRCE_W<ISR1rs> {
        CRCE_W::new(self, 4)
    }
    ///Bit 5 - Packet size error
    #[inline(always)]
    #[must_use]
    pub fn pse(&mut self) -> PSE_W<ISR1rs> {
        PSE_W::new(self, 5)
    }
    ///Bit 6 - EoTp error
    #[inline(always)]
    #[must_use]
    pub fn eotpe(&mut self) -> EOTPE_W<ISR1rs> {
        EOTPE_W::new(self, 6)
    }
    ///Bit 7 - LTDC payload write error
    #[inline(always)]
    #[must_use]
    pub fn lpwre(&mut self) -> LPWRE_W<ISR1rs> {
        LPWRE_W::new(self, 7)
    }
    ///Bit 8 - Generic command write error
    #[inline(always)]
    #[must_use]
    pub fn gcwre(&mut self) -> GCWRE_W<ISR1rs> {
        GCWRE_W::new(self, 8)
    }
    ///Bit 9 - Generic payload write error
    #[inline(always)]
    #[must_use]
    pub fn gpwre(&mut self) -> GPWRE_W<ISR1rs> {
        GPWRE_W::new(self, 9)
    }
    ///Bit 10 - Generic payload transmit error
    #[inline(always)]
    #[must_use]
    pub fn gptxe(&mut self) -> GPTXE_W<ISR1rs> {
        GPTXE_W::new(self, 10)
    }
    ///Bit 11 - Generic payload read error
    #[inline(always)]
    #[must_use]
    pub fn gprde(&mut self) -> GPRDE_W<ISR1rs> {
        GPRDE_W::new(self, 11)
    }
    ///Bit 12 - Generic payload receive error
    #[inline(always)]
    #[must_use]
    pub fn gprxe(&mut self) -> GPRXE_W<ISR1rs> {
        GPRXE_W::new(self, 12)
    }
}
/**DSI Host interrupt and status register 1

You can [`read`](crate::Reg::read) this register and get [`isr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:ISR1)*/
pub struct ISR1rs;
impl crate::RegisterSpec for ISR1rs {
    type Ux = u32;
}
///`read()` method returns [`isr1::R`](R) reader structure
impl crate::Readable for ISR1rs {}
///`write(|w| ..)` method takes [`isr1::W`](W) writer structure
impl crate::Writable for ISR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ISR1 to value 0
impl crate::Resettable for ISR1rs {
    const RESET_VALUE: u32 = 0;
}
