#[doc = "Register `APB1HENR` reader"]
pub type R = crate::R<APB1HENRrs>;
#[doc = "Register `APB1HENR` writer"]
pub type W = crate::W<APB1HENRrs>;
#[doc = "clock recovery system peripheral clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSEN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<CRSEN> for bool {
    #[inline(always)]
    fn from(variant: CRSEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSEN` reader - clock recovery system peripheral clock enable Set and reset by software."]
pub type CRSEN_R = crate::BitReader<CRSEN>;
impl CRSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRSEN {
        match self.bits {
            false => CRSEN::Disabled,
            true => CRSEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRSEN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRSEN::Enabled
    }
}
#[doc = "Field `CRSEN` writer - clock recovery system peripheral clock enable Set and reset by software."]
pub type CRSEN_W<'a, REG> = crate::BitWriter<'a, REG, CRSEN>;
impl<'a, REG> CRSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRSEN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRSEN::Enabled)
    }
}
#[doc = "Field `SWPMIEN` reader - SWPMI peripheral clocks enable Set and reset by software."]
pub use CRSEN_R as SWPMIEN_R;
#[doc = "Field `OPAMPEN` reader - OPAMP peripheral clock enable Set and reset by software."]
pub use CRSEN_R as OPAMPEN_R;
#[doc = "Field `MDIOSEN` reader - MDIOS peripheral clock enable Set and reset by software."]
pub use CRSEN_R as MDIOSEN_R;
#[doc = "Field `FDCANEN` reader - FDCAN peripheral clocks enable Set and reset by software. The peripheral clocks of the FDCAN are the kernel clock selected by FDCANSEL and provided to fdcan_ker_ck input, and the rcc_pclk1 bus interface clock."]
pub use CRSEN_R as FDCANEN_R;
#[doc = "Field `SWPMIEN` writer - SWPMI peripheral clocks enable Set and reset by software."]
pub use CRSEN_W as SWPMIEN_W;
#[doc = "Field `OPAMPEN` writer - OPAMP peripheral clock enable Set and reset by software."]
pub use CRSEN_W as OPAMPEN_W;
#[doc = "Field `MDIOSEN` writer - MDIOS peripheral clock enable Set and reset by software."]
pub use CRSEN_W as MDIOSEN_W;
#[doc = "Field `FDCANEN` writer - FDCAN peripheral clocks enable Set and reset by software. The peripheral clocks of the FDCAN are the kernel clock selected by FDCANSEL and provided to fdcan_ker_ck input, and the rcc_pclk1 bus interface clock."]
pub use CRSEN_W as FDCANEN_W;
impl R {
    #[doc = "Bit 1 - clock recovery system peripheral clock enable Set and reset by software."]
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SWPMI peripheral clocks enable Set and reset by software."]
    #[inline(always)]
    pub fn swpmien(&self) -> SWPMIEN_R {
        SWPMIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable Set and reset by software."]
    #[inline(always)]
    pub fn opampen(&self) -> OPAMPEN_R {
        OPAMPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable Set and reset by software."]
    #[inline(always)]
    pub fn mdiosen(&self) -> MDIOSEN_R {
        MDIOSEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - FDCAN peripheral clocks enable Set and reset by software. The peripheral clocks of the FDCAN are the kernel clock selected by FDCANSEL and provided to fdcan_ker_ck input, and the rcc_pclk1 bus interface clock."]
    #[inline(always)]
    pub fn fdcanen(&self) -> FDCANEN_R {
        FDCANEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - clock recovery system peripheral clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn crsen(&mut self) -> CRSEN_W<APB1HENRrs> {
        CRSEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - SWPMI peripheral clocks enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn swpmien(&mut self) -> SWPMIEN_W<APB1HENRrs> {
        SWPMIEN_W::new(self, 2)
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn opampen(&mut self) -> OPAMPEN_W<APB1HENRrs> {
        OPAMPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn mdiosen(&mut self) -> MDIOSEN_W<APB1HENRrs> {
        MDIOSEN_W::new(self, 5)
    }
    #[doc = "Bit 8 - FDCAN peripheral clocks enable Set and reset by software. The peripheral clocks of the FDCAN are the kernel clock selected by FDCANSEL and provided to fdcan_ker_ck input, and the rcc_pclk1 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn fdcanen(&mut self) -> FDCANEN_W<APB1HENRrs> {
        FDCANEN_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1henr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1henr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1HENRrs;
impl crate::RegisterSpec for APB1HENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1henr::R`](R) reader structure"]
impl crate::Readable for APB1HENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb1henr::W`](W) writer structure"]
impl crate::Writable for APB1HENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1HENR to value 0"]
impl crate::Resettable for APB1HENRrs {
    const RESET_VALUE: u32 = 0;
}
