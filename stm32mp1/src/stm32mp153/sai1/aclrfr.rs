///Register `ACLRFR` writer
pub type W = crate::W<ACLRFRrs>;
///Field `COVRUDR` writer - COVRUDR
pub type COVRUDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMUTEDET` writer - CMUTEDET
pub type CMUTEDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWCKCFG` writer - CWCKCFG
pub type CWCKCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCNRDY` writer - CCNRDY
pub type CCNRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAFSDET` writer - CAFSDET
pub type CAFSDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLFSDET` writer - CLFSDET
pub type CLFSDET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ACLRFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - COVRUDR
    #[inline(always)]
    pub fn covrudr(&mut self) -> COVRUDR_W<'_, ACLRFRrs> {
        COVRUDR_W::new(self, 0)
    }
    ///Bit 1 - CMUTEDET
    #[inline(always)]
    pub fn cmutedet(&mut self) -> CMUTEDET_W<'_, ACLRFRrs> {
        CMUTEDET_W::new(self, 1)
    }
    ///Bit 2 - CWCKCFG
    #[inline(always)]
    pub fn cwckcfg(&mut self) -> CWCKCFG_W<'_, ACLRFRrs> {
        CWCKCFG_W::new(self, 2)
    }
    ///Bit 4 - CCNRDY
    #[inline(always)]
    pub fn ccnrdy(&mut self) -> CCNRDY_W<'_, ACLRFRrs> {
        CCNRDY_W::new(self, 4)
    }
    ///Bit 5 - CAFSDET
    #[inline(always)]
    pub fn cafsdet(&mut self) -> CAFSDET_W<'_, ACLRFRrs> {
        CAFSDET_W::new(self, 5)
    }
    ///Bit 6 - CLFSDET
    #[inline(always)]
    pub fn clfsdet(&mut self) -> CLFSDET_W<'_, ACLRFRrs> {
        CLFSDET_W::new(self, 6)
    }
}
/**Clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aclrfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SAI1:ACLRFR)*/
pub struct ACLRFRrs;
impl crate::RegisterSpec for ACLRFRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`aclrfr::W`](W) writer structure
impl crate::Writable for ACLRFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACLRFR to value 0
impl crate::Resettable for ACLRFRrs {}
