///Register `MACARPAR` reader
pub type R = crate::R<MACARPARrs>;
///Register `MACARPAR` writer
pub type W = crate::W<MACARPARrs>;
///Field `ARPPA` reader - ARP Protocol Address This field contains the IPv4 Destination Address of the MAC. This address is used for perfect match with the Protocol Address of Target field in the received ARP packet.
pub type ARPPA_R = crate::FieldReader<u32>;
///Field `ARPPA` writer - ARP Protocol Address This field contains the IPv4 Destination Address of the MAC. This address is used for perfect match with the Protocol Address of Target field in the received ARP packet.
pub type ARPPA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ARP Protocol Address This field contains the IPv4 Destination Address of the MAC. This address is used for perfect match with the Protocol Address of Target field in the received ARP packet.
    #[inline(always)]
    pub fn arppa(&self) -> ARPPA_R {
        ARPPA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACARPAR")
            .field("arppa", &self.arppa())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ARP Protocol Address This field contains the IPv4 Destination Address of the MAC. This address is used for perfect match with the Protocol Address of Target field in the received ARP packet.
    #[inline(always)]
    pub fn arppa(&mut self) -> ARPPA_W<'_, MACARPARrs> {
        ARPPA_W::new(self, 0)
    }
}
/**ARP address register

You can [`read`](crate::Reg::read) this register and get [`macarpar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macarpar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#ETH:MACARPAR)*/
pub struct MACARPARrs;
impl crate::RegisterSpec for MACARPARrs {
    type Ux = u32;
}
///`read()` method returns [`macarpar::R`](R) reader structure
impl crate::Readable for MACARPARrs {}
///`write(|w| ..)` method takes [`macarpar::W`](W) writer structure
impl crate::Writable for MACARPARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACARPAR to value 0
impl crate::Resettable for MACARPARrs {}
