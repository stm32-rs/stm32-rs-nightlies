#[doc = "Register `MACWTR` reader"]
pub type R = crate::R<MACWTRrs>;
#[doc = "Register `MACWTR` writer"]
pub type W = crate::W<MACWTRrs>;
#[doc = "Field `WTO` reader - Watchdog Timeout When the PWE bit is set and the WD bit of the Operating mode configuration register (ETH_MACCR) register is reset, this field is used as watchdog timeout for a received packet. If the length of a received packet exceeds the value of this field, such packet is terminated and declared as an error packet. Encoding is as follows: .. Note: When the PWE bit is set, the value in this field should be more than 1,522 (0x05F2). Otherwise, the IEEE 802.3-specified valid tagged packets are declared as error packets and then dropped."]
pub type WTO_R = crate::FieldReader;
#[doc = "Field `WTO` writer - Watchdog Timeout When the PWE bit is set and the WD bit of the Operating mode configuration register (ETH_MACCR) register is reset, this field is used as watchdog timeout for a received packet. If the length of a received packet exceeds the value of this field, such packet is terminated and declared as an error packet. Encoding is as follows: .. Note: When the PWE bit is set, the value in this field should be more than 1,522 (0x05F2). Otherwise, the IEEE 802.3-specified valid tagged packets are declared as error packets and then dropped."]
pub type WTO_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PWE` reader - Programmable Watchdog Enable When this bit is set and the WD bit of the Operating mode configuration register (ETH_MACCR) register is reset, the WTO field is used as watchdog timeout for a received packet. When this bit is cleared, the watchdog timeout for a received packet is controlled by setting of WD and JE bits in Operating mode configuration register (ETH_MACCR) register."]
pub type PWE_R = crate::BitReader;
#[doc = "Field `PWE` writer - Programmable Watchdog Enable When this bit is set and the WD bit of the Operating mode configuration register (ETH_MACCR) register is reset, the WTO field is used as watchdog timeout for a received packet. When this bit is cleared, the watchdog timeout for a received packet is controlled by setting of WD and JE bits in Operating mode configuration register (ETH_MACCR) register."]
pub type PWE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Watchdog Timeout When the PWE bit is set and the WD bit of the Operating mode configuration register (ETH_MACCR) register is reset, this field is used as watchdog timeout for a received packet. If the length of a received packet exceeds the value of this field, such packet is terminated and declared as an error packet. Encoding is as follows: .. Note: When the PWE bit is set, the value in this field should be more than 1,522 (0x05F2). Otherwise, the IEEE 802.3-specified valid tagged packets are declared as error packets and then dropped."]
    #[inline(always)]
    pub fn wto(&self) -> WTO_R {
        WTO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Programmable Watchdog Enable When this bit is set and the WD bit of the Operating mode configuration register (ETH_MACCR) register is reset, the WTO field is used as watchdog timeout for a received packet. When this bit is cleared, the watchdog timeout for a received packet is controlled by setting of WD and JE bits in Operating mode configuration register (ETH_MACCR) register."]
    #[inline(always)]
    pub fn pwe(&self) -> PWE_R {
        PWE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Watchdog Timeout When the PWE bit is set and the WD bit of the Operating mode configuration register (ETH_MACCR) register is reset, this field is used as watchdog timeout for a received packet. If the length of a received packet exceeds the value of this field, such packet is terminated and declared as an error packet. Encoding is as follows: .. Note: When the PWE bit is set, the value in this field should be more than 1,522 (0x05F2). Otherwise, the IEEE 802.3-specified valid tagged packets are declared as error packets and then dropped."]
    #[inline(always)]
    #[must_use]
    pub fn wto(&mut self) -> WTO_W<MACWTRrs> {
        WTO_W::new(self, 0)
    }
    #[doc = "Bit 8 - Programmable Watchdog Enable When this bit is set and the WD bit of the Operating mode configuration register (ETH_MACCR) register is reset, the WTO field is used as watchdog timeout for a received packet. When this bit is cleared, the watchdog timeout for a received packet is controlled by setting of WD and JE bits in Operating mode configuration register (ETH_MACCR) register."]
    #[inline(always)]
    #[must_use]
    pub fn pwe(&mut self) -> PWE_W<MACWTRrs> {
        PWE_W::new(self, 8)
    }
}
#[doc = "Watchdog timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macwtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macwtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACWTRrs;
impl crate::RegisterSpec for MACWTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macwtr::R`](R) reader structure"]
impl crate::Readable for MACWTRrs {}
#[doc = "`write(|w| ..)` method takes [`macwtr::W`](W) writer structure"]
impl crate::Writable for MACWTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACWTR to value 0"]
impl crate::Resettable for MACWTRrs {
    const RESET_VALUE: u32 = 0;
}
