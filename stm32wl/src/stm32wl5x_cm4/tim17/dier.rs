#[doc = "Register `DIER` reader"]
pub type R = crate::R<DIERrs>;
#[doc = "Register `DIER` writer"]
pub type W = crate::W<DIERrs>;
#[doc = "Update interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIE {
    #[doc = "0: Update interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Update interrupt enabled"]
    Enabled = 1,
}
impl From<UIE> for bool {
    #[inline(always)]
    fn from(variant: UIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIE` reader - Update interrupt enable"]
pub type UIE_R = crate::BitReader<UIE>;
impl UIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UIE {
        match self.bits {
            false => UIE::Disabled,
            true => UIE::Enabled,
        }
    }
    #[doc = "Update interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UIE::Disabled
    }
    #[doc = "Update interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UIE::Enabled
    }
}
#[doc = "Field `UIE` writer - Update interrupt enable"]
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG, UIE>;
impl<'a, REG> UIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UIE::Disabled)
    }
    #[doc = "Update interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UIE::Enabled)
    }
}
#[doc = "Field `CCIE(1-1)` reader - Capture/Compare %s interrupt enable"]
pub type CCIE_R = crate::BitReader;
#[doc = "Field `CCIE(1-1)` writer - Capture/Compare %s interrupt enable"]
pub type CCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMIE` reader - COM interrupt enable"]
pub type COMIE_R = crate::BitReader;
#[doc = "Field `COMIE` writer - COM interrupt enable"]
pub type COMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIE` reader - Break interrupt enable"]
pub type BIE_R = crate::BitReader;
#[doc = "Field `BIE` writer - Break interrupt enable"]
pub type BIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Update DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDE {
    #[doc = "0: Update DMA request disabled"]
    Disabled = 0,
    #[doc = "1: Update DMA request enabled"]
    Enabled = 1,
}
impl From<UDE> for bool {
    #[inline(always)]
    fn from(variant: UDE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDE` reader - Update DMA request enable"]
pub type UDE_R = crate::BitReader<UDE>;
impl UDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UDE {
        match self.bits {
            false => UDE::Disabled,
            true => UDE::Enabled,
        }
    }
    #[doc = "Update DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UDE::Disabled
    }
    #[doc = "Update DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UDE::Enabled
    }
}
#[doc = "Field `UDE` writer - Update DMA request enable"]
pub type UDE_W<'a, REG> = crate::BitWriter<'a, REG, UDE>;
impl<'a, REG> UDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UDE::Disabled)
    }
    #[doc = "Update DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UDE::Enabled)
    }
}
#[doc = "Field `CCDE(1-1)` reader - Capture/Compare %s DMA request enable"]
pub type CCDE_R = crate::BitReader;
#[doc = "Field `CCDE(1-1)` writer - Capture/Compare %s DMA request enable"]
pub type CCDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Capture/Compare (1-1) interrupt enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1IE` field"]
    #[inline(always)]
    pub fn ccie(&self, n: u8) -> CCIE_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCIE_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (1-1) interrupt enable"]
    #[inline(always)]
    pub fn ccie_iter(&self) -> impl Iterator<Item = CCIE_R> + '_ {
        (0..1).map(move |n| CCIE_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn comie(&self) -> COMIE_R {
        COMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Capture/Compare (1-1) DMA request enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1DE` field"]
    #[inline(always)]
    pub fn ccde(&self, n: u8) -> CCDE_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCDE_R::new(((self.bits >> (n * 0 + 9)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (1-1) DMA request enable"]
    #[inline(always)]
    pub fn ccde_iter(&self) -> impl Iterator<Item = CCDE_R> + '_ {
        (0..1).map(move |n| CCDE_R::new(((self.bits >> (n * 0 + 9)) & 1) != 0))
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub fn cc1de(&self) -> CCDE_R {
        CCDE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UIE_W<DIERrs> {
        UIE_W::new(self, 0)
    }
    #[doc = "Capture/Compare (1-1) interrupt enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1IE` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccie(&mut self, n: u8) -> CCIE_W<DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCIE_W::new(self, n * 0 + 1)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ie(&mut self) -> CCIE_W<DIERrs> {
        CCIE_W::new(self, 1)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn comie(&mut self) -> COMIE_W<DIERrs> {
        COMIE_W::new(self, 5)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bie(&mut self) -> BIE_W<DIERrs> {
        BIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ude(&mut self) -> UDE_W<DIERrs> {
        UDE_W::new(self, 8)
    }
    #[doc = "Capture/Compare (1-1) DMA request enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1DE` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccde(&mut self, n: u8) -> CCDE_W<DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCDE_W::new(self, n * 0 + 9)
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1de(&mut self) -> CCDE_W<DIERrs> {
        CCDE_W::new(self, 9)
    }
}
#[doc = "TIM16/TIM17 DMA/interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIERrs;
impl crate::RegisterSpec for DIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dier::R`](R) reader structure"]
impl crate::Readable for DIERrs {}
#[doc = "`write(|w| ..)` method takes [`dier::W`](W) writer structure"]
impl crate::Writable for DIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIER to value 0"]
impl crate::Resettable for DIERrs {
    const RESET_VALUE: u32 = 0;
}
