#[doc = "Register `MACA3HR` reader"]
pub type R = crate::R<MACA3HRrs>;
#[doc = "Register `MACA3HR` writer"]
pub type W = crate::W<MACA3HRrs>;
#[doc = "Field `ADDRHI` reader - MAC Address1 \\[47:32\\]
This field contains the upper 16 bits\\[47:32\\]
of the second 6-byte MAC address."]
pub type ADDRHI_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRHI` writer - MAC Address1 \\[47:32\\]
This field contains the upper 16 bits\\[47:32\\]
of the second 6-byte MAC address."]
pub type ADDRHI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBC` reader - Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes. When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers. Each bit controls the masking of the bytes as follows: Bit 29: ETH_MACAxHR\\[15:8\\]
Bit 28: ETH_MACAxHR\\[7:0\\]
Bit 27: ETH_MACAxLR\\[31:24\\]
Bit 26: ETH_MACAxLR\\[23:16\\]
Bit 25: ETH_MACAxLR\\[15:8\\]
Bit 24: ETH_MACAxLR\\[7:0\\]
You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
pub type MBC_R = crate::FieldReader;
#[doc = "Field `MBC` writer - Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes. When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers. Each bit controls the masking of the bytes as follows: Bit 29: ETH_MACAxHR\\[15:8\\]
Bit 28: ETH_MACAxHR\\[7:0\\]
Bit 27: ETH_MACAxLR\\[31:24\\]
Bit 26: ETH_MACAxLR\\[23:16\\]
Bit 25: ETH_MACAxLR\\[15:8\\]
Bit 24: ETH_MACAxLR\\[7:0\\]
You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
pub type MBC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SA` reader - Source Address When this bit is set, the MAC Addressx\\[47:0\\]
is used to compare with the SA fields of the received packet. When this bit is reset, the MAC Address x\\[47:0\\]
is used to compare with the DA fields of the received packet."]
pub type SA_R = crate::BitReader;
#[doc = "Field `SA` writer - Source Address When this bit is set, the MAC Addressx\\[47:0\\]
is used to compare with the SA fields of the received packet. When this bit is reset, the MAC Address x\\[47:0\\]
is used to compare with the DA fields of the received packet."]
pub type SA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE` reader - Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering. When this bit is reset, the address filter module ignores the address for filtering."]
pub type AE_R = crate::BitReader;
#[doc = "Field `AE` writer - Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering. When this bit is reset, the address filter module ignores the address for filtering."]
pub type AE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - MAC Address1 \\[47:32\\]
This field contains the upper 16 bits\\[47:32\\]
of the second 6-byte MAC address."]
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes. When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers. Each bit controls the masking of the bytes as follows: Bit 29: ETH_MACAxHR\\[15:8\\]
Bit 28: ETH_MACAxHR\\[7:0\\]
Bit 27: ETH_MACAxLR\\[31:24\\]
Bit 26: ETH_MACAxLR\\[23:16\\]
Bit 25: ETH_MACAxLR\\[15:8\\]
Bit 24: ETH_MACAxLR\\[7:0\\]
You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source Address When this bit is set, the MAC Addressx\\[47:0\\]
is used to compare with the SA fields of the received packet. When this bit is reset, the MAC Address x\\[47:0\\]
is used to compare with the DA fields of the received packet."]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering. When this bit is reset, the address filter module ignores the address for filtering."]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address1 \\[47:32\\]
This field contains the upper 16 bits\\[47:32\\]
of the second 6-byte MAC address."]
    #[inline(always)]
    #[must_use]
    pub fn addrhi(&mut self) -> ADDRHI_W<MACA3HRrs> {
        ADDRHI_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes. When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers. Each bit controls the masking of the bytes as follows: Bit 29: ETH_MACAxHR\\[15:8\\]
Bit 28: ETH_MACAxHR\\[7:0\\]
Bit 27: ETH_MACAxLR\\[31:24\\]
Bit 26: ETH_MACAxLR\\[23:16\\]
Bit 25: ETH_MACAxLR\\[15:8\\]
Bit 24: ETH_MACAxLR\\[7:0\\]
You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
    #[inline(always)]
    #[must_use]
    pub fn mbc(&mut self) -> MBC_W<MACA3HRrs> {
        MBC_W::new(self, 24)
    }
    #[doc = "Bit 30 - Source Address When this bit is set, the MAC Addressx\\[47:0\\]
is used to compare with the SA fields of the received packet. When this bit is reset, the MAC Address x\\[47:0\\]
is used to compare with the DA fields of the received packet."]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<MACA3HRrs> {
        SA_W::new(self, 30)
    }
    #[doc = "Bit 31 - Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering. When this bit is reset, the address filter module ignores the address for filtering."]
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AE_W<MACA3HRrs> {
        AE_W::new(self, 31)
    }
}
#[doc = "MAC Address 3 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3hr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3hr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA3HRrs;
impl crate::RegisterSpec for MACA3HRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca3hr::R`](R) reader structure"]
impl crate::Readable for MACA3HRrs {}
#[doc = "`write(|w| ..)` method takes [`maca3hr::W`](W) writer structure"]
impl crate::Writable for MACA3HRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA3HR to value 0xffff"]
impl crate::Resettable for MACA3HRrs {
    const RESET_VALUE: u32 = 0xffff;
}
