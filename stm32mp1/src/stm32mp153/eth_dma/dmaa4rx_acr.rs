///Register `DMAA4RxACR` reader
pub type R = crate::R<DMAA4RX_ACRrs>;
///Register `DMAA4RxACR` writer
pub type W = crate::W<DMAA4RX_ACRrs>;
///Field `RDWC` reader - RDWC
pub type RDWC_R = crate::FieldReader;
///Field `RDWC` writer - RDWC
pub type RDWC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RPC` reader - RPC
pub type RPC_R = crate::FieldReader;
///Field `RPC` writer - RPC
pub type RPC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RHC` reader - RHC
pub type RHC_R = crate::FieldReader;
///Field `RHC` writer - RHC
pub type RHC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RDC` reader - RDC
pub type RDC_R = crate::FieldReader;
///Field `RDC` writer - RDC
pub type RDC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - RDWC
    #[inline(always)]
    pub fn rdwc(&self) -> RDWC_R {
        RDWC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - RPC
    #[inline(always)]
    pub fn rpc(&self) -> RPC_R {
        RPC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:19 - RHC
    #[inline(always)]
    pub fn rhc(&self) -> RHC_R {
        RHC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:25 - RDC
    #[inline(always)]
    pub fn rdc(&self) -> RDC_R {
        RDC_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAA4RxACR")
            .field("rdwc", &self.rdwc())
            .field("rpc", &self.rpc())
            .field("rhc", &self.rhc())
            .field("rdc", &self.rdc())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - RDWC
    #[inline(always)]
    pub fn rdwc(&mut self) -> RDWC_W<'_, DMAA4RX_ACRrs> {
        RDWC_W::new(self, 0)
    }
    ///Bits 8:11 - RPC
    #[inline(always)]
    pub fn rpc(&mut self) -> RPC_W<'_, DMAA4RX_ACRrs> {
        RPC_W::new(self, 8)
    }
    ///Bits 16:19 - RHC
    #[inline(always)]
    pub fn rhc(&mut self) -> RHC_W<'_, DMAA4RX_ACRrs> {
        RHC_W::new(self, 16)
    }
    ///Bits 24:25 - RDC
    #[inline(always)]
    pub fn rdc(&mut self) -> RDC_W<'_, DMAA4RX_ACRrs> {
        RDC_W::new(self, 24)
    }
}
/**AXI4 receive channel ACE control register

You can [`read`](crate::Reg::read) this register and get [`dmaa4rx_acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaa4rx_acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAA4RxACR)*/
pub struct DMAA4RX_ACRrs;
impl crate::RegisterSpec for DMAA4RX_ACRrs {
    type Ux = u32;
}
///`read()` method returns [`dmaa4rx_acr::R`](R) reader structure
impl crate::Readable for DMAA4RX_ACRrs {}
///`write(|w| ..)` method takes [`dmaa4rx_acr::W`](W) writer structure
impl crate::Writable for DMAA4RX_ACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAA4RxACR to value 0
impl crate::Resettable for DMAA4RX_ACRrs {}
