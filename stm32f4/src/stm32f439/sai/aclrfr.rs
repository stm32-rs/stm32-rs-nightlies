///Register `ACLRFR` reader
pub type R = crate::R<ACLRFRrs>;
///Register `ACLRFR` writer
pub type W = crate::W<ACLRFRrs>;
///Field `OVRUDR` reader - Clear overrun / underrun
pub type OVRUDR_R = crate::BitReader;
///Field `OVRUDR` writer - Clear overrun / underrun
pub type OVRUDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUTEDET` reader - Mute detection flag
pub type MUTEDET_R = crate::BitReader;
///Field `MUTEDET` writer - Mute detection flag
pub type MUTEDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WCKCFG` reader - Clear wrong clock configuration flag
pub type WCKCFG_R = crate::BitReader;
///Field `WCKCFG` writer - Clear wrong clock configuration flag
pub type WCKCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNRDY` reader - Clear codec not ready flag
pub type CNRDY_R = crate::BitReader;
///Field `CNRDY` writer - Clear codec not ready flag
pub type CNRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAFSDET` reader - Clear anticipated frame synchronization detection flag.
pub type CAFSDET_R = crate::BitReader;
///Field `CAFSDET` writer - Clear anticipated frame synchronization detection flag.
pub type CAFSDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LFSDET` reader - Clear late frame synchronization detection flag
pub type LFSDET_R = crate::BitReader;
///Field `LFSDET` writer - Clear late frame synchronization detection flag
pub type LFSDET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clear overrun / underrun
    #[inline(always)]
    pub fn ovrudr(&self) -> OVRUDR_R {
        OVRUDR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Mute detection flag
    #[inline(always)]
    pub fn mutedet(&self) -> MUTEDET_R {
        MUTEDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clear wrong clock configuration flag
    #[inline(always)]
    pub fn wckcfg(&self) -> WCKCFG_R {
        WCKCFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Clear codec not ready flag
    #[inline(always)]
    pub fn cnrdy(&self) -> CNRDY_R {
        CNRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Clear anticipated frame synchronization detection flag.
    #[inline(always)]
    pub fn cafsdet(&self) -> CAFSDET_R {
        CAFSDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Clear late frame synchronization detection flag
    #[inline(always)]
    pub fn lfsdet(&self) -> LFSDET_R {
        LFSDET_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACLRFR")
            .field("lfsdet", &self.lfsdet())
            .field("cafsdet", &self.cafsdet())
            .field("cnrdy", &self.cnrdy())
            .field("wckcfg", &self.wckcfg())
            .field("mutedet", &self.mutedet())
            .field("ovrudr", &self.ovrudr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clear overrun / underrun
    #[inline(always)]
    pub fn ovrudr(&mut self) -> OVRUDR_W<'_, ACLRFRrs> {
        OVRUDR_W::new(self, 0)
    }
    ///Bit 1 - Mute detection flag
    #[inline(always)]
    pub fn mutedet(&mut self) -> MUTEDET_W<'_, ACLRFRrs> {
        MUTEDET_W::new(self, 1)
    }
    ///Bit 2 - Clear wrong clock configuration flag
    #[inline(always)]
    pub fn wckcfg(&mut self) -> WCKCFG_W<'_, ACLRFRrs> {
        WCKCFG_W::new(self, 2)
    }
    ///Bit 4 - Clear codec not ready flag
    #[inline(always)]
    pub fn cnrdy(&mut self) -> CNRDY_W<'_, ACLRFRrs> {
        CNRDY_W::new(self, 4)
    }
    ///Bit 5 - Clear anticipated frame synchronization detection flag.
    #[inline(always)]
    pub fn cafsdet(&mut self) -> CAFSDET_W<'_, ACLRFRrs> {
        CAFSDET_W::new(self, 5)
    }
    ///Bit 6 - Clear late frame synchronization detection flag
    #[inline(always)]
    pub fn lfsdet(&mut self) -> LFSDET_W<'_, ACLRFRrs> {
        LFSDET_W::new(self, 6)
    }
}
/**AClear flag register

You can [`read`](crate::Reg::read) this register and get [`aclrfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aclrfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SAI:ACLRFR)*/
pub struct ACLRFRrs;
impl crate::RegisterSpec for ACLRFRrs {
    type Ux = u32;
}
///`read()` method returns [`aclrfr::R`](R) reader structure
impl crate::Readable for ACLRFRrs {}
///`write(|w| ..)` method takes [`aclrfr::W`](W) writer structure
impl crate::Writable for ACLRFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACLRFR to value 0
impl crate::Resettable for ACLRFRrs {}
