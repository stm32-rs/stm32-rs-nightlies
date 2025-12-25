///Register `DMAC1RXRLR` reader
pub type R = crate::R<DMAC1RXRLRrs>;
///Register `DMAC1RXRLR` writer
pub type W = crate::W<DMAC1RXRLRrs>;
///Field `RDRL` reader - Receive Descriptor Ring Length
pub type RDRL_R = crate::FieldReader<u16>;
///Field `RDRL` writer - Receive Descriptor Ring Length
pub type RDRL_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `ARBS` reader - Alternate Receive Buffer Size
pub type ARBS_R = crate::FieldReader;
///Field `ARBS` writer - Alternate Receive Buffer Size
pub type ARBS_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:9 - Receive Descriptor Ring Length
    #[inline(always)]
    pub fn rdrl(&self) -> RDRL_R {
        RDRL_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 17:23 - Alternate Receive Buffer Size
    #[inline(always)]
    pub fn arbs(&self) -> ARBS_R {
        ARBS_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC1RXRLR")
            .field("rdrl", &self.rdrl())
            .field("arbs", &self.arbs())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Receive Descriptor Ring Length
    #[inline(always)]
    pub fn rdrl(&mut self) -> RDRL_W<'_, DMAC1RXRLRrs> {
        RDRL_W::new(self, 0)
    }
    ///Bits 17:23 - Alternate Receive Buffer Size
    #[inline(always)]
    pub fn arbs(&mut self) -> ARBS_W<'_, DMAC1RXRLRrs> {
        ARBS_W::new(self, 17)
    }
}
/**Channel 1 R1 descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`dmac1rxrlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1rxrlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:DMAC1RXRLR)*/
pub struct DMAC1RXRLRrs;
impl crate::RegisterSpec for DMAC1RXRLRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac1rxrlr::R`](R) reader structure
impl crate::Readable for DMAC1RXRLRrs {}
///`write(|w| ..)` method takes [`dmac1rxrlr::W`](W) writer structure
impl crate::Writable for DMAC1RXRLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC1RXRLR to value 0
impl crate::Resettable for DMAC1RXRLRrs {}
