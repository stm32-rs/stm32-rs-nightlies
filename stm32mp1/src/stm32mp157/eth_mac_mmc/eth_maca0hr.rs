#[doc = "Register `ETH_MACA0HR` reader"]
pub type R = crate::R<ETH_MACA0HRrs>;
#[doc = "Register `ETH_MACA0HR` writer"]
pub type W = crate::W<ETH_MACA0HRrs>;
#[doc = "Field `ADDRHI` reader - ADDRHI"]
pub type ADDRHI_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRHI` writer - ADDRHI"]
pub type ADDRHI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `AE` reader - AE"]
pub type AE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - ADDRHI"]
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - AE"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDRHI"]
    #[inline(always)]
    #[must_use]
    pub fn addrhi(&mut self) -> ADDRHI_W<ETH_MACA0HRrs> {
        ADDRHI_W::new(self, 0)
    }
}
#[doc = "The MAC Address0 High register holds the upper 16 bits of the first 6-byte MAC address of the station. The first DA byte that is received on the GMII interface corresponds to the LS byte (Bits \\[7:0\\]) of the MAC Address Low register. For example, if 0x112233445566 is received (0x11 in lane 0 of the first column) on the GMII as the destination address, then the MacAddress0 Register \\[47:0\\]
is compared with 0x665544332211. If the MAC address registers are configured to be double-synchronized to the GMII clock domains, then the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the MAC Address0 Low Register are written. For proper synchronization updates, the consecutive writes to this Address Low Register should be performed after at least four clock cycles in the destination clock domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_maca0hr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_maca0hr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACA0HRrs;
impl crate::RegisterSpec for ETH_MACA0HRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_maca0hr::R`](R) reader structure"]
impl crate::Readable for ETH_MACA0HRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_maca0hr::W`](W) writer structure"]
impl crate::Writable for ETH_MACA0HRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACA0HR to value 0x8000_ffff"]
impl crate::Resettable for ETH_MACA0HRrs {
    const RESET_VALUE: u32 = 0x8000_ffff;
}
