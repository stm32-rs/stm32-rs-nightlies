#[doc = "Register `PLLSAI1CFGR` reader"]
pub type R = crate::R<PLLSAI1CFGRrs>;
#[doc = "Register `PLLSAI1CFGR` writer"]
pub type W = crate::W<PLLSAI1CFGRrs>;
#[doc = "Field `PLLSAI1N` reader - SAI1PLL multiplication factor for VCO"]
pub type PLLSAI1N_R = crate::FieldReader;
#[doc = "Field `PLLSAI1N` writer - SAI1PLL multiplication factor for VCO"]
pub type PLLSAI1N_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PLLSAI1PEN` reader - SAI1PLL PLLSAI1CLK output enable"]
pub type PLLSAI1PEN_R = crate::BitReader;
#[doc = "Field `PLLSAI1PEN` writer - SAI1PLL PLLSAI1CLK output enable"]
pub type PLLSAI1PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI1P` reader - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)"]
pub type PLLSAI1P_R = crate::BitReader;
#[doc = "Field `PLLSAI1P` writer - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)"]
pub type PLLSAI1P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI1QEN` reader - SAI1PLL PLLUSB2CLK output enable"]
pub type PLLSAI1QEN_R = crate::BitReader;
#[doc = "Field `PLLSAI1QEN` writer - SAI1PLL PLLUSB2CLK output enable"]
pub type PLLSAI1QEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI1Q` reader - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)"]
pub type PLLSAI1Q_R = crate::FieldReader;
#[doc = "Field `PLLSAI1Q` writer - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)"]
pub type PLLSAI1Q_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLSAI1REN` reader - PLLSAI1 PLLADC1CLK output enable"]
pub type PLLSAI1REN_R = crate::BitReader;
#[doc = "Field `PLLSAI1REN` writer - PLLSAI1 PLLADC1CLK output enable"]
pub type PLLSAI1REN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI1R` reader - PLLSAI1 division factor for PLLADC1CLK (ADC clock)"]
pub type PLLSAI1R_R = crate::FieldReader;
#[doc = "Field `PLLSAI1R` writer - PLLSAI1 division factor for PLLADC1CLK (ADC clock)"]
pub type PLLSAI1R_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 8:14 - SAI1PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn pllsai1n(&self) -> PLLSAI1N_R {
        PLLSAI1N_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - SAI1PLL PLLSAI1CLK output enable"]
    #[inline(always)]
    pub fn pllsai1pen(&self) -> PLLSAI1PEN_R {
        PLLSAI1PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    pub fn pllsai1p(&self) -> PLLSAI1P_R {
        PLLSAI1P_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - SAI1PLL PLLUSB2CLK output enable"]
    #[inline(always)]
    pub fn pllsai1qen(&self) -> PLLSAI1QEN_R {
        PLLSAI1QEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)"]
    #[inline(always)]
    pub fn pllsai1q(&self) -> PLLSAI1Q_R {
        PLLSAI1Q_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - PLLSAI1 PLLADC1CLK output enable"]
    #[inline(always)]
    pub fn pllsai1ren(&self) -> PLLSAI1REN_R {
        PLLSAI1REN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - PLLSAI1 division factor for PLLADC1CLK (ADC clock)"]
    #[inline(always)]
    pub fn pllsai1r(&self) -> PLLSAI1R_R {
        PLLSAI1R_R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:14 - SAI1PLL multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai1n(&mut self) -> PLLSAI1N_W<PLLSAI1CFGRrs> {
        PLLSAI1N_W::new(self, 8)
    }
    #[doc = "Bit 16 - SAI1PLL PLLSAI1CLK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai1pen(&mut self) -> PLLSAI1PEN_W<PLLSAI1CFGRrs> {
        PLLSAI1PEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai1p(&mut self) -> PLLSAI1P_W<PLLSAI1CFGRrs> {
        PLLSAI1P_W::new(self, 17)
    }
    #[doc = "Bit 20 - SAI1PLL PLLUSB2CLK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai1qen(&mut self) -> PLLSAI1QEN_W<PLLSAI1CFGRrs> {
        PLLSAI1QEN_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai1q(&mut self) -> PLLSAI1Q_W<PLLSAI1CFGRrs> {
        PLLSAI1Q_W::new(self, 21)
    }
    #[doc = "Bit 24 - PLLSAI1 PLLADC1CLK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai1ren(&mut self) -> PLLSAI1REN_W<PLLSAI1CFGRrs> {
        PLLSAI1REN_W::new(self, 24)
    }
    #[doc = "Bits 25:26 - PLLSAI1 division factor for PLLADC1CLK (ADC clock)"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai1r(&mut self) -> PLLSAI1R_W<PLLSAI1CFGRrs> {
        PLLSAI1R_W::new(self, 25)
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
#[doc = "`reset()` method sets PLLSAI1CFGR to value 0x1000"]
impl crate::Resettable for PLLSAI1CFGRrs {
    const RESET_VALUE: u32 = 0x1000;
}
