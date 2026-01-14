///Register `MTLRXQDMAMR` reader
pub type R = crate::R<MTLRXQDMAMRrs>;
///Register `MTLRXQDMAMR` writer
pub type W = crate::W<MTLRXQDMAMRrs>;
///Field `Q0MDMACH` reader - Queue 0 Mapped to DMA Channel
pub type Q0MDMACH_R = crate::BitReader;
///Field `Q0MDMACH` writer - Queue 0 Mapped to DMA Channel
pub type Q0MDMACH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `Q0DDMACH` reader - Queue 0 Enabled for DA-based DMA Channel Selection
pub type Q0DDMACH_R = crate::BitReader;
///Field `Q0DDMACH` writer - Queue 0 Enabled for DA-based DMA Channel Selection
pub type Q0DDMACH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `Q1MDMACH` reader - Queue 1 Mapped to DMA Channel
pub type Q1MDMACH_R = crate::BitReader;
///Field `Q1MDMACH` writer - Queue 1 Mapped to DMA Channel
pub type Q1MDMACH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `Q1DDMACH` reader - Queue 1 Enabled for DA-based DMA Channel Selection
pub type Q1DDMACH_R = crate::BitReader;
///Field `Q1DDMACH` writer - Queue 1 Enabled for DA-based DMA Channel Selection
pub type Q1DDMACH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Queue 0 Mapped to DMA Channel
    #[inline(always)]
    pub fn q0mdmach(&self) -> Q0MDMACH_R {
        Q0MDMACH_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Queue 0 Enabled for DA-based DMA Channel Selection
    #[inline(always)]
    pub fn q0ddmach(&self) -> Q0DDMACH_R {
        Q0DDMACH_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Queue 1 Mapped to DMA Channel
    #[inline(always)]
    pub fn q1mdmach(&self) -> Q1MDMACH_R {
        Q1MDMACH_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Queue 1 Enabled for DA-based DMA Channel Selection
    #[inline(always)]
    pub fn q1ddmach(&self) -> Q1DDMACH_R {
        Q1DDMACH_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLRXQDMAMR")
            .field("q0mdmach", &self.q0mdmach())
            .field("q0ddmach", &self.q0ddmach())
            .field("q1mdmach", &self.q1mdmach())
            .field("q1ddmach", &self.q1ddmach())
            .finish()
    }
}
impl W {
    ///Bit 0 - Queue 0 Mapped to DMA Channel
    #[inline(always)]
    pub fn q0mdmach(&mut self) -> Q0MDMACH_W<'_, MTLRXQDMAMRrs> {
        Q0MDMACH_W::new(self, 0)
    }
    ///Bit 4 - Queue 0 Enabled for DA-based DMA Channel Selection
    #[inline(always)]
    pub fn q0ddmach(&mut self) -> Q0DDMACH_W<'_, MTLRXQDMAMRrs> {
        Q0DDMACH_W::new(self, 4)
    }
    ///Bit 8 - Queue 1 Mapped to DMA Channel
    #[inline(always)]
    pub fn q1mdmach(&mut self) -> Q1MDMACH_W<'_, MTLRXQDMAMRrs> {
        Q1MDMACH_W::new(self, 8)
    }
    ///Bit 12 - Queue 1 Enabled for DA-based DMA Channel Selection
    #[inline(always)]
    pub fn q1ddmach(&mut self) -> Q1DDMACH_W<'_, MTLRXQDMAMRrs> {
        Q1DDMACH_W::new(self, 12)
    }
}
/**Rx Queue and DMA Channel Mapping Register

You can [`read`](crate::Reg::read) this register and get [`mtlrxqdmamr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrxqdmamr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MTLRXQDMAMR)*/
pub struct MTLRXQDMAMRrs;
impl crate::RegisterSpec for MTLRXQDMAMRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlrxqdmamr::R`](R) reader structure
impl crate::Readable for MTLRXQDMAMRrs {}
///`write(|w| ..)` method takes [`mtlrxqdmamr::W`](W) writer structure
impl crate::Writable for MTLRXQDMAMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLRXQDMAMR to value 0
impl crate::Resettable for MTLRXQDMAMRrs {}
