#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Field `SW` reader - System clock switch"]
pub type SW_R = crate::FieldReader;
#[doc = "Field `SW` writer - System clock switch"]
pub type SW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWS` reader - System clock switch status"]
pub type SWS_R = crate::FieldReader;
#[doc = "Field `HPRE` reader - AHB prescaler"]
pub type HPRE_R = crate::FieldReader;
#[doc = "Field `HPRE` writer - AHB prescaler"]
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPRE1` reader - APB low-speed prescaler (APB1)"]
pub type PPRE1_R = crate::FieldReader;
#[doc = "Field `PPRE1` writer - APB low-speed prescaler (APB1)"]
pub type PPRE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PPRE2` reader - APB high-speed prescaler (APB2)"]
pub type PPRE2_R = crate::FieldReader;
#[doc = "Field `PPRE2` writer - APB high-speed prescaler (APB2)"]
pub type PPRE2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLSRC` reader - PLL entry clock source"]
pub type PLLSRC_R = crate::BitReader;
#[doc = "Field `PLLSRC` writer - PLL entry clock source"]
pub type PLLSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLMUL` reader - PLL multiplication factor"]
pub type PLLMUL_R = crate::FieldReader;
#[doc = "Field `PLLMUL` writer - PLL multiplication factor"]
pub type PLLMUL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLLDIV` reader - PLL output division"]
pub type PLLDIV_R = crate::FieldReader;
#[doc = "Field `PLLDIV` writer - PLL output division"]
pub type PLLDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCOSEL` reader - Microcontroller clock output selection"]
pub type MCOSEL_R = crate::FieldReader;
#[doc = "Field `MCOSEL` writer - Microcontroller clock output selection"]
pub type MCOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCOPRE` reader - Microcontroller clock output prescaler"]
pub type MCOPRE_R = crate::FieldReader;
#[doc = "Field `MCOPRE` writer - Microcontroller clock output prescaler"]
pub type MCOPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB low-speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 16 - PLL entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:21 - PLL multiplication factor"]
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - PLL output division"]
    #[inline(always)]
    pub fn plldiv(&self) -> PLLDIV_R {
        PLLDIV_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output selection"]
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Microcontroller clock output prescaler"]
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<CFGRrs> {
        SW_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<CFGRrs> {
        HPRE_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - APB low-speed prescaler (APB1)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre1(&mut self) -> PPRE1_W<CFGRrs> {
        PPRE1_W::new(self, 8)
    }
    #[doc = "Bits 11:13 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre2(&mut self) -> PPRE2_W<CFGRrs> {
        PPRE2_W::new(self, 11)
    }
    #[doc = "Bit 16 - PLL entry clock source"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<CFGRrs> {
        PLLSRC_W::new(self, 16)
    }
    #[doc = "Bits 18:21 - PLL multiplication factor"]
    #[inline(always)]
    #[must_use]
    pub fn pllmul(&mut self) -> PLLMUL_W<CFGRrs> {
        PLLMUL_W::new(self, 18)
    }
    #[doc = "Bits 22:23 - PLL output division"]
    #[inline(always)]
    #[must_use]
    pub fn plldiv(&mut self) -> PLLDIV_W<CFGRrs> {
        PLLDIV_W::new(self, 22)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output selection"]
    #[inline(always)]
    #[must_use]
    pub fn mcosel(&mut self) -> MCOSEL_W<CFGRrs> {
        MCOSEL_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Microcontroller clock output prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn mcopre(&mut self) -> MCOPRE_W<CFGRrs> {
        MCOPRE_W::new(self, 28)
    }
}
#[doc = "Clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
