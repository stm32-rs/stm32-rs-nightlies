#[doc = "Register `DIER` reader"]
pub type R = crate::R<DIERrs>;
#[doc = "Register `DIER` writer"]
pub type W = crate::W<DIERrs>;
#[doc = "Field `UIE` reader - Update interrupt enable"]
pub type UIE_R = crate::BitReader;
#[doc = "Field `UIE` writer - Update interrupt enable"]
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCIE(1-4)` reader - Capture/Compare %s interrupt enable"]
pub type CCIE_R = crate::BitReader;
#[doc = "Field `CCIE(1-4)` writer - Capture/Compare %s interrupt enable"]
pub type CCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMIE` reader - COM interrupt enable"]
pub type COMIE_R = crate::BitReader;
#[doc = "Field `COMIE` writer - COM interrupt enable"]
pub type COMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - Trigger interrupt enable"]
pub type TIE_R = crate::BitReader;
#[doc = "Field `TIE` writer - Trigger interrupt enable"]
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIE` reader - Break interrupt enable"]
pub type BIE_R = crate::BitReader;
#[doc = "Field `BIE` writer - Break interrupt enable"]
pub type BIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDE` reader - Update DMA request enable"]
pub type UDE_R = crate::BitReader;
#[doc = "Field `UDE` writer - Update DMA request enable"]
pub type UDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCDE(1-4)` reader - Capture/Compare %s DMA request enable"]
pub type CCDE_R = crate::BitReader;
#[doc = "Field `CCDE(1-4)` writer - Capture/Compare %s DMA request enable"]
pub type CCDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMDE` reader - COM DMA request enable"]
pub type COMDE_R = crate::BitReader;
#[doc = "Field `COMDE` writer - COM DMA request enable"]
pub type COMDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDE` reader - Trigger DMA request enable"]
pub type TDE_R = crate::BitReader;
#[doc = "Field `TDE` writer - Trigger DMA request enable"]
pub type TDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDXIE` reader - Index interrupt enable"]
pub type IDXIE_R = crate::BitReader;
#[doc = "Field `IDXIE` writer - Index interrupt enable"]
pub type IDXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRIE` reader - Direction Change interrupt enable"]
pub type DIRIE_R = crate::BitReader;
#[doc = "Field `DIRIE` writer - Direction Change interrupt enable"]
pub type DIRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IERRIE` reader - Index Error interrupt enable"]
pub type IERRIE_R = crate::BitReader;
#[doc = "Field `IERRIE` writer - Index Error interrupt enable"]
pub type IERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERRIE` reader - Transition Error interrupt enable"]
pub type TERRIE_R = crate::BitReader;
#[doc = "Field `TERRIE` writer - Transition Error interrupt enable"]
pub type TERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Capture/Compare (1-4) interrupt enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1IE` field"]
    #[inline(always)]
    pub fn ccie(&self, n: u8) -> CCIE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCIE_R::new(((self.bits >> (n + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (1-4) interrupt enable"]
    #[inline(always)]
    pub fn ccie_iter(&self) -> impl Iterator<Item = CCIE_R> + '_ {
        (0..4).map(move |n| CCIE_R::new(((self.bits >> (n + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    pub fn cc2ie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt enable"]
    #[inline(always)]
    pub fn cc3ie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt enable"]
    #[inline(always)]
    pub fn cc4ie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn comie(&self) -> COMIE_R {
        COMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
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
    #[doc = "Capture/Compare (1-4) DMA request enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1DE` field"]
    #[inline(always)]
    pub fn ccde(&self, n: u8) -> CCDE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCDE_R::new(((self.bits >> (n + 9)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (1-4) DMA request enable"]
    #[inline(always)]
    pub fn ccde_iter(&self) -> impl Iterator<Item = CCDE_R> + '_ {
        (0..4).map(move |n| CCDE_R::new(((self.bits >> (n + 9)) & 1) != 0))
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub fn cc1de(&self) -> CCDE_R {
        CCDE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/Compare 2 DMA request enable"]
    #[inline(always)]
    pub fn cc2de(&self) -> CCDE_R {
        CCDE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 DMA request enable"]
    #[inline(always)]
    pub fn cc3de(&self) -> CCDE_R {
        CCDE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 DMA request enable"]
    #[inline(always)]
    pub fn cc4de(&self) -> CCDE_R {
        CCDE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline(always)]
    pub fn comde(&self) -> COMDE_R {
        COMDE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 20 - Index interrupt enable"]
    #[inline(always)]
    pub fn idxie(&self) -> IDXIE_R {
        IDXIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Direction Change interrupt enable"]
    #[inline(always)]
    pub fn dirie(&self) -> DIRIE_R {
        DIRIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Index Error interrupt enable"]
    #[inline(always)]
    pub fn ierrie(&self) -> IERRIE_R {
        IERRIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transition Error interrupt enable"]
    #[inline(always)]
    pub fn terrie(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UIE_W<DIERrs> {
        UIE_W::new(self, 0)
    }
    #[doc = "Capture/Compare (1-4) interrupt enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1IE` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccie(&mut self, n: u8) -> CCIE_W<DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCIE_W::new(self, n + 1)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ie(&mut self) -> CCIE_W<DIERrs> {
        CCIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2ie(&mut self) -> CCIE_W<DIERrs> {
        CCIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc3ie(&mut self) -> CCIE_W<DIERrs> {
        CCIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc4ie(&mut self) -> CCIE_W<DIERrs> {
        CCIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn comie(&mut self) -> COMIE_W<DIERrs> {
        COMIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<DIERrs> {
        TIE_W::new(self, 6)
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
    #[doc = "Capture/Compare (1-4) DMA request enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1DE` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccde(&mut self, n: u8) -> CCDE_W<DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCDE_W::new(self, n + 9)
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1de(&mut self) -> CCDE_W<DIERrs> {
        CCDE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/Compare 2 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2de(&mut self) -> CCDE_W<DIERrs> {
        CCDE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/Compare 3 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc3de(&mut self) -> CCDE_W<DIERrs> {
        CCDE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/Compare 4 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc4de(&mut self) -> CCDE_W<DIERrs> {
        CCDE_W::new(self, 12)
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn comde(&mut self) -> COMDE_W<DIERrs> {
        COMDE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn tde(&mut self) -> TDE_W<DIERrs> {
        TDE_W::new(self, 14)
    }
    #[doc = "Bit 20 - Index interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn idxie(&mut self) -> IDXIE_W<DIERrs> {
        IDXIE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Direction Change interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirie(&mut self) -> DIRIE_W<DIERrs> {
        DIRIE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Index Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ierrie(&mut self) -> IERRIE_W<DIERrs> {
        IERRIE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Transition Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn terrie(&mut self) -> TERRIE_W<DIERrs> {
        TERRIE_W::new(self, 23)
    }
}
#[doc = "DMA/Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
