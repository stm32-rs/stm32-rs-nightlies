///Register `ETH_MACARPAR` reader
pub type R = crate::R<ETH_MACARPARrs>;
///Register `ETH_MACARPAR` writer
pub type W = crate::W<ETH_MACARPARrs>;
///Field `ARPPA` reader - ARPPA
pub type ARPPA_R = crate::FieldReader<u32>;
///Field `ARPPA` writer - ARPPA
pub type ARPPA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ARPPA
    #[inline(always)]
    pub fn arppa(&self) -> ARPPA_R {
        ARPPA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_MACARPAR")
            .field("arppa", &self.arppa())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ARPPA
    #[inline(always)]
    #[must_use]
    pub fn arppa(&mut self) -> ARPPA_W<ETH_MACARPARrs> {
        ARPPA_W::new(self, 0)
    }
}
/**The ARP Address register contains the IPv4 Destination Address of the MAC.

You can [`read`](crate::Reg::read) this register and get [`eth_macarpar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_macarpar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:ETH_MACARPAR)*/
pub struct ETH_MACARPARrs;
impl crate::RegisterSpec for ETH_MACARPARrs {
    type Ux = u32;
}
///`read()` method returns [`eth_macarpar::R`](R) reader structure
impl crate::Readable for ETH_MACARPARrs {}
///`write(|w| ..)` method takes [`eth_macarpar::W`](W) writer structure
impl crate::Writable for ETH_MACARPARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_MACARPAR to value 0
impl crate::Resettable for ETH_MACARPARrs {
    const RESET_VALUE: u32 = 0;
}
