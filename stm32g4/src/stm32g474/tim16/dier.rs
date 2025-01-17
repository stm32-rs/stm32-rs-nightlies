///Register `DIER` reader
pub type R = crate::R<DIERrs>;
///Register `DIER` writer
pub type W = crate::W<DIERrs>;
///Field `UIE` reader - Update interrupt enable
pub type UIE_R = crate::BitReader;
///Field `UIE` writer - Update interrupt enable
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCIE(1-1)` reader - Capture/Compare %s interrupt enable
pub type CCIE_R = crate::BitReader;
///Field `CCIE(1-1)` writer - Capture/Compare %s interrupt enable
pub type CCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMIE` reader - COM interrupt enable
pub type COMIE_R = crate::BitReader;
///Field `COMIE` writer - COM interrupt enable
pub type COMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIE` reader - Break interrupt enable
pub type BIE_R = crate::BitReader;
///Field `BIE` writer - Break interrupt enable
pub type BIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDE` reader - Update DMA request enable
pub type UDE_R = crate::BitReader;
///Field `UDE` writer - Update DMA request enable
pub type UDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCDE(1-1)` reader - Capture/Compare %s DMA request enable
pub type CCDE_R = crate::BitReader;
///Field `CCDE(1-1)` writer - Capture/Compare %s DMA request enable
pub type CCDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMDE` reader - COM DMA request enable
pub type COMDE_R = crate::BitReader;
///Field `COMDE` writer - COM DMA request enable
pub type COMDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    ///Capture/Compare (1-1) interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1IE` field.</div>
    #[inline(always)]
    pub fn ccie(&self, n: u8) -> CCIE_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCIE_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture/Compare (1-1) interrupt enable
    #[inline(always)]
    pub fn ccie_iter(&self) -> impl Iterator<Item = CCIE_R> + '_ {
        (0..1).map(move |n| CCIE_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0))
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - COM interrupt enable
    #[inline(always)]
    pub fn comie(&self) -> COMIE_R {
        COMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Break interrupt enable
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Capture/Compare (1-1) DMA request enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1DE` field.</div>
    #[inline(always)]
    pub fn ccde(&self, n: u8) -> CCDE_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCDE_R::new(((self.bits >> (n * 0 + 9)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture/Compare (1-1) DMA request enable
    #[inline(always)]
    pub fn ccde_iter(&self) -> impl Iterator<Item = CCDE_R> + '_ {
        (0..1).map(move |n| CCDE_R::new(((self.bits >> (n * 0 + 9)) & 1) != 0))
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&self) -> CCDE_R {
        CCDE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - COM DMA request enable
    #[inline(always)]
    pub fn comde(&self) -> COMDE_R {
        COMDE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER")
            .field("comde", &self.comde())
            .field("cc1de", &self.cc1de())
            .field("ude", &self.ude())
            .field("bie", &self.bie())
            .field("comie", &self.comie())
            .field("cc1ie", &self.cc1ie())
            .field("uie", &self.uie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<DIERrs> {
        UIE_W::new(self, 0)
    }
    ///Capture/Compare (1-1) interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1IE` field.</div>
    #[inline(always)]
    pub fn ccie(&mut self, n: u8) -> CCIE_W<DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCIE_W::new(self, n * 0 + 1)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CCIE_W<DIERrs> {
        CCIE_W::new(self, 1)
    }
    ///Bit 5 - COM interrupt enable
    #[inline(always)]
    pub fn comie(&mut self) -> COMIE_W<DIERrs> {
        COMIE_W::new(self, 5)
    }
    ///Bit 7 - Break interrupt enable
    #[inline(always)]
    pub fn bie(&mut self) -> BIE_W<DIERrs> {
        BIE_W::new(self, 7)
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&mut self) -> UDE_W<DIERrs> {
        UDE_W::new(self, 8)
    }
    ///Capture/Compare (1-1) DMA request enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1DE` field.</div>
    #[inline(always)]
    pub fn ccde(&mut self, n: u8) -> CCDE_W<DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCDE_W::new(self, n * 0 + 9)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&mut self) -> CCDE_W<DIERrs> {
        CCDE_W::new(self, 9)
    }
    ///Bit 13 - COM DMA request enable
    #[inline(always)]
    pub fn comde(&mut self) -> COMDE_W<DIERrs> {
        COMDE_W::new(self, 13)
    }
}
/**DMA/Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#TIM16:DIER)*/
pub struct DIERrs;
impl crate::RegisterSpec for DIERrs {
    type Ux = u32;
}
///`read()` method returns [`dier::R`](R) reader structure
impl crate::Readable for DIERrs {}
///`write(|w| ..)` method takes [`dier::W`](W) writer structure
impl crate::Writable for DIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DIER to value 0
impl crate::Resettable for DIERrs {
    const RESET_VALUE: u32 = 0;
}
