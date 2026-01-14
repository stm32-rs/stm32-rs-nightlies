///Register `MACA2HR` reader
pub type R = crate::R<MACA2HRrs>;
///Register `MACA2HR` writer
pub type W = crate::W<MACA2HRrs>;
///Field `ADDRHI` reader - MAC Address1 \[47:32\] This field contains the upper 16 bits\[47:32\] of the second 6-byte MAC address.
pub type ADDRHI_R = crate::FieldReader<u16>;
///Field `ADDRHI` writer - MAC Address1 \[47:32\] This field contains the upper 16 bits\[47:32\] of the second 6-byte MAC address.
pub type ADDRHI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `MBC` reader - Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes. When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers. Each bit controls the masking of the bytes as follows: Bit 29: ETH_MACAxHR\[15:8\] Bit 28: ETH_MACAxHR\[7:0\] Bit 27: ETH_MACAxLR\[31:24\] Bit 26: ETH_MACAxLR\[23:16\] Bit 25: ETH_MACAxLR\[15:8\] Bit 24: ETH_MACAxLR\[7:0\] You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address.
pub type MBC_R = crate::FieldReader;
///Field `MBC` writer - Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes. When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers. Each bit controls the masking of the bytes as follows: Bit 29: ETH_MACAxHR\[15:8\] Bit 28: ETH_MACAxHR\[7:0\] Bit 27: ETH_MACAxLR\[31:24\] Bit 26: ETH_MACAxLR\[23:16\] Bit 25: ETH_MACAxLR\[15:8\] Bit 24: ETH_MACAxLR\[7:0\] You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address.
pub type MBC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SA` reader - Source Address When this bit is set, the MAC Address1\[47:0\] is used to compare with the SA fields of the received packet. When this bit is reset, the MAC Address x\[47:0\] is used to compare with the DA fields of the received packet.
pub type SA_R = crate::BitReader;
///Field `SA` writer - Source Address When this bit is set, the MAC Address1\[47:0\] is used to compare with the SA fields of the received packet. When this bit is reset, the MAC Address x\[47:0\] is used to compare with the DA fields of the received packet.
pub type SA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AE` reader - Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering. When this bit is reset, the address filter module ignores the address for filtering.
pub type AE_R = crate::BitReader;
///Field `AE` writer - Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering. When this bit is reset, the address filter module ignores the address for filtering.
pub type AE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - MAC Address1 \[47:32\] This field contains the upper 16 bits\[47:32\] of the second 6-byte MAC address.
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 24:29 - Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes. When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers. Each bit controls the masking of the bytes as follows: Bit 29: ETH_MACAxHR\[15:8\] Bit 28: ETH_MACAxHR\[7:0\] Bit 27: ETH_MACAxLR\[31:24\] Bit 26: ETH_MACAxLR\[23:16\] Bit 25: ETH_MACAxLR\[15:8\] Bit 24: ETH_MACAxLR\[7:0\] You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address.
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bit 30 - Source Address When this bit is set, the MAC Address1\[47:0\] is used to compare with the SA fields of the received packet. When this bit is reset, the MAC Address x\[47:0\] is used to compare with the DA fields of the received packet.
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering. When this bit is reset, the address filter module ignores the address for filtering.
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA2HR")
            .field("addrhi", &self.addrhi())
            .field("mbc", &self.mbc())
            .field("sa", &self.sa())
            .field("ae", &self.ae())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - MAC Address1 \[47:32\] This field contains the upper 16 bits\[47:32\] of the second 6-byte MAC address.
    #[inline(always)]
    pub fn addrhi(&mut self) -> ADDRHI_W<'_, MACA2HRrs> {
        ADDRHI_W::new(self, 0)
    }
    ///Bits 24:29 - Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes. When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers. Each bit controls the masking of the bytes as follows: Bit 29: ETH_MACAxHR\[15:8\] Bit 28: ETH_MACAxHR\[7:0\] Bit 27: ETH_MACAxLR\[31:24\] Bit 26: ETH_MACAxLR\[23:16\] Bit 25: ETH_MACAxLR\[15:8\] Bit 24: ETH_MACAxLR\[7:0\] You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address.
    #[inline(always)]
    pub fn mbc(&mut self) -> MBC_W<'_, MACA2HRrs> {
        MBC_W::new(self, 24)
    }
    ///Bit 30 - Source Address When this bit is set, the MAC Address1\[47:0\] is used to compare with the SA fields of the received packet. When this bit is reset, the MAC Address x\[47:0\] is used to compare with the DA fields of the received packet.
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W<'_, MACA2HRrs> {
        SA_W::new(self, 30)
    }
    ///Bit 31 - Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering. When this bit is reset, the address filter module ignores the address for filtering.
    #[inline(always)]
    pub fn ae(&mut self) -> AE_W<'_, MACA2HRrs> {
        AE_W::new(self, 31)
    }
}
/**MAC Address 2 high register

You can [`read`](crate::Reg::read) this register and get [`maca2hr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca2hr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#ETH:MACA2HR)*/
pub struct MACA2HRrs;
impl crate::RegisterSpec for MACA2HRrs {
    type Ux = u32;
}
///`read()` method returns [`maca2hr::R`](R) reader structure
impl crate::Readable for MACA2HRrs {}
///`write(|w| ..)` method takes [`maca2hr::W`](W) writer structure
impl crate::Writable for MACA2HRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACA2HR to value 0xffff
impl crate::Resettable for MACA2HRrs {
    const RESET_VALUE: u32 = 0xffff;
}
