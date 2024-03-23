#[doc = "Register `DMACRXIWTR` reader"]
pub type R = crate::R<DMACRXIWTRrs>;
#[doc = "Register `DMACRXIWTR` writer"]
pub type W = crate::W<DMACRXIWTRrs>;
#[doc = "Field `RWT` reader - Receive Interrupt Watchdog Timer Count This field indicates the number of system clock cycles, multiplied by factor indicated in RWTU field, for which the watchdog timer is set. The watchdog timer is triggered with the programmed value after the Rx DMA completes the transfer of a packet for which the RI bit is not set in the ETH_DMACSR, because of the setting of Interrupt Enable bit in the corresponding descriptor RDES3\\[30\\]. When the watchdog timer runs out, the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per the Interrupt Enable bit RDES3\\[30\\]
of any received packet."]
pub type RWT_R = crate::FieldReader;
#[doc = "Field `RWT` writer - Receive Interrupt Watchdog Timer Count This field indicates the number of system clock cycles, multiplied by factor indicated in RWTU field, for which the watchdog timer is set. The watchdog timer is triggered with the programmed value after the Rx DMA completes the transfer of a packet for which the RI bit is not set in the ETH_DMACSR, because of the setting of Interrupt Enable bit in the corresponding descriptor RDES3\\[30\\]. When the watchdog timer runs out, the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per the Interrupt Enable bit RDES3\\[30\\]
of any received packet."]
pub type RWT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RWTU` reader - Receive Interrupt Watchdog Timer Count Units This field indicates the number of system clock cycles corresponding to one unit in RWT\\[7:0\\]
field. For example, when RWT\\[7:0\\]�=�2 and RWTU\\[1:0\\]�=�1, the watchdog timer is set for 2�*�512�=�1024 system clock cycles."]
pub type RWTU_R = crate::FieldReader;
#[doc = "Field `RWTU` writer - Receive Interrupt Watchdog Timer Count Units This field indicates the number of system clock cycles corresponding to one unit in RWT\\[7:0\\]
field. For example, when RWT\\[7:0\\]�=�2 and RWTU\\[1:0\\]�=�1, the watchdog timer is set for 2�*�512�=�1024 system clock cycles."]
pub type RWTU_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Receive Interrupt Watchdog Timer Count This field indicates the number of system clock cycles, multiplied by factor indicated in RWTU field, for which the watchdog timer is set. The watchdog timer is triggered with the programmed value after the Rx DMA completes the transfer of a packet for which the RI bit is not set in the ETH_DMACSR, because of the setting of Interrupt Enable bit in the corresponding descriptor RDES3\\[30\\]. When the watchdog timer runs out, the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per the Interrupt Enable bit RDES3\\[30\\]
of any received packet."]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Receive Interrupt Watchdog Timer Count Units This field indicates the number of system clock cycles corresponding to one unit in RWT\\[7:0\\]
field. For example, when RWT\\[7:0\\]�=�2 and RWTU\\[1:0\\]�=�1, the watchdog timer is set for 2�*�512�=�1024 system clock cycles."]
    #[inline(always)]
    pub fn rwtu(&self) -> RWTU_R {
        RWTU_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Interrupt Watchdog Timer Count This field indicates the number of system clock cycles, multiplied by factor indicated in RWTU field, for which the watchdog timer is set. The watchdog timer is triggered with the programmed value after the Rx DMA completes the transfer of a packet for which the RI bit is not set in the ETH_DMACSR, because of the setting of Interrupt Enable bit in the corresponding descriptor RDES3\\[30\\]. When the watchdog timer runs out, the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per the Interrupt Enable bit RDES3\\[30\\]
of any received packet."]
    #[inline(always)]
    #[must_use]
    pub fn rwt(&mut self) -> RWT_W<DMACRXIWTRrs> {
        RWT_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - Receive Interrupt Watchdog Timer Count Units This field indicates the number of system clock cycles corresponding to one unit in RWT\\[7:0\\]
field. For example, when RWT\\[7:0\\]�=�2 and RWTU\\[1:0\\]�=�1, the watchdog timer is set for 2�*�512�=�1024 system clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn rwtu(&mut self) -> RWTU_W<DMACRXIWTRrs> {
        RWTU_W::new(self, 16)
    }
}
#[doc = "Channel Rx interrupt watchdog timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrxiwtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrxiwtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACRXIWTRrs;
impl crate::RegisterSpec for DMACRXIWTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrxiwtr::R`](R) reader structure"]
impl crate::Readable for DMACRXIWTRrs {}
#[doc = "`write(|w| ..)` method takes [`dmacrxiwtr::W`](W) writer structure"]
impl crate::Writable for DMACRXIWTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACRXIWTR to value 0"]
impl crate::Resettable for DMACRXIWTRrs {
    const RESET_VALUE: u32 = 0;
}
