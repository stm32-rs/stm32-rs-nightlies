#[doc = "Register `C2AHB3ENR` reader"]
pub type R = crate::R<C2AHB3ENRrs>;
#[doc = "Register `C2AHB3ENR` writer"]
pub type W = crate::W<C2AHB3ENRrs>;
#[doc = "CPU2 PKA accelerator clock enable\n\nValue on reset: 0"]
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
#[doc = "Field `PKAEN` reader - CPU2 PKA accelerator clock enable"]
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
#[doc = "Field `PKAEN` writer - CPU2 PKA accelerator clock enable"]
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
#[doc = "Field `AESEN` reader - CPU2 AES accelerator clock enable"]
pub use PKAEN_R as AESEN_R;
#[doc = "Field `RNGEN` reader - CPU2 True RNG clocks enable"]
pub use PKAEN_R as RNGEN_R;
#[doc = "Field `HSEMEN` reader - CPU2 HSEM clock enable"]
pub use PKAEN_R as HSEMEN_R;
#[doc = "Field `IPCCEN` reader - CPU2 IPCC interface clock enable"]
pub use PKAEN_R as IPCCEN_R;
#[doc = "Field `FLASHEN` reader - CPU2 Flash interface clock enable"]
pub use PKAEN_R as FLASHEN_R;
#[doc = "Field `AESEN` writer - CPU2 AES accelerator clock enable"]
pub use PKAEN_W as AESEN_W;
#[doc = "Field `RNGEN` writer - CPU2 True RNG clocks enable"]
pub use PKAEN_W as RNGEN_W;
#[doc = "Field `HSEMEN` writer - CPU2 HSEM clock enable"]
pub use PKAEN_W as HSEMEN_W;
#[doc = "Field `IPCCEN` writer - CPU2 IPCC interface clock enable"]
pub use PKAEN_W as IPCCEN_W;
#[doc = "Field `FLASHEN` writer - CPU2 Flash interface clock enable"]
pub use PKAEN_W as FLASHEN_W;
impl R {
    #[doc = "Bit 16 - CPU2 PKA accelerator clock enable"]
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU2 AES accelerator clock enable"]
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU2 True RNG clocks enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU2 HSEM clock enable"]
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU2 IPCC interface clock enable"]
    #[inline(always)]
    pub fn ipccen(&self) -> IPCCEN_R {
        IPCCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25 - CPU2 Flash interface clock enable"]
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - CPU2 PKA accelerator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pkaen(&mut self) -> PKAEN_W<C2AHB3ENRrs> {
        PKAEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - CPU2 AES accelerator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn aesen(&mut self) -> AESEN_W<C2AHB3ENRrs> {
        AESEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - CPU2 True RNG clocks enable"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<C2AHB3ENRrs> {
        RNGEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - CPU2 HSEM clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsemen(&mut self) -> HSEMEN_W<C2AHB3ENRrs> {
        HSEMEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - CPU2 IPCC interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ipccen(&mut self) -> IPCCEN_W<C2AHB3ENRrs> {
        IPCCEN_W::new(self, 20)
    }
    #[doc = "Bit 25 - CPU2 Flash interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn flashen(&mut self) -> FLASHEN_W<C2AHB3ENRrs> {
        FLASHEN_W::new(self, 25)
    }
}
#[doc = "CPU2 AHB3 peripheral clock enable register \\[dual core device only\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2ahb3enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2ahb3enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2AHB3ENRrs;
impl crate::RegisterSpec for C2AHB3ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2ahb3enr::R`](R) reader structure"]
impl crate::Readable for C2AHB3ENRrs {}
#[doc = "`write(|w| ..)` method takes [`c2ahb3enr::W`](W) writer structure"]
impl crate::Writable for C2AHB3ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2AHB3ENR to value 0x0208_0000"]
impl crate::Resettable for C2AHB3ENRrs {
    const RESET_VALUE: u32 = 0x0208_0000;
}
