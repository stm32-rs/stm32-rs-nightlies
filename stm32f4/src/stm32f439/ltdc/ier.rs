///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `LIE` reader - Line Interrupt Enable
pub type LIE_R = crate::BitReader;
///Field `LIE` writer - Line Interrupt Enable
pub type LIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FUIE` reader - FIFO Underrun Interrupt Enable
pub type FUIE_R = crate::BitReader;
///Field `FUIE` writer - FIFO Underrun Interrupt Enable
pub type FUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TERRIE` reader - Transfer Error Interrupt Enable
pub type TERRIE_R = crate::BitReader;
///Field `TERRIE` writer - Transfer Error Interrupt Enable
pub type TERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RRIE` reader - Register Reload interrupt enable
pub type RRIE_R = crate::BitReader;
///Field `RRIE` writer - Register Reload interrupt enable
pub type RRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Line Interrupt Enable
    #[inline(always)]
    pub fn lie(&self) -> LIE_R {
        LIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FIFO Underrun Interrupt Enable
    #[inline(always)]
    pub fn fuie(&self) -> FUIE_R {
        FUIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transfer Error Interrupt Enable
    #[inline(always)]
    pub fn terrie(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Register Reload interrupt enable
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("rrie", &self.rrie())
            .field("terrie", &self.terrie())
            .field("fuie", &self.fuie())
            .field("lie", &self.lie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Line Interrupt Enable
    #[inline(always)]
    pub fn lie(&mut self) -> LIE_W<'_, IERrs> {
        LIE_W::new(self, 0)
    }
    ///Bit 1 - FIFO Underrun Interrupt Enable
    #[inline(always)]
    pub fn fuie(&mut self) -> FUIE_W<'_, IERrs> {
        FUIE_W::new(self, 1)
    }
    ///Bit 2 - Transfer Error Interrupt Enable
    #[inline(always)]
    pub fn terrie(&mut self) -> TERRIE_W<'_, IERrs> {
        TERRIE_W::new(self, 2)
    }
    ///Bit 3 - Register Reload interrupt enable
    #[inline(always)]
    pub fn rrie(&mut self) -> RRIE_W<'_, IERrs> {
        RRIE_W::new(self, 3)
    }
}
/**Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#LTDC:IER)*/
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
