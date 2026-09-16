///Register `RPCFGR` reader
pub type R = crate::R<RPCFGRrs>;
///Register `RPCFGR` writer
pub type W = crate::W<RPCFGRrs>;
///Field `RPCFG0` reader - Configurable resource 0 protection
pub type RPCFG0_R = crate::BitReader;
///Field `RPCFG0` writer - Configurable resource 0 protection
pub type RPCFG0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Configurable resource 0 protection
    #[inline(always)]
    pub fn rpcfg0(&self) -> RPCFG0_R {
        RPCFG0_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPCFGR")
            .field("rpcfg0", &self.rpcfg0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Configurable resource 0 protection
    #[inline(always)]
    pub fn rpcfg0(&mut self) -> RPCFG0_W<'_, RPCFGRrs> {
        RPCFG0_W::new(self, 0)
    }
}
/**TAMP resources protection configuration register

You can [`read`](crate::Reg::read) this register and get [`rpcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#TAMP:RPCFGR)*/
pub struct RPCFGRrs;
impl crate::RegisterSpec for RPCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`rpcfgr::R`](R) reader structure
impl crate::Readable for RPCFGRrs {}
///`write(|w| ..)` method takes [`rpcfgr::W`](W) writer structure
impl crate::Writable for RPCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RPCFGR to value 0
impl crate::Resettable for RPCFGRrs {}
