#[doc = "Register `PSR` reader"]
pub type R = crate::R<PSRrs>;
#[doc = "Register `PSR` writer"]
pub type W = crate::W<PSRrs>;
#[doc = "Field `PD` reader - PHY direction"]
pub type PD_R = crate::BitReader;
#[doc = "Field `PD` writer - PHY direction"]
pub type PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSSC` reader - PHY stop state clock lane"]
pub type PSSC_R = crate::BitReader;
#[doc = "Field `PSSC` writer - PHY stop state clock lane"]
pub type PSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UANC` reader - ULPS active not clock lane"]
pub type UANC_R = crate::BitReader;
#[doc = "Field `UANC` writer - ULPS active not clock lane"]
pub type UANC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSS0` reader - PHY stop state lane 0"]
pub type PSS0_R = crate::BitReader;
#[doc = "Field `PSS0` writer - PHY stop state lane 0"]
pub type PSS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UAN0` reader - ULPS active not lane 1"]
pub type UAN0_R = crate::BitReader;
#[doc = "Field `UAN0` writer - ULPS active not lane 1"]
pub type UAN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUE0` reader - RX ULPS escape lane 0"]
pub type RUE0_R = crate::BitReader;
#[doc = "Field `RUE0` writer - RX ULPS escape lane 0"]
pub type RUE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSS1` reader - PHY stop state lane 1"]
pub type PSS1_R = crate::BitReader;
#[doc = "Field `PSS1` writer - PHY stop state lane 1"]
pub type PSS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UAN1` reader - ULPS active not lane 1"]
pub type UAN1_R = crate::BitReader;
#[doc = "Field `UAN1` writer - ULPS active not lane 1"]
pub type UAN1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - PHY direction"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PHY stop state clock lane"]
    #[inline(always)]
    pub fn pssc(&self) -> PSSC_R {
        PSSC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ULPS active not clock lane"]
    #[inline(always)]
    pub fn uanc(&self) -> UANC_R {
        UANC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PHY stop state lane 0"]
    #[inline(always)]
    pub fn pss0(&self) -> PSS0_R {
        PSS0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ULPS active not lane 1"]
    #[inline(always)]
    pub fn uan0(&self) -> UAN0_R {
        UAN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX ULPS escape lane 0"]
    #[inline(always)]
    pub fn rue0(&self) -> RUE0_R {
        RUE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PHY stop state lane 1"]
    #[inline(always)]
    pub fn pss1(&self) -> PSS1_R {
        PSS1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ULPS active not lane 1"]
    #[inline(always)]
    pub fn uan1(&self) -> UAN1_R {
        UAN1_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - PHY direction"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<PSRrs> {
        PD_W::new(self, 1)
    }
    #[doc = "Bit 2 - PHY stop state clock lane"]
    #[inline(always)]
    #[must_use]
    pub fn pssc(&mut self) -> PSSC_W<PSRrs> {
        PSSC_W::new(self, 2)
    }
    #[doc = "Bit 3 - ULPS active not clock lane"]
    #[inline(always)]
    #[must_use]
    pub fn uanc(&mut self) -> UANC_W<PSRrs> {
        UANC_W::new(self, 3)
    }
    #[doc = "Bit 4 - PHY stop state lane 0"]
    #[inline(always)]
    #[must_use]
    pub fn pss0(&mut self) -> PSS0_W<PSRrs> {
        PSS0_W::new(self, 4)
    }
    #[doc = "Bit 5 - ULPS active not lane 1"]
    #[inline(always)]
    #[must_use]
    pub fn uan0(&mut self) -> UAN0_W<PSRrs> {
        UAN0_W::new(self, 5)
    }
    #[doc = "Bit 6 - RX ULPS escape lane 0"]
    #[inline(always)]
    #[must_use]
    pub fn rue0(&mut self) -> RUE0_W<PSRrs> {
        RUE0_W::new(self, 6)
    }
    #[doc = "Bit 7 - PHY stop state lane 1"]
    #[inline(always)]
    #[must_use]
    pub fn pss1(&mut self) -> PSS1_W<PSRrs> {
        PSS1_W::new(self, 7)
    }
    #[doc = "Bit 8 - ULPS active not lane 1"]
    #[inline(always)]
    #[must_use]
    pub fn uan1(&mut self) -> UAN1_W<PSRrs> {
        UAN1_W::new(self, 8)
    }
}
#[doc = "DSI Host PHY status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSRrs;
impl crate::RegisterSpec for PSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PSRrs {}
#[doc = "`write(|w| ..)` method takes [`psr::W`](W) writer structure"]
impl crate::Writable for PSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSR to value 0x1528"]
impl crate::Resettable for PSRrs {
    const RESET_VALUE: u32 = 0x1528;
}
