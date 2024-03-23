#[doc = "Register `PLLSAI2CFGR` reader"]
pub type R = crate::R<PLLSAI2CFGRrs>;
#[doc = "Register `PLLSAI2CFGR` writer"]
pub type W = crate::W<PLLSAI2CFGRrs>;
#[doc = "Field `PLLSAI2N` reader - SAI2PLL multiplication factor for VCO"]
pub type PLLSAI2N_R = crate::FieldReader;
#[doc = "Field `PLLSAI2N` writer - SAI2PLL multiplication factor for VCO"]
pub type PLLSAI2N_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PLLSAI2PEN` reader - SAI2PLL PLLSAI2CLK output enable"]
pub type PLLSAI2PEN_R = crate::BitReader;
#[doc = "Field `PLLSAI2PEN` writer - SAI2PLL PLLSAI2CLK output enable"]
pub type PLLSAI2PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI2P` reader - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
pub type PLLSAI2P_R = crate::BitReader;
#[doc = "Field `PLLSAI2P` writer - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
pub type PLLSAI2P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI2REN` reader - PLLSAI2 PLLADC2CLK output enable"]
pub type PLLSAI2REN_R = crate::BitReader;
#[doc = "Field `PLLSAI2REN` writer - PLLSAI2 PLLADC2CLK output enable"]
pub type PLLSAI2REN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI2R` reader - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
pub type PLLSAI2R_R = crate::FieldReader;
#[doc = "Field `PLLSAI2R` writer - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
pub type PLLSAI2R_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLSAI2PDIV` reader - PLLSAI2 division factor for PLLSAI2CLK"]
pub type PLLSAI2PDIV_R = crate::FieldReader;
#[doc = "Field `PLLSAI2PDIV` writer - PLLSAI2 division factor for PLLSAI2CLK"]
pub type PLLSAI2PDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 8:14 - SAI2PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn pllsai2n(&self) -> PLLSAI2N_R {
        PLLSAI2N_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - SAI2PLL PLLSAI2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2pen(&self) -> PLLSAI2PEN_R {
        PLLSAI2PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    pub fn pllsai2p(&self) -> PLLSAI2P_R {
        PLLSAI2P_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - PLLSAI2 PLLADC2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2ren(&self) -> PLLSAI2REN_R {
        PLLSAI2REN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
    #[inline(always)]
    pub fn pllsai2r(&self) -> PLLSAI2R_R {
        PLLSAI2R_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:31 - PLLSAI2 division factor for PLLSAI2CLK"]
    #[inline(always)]
    pub fn pllsai2pdiv(&self) -> PLLSAI2PDIV_R {
        PLLSAI2PDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:14 - SAI2PLL multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2n(&mut self) -> PLLSAI2N_W<PLLSAI2CFGRrs> {
        PLLSAI2N_W::new(self, 8)
    }
    #[doc = "Bit 16 - SAI2PLL PLLSAI2CLK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2pen(&mut self) -> PLLSAI2PEN_W<PLLSAI2CFGRrs> {
        PLLSAI2PEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2p(&mut self) -> PLLSAI2P_W<PLLSAI2CFGRrs> {
        PLLSAI2P_W::new(self, 17)
    }
    #[doc = "Bit 24 - PLLSAI2 PLLADC2CLK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2ren(&mut self) -> PLLSAI2REN_W<PLLSAI2CFGRrs> {
        PLLSAI2REN_W::new(self, 24)
    }
    #[doc = "Bits 25:26 - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2r(&mut self) -> PLLSAI2R_W<PLLSAI2CFGRrs> {
        PLLSAI2R_W::new(self, 25)
    }
    #[doc = "Bits 27:31 - PLLSAI2 division factor for PLLSAI2CLK"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2pdiv(&mut self) -> PLLSAI2PDIV_W<PLLSAI2CFGRrs> {
        PLLSAI2PDIV_W::new(self, 27)
    }
}
#[doc = "PLLSAI2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllsai2cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllsai2cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLSAI2CFGRrs;
impl crate::RegisterSpec for PLLSAI2CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllsai2cfgr::R`](R) reader structure"]
impl crate::Readable for PLLSAI2CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`pllsai2cfgr::W`](W) writer structure"]
impl crate::Writable for PLLSAI2CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLSAI2CFGR to value 0x1000"]
impl crate::Resettable for PLLSAI2CFGRrs {
    const RESET_VALUE: u32 = 0x1000;
}
