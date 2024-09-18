///Register `ETH_MACA2LR` reader
pub type R = crate::R<ETH_MACA2LRrs>;
///Register `ETH_MACA2LR` writer
pub type W = crate::W<ETH_MACA2LRrs>;
///Field `ADDRLO` reader - ADDRLO
pub type ADDRLO_R = crate::FieldReader<u32>;
///Field `ADDRLO` writer - ADDRLO
pub type ADDRLO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ADDRLO
    #[inline(always)]
    pub fn addrlo(&self) -> ADDRLO_R {
        ADDRLO_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_MACA2LR")
            .field("addrlo", &self.addrlo())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ADDRLO
    #[inline(always)]
    #[must_use]
    pub fn addrlo(&mut self) -> ADDRLO_W<ETH_MACA2LRrs> {
        ADDRLO_W::new(self, 0)
    }
}
/**The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.

You can [`read`](crate::Reg::read) this register and get [`eth_maca2lr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_maca2lr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:ETH_MACA2LR)*/
pub struct ETH_MACA2LRrs;
impl crate::RegisterSpec for ETH_MACA2LRrs {
    type Ux = u32;
}
///`read()` method returns [`eth_maca2lr::R`](R) reader structure
impl crate::Readable for ETH_MACA2LRrs {}
///`write(|w| ..)` method takes [`eth_maca2lr::W`](W) writer structure
impl crate::Writable for ETH_MACA2LRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_MACA2LR to value 0xffff_ffff
impl crate::Resettable for ETH_MACA2LRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
