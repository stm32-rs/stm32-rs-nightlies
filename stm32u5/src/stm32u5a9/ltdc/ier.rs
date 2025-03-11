///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `LIE` reader - line interrupt enable This bit is set and cleared by software.
pub type LIE_R = crate::BitReader;
///Field `LIE` writer - line interrupt enable This bit is set and cleared by software.
pub type LIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FUIE` reader - FIFO underrun interrupt enable This bit is set and cleared by software.
pub type FUIE_R = crate::BitReader;
///Field `FUIE` writer - FIFO underrun interrupt enable This bit is set and cleared by software.
pub type FUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TERRIE` reader - transfer error interrupt enable This bit is set and cleared by software.
pub type TERRIE_R = crate::BitReader;
///Field `TERRIE` writer - transfer error interrupt enable This bit is set and cleared by software.
pub type TERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RRIE` reader - register reload interrupt enable This bit is set and cleared by software.
pub type RRIE_R = crate::BitReader;
///Field `RRIE` writer - register reload interrupt enable This bit is set and cleared by software.
pub type RRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - line interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lie(&self) -> LIE_R {
        LIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FIFO underrun interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn fuie(&self) -> FUIE_R {
        FUIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - transfer error interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn terrie(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - register reload interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("lie", &self.lie())
            .field("fuie", &self.fuie())
            .field("terrie", &self.terrie())
            .field("rrie", &self.rrie())
            .finish()
    }
}
impl W {
    ///Bit 0 - line interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lie(&mut self) -> LIE_W<IERrs> {
        LIE_W::new(self, 0)
    }
    ///Bit 1 - FIFO underrun interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn fuie(&mut self) -> FUIE_W<IERrs> {
        FUIE_W::new(self, 1)
    }
    ///Bit 2 - transfer error interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn terrie(&mut self) -> TERRIE_W<IERrs> {
        TERRIE_W::new(self, 2)
    }
    ///Bit 3 - register reload interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rrie(&mut self) -> RRIE_W<IERrs> {
        RRIE_W::new(self, 3)
    }
}
/**LTDC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
