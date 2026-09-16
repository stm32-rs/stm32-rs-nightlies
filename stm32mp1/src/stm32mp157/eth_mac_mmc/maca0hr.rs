///Register `MACA0HR` reader
pub type R = crate::R<MACA0HRrs>;
///Register `MACA0HR` writer
pub type W = crate::W<MACA0HRrs>;
///Field `ADDRHI` reader - ADDRHI
pub type ADDRHI_R = crate::FieldReader<u16>;
///Field `ADDRHI` writer - ADDRHI
pub type ADDRHI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `AE` reader - AE
pub type AE_R = crate::BitReader;
impl R {
    ///Bits 0:15 - ADDRHI
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 31 - AE
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA0HR")
            .field("addrhi", &self.addrhi())
            .field("ae", &self.ae())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - ADDRHI
    #[inline(always)]
    pub fn addrhi(&mut self) -> ADDRHI_W<'_, MACA0HRrs> {
        ADDRHI_W::new(self, 0)
    }
}
/**The MAC Address0 High register holds the upper 16 bits of the first 6-byte MAC address of the station. The first DA byte that is received on the GMII interface corresponds to the LS byte (Bits \[7:0\]) of the MAC Address Low register. For example, if 0x112233445566 is received (0x11 in lane 0 of the first column) on the GMII as the destination address, then the MacAddress0 Register \[47:0\] is compared with 0x665544332211. If the MAC address registers are configured to be double-synchronized to the GMII clock domains, then the synchronization is triggered only when Bits\[31:24\] (in little-endian mode) or Bits\[7:0\] (in big-endian mode) of the MAC Address0 Low Register are written. For proper synchronization updates, the consecutive writes to this Address Low Register should be performed after at least four clock cycles in the destination clock domain.

You can [`read`](crate::Reg::read) this register and get [`maca0hr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca0hr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:MACA0HR)*/
pub struct MACA0HRrs;
impl crate::RegisterSpec for MACA0HRrs {
    type Ux = u32;
}
///`read()` method returns [`maca0hr::R`](R) reader structure
impl crate::Readable for MACA0HRrs {}
///`write(|w| ..)` method takes [`maca0hr::W`](W) writer structure
impl crate::Writable for MACA0HRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACA0HR to value 0x8000_ffff
impl crate::Resettable for MACA0HRrs {
    const RESET_VALUE: u32 = 0x8000_ffff;
}
