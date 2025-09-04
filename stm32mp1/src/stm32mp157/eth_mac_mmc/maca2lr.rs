///Register `MACA2LR` reader
pub type R = crate::R<MACA2LRrs>;
///Register `MACA2LR` writer
pub type W = crate::W<MACA2LRrs>;
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
        f.debug_struct("MACA2LR")
            .field("addrlo", &self.addrlo())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ADDRLO
    #[inline(always)]
    pub fn addrlo(&mut self) -> ADDRLO_W<MACA2LRrs> {
        ADDRLO_W::new(self, 0)
    }
}
/**The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.

You can [`read`](crate::Reg::read) this register and get [`maca2lr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca2lr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:MACA2LR)*/
pub struct MACA2LRrs;
impl crate::RegisterSpec for MACA2LRrs {
    type Ux = u32;
}
///`read()` method returns [`maca2lr::R`](R) reader structure
impl crate::Readable for MACA2LRrs {}
///`write(|w| ..)` method takes [`maca2lr::W`](W) writer structure
impl crate::Writable for MACA2LRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACA2LR to value 0xffff_ffff
impl crate::Resettable for MACA2LRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
