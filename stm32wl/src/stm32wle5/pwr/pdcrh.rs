#[doc = "Register `PDCRH` reader"]
pub type R = crate::R<PDCRHrs>;
#[doc = "Register `PDCRH` writer"]
pub type W = crate::W<PDCRHrs>;
#[doc = "pull-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD3 {
    #[doc = "0: Disable the pull-down on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    Disabled = 0,
    #[doc = "1: Enable the pull-down on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    Enabled = 1,
}
impl From<PD3> for bool {
    #[inline(always)]
    fn from(variant: PD3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD3` reader - pull-down"]
pub type PD3_R = crate::BitReader<PD3>;
impl PD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PD3 {
        match self.bits {
            false => PD3::Disabled,
            true => PD3::Enabled,
        }
    }
    #[doc = "Disable the pull-down on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PD3::Disabled
    }
    #[doc = "Enable the pull-down on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PD3::Enabled
    }
}
#[doc = "Field `PD3` writer - pull-down"]
pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG, PD3>;
impl<'a, REG> PD3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the pull-down on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PD3::Disabled)
    }
    #[doc = "Enable the pull-down on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PD3::Enabled)
    }
}
impl R {
    #[doc = "Bit 3 - pull-down"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<PDCRHrs> {
        PD3_W::new(self, 3)
    }
}
#[doc = "Power Port H pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDCRHrs;
impl crate::RegisterSpec for PDCRHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdcrh::R`](R) reader structure"]
impl crate::Readable for PDCRHrs {}
#[doc = "`write(|w| ..)` method takes [`pdcrh::W`](W) writer structure"]
impl crate::Writable for PDCRHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDCRH to value 0"]
impl crate::Resettable for PDCRHrs {
    const RESET_VALUE: u32 = 0;
}
