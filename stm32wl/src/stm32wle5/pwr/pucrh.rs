#[doc = "Register `PUCRH` reader"]
pub type R = crate::R<PUCRHrs>;
#[doc = "Register `PUCRH` writer"]
pub type W = crate::W<PUCRHrs>;
#[doc = "pull-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PU3 {
    #[doc = "0: Disable pull-up on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    Disabled = 0,
    #[doc = "1: Enable pull-up on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PH\\[y\\]
bit is also set"]
    Enabled = 1,
}
impl From<PU3> for bool {
    #[inline(always)]
    fn from(variant: PU3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PU3` reader - pull-up"]
pub type PU3_R = crate::BitReader<PU3>;
impl PU3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PU3 {
        match self.bits {
            false => PU3::Disabled,
            true => PU3::Enabled,
        }
    }
    #[doc = "Disable pull-up on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PU3::Disabled
    }
    #[doc = "Enable pull-up on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PH\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PU3::Enabled
    }
}
#[doc = "Field `PU3` writer - pull-up"]
pub type PU3_W<'a, REG> = crate::BitWriter<'a, REG, PU3>;
impl<'a, REG> PU3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable pull-up on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PU3::Disabled)
    }
    #[doc = "Enable pull-up on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PH\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PU3::Enabled)
    }
}
impl R {
    #[doc = "Bit 3 - pull-up"]
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - pull-up"]
    #[inline(always)]
    #[must_use]
    pub fn pu3(&mut self) -> PU3_W<PUCRHrs> {
        PU3_W::new(self, 3)
    }
}
#[doc = "Power Port H pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUCRHrs;
impl crate::RegisterSpec for PUCRHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pucrh::R`](R) reader structure"]
impl crate::Readable for PUCRHrs {}
#[doc = "`write(|w| ..)` method takes [`pucrh::W`](W) writer structure"]
impl crate::Writable for PUCRHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUCRH to value 0"]
impl crate::Resettable for PUCRHrs {
    const RESET_VALUE: u32 = 0;
}
