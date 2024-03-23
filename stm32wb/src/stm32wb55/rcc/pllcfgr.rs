#[doc = "Register `PLLCFGR` reader"]
pub type R = crate::R<PLLCFGRrs>;
#[doc = "Register `PLLCFGR` writer"]
pub type W = crate::W<PLLCFGRrs>;
#[doc = "Field `PLLSRC` reader - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
pub type PLLSRC_R = crate::FieldReader;
#[doc = "Field `PLLSRC` writer - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
pub type PLLSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLM` reader - Division factor M for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
pub type PLLM_R = crate::FieldReader;
#[doc = "Field `PLLM` writer - Division factor M for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
pub type PLLM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLN` reader - Main PLLSYS multiplication factor N"]
pub type PLLN_R = crate::FieldReader;
#[doc = "Field `PLLN` writer - Main PLLSYS multiplication factor N"]
pub type PLLN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PLLPEN` reader - Main PLLSYSP output enable"]
pub type PLLPEN_R = crate::BitReader;
#[doc = "Field `PLLPEN` writer - Main PLLSYSP output enable"]
pub type PLLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLP` reader - Main PLL division factor P for PPLSYSSAICLK"]
pub type PLLP_R = crate::FieldReader;
#[doc = "Field `PLLP` writer - Main PLL division factor P for PPLSYSSAICLK"]
pub type PLLP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PLLQEN` reader - Main PLLSYSQ output enable"]
pub type PLLQEN_R = crate::BitReader;
#[doc = "Field `PLLQEN` writer - Main PLLSYSQ output enable"]
pub type PLLQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLQ` reader - Main PLLSYS division factor Q for PLLSYSUSBCLK"]
pub type PLLQ_R = crate::FieldReader;
#[doc = "Field `PLLQ` writer - Main PLLSYS division factor Q for PLLSYSUSBCLK"]
pub type PLLQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLREN` reader - Main PLLSYSR PLLCLK output enable"]
pub type PLLREN_R = crate::BitReader;
#[doc = "Field `PLLREN` writer - Main PLLSYSR PLLCLK output enable"]
pub type PLLREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLR` reader - Main PLLSYS division factor R for SYSCLK (system clock)"]
pub type PLLR_R = crate::FieldReader;
#[doc = "Field `PLLR` writer - Main PLLSYS division factor R for SYSCLK (system clock)"]
pub type PLLR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Division factor M for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:14 - Main PLLSYS multiplication factor N"]
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Main PLLSYSP output enable"]
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - Main PLL division factor P for PPLSYSSAICLK"]
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Main PLLSYSQ output enable"]
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Main PLLSYS division factor Q for PLLSYSUSBCLK"]
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - Main PLLSYSR PLLCLK output enable"]
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Main PLLSYS division factor R for SYSCLK (system clock)"]
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<PLLCFGRrs> {
        PLLSRC_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Division factor M for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllm(&mut self) -> PLLM_W<PLLCFGRrs> {
        PLLM_W::new(self, 4)
    }
    #[doc = "Bits 8:14 - Main PLLSYS multiplication factor N"]
    #[inline(always)]
    #[must_use]
    pub fn plln(&mut self) -> PLLN_W<PLLCFGRrs> {
        PLLN_W::new(self, 8)
    }
    #[doc = "Bit 16 - Main PLLSYSP output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllpen(&mut self) -> PLLPEN_W<PLLCFGRrs> {
        PLLPEN_W::new(self, 16)
    }
    #[doc = "Bits 17:21 - Main PLL division factor P for PPLSYSSAICLK"]
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PLLP_W<PLLCFGRrs> {
        PLLP_W::new(self, 17)
    }
    #[doc = "Bit 24 - Main PLLSYSQ output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllqen(&mut self) -> PLLQEN_W<PLLCFGRrs> {
        PLLQEN_W::new(self, 24)
    }
    #[doc = "Bits 25:27 - Main PLLSYS division factor Q for PLLSYSUSBCLK"]
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PLLQ_W<PLLCFGRrs> {
        PLLQ_W::new(self, 25)
    }
    #[doc = "Bit 28 - Main PLLSYSR PLLCLK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllren(&mut self) -> PLLREN_W<PLLCFGRrs> {
        PLLREN_W::new(self, 28)
    }
    #[doc = "Bits 29:31 - Main PLLSYS division factor R for SYSCLK (system clock)"]
    #[inline(always)]
    #[must_use]
    pub fn pllr(&mut self) -> PLLR_W<PLLCFGRrs> {
        PLLR_W::new(self, 29)
    }
}
#[doc = "PLLSYS configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLCFGRrs;
impl crate::RegisterSpec for PLLCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcfgr::R`](R) reader structure"]
impl crate::Readable for PLLCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`pllcfgr::W`](W) writer structure"]
impl crate::Writable for PLLCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLCFGR to value 0x2204_0100"]
impl crate::Resettable for PLLCFGRrs {
    const RESET_VALUE: u32 = 0x2204_0100;
}
