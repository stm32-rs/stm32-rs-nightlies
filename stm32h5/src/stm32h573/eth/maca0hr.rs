#[doc = "Register `MACA0HR` reader"]
pub type R = crate::R<MACA0HRrs>;
#[doc = "Register `MACA0HR` writer"]
pub type W = crate::W<MACA0HRrs>;
#[doc = "Field `ADDRHI` reader - MAC Address0\\[47:32\\]
This field contains the upper 16 bits \\[47:32\\]
of the first 6-byte MAC address. The MAC uses this field for filtering the received packets and inserting the MAC address in the Transmit Flow Control (Pause) Packets."]
pub type ADDRHI_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRHI` writer - MAC Address0\\[47:32\\]
This field contains the upper 16 bits \\[47:32\\]
of the first 6-byte MAC address. The MAC uses this field for filtering the received packets and inserting the MAC address in the Transmit Flow Control (Pause) Packets."]
pub type ADDRHI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `AE` reader - Address Enable This bit is always set to 1."]
pub type AE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - MAC Address0\\[47:32\\]
This field contains the upper 16 bits \\[47:32\\]
of the first 6-byte MAC address. The MAC uses this field for filtering the received packets and inserting the MAC address in the Transmit Flow Control (Pause) Packets."]
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Address Enable This bit is always set to 1."]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address0\\[47:32\\]
This field contains the upper 16 bits \\[47:32\\]
of the first 6-byte MAC address. The MAC uses this field for filtering the received packets and inserting the MAC address in the Transmit Flow Control (Pause) Packets."]
    #[inline(always)]
    #[must_use]
    pub fn addrhi(&mut self) -> ADDRHI_W<MACA0HRrs> {
        ADDRHI_W::new(self, 0)
    }
}
#[doc = "MAC Address 0 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0hr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0hr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA0HRrs;
impl crate::RegisterSpec for MACA0HRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca0hr::R`](R) reader structure"]
impl crate::Readable for MACA0HRrs {}
#[doc = "`write(|w| ..)` method takes [`maca0hr::W`](W) writer structure"]
impl crate::Writable for MACA0HRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA0HR to value 0x8000_ffff"]
impl crate::Resettable for MACA0HRrs {
    const RESET_VALUE: u32 = 0x8000_ffff;
}
