///Register `MACRXQCR` reader
pub type R = crate::R<MACRXQCRrs>;
///Register `MACRXQCR` writer
pub type W = crate::W<MACRXQCRrs>;
///Field `UFFQE` reader - Unicast Address Filter Fail Packets Queuing Enable.
pub type UFFQE_R = crate::BitReader;
///Field `UFFQE` writer - Unicast Address Filter Fail Packets Queuing Enable.
pub type UFFQE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UFFQ` reader - Unicast Address Filter Fail Packets Queue.
pub type UFFQ_R = crate::BitReader;
///Field `UFFQ` writer - Unicast Address Filter Fail Packets Queue.
pub type UFFQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MFFQE` reader - Multicast Address Filter Fail Packets Queuing Enable.
pub type MFFQE_R = crate::BitReader;
///Field `MFFQE` writer - Multicast Address Filter Fail Packets Queuing Enable.
pub type MFFQE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MFFQ` reader - Multicast Address Filter Fail Packets Queue.
pub type MFFQ_R = crate::BitReader;
///Field `MFFQ` writer - Multicast Address Filter Fail Packets Queue.
pub type MFFQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VFFQE` reader - VLAN Tag Filter Fail Packets Queuing Enable
pub type VFFQE_R = crate::BitReader;
///Field `VFFQE` writer - VLAN Tag Filter Fail Packets Queuing Enable
pub type VFFQE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VFFQ` reader - VLAN Tag Filter Fail Packets Queue
pub type VFFQ_R = crate::BitReader;
///Field `VFFQ` writer - VLAN Tag Filter Fail Packets Queue
pub type VFFQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Unicast Address Filter Fail Packets Queuing Enable.
    #[inline(always)]
    pub fn uffqe(&self) -> UFFQE_R {
        UFFQE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Unicast Address Filter Fail Packets Queue.
    #[inline(always)]
    pub fn uffq(&self) -> UFFQ_R {
        UFFQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Multicast Address Filter Fail Packets Queuing Enable.
    #[inline(always)]
    pub fn mffqe(&self) -> MFFQE_R {
        MFFQE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Multicast Address Filter Fail Packets Queue.
    #[inline(always)]
    pub fn mffq(&self) -> MFFQ_R {
        MFFQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - VLAN Tag Filter Fail Packets Queuing Enable
    #[inline(always)]
    pub fn vffqe(&self) -> VFFQE_R {
        VFFQE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - VLAN Tag Filter Fail Packets Queue
    #[inline(always)]
    pub fn vffq(&self) -> VFFQ_R {
        VFFQ_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACRXQCR")
            .field("uffqe", &self.uffqe())
            .field("uffq", &self.uffq())
            .field("mffqe", &self.mffqe())
            .field("mffq", &self.mffq())
            .field("vffqe", &self.vffqe())
            .field("vffq", &self.vffq())
            .finish()
    }
}
impl W {
    ///Bit 0 - Unicast Address Filter Fail Packets Queuing Enable.
    #[inline(always)]
    pub fn uffqe(&mut self) -> UFFQE_W<'_, MACRXQCRrs> {
        UFFQE_W::new(self, 0)
    }
    ///Bit 1 - Unicast Address Filter Fail Packets Queue.
    #[inline(always)]
    pub fn uffq(&mut self) -> UFFQ_W<'_, MACRXQCRrs> {
        UFFQ_W::new(self, 1)
    }
    ///Bit 8 - Multicast Address Filter Fail Packets Queuing Enable.
    #[inline(always)]
    pub fn mffqe(&mut self) -> MFFQE_W<'_, MACRXQCRrs> {
        MFFQE_W::new(self, 8)
    }
    ///Bit 9 - Multicast Address Filter Fail Packets Queue.
    #[inline(always)]
    pub fn mffq(&mut self) -> MFFQ_W<'_, MACRXQCRrs> {
        MFFQ_W::new(self, 9)
    }
    ///Bit 16 - VLAN Tag Filter Fail Packets Queuing Enable
    #[inline(always)]
    pub fn vffqe(&mut self) -> VFFQE_W<'_, MACRXQCRrs> {
        VFFQE_W::new(self, 16)
    }
    ///Bit 17 - VLAN Tag Filter Fail Packets Queue
    #[inline(always)]
    pub fn vffq(&mut self) -> VFFQ_W<'_, MACRXQCRrs> {
        VFFQ_W::new(self, 17)
    }
}
/**Rx Queue control register

You can [`read`](crate::Reg::read) this register and get [`macrxqcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrxqcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:MACRXQCR)*/
pub struct MACRXQCRrs;
impl crate::RegisterSpec for MACRXQCRrs {
    type Ux = u32;
}
///`read()` method returns [`macrxqcr::R`](R) reader structure
impl crate::Readable for MACRXQCRrs {}
///`write(|w| ..)` method takes [`macrxqcr::W`](W) writer structure
impl crate::Writable for MACRXQCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACRXQCR to value 0
impl crate::Resettable for MACRXQCRrs {}
