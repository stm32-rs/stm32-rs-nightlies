#[doc = "Register `AHB3ENR` reader"]
pub type R = crate::R<AHB3ENRrs>;
#[doc = "Register `AHB3ENR` writer"]
pub type W = crate::W<AHB3ENRrs>;
#[doc = "PKAEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKAEN {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<PKAEN> for bool {
    #[inline(always)]
    fn from(variant: PKAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PKAEN` reader - PKAEN"]
pub type PKAEN_R = crate::BitReader<PKAEN>;
impl PKAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PKAEN {
        match self.bits {
            false => PKAEN::Disabled,
            true => PKAEN::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PKAEN::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PKAEN::Enabled
    }
}
#[doc = "Field `PKAEN` writer - PKAEN"]
pub type PKAEN_W<'a, REG> = crate::BitWriter<'a, REG, PKAEN>;
impl<'a, REG> PKAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PKAEN::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PKAEN::Enabled)
    }
}
#[doc = "Field `AESEN` reader - AESEN"]
pub use PKAEN_R as AESEN_R;
#[doc = "Field `RNGEN` reader - RNGEN"]
pub use PKAEN_R as RNGEN_R;
#[doc = "Field `HSEMEN` reader - HSEMEN"]
pub use PKAEN_R as HSEMEN_R;
#[doc = "Field `IPCCEN` reader - IPCCEN"]
pub use PKAEN_R as IPCCEN_R;
#[doc = "Field `FLASHEN` reader - CPU1 Flash interface clock enable"]
pub use PKAEN_R as FLASHEN_R;
#[doc = "Field `AESEN` writer - AESEN"]
pub use PKAEN_W as AESEN_W;
#[doc = "Field `RNGEN` writer - RNGEN"]
pub use PKAEN_W as RNGEN_W;
#[doc = "Field `HSEMEN` writer - HSEMEN"]
pub use PKAEN_W as HSEMEN_W;
#[doc = "Field `IPCCEN` writer - IPCCEN"]
pub use PKAEN_W as IPCCEN_W;
#[doc = "Field `FLASHEN` writer - CPU1 Flash interface clock enable"]
pub use PKAEN_W as FLASHEN_W;
impl R {
    #[doc = "Bit 16 - PKAEN"]
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AESEN"]
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RNGEN"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSEMEN"]
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - IPCCEN"]
    #[inline(always)]
    pub fn ipccen(&self) -> IPCCEN_R {
        IPCCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25 - CPU1 Flash interface clock enable"]
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - PKAEN"]
    #[inline(always)]
    #[must_use]
    pub fn pkaen(&mut self) -> PKAEN_W<AHB3ENRrs> {
        PKAEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - AESEN"]
    #[inline(always)]
    #[must_use]
    pub fn aesen(&mut self) -> AESEN_W<AHB3ENRrs> {
        AESEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - RNGEN"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<AHB3ENRrs> {
        RNGEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - HSEMEN"]
    #[inline(always)]
    #[must_use]
    pub fn hsemen(&mut self) -> HSEMEN_W<AHB3ENRrs> {
        HSEMEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - IPCCEN"]
    #[inline(always)]
    #[must_use]
    pub fn ipccen(&mut self) -> IPCCEN_W<AHB3ENRrs> {
        IPCCEN_W::new(self, 20)
    }
    #[doc = "Bit 25 - CPU1 Flash interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn flashen(&mut self) -> FLASHEN_W<AHB3ENRrs> {
        FLASHEN_W::new(self, 25)
    }
}
#[doc = "AHB3 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB3ENRrs;
impl crate::RegisterSpec for AHB3ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3enr::R`](R) reader structure"]
impl crate::Readable for AHB3ENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb3enr::W`](W) writer structure"]
impl crate::Writable for AHB3ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB3ENR to value 0x0208_0000"]
impl crate::Resettable for AHB3ENRrs {
    const RESET_VALUE: u32 = 0x0208_0000;
}
