///Register `MACPMTCSR` reader
pub type R = crate::R<MACPMTCSRrs>;
///Register `MACPMTCSR` writer
pub type W = crate::W<MACPMTCSRrs>;
///Field `PD` reader - Power down
pub type PD_R = crate::BitReader;
///Field `PD` writer - Power down
pub type PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPE` reader - Magic Packet enable
pub type MPE_R = crate::BitReader;
///Field `MPE` writer - Magic Packet enable
pub type MPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WFE` reader - Wakeup frame enable
pub type WFE_R = crate::BitReader;
///Field `WFE` writer - Wakeup frame enable
pub type WFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPR` reader - Magic packet received
pub type MPR_R = crate::BitReader;
///Field `MPR` writer - Magic packet received
pub type MPR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WFR` reader - Wakeup frame received
pub type WFR_R = crate::BitReader;
///Field `WFR` writer - Wakeup frame received
pub type WFR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GU` reader - Global unicast
pub type GU_R = crate::BitReader;
///Field `GU` writer - Global unicast
pub type GU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WFFRPR` reader - Wakeup frame filter register pointer reset
pub type WFFRPR_R = crate::BitReader;
///Field `WFFRPR` writer - Wakeup frame filter register pointer reset
pub type WFFRPR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Power down
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Magic Packet enable
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup frame enable
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - Magic packet received
    #[inline(always)]
    pub fn mpr(&self) -> MPR_R {
        MPR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Wakeup frame received
    #[inline(always)]
    pub fn wfr(&self) -> WFR_R {
        WFR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - Global unicast
    #[inline(always)]
    pub fn gu(&self) -> GU_R {
        GU_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 31 - Wakeup frame filter register pointer reset
    #[inline(always)]
    pub fn wffrpr(&self) -> WFFRPR_R {
        WFFRPR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPMTCSR")
            .field("pd", &self.pd())
            .field("mpe", &self.mpe())
            .field("wfe", &self.wfe())
            .field("mpr", &self.mpr())
            .field("wfr", &self.wfr())
            .field("gu", &self.gu())
            .field("wffrpr", &self.wffrpr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Power down
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W<'_, MACPMTCSRrs> {
        PD_W::new(self, 0)
    }
    ///Bit 1 - Magic Packet enable
    #[inline(always)]
    pub fn mpe(&mut self) -> MPE_W<'_, MACPMTCSRrs> {
        MPE_W::new(self, 1)
    }
    ///Bit 2 - Wakeup frame enable
    #[inline(always)]
    pub fn wfe(&mut self) -> WFE_W<'_, MACPMTCSRrs> {
        WFE_W::new(self, 2)
    }
    ///Bit 5 - Magic packet received
    #[inline(always)]
    pub fn mpr(&mut self) -> MPR_W<'_, MACPMTCSRrs> {
        MPR_W::new(self, 5)
    }
    ///Bit 6 - Wakeup frame received
    #[inline(always)]
    pub fn wfr(&mut self) -> WFR_W<'_, MACPMTCSRrs> {
        WFR_W::new(self, 6)
    }
    ///Bit 9 - Global unicast
    #[inline(always)]
    pub fn gu(&mut self) -> GU_W<'_, MACPMTCSRrs> {
        GU_W::new(self, 9)
    }
    ///Bit 31 - Wakeup frame filter register pointer reset
    #[inline(always)]
    pub fn wffrpr(&mut self) -> WFFRPR_W<'_, MACPMTCSRrs> {
        WFFRPR_W::new(self, 31)
    }
}
/**Ethernet MAC PMT control and status register (ETH_MACPMTCSR)

You can [`read`](crate::Reg::read) this register and get [`macpmtcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpmtcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#Ethernet_MAC:MACPMTCSR)*/
pub struct MACPMTCSRrs;
impl crate::RegisterSpec for MACPMTCSRrs {
    type Ux = u32;
}
///`read()` method returns [`macpmtcsr::R`](R) reader structure
impl crate::Readable for MACPMTCSRrs {}
///`write(|w| ..)` method takes [`macpmtcsr::W`](W) writer structure
impl crate::Writable for MACPMTCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPMTCSR to value 0
impl crate::Resettable for MACPMTCSRrs {}
