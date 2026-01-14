///Register `MAC_TMRQR` reader
pub type R = crate::R<MAC_TMRQRrs>;
///Register `MAC_TMRQR` writer
pub type W = crate::W<MAC_TMRQRrs>;
///Field `TYP` reader - Type field Value
pub type TYP_R = crate::FieldReader<u16>;
///Field `TYP` writer - Type field Value
pub type TYP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `TMRQ` reader - Type Match Rx Queue Number
pub type TMRQ_R = crate::FieldReader;
///Field `TMRQ` writer - Type Match Rx Queue Number
pub type TMRQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PFEX` reader - Preemption or Express Packet
pub type PFEX_R = crate::BitReader;
///Field `PFEX` writer - Preemption or Express Packet
pub type PFEX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - Type field Value
    #[inline(always)]
    pub fn typ(&self) -> TYP_R {
        TYP_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:18 - Type Match Rx Queue Number
    #[inline(always)]
    pub fn tmrq(&self) -> TMRQ_R {
        TMRQ_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 20 - Preemption or Express Packet
    #[inline(always)]
    pub fn pfex(&self) -> PFEX_R {
        PFEX_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAC_TMRQR")
            .field("typ", &self.typ())
            .field("tmrq", &self.tmrq())
            .field("pfex", &self.pfex())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Type field Value
    #[inline(always)]
    pub fn typ(&mut self) -> TYP_W<'_, MAC_TMRQRrs> {
        TYP_W::new(self, 0)
    }
    ///Bits 16:18 - Type Match Rx Queue Number
    #[inline(always)]
    pub fn tmrq(&mut self) -> TMRQ_W<'_, MAC_TMRQRrs> {
        TMRQ_W::new(self, 16)
    }
    ///Bit 20 - Preemption or Express Packet
    #[inline(always)]
    pub fn pfex(&mut self) -> PFEX_W<'_, MAC_TMRQRrs> {
        PFEX_W::new(self, 20)
    }
}
/**MAC type-based Rx Queue mapping register

You can [`read`](crate::Reg::read) this register and get [`mac_tmrqr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_tmrqr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MAC_TMRQR)*/
pub struct MAC_TMRQRrs;
impl crate::RegisterSpec for MAC_TMRQRrs {
    type Ux = u32;
}
///`read()` method returns [`mac_tmrqr::R`](R) reader structure
impl crate::Readable for MAC_TMRQRrs {}
///`write(|w| ..)` method takes [`mac_tmrqr::W`](W) writer structure
impl crate::Writable for MAC_TMRQRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MAC_TMRQR to value 0
impl crate::Resettable for MAC_TMRQRrs {}
