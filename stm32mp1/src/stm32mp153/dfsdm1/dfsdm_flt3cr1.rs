#[doc = "Register `DFSDM_FLT3CR1` reader"]
pub type R = crate::R<DFSDM_FLT3CR1rs>;
#[doc = "Register `DFSDM_FLT3CR1` writer"]
pub type W = crate::W<DFSDM_FLT3CR1rs>;
#[doc = "Field `DFEN` reader - DFEN"]
pub type DFEN_R = crate::BitReader;
#[doc = "Field `DFEN` writer - DFEN"]
pub type DFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JSWSTART` reader - JSWSTART"]
pub type JSWSTART_R = crate::BitReader;
#[doc = "Field `JSWSTART` writer - JSWSTART"]
pub type JSWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JSYNC` reader - JSYNC"]
pub type JSYNC_R = crate::BitReader;
#[doc = "Field `JSYNC` writer - JSYNC"]
pub type JSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JSCAN` reader - JSCAN"]
pub type JSCAN_R = crate::BitReader;
#[doc = "Field `JSCAN` writer - JSCAN"]
pub type JSCAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JDMAEN` reader - JDMAEN"]
pub type JDMAEN_R = crate::BitReader;
#[doc = "Field `JDMAEN` writer - JDMAEN"]
pub type JDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEXTSEL` reader - JEXTSEL"]
pub type JEXTSEL_R = crate::FieldReader;
#[doc = "Field `JEXTSEL` writer - JEXTSEL"]
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JEXTEN` reader - JEXTEN"]
pub type JEXTEN_R = crate::FieldReader;
#[doc = "Field `JEXTEN` writer - JEXTEN"]
pub type JEXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSWSTART` reader - RSWSTART"]
pub type RSWSTART_R = crate::BitReader;
#[doc = "Field `RSWSTART` writer - RSWSTART"]
pub type RSWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCONT` reader - RCONT"]
pub type RCONT_R = crate::BitReader;
#[doc = "Field `RCONT` writer - RCONT"]
pub type RCONT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSYNC` reader - RSYNC"]
pub type RSYNC_R = crate::BitReader;
#[doc = "Field `RSYNC` writer - RSYNC"]
pub type RSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDMAEN` reader - RDMAEN"]
pub type RDMAEN_R = crate::BitReader;
#[doc = "Field `RDMAEN` writer - RDMAEN"]
pub type RDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCH` reader - RCH"]
pub type RCH_R = crate::FieldReader;
#[doc = "Field `RCH` writer - RCH"]
pub type RCH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FAST` reader - FAST"]
pub type FAST_R = crate::BitReader;
#[doc = "Field `FAST` writer - FAST"]
pub type FAST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWFSEL` reader - AWFSEL"]
pub type AWFSEL_R = crate::BitReader;
#[doc = "Field `AWFSEL` writer - AWFSEL"]
pub type AWFSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DFEN"]
    #[inline(always)]
    pub fn dfen(&self) -> DFEN_R {
        DFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - JSWSTART"]
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - JSYNC"]
    #[inline(always)]
    pub fn jsync(&self) -> JSYNC_R {
        JSYNC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - JSCAN"]
    #[inline(always)]
    pub fn jscan(&self) -> JSCAN_R {
        JSCAN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JDMAEN"]
    #[inline(always)]
    pub fn jdmaen(&self) -> JDMAEN_R {
        JDMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:12 - JEXTSEL"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - JEXTEN"]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 17 - RSWSTART"]
    #[inline(always)]
    pub fn rswstart(&self) -> RSWSTART_R {
        RSWSTART_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RCONT"]
    #[inline(always)]
    pub fn rcont(&self) -> RCONT_R {
        RCONT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RSYNC"]
    #[inline(always)]
    pub fn rsync(&self) -> RSYNC_R {
        RSYNC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - RDMAEN"]
    #[inline(always)]
    pub fn rdmaen(&self) -> RDMAEN_R {
        RDMAEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:26 - RCH"]
    #[inline(always)]
    pub fn rch(&self) -> RCH_R {
        RCH_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 29 - FAST"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - AWFSEL"]
    #[inline(always)]
    pub fn awfsel(&self) -> AWFSEL_R {
        AWFSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DFEN"]
    #[inline(always)]
    #[must_use]
    pub fn dfen(&mut self) -> DFEN_W<DFSDM_FLT3CR1rs> {
        DFEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - JSWSTART"]
    #[inline(always)]
    #[must_use]
    pub fn jswstart(&mut self) -> JSWSTART_W<DFSDM_FLT3CR1rs> {
        JSWSTART_W::new(self, 1)
    }
    #[doc = "Bit 3 - JSYNC"]
    #[inline(always)]
    #[must_use]
    pub fn jsync(&mut self) -> JSYNC_W<DFSDM_FLT3CR1rs> {
        JSYNC_W::new(self, 3)
    }
    #[doc = "Bit 4 - JSCAN"]
    #[inline(always)]
    #[must_use]
    pub fn jscan(&mut self) -> JSCAN_W<DFSDM_FLT3CR1rs> {
        JSCAN_W::new(self, 4)
    }
    #[doc = "Bit 5 - JDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn jdmaen(&mut self) -> JDMAEN_W<DFSDM_FLT3CR1rs> {
        JDMAEN_W::new(self, 5)
    }
    #[doc = "Bits 8:12 - JEXTSEL"]
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<DFSDM_FLT3CR1rs> {
        JEXTSEL_W::new(self, 8)
    }
    #[doc = "Bits 13:14 - JEXTEN"]
    #[inline(always)]
    #[must_use]
    pub fn jexten(&mut self) -> JEXTEN_W<DFSDM_FLT3CR1rs> {
        JEXTEN_W::new(self, 13)
    }
    #[doc = "Bit 17 - RSWSTART"]
    #[inline(always)]
    #[must_use]
    pub fn rswstart(&mut self) -> RSWSTART_W<DFSDM_FLT3CR1rs> {
        RSWSTART_W::new(self, 17)
    }
    #[doc = "Bit 18 - RCONT"]
    #[inline(always)]
    #[must_use]
    pub fn rcont(&mut self) -> RCONT_W<DFSDM_FLT3CR1rs> {
        RCONT_W::new(self, 18)
    }
    #[doc = "Bit 19 - RSYNC"]
    #[inline(always)]
    #[must_use]
    pub fn rsync(&mut self) -> RSYNC_W<DFSDM_FLT3CR1rs> {
        RSYNC_W::new(self, 19)
    }
    #[doc = "Bit 21 - RDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn rdmaen(&mut self) -> RDMAEN_W<DFSDM_FLT3CR1rs> {
        RDMAEN_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - RCH"]
    #[inline(always)]
    #[must_use]
    pub fn rch(&mut self) -> RCH_W<DFSDM_FLT3CR1rs> {
        RCH_W::new(self, 24)
    }
    #[doc = "Bit 29 - FAST"]
    #[inline(always)]
    #[must_use]
    pub fn fast(&mut self) -> FAST_W<DFSDM_FLT3CR1rs> {
        FAST_W::new(self, 29)
    }
    #[doc = "Bit 30 - AWFSEL"]
    #[inline(always)]
    #[must_use]
    pub fn awfsel(&mut self) -> AWFSEL_W<DFSDM_FLT3CR1rs> {
        AWFSEL_W::new(self, 30)
    }
}
#[doc = "DFSDM filter 3 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt3cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt3cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT3CR1rs;
impl crate::RegisterSpec for DFSDM_FLT3CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt3cr1::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT3CR1rs {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_flt3cr1::W`](W) writer structure"]
impl crate::Writable for DFSDM_FLT3CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSDM_FLT3CR1 to value 0"]
impl crate::Resettable for DFSDM_FLT3CR1rs {
    const RESET_VALUE: u32 = 0;
}
