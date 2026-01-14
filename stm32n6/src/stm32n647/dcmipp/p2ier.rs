///Register `P2IER` reader
pub type R = crate::R<P2IERrs>;
///Register `P2IER` writer
pub type W = crate::W<P2IERrs>;
///Field `LINEIE` reader - Multi-line capture completed interrupt enable
pub type LINEIE_R = crate::BitReader;
///Field `LINEIE` writer - Multi-line capture completed interrupt enable
pub type LINEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRAMEIE` reader - Frame capture completed interrupt enable
pub type FRAMEIE_R = crate::BitReader;
///Field `FRAMEIE` writer - Frame capture completed interrupt enable
pub type FRAMEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSYNCIE` reader - VSYNC interrupt enable
pub type VSYNCIE_R = crate::BitReader;
///Field `VSYNCIE` writer - VSYNC interrupt enable
pub type VSYNCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRIE` reader - Overrun interrupt enable
pub type OVRIE_R = crate::BitReader;
///Field `OVRIE` writer - Overrun interrupt enable
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Multi-line capture completed interrupt enable
    #[inline(always)]
    pub fn lineie(&self) -> LINEIE_R {
        LINEIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Frame capture completed interrupt enable
    #[inline(always)]
    pub fn frameie(&self) -> FRAMEIE_R {
        FRAMEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VSYNC interrupt enable
    #[inline(always)]
    pub fn vsyncie(&self) -> VSYNCIE_R {
        VSYNCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 7 - Overrun interrupt enable
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2IER")
            .field("lineie", &self.lineie())
            .field("frameie", &self.frameie())
            .field("vsyncie", &self.vsyncie())
            .field("ovrie", &self.ovrie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Multi-line capture completed interrupt enable
    #[inline(always)]
    pub fn lineie(&mut self) -> LINEIE_W<'_, P2IERrs> {
        LINEIE_W::new(self, 0)
    }
    ///Bit 1 - Frame capture completed interrupt enable
    #[inline(always)]
    pub fn frameie(&mut self) -> FRAMEIE_W<'_, P2IERrs> {
        FRAMEIE_W::new(self, 1)
    }
    ///Bit 2 - VSYNC interrupt enable
    #[inline(always)]
    pub fn vsyncie(&mut self) -> VSYNCIE_W<'_, P2IERrs> {
        VSYNCIE_W::new(self, 2)
    }
    ///Bit 7 - Overrun interrupt enable
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W<'_, P2IERrs> {
        OVRIE_W::new(self, 7)
    }
}
/**DCMIPP Pipe2 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`p2ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:P2IER)*/
pub struct P2IERrs;
impl crate::RegisterSpec for P2IERrs {
    type Ux = u32;
}
///`read()` method returns [`p2ier::R`](R) reader structure
impl crate::Readable for P2IERrs {}
///`write(|w| ..)` method takes [`p2ier::W`](W) writer structure
impl crate::Writable for P2IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P2IER to value 0
impl crate::Resettable for P2IERrs {}
