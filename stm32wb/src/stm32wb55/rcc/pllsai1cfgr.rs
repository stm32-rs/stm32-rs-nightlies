#[doc = "Register `PLLSAI1CFGR` reader"]
pub type R = crate::R<PLLSAI1CFGRrs>;
#[doc = "Register `PLLSAI1CFGR` writer"]
pub type W = crate::W<PLLSAI1CFGRrs>;
#[doc = "Field `PLLN` reader - SAIPLL multiplication factor for VCO"]
pub type PLLN_R = crate::FieldReader;
#[doc = "Field `PLLN` writer - SAIPLL multiplication factor for VCO"]
pub type PLLN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PLLPEN` reader - SAIPLL PLLSAI1CLK output enable"]
pub type PLLPEN_R = crate::BitReader;
#[doc = "Field `PLLPEN` writer - SAIPLL PLLSAI1CLK output enable"]
pub type PLLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLP` reader - SAI1PLL division factor P for PLLSAICLK (SAI1clock)"]
pub type PLLP_R = crate::FieldReader;
#[doc = "Field `PLLP` writer - SAI1PLL division factor P for PLLSAICLK (SAI1clock)"]
pub type PLLP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PLLQEN` reader - SAIPLL PLLSAIUSBCLK output enable"]
pub type PLLQEN_R = crate::BitReader;
#[doc = "Field `PLLQEN` writer - SAIPLL PLLSAIUSBCLK output enable"]
pub type PLLQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLQ` reader - SAIPLL division factor Q for PLLSAIUSBCLK (48 MHz clock)"]
pub type PLLQ_R = crate::FieldReader;
#[doc = "Field `PLLQ` writer - SAIPLL division factor Q for PLLSAIUSBCLK (48 MHz clock)"]
pub type PLLQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLREN` reader - PLLSAI PLLADC1CLK output enable"]
pub type PLLREN_R = crate::BitReader;
#[doc = "Field `PLLREN` writer - PLLSAI PLLADC1CLK output enable"]
pub type PLLREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLR` reader - PLLSAI division factor R for PLLADC1CLK (ADC clock)"]
pub type PLLR_R = crate::FieldReader;
#[doc = "Field `PLLR` writer - PLLSAI division factor R for PLLADC1CLK (ADC clock)"]
pub type PLLR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 8:14 - SAIPLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - SAIPLL PLLSAI1CLK output enable"]
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - SAI1PLL division factor P for PLLSAICLK (SAI1clock)"]
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - SAIPLL PLLSAIUSBCLK output enable"]
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - SAIPLL division factor Q for PLLSAIUSBCLK (48 MHz clock)"]
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - PLLSAI PLLADC1CLK output enable"]
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - PLLSAI division factor R for PLLADC1CLK (ADC clock)"]
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:14 - SAIPLL multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn plln(&mut self) -> PLLN_W<PLLSAI1CFGRrs> {
        PLLN_W::new(self, 8)
    }
    #[doc = "Bit 16 - SAIPLL PLLSAI1CLK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllpen(&mut self) -> PLLPEN_W<PLLSAI1CFGRrs> {
        PLLPEN_W::new(self, 16)
    }
    #[doc = "Bits 17:21 - SAI1PLL division factor P for PLLSAICLK (SAI1clock)"]
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PLLP_W<PLLSAI1CFGRrs> {
        PLLP_W::new(self, 17)
    }
    #[doc = "Bit 24 - SAIPLL PLLSAIUSBCLK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllqen(&mut self) -> PLLQEN_W<PLLSAI1CFGRrs> {
        PLLQEN_W::new(self, 24)
    }
    #[doc = "Bits 25:27 - SAIPLL division factor Q for PLLSAIUSBCLK (48 MHz clock)"]
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PLLQ_W<PLLSAI1CFGRrs> {
        PLLQ_W::new(self, 25)
    }
    #[doc = "Bit 28 - PLLSAI PLLADC1CLK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllren(&mut self) -> PLLREN_W<PLLSAI1CFGRrs> {
        PLLREN_W::new(self, 28)
    }
    #[doc = "Bits 29:31 - PLLSAI division factor R for PLLADC1CLK (ADC clock)"]
    #[inline(always)]
    #[must_use]
    pub fn pllr(&mut self) -> PLLR_W<PLLSAI1CFGRrs> {
        PLLR_W::new(self, 29)
    }
}
#[doc = "PLLSAI1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllsai1cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllsai1cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLSAI1CFGRrs;
impl crate::RegisterSpec for PLLSAI1CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllsai1cfgr::R`](R) reader structure"]
impl crate::Readable for PLLSAI1CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`pllsai1cfgr::W`](W) writer structure"]
impl crate::Writable for PLLSAI1CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLSAI1CFGR to value 0x2204_0100"]
impl crate::Resettable for PLLSAI1CFGRrs {
    const RESET_VALUE: u32 = 0x2204_0100;
}
