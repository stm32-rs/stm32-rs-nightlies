#[doc = "Register `AHB3FZR` reader"]
pub type R = crate::R<AHB3FZRrs>;
#[doc = "Register `AHB3FZR` writer"]
pub type W = crate::W<AHB3FZRrs>;
#[doc = "Field `DBG_LPDMA0_STOP` reader - LPDMA channel 0 stop in debug"]
pub type DBG_LPDMA0_STOP_R = crate::BitReader;
#[doc = "Field `DBG_LPDMA0_STOP` writer - LPDMA channel 0 stop in debug"]
pub type DBG_LPDMA0_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPDMA1_STOP` reader - LPDMA channel 1 stop in debug"]
pub type DBG_LPDMA1_STOP_R = crate::BitReader;
#[doc = "Field `DBG_LPDMA1_STOP` writer - LPDMA channel 1 stop in debug"]
pub type DBG_LPDMA1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPDMA2_STOP` reader - LPDMA channel 2 stop in debug"]
pub type DBG_LPDMA2_STOP_R = crate::BitReader;
#[doc = "Field `DBG_LPDMA2_STOP` writer - LPDMA channel 2 stop in debug"]
pub type DBG_LPDMA2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPDMA3_STOP` reader - LPDMA channel 3 stop in debug"]
pub type DBG_LPDMA3_STOP_R = crate::BitReader;
#[doc = "Field `DBG_LPDMA3_STOP` writer - LPDMA channel 3 stop in debug"]
pub type DBG_LPDMA3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LPDMA channel 0 stop in debug"]
    #[inline(always)]
    pub fn dbg_lpdma0_stop(&self) -> DBG_LPDMA0_STOP_R {
        DBG_LPDMA0_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPDMA channel 1 stop in debug"]
    #[inline(always)]
    pub fn dbg_lpdma1_stop(&self) -> DBG_LPDMA1_STOP_R {
        DBG_LPDMA1_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LPDMA channel 2 stop in debug"]
    #[inline(always)]
    pub fn dbg_lpdma2_stop(&self) -> DBG_LPDMA2_STOP_R {
        DBG_LPDMA2_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPDMA channel 3 stop in debug"]
    #[inline(always)]
    pub fn dbg_lpdma3_stop(&self) -> DBG_LPDMA3_STOP_R {
        DBG_LPDMA3_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPDMA channel 0 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lpdma0_stop(&mut self) -> DBG_LPDMA0_STOP_W<AHB3FZRrs> {
        DBG_LPDMA0_STOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - LPDMA channel 1 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lpdma1_stop(&mut self) -> DBG_LPDMA1_STOP_W<AHB3FZRrs> {
        DBG_LPDMA1_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - LPDMA channel 2 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lpdma2_stop(&mut self) -> DBG_LPDMA2_STOP_W<AHB3FZRrs> {
        DBG_LPDMA2_STOP_W::new(self, 2)
    }
    #[doc = "Bit 3 - LPDMA channel 3 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lpdma3_stop(&mut self) -> DBG_LPDMA3_STOP_W<AHB3FZRrs> {
        DBG_LPDMA3_STOP_W::new(self, 3)
    }
}
#[doc = "Debug MCU AHB3 peripheral freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3fzr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3fzr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB3FZRrs;
impl crate::RegisterSpec for AHB3FZRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3fzr::R`](R) reader structure"]
impl crate::Readable for AHB3FZRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb3fzr::W`](W) writer structure"]
impl crate::Writable for AHB3FZRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB3FZR to value 0"]
impl crate::Resettable for AHB3FZRrs {
    const RESET_VALUE: u32 = 0;
}
