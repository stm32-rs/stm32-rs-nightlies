///Register `CMIER` reader
pub type R = crate::R<CMIERrs>;
///Register `CMIER` writer
pub type W = crate::W<CMIERrs>;
///Field `ATXERRIE` reader - AXI transfer error interrupt enable for IP-Plug
pub type ATXERRIE_R = crate::BitReader;
///Field `ATXERRIE` writer - AXI transfer error interrupt enable for IP-Plug
pub type ATXERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRERRIE` reader - Limit interrupt enable for the parallel Interface
pub type PRERRIE_R = crate::BitReader;
///Field `PRERRIE` writer - Limit interrupt enable for the parallel Interface
pub type PRERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P0LINEIE` reader - Multi-line capture complete interrupt enable for Pipe0
pub type P0LINEIE_R = crate::BitReader;
///Field `P0LINEIE` writer - Multi-line capture complete interrupt enable for Pipe0
pub type P0LINEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P0FRAMEIE` reader - Frame capture complete interrupt enable for Pipe0
pub type P0FRAMEIE_R = crate::BitReader;
///Field `P0FRAMEIE` writer - Frame capture complete interrupt enable for Pipe0
pub type P0FRAMEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P0VSYNCIE` reader - Vertical sync interrupt enable for Pipe0
pub type P0VSYNCIE_R = crate::BitReader;
///Field `P0VSYNCIE` writer - Vertical sync interrupt enable for Pipe0
pub type P0VSYNCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P0LIMITIE` reader - Limit interrupt enable for Pipe0
pub type P0LIMITIE_R = crate::BitReader;
///Field `P0LIMITIE` writer - Limit interrupt enable for Pipe0
pub type P0LIMITIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `P0OVRIE` reader - Overrun interrupt enable for Pipe0
pub type P0OVRIE_R = crate::BitReader;
///Field `P0OVRIE` writer - Overrun interrupt enable for Pipe0
pub type P0OVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 5 - AXI transfer error interrupt enable for IP-Plug
    #[inline(always)]
    pub fn atxerrie(&self) -> ATXERRIE_R {
        ATXERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Limit interrupt enable for the parallel Interface
    #[inline(always)]
    pub fn prerrie(&self) -> PRERRIE_R {
        PRERRIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Multi-line capture complete interrupt enable for Pipe0
    #[inline(always)]
    pub fn p0lineie(&self) -> P0LINEIE_R {
        P0LINEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Frame capture complete interrupt enable for Pipe0
    #[inline(always)]
    pub fn p0frameie(&self) -> P0FRAMEIE_R {
        P0FRAMEIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Vertical sync interrupt enable for Pipe0
    #[inline(always)]
    pub fn p0vsyncie(&self) -> P0VSYNCIE_R {
        P0VSYNCIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - Limit interrupt enable for Pipe0
    #[inline(always)]
    pub fn p0limitie(&self) -> P0LIMITIE_R {
        P0LIMITIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Overrun interrupt enable for Pipe0
    #[inline(always)]
    pub fn p0ovrie(&self) -> P0OVRIE_R {
        P0OVRIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMIER")
            .field("atxerrie", &self.atxerrie())
            .field("prerrie", &self.prerrie())
            .field("p0lineie", &self.p0lineie())
            .field("p0frameie", &self.p0frameie())
            .field("p0vsyncie", &self.p0vsyncie())
            .field("p0limitie", &self.p0limitie())
            .field("p0ovrie", &self.p0ovrie())
            .finish()
    }
}
impl W {
    ///Bit 5 - AXI transfer error interrupt enable for IP-Plug
    #[inline(always)]
    pub fn atxerrie(&mut self) -> ATXERRIE_W<'_, CMIERrs> {
        ATXERRIE_W::new(self, 5)
    }
    ///Bit 6 - Limit interrupt enable for the parallel Interface
    #[inline(always)]
    pub fn prerrie(&mut self) -> PRERRIE_W<'_, CMIERrs> {
        PRERRIE_W::new(self, 6)
    }
    ///Bit 8 - Multi-line capture complete interrupt enable for Pipe0
    #[inline(always)]
    pub fn p0lineie(&mut self) -> P0LINEIE_W<'_, CMIERrs> {
        P0LINEIE_W::new(self, 8)
    }
    ///Bit 9 - Frame capture complete interrupt enable for Pipe0
    #[inline(always)]
    pub fn p0frameie(&mut self) -> P0FRAMEIE_W<'_, CMIERrs> {
        P0FRAMEIE_W::new(self, 9)
    }
    ///Bit 10 - Vertical sync interrupt enable for Pipe0
    #[inline(always)]
    pub fn p0vsyncie(&mut self) -> P0VSYNCIE_W<'_, CMIERrs> {
        P0VSYNCIE_W::new(self, 10)
    }
    ///Bit 14 - Limit interrupt enable for Pipe0
    #[inline(always)]
    pub fn p0limitie(&mut self) -> P0LIMITIE_W<'_, CMIERrs> {
        P0LIMITIE_W::new(self, 14)
    }
    ///Bit 15 - Overrun interrupt enable for Pipe0
    #[inline(always)]
    pub fn p0ovrie(&mut self) -> P0OVRIE_W<'_, CMIERrs> {
        P0OVRIE_W::new(self, 15)
    }
}
/**DCMIPP common interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cmier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:CMIER)*/
pub struct CMIERrs;
impl crate::RegisterSpec for CMIERrs {
    type Ux = u32;
}
///`read()` method returns [`cmier::R`](R) reader structure
impl crate::Readable for CMIERrs {}
///`write(|w| ..)` method takes [`cmier::W`](W) writer structure
impl crate::Writable for CMIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMIER to value 0
impl crate::Resettable for CMIERrs {}
