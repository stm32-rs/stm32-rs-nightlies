#[doc = "Register `AHB2ENR` reader"]
pub type R = crate::R<AHB2ENRrs>;
#[doc = "Register `AHB2ENR` writer"]
pub type W = crate::W<AHB2ENRrs>;
#[doc = "CPU1 IO port A clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<GPIOAEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOAEN` reader - CPU1 IO port A clock enable"]
pub type GPIOAEN_R = crate::BitReader<GPIOAEN>;
impl GPIOAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOAEN {
        match self.bits {
            false => GPIOAEN::Disabled,
            true => GPIOAEN::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN::Enabled
    }
}
#[doc = "Field `GPIOAEN` writer - CPU1 IO port A clock enable"]
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOAEN>;
impl<'a, REG> GPIOAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Enabled)
    }
}
#[doc = "Field `GPIOBEN` reader - CPU1 IO port B clock enable"]
pub use GPIOAEN_R as GPIOBEN_R;
#[doc = "Field `GPIOCEN` reader - CPU1 IO port C clock enable"]
pub use GPIOAEN_R as GPIOCEN_R;
#[doc = "Field `GPIOHEN` reader - CPU1 IO port H clock enable"]
pub use GPIOAEN_R as GPIOHEN_R;
#[doc = "Field `GPIOBEN` writer - CPU1 IO port B clock enable"]
pub use GPIOAEN_W as GPIOBEN_W;
#[doc = "Field `GPIOCEN` writer - CPU1 IO port C clock enable"]
pub use GPIOAEN_W as GPIOCEN_W;
#[doc = "Field `GPIOHEN` writer - CPU1 IO port H clock enable"]
pub use GPIOAEN_W as GPIOHEN_W;
impl R {
    #[doc = "Bit 0 - CPU1 IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU1 IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU1 IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU1 IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU1 IO port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<AHB2ENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU1 IO port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<AHB2ENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU1 IO port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<AHB2ENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    #[doc = "Bit 7 - CPU1 IO port H clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<AHB2ENRrs> {
        GPIOHEN_W::new(self, 7)
    }
}
#[doc = "AHB2 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2ENRrs;
impl crate::RegisterSpec for AHB2ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2enr::R`](R) reader structure"]
impl crate::Readable for AHB2ENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure"]
impl crate::Writable for AHB2ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2ENR to value 0"]
impl crate::Resettable for AHB2ENRrs {
    const RESET_VALUE: u32 = 0;
}
