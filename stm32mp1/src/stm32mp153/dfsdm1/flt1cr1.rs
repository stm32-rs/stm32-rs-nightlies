///Register `FLT1CR1` reader
pub type R = crate::R<FLT1CR1rs>;
///Register `FLT1CR1` writer
pub type W = crate::W<FLT1CR1rs>;
///Field `DFEN` reader - DFEN
pub type DFEN_R = crate::BitReader;
///Field `DFEN` writer - DFEN
pub type DFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JSWSTART` reader - JSWSTART
pub type JSWSTART_R = crate::BitReader;
///Field `JSWSTART` writer - JSWSTART
pub type JSWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JSYNC` reader - JSYNC
pub type JSYNC_R = crate::BitReader;
///Field `JSYNC` writer - JSYNC
pub type JSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JSCAN` reader - JSCAN
pub type JSCAN_R = crate::BitReader;
///Field `JSCAN` writer - JSCAN
pub type JSCAN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JDMAEN` reader - JDMAEN
pub type JDMAEN_R = crate::BitReader;
///Field `JDMAEN` writer - JDMAEN
pub type JDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JEXTSEL` reader - JEXTSEL
pub type JEXTSEL_R = crate::FieldReader;
///Field `JEXTSEL` writer - JEXTSEL
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `JEXTEN` reader - JEXTEN
pub type JEXTEN_R = crate::FieldReader;
///Field `JEXTEN` writer - JEXTEN
pub type JEXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSWSTART` reader - RSWSTART
pub type RSWSTART_R = crate::BitReader;
///Field `RSWSTART` writer - RSWSTART
pub type RSWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCONT` reader - RCONT
pub type RCONT_R = crate::BitReader;
///Field `RCONT` writer - RCONT
pub type RCONT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSYNC` reader - RSYNC
pub type RSYNC_R = crate::BitReader;
///Field `RSYNC` writer - RSYNC
pub type RSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDMAEN` reader - RDMAEN
pub type RDMAEN_R = crate::BitReader;
///Field `RDMAEN` writer - RDMAEN
pub type RDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCH` reader - RCH
pub type RCH_R = crate::FieldReader;
///Field `RCH` writer - RCH
pub type RCH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FAST` reader - FAST
pub type FAST_R = crate::BitReader;
///Field `FAST` writer - FAST
pub type FAST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWFSEL` reader - AWFSEL
pub type AWFSEL_R = crate::BitReader;
///Field `AWFSEL` writer - AWFSEL
pub type AWFSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DFEN
    #[inline(always)]
    pub fn dfen(&self) -> DFEN_R {
        DFEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - JSWSTART
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - JSYNC
    #[inline(always)]
    pub fn jsync(&self) -> JSYNC_R {
        JSYNC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - JSCAN
    #[inline(always)]
    pub fn jscan(&self) -> JSCAN_R {
        JSCAN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - JDMAEN
    #[inline(always)]
    pub fn jdmaen(&self) -> JDMAEN_R {
        JDMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:12 - JEXTSEL
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 13:14 - JEXTEN
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 17 - RSWSTART
    #[inline(always)]
    pub fn rswstart(&self) -> RSWSTART_R {
        RSWSTART_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - RCONT
    #[inline(always)]
    pub fn rcont(&self) -> RCONT_R {
        RCONT_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - RSYNC
    #[inline(always)]
    pub fn rsync(&self) -> RSYNC_R {
        RSYNC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - RDMAEN
    #[inline(always)]
    pub fn rdmaen(&self) -> RDMAEN_R {
        RDMAEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 24:26 - RCH
    #[inline(always)]
    pub fn rch(&self) -> RCH_R {
        RCH_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 29 - FAST
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - AWFSEL
    #[inline(always)]
    pub fn awfsel(&self) -> AWFSEL_R {
        AWFSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT1CR1")
            .field("dfen", &self.dfen())
            .field("jswstart", &self.jswstart())
            .field("jsync", &self.jsync())
            .field("jscan", &self.jscan())
            .field("jdmaen", &self.jdmaen())
            .field("jextsel", &self.jextsel())
            .field("jexten", &self.jexten())
            .field("rswstart", &self.rswstart())
            .field("rcont", &self.rcont())
            .field("rsync", &self.rsync())
            .field("rdmaen", &self.rdmaen())
            .field("rch", &self.rch())
            .field("fast", &self.fast())
            .field("awfsel", &self.awfsel())
            .finish()
    }
}
impl W {
    ///Bit 0 - DFEN
    #[inline(always)]
    pub fn dfen(&mut self) -> DFEN_W<'_, FLT1CR1rs> {
        DFEN_W::new(self, 0)
    }
    ///Bit 1 - JSWSTART
    #[inline(always)]
    pub fn jswstart(&mut self) -> JSWSTART_W<'_, FLT1CR1rs> {
        JSWSTART_W::new(self, 1)
    }
    ///Bit 3 - JSYNC
    #[inline(always)]
    pub fn jsync(&mut self) -> JSYNC_W<'_, FLT1CR1rs> {
        JSYNC_W::new(self, 3)
    }
    ///Bit 4 - JSCAN
    #[inline(always)]
    pub fn jscan(&mut self) -> JSCAN_W<'_, FLT1CR1rs> {
        JSCAN_W::new(self, 4)
    }
    ///Bit 5 - JDMAEN
    #[inline(always)]
    pub fn jdmaen(&mut self) -> JDMAEN_W<'_, FLT1CR1rs> {
        JDMAEN_W::new(self, 5)
    }
    ///Bits 8:12 - JEXTSEL
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W<'_, FLT1CR1rs> {
        JEXTSEL_W::new(self, 8)
    }
    ///Bits 13:14 - JEXTEN
    #[inline(always)]
    pub fn jexten(&mut self) -> JEXTEN_W<'_, FLT1CR1rs> {
        JEXTEN_W::new(self, 13)
    }
    ///Bit 17 - RSWSTART
    #[inline(always)]
    pub fn rswstart(&mut self) -> RSWSTART_W<'_, FLT1CR1rs> {
        RSWSTART_W::new(self, 17)
    }
    ///Bit 18 - RCONT
    #[inline(always)]
    pub fn rcont(&mut self) -> RCONT_W<'_, FLT1CR1rs> {
        RCONT_W::new(self, 18)
    }
    ///Bit 19 - RSYNC
    #[inline(always)]
    pub fn rsync(&mut self) -> RSYNC_W<'_, FLT1CR1rs> {
        RSYNC_W::new(self, 19)
    }
    ///Bit 21 - RDMAEN
    #[inline(always)]
    pub fn rdmaen(&mut self) -> RDMAEN_W<'_, FLT1CR1rs> {
        RDMAEN_W::new(self, 21)
    }
    ///Bits 24:26 - RCH
    #[inline(always)]
    pub fn rch(&mut self) -> RCH_W<'_, FLT1CR1rs> {
        RCH_W::new(self, 24)
    }
    ///Bit 29 - FAST
    #[inline(always)]
    pub fn fast(&mut self) -> FAST_W<'_, FLT1CR1rs> {
        FAST_W::new(self, 29)
    }
    ///Bit 30 - AWFSEL
    #[inline(always)]
    pub fn awfsel(&mut self) -> AWFSEL_W<'_, FLT1CR1rs> {
        AWFSEL_W::new(self, 30)
    }
}
/**DFSDM filter 1 control register 1

You can [`read`](crate::Reg::read) this register and get [`flt1cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt1cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:FLT1CR1)*/
pub struct FLT1CR1rs;
impl crate::RegisterSpec for FLT1CR1rs {
    type Ux = u32;
}
///`read()` method returns [`flt1cr1::R`](R) reader structure
impl crate::Readable for FLT1CR1rs {}
///`write(|w| ..)` method takes [`flt1cr1::W`](W) writer structure
impl crate::Writable for FLT1CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLT1CR1 to value 0
impl crate::Resettable for FLT1CR1rs {}
