///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `TMONEN` reader - TMONEN
pub type TMONEN_R = crate::BitReader;
///Field `TMONEN` writer - TMONEN
pub type TMONEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VMONEN` reader - VMONEN
pub type VMONEN_R = crate::BitReader;
///Field `VMONEN` writer - VMONEN
pub type VMONEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTMONEN` reader - WUTMONEN
pub type WUTMONEN_R = crate::BitReader;
///Field `WUTMONEN` writer - WUTMONEN
pub type WUTMONEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - TMONEN
    #[inline(always)]
    pub fn tmonen(&self) -> TMONEN_R {
        TMONEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VMONEN
    #[inline(always)]
    pub fn vmonen(&self) -> VMONEN_R {
        VMONEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - WUTMONEN
    #[inline(always)]
    pub fn wutmonen(&self) -> WUTMONEN_R {
        WUTMONEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("tmonen", &self.tmonen())
            .field("vmonen", &self.vmonen())
            .field("wutmonen", &self.wutmonen())
            .finish()
    }
}
impl W {
    ///Bit 1 - TMONEN
    #[inline(always)]
    pub fn tmonen(&mut self) -> TMONEN_W<'_, CFGRrs> {
        TMONEN_W::new(self, 1)
    }
    ///Bit 2 - VMONEN
    #[inline(always)]
    pub fn vmonen(&mut self) -> VMONEN_W<'_, CFGRrs> {
        VMONEN_W::new(self, 2)
    }
    ///Bit 3 - WUTMONEN
    #[inline(always)]
    pub fn wutmonen(&mut self) -> WUTMONEN_W<'_, CFGRrs> {
        WUTMONEN_W::new(self, 3)
    }
}
/**TAMP configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#TAMP:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}
