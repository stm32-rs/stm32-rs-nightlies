///Register `PCGCCTL` reader
pub type R = crate::R<PCGCCTLrs>;
///Register `PCGCCTL` writer
pub type W = crate::W<PCGCCTLrs>;
///Field `STPPCLK` reader - Stop PHY clock The application sets this bit to stop the PHY clock when the USB is suspended, the session is not valid, or the device is disconnected. The application clears this bit when the USB is resumed or a new session starts.
pub type STPPCLK_R = crate::BitReader;
///Field `STPPCLK` writer - Stop PHY clock The application sets this bit to stop the PHY clock when the USB is suspended, the session is not valid, or the device is disconnected. The application clears this bit when the USB is resumed or a new session starts.
pub type STPPCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GATEHCLK` reader - Gate HCLK The application sets this bit to gate HCLK to modules other than the AHB Slave and Master and wakeup logic when the USB is suspended or the session is not valid. The application clears this bit when the USB is resumed or a new session starts.
pub type GATEHCLK_R = crate::BitReader;
///Field `GATEHCLK` writer - Gate HCLK The application sets this bit to gate HCLK to modules other than the AHB Slave and Master and wakeup logic when the USB is suspended or the session is not valid. The application clears this bit when the USB is resumed or a new session starts.
pub type GATEHCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PHYSUSP` reader - PHY suspended Indicates that the PHY has been suspended. This bit is updated once the PHY is suspended after the application has set the STPPCLK bit.
pub type PHYSUSP_R = crate::BitReader;
///Field `ENL1GTG` reader - Enable sleep clock gating When this bit is set, core internal clock gating is enabled in Sleep state if the core cannot assert utmi_l1_suspend_n. When this bit is not set, the PHY clock is not gated in Sleep state.
pub type ENL1GTG_R = crate::BitReader;
///Field `ENL1GTG` writer - Enable sleep clock gating When this bit is set, core internal clock gating is enabled in Sleep state if the core cannot assert utmi_l1_suspend_n. When this bit is not set, the PHY clock is not gated in Sleep state.
pub type ENL1GTG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PHYSLEEP` reader - PHY in Sleep This bit indicates that the PHY is in the Sleep state.
pub type PHYSLEEP_R = crate::BitReader;
///Field `SUSP` reader - Deep Sleep This bit indicates that the PHY is in Deep Sleep when in L1 state.
pub type SUSP_R = crate::BitReader;
impl R {
    ///Bit 0 - Stop PHY clock The application sets this bit to stop the PHY clock when the USB is suspended, the session is not valid, or the device is disconnected. The application clears this bit when the USB is resumed or a new session starts.
    #[inline(always)]
    pub fn stppclk(&self) -> STPPCLK_R {
        STPPCLK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Gate HCLK The application sets this bit to gate HCLK to modules other than the AHB Slave and Master and wakeup logic when the USB is suspended or the session is not valid. The application clears this bit when the USB is resumed or a new session starts.
    #[inline(always)]
    pub fn gatehclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - PHY suspended Indicates that the PHY has been suspended. This bit is updated once the PHY is suspended after the application has set the STPPCLK bit.
    #[inline(always)]
    pub fn physusp(&self) -> PHYSUSP_R {
        PHYSUSP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Enable sleep clock gating When this bit is set, core internal clock gating is enabled in Sleep state if the core cannot assert utmi_l1_suspend_n. When this bit is not set, the PHY clock is not gated in Sleep state.
    #[inline(always)]
    pub fn enl1gtg(&self) -> ENL1GTG_R {
        ENL1GTG_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PHY in Sleep This bit indicates that the PHY is in the Sleep state.
    #[inline(always)]
    pub fn physleep(&self) -> PHYSLEEP_R {
        PHYSLEEP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Deep Sleep This bit indicates that the PHY is in Deep Sleep when in L1 state.
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCGCCTL")
            .field("stppclk", &self.stppclk())
            .field("gatehclk", &self.gatehclk())
            .field("physusp", &self.physusp())
            .field("enl1gtg", &self.enl1gtg())
            .field("physleep", &self.physleep())
            .field("susp", &self.susp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Stop PHY clock The application sets this bit to stop the PHY clock when the USB is suspended, the session is not valid, or the device is disconnected. The application clears this bit when the USB is resumed or a new session starts.
    #[inline(always)]
    pub fn stppclk(&mut self) -> STPPCLK_W<'_, PCGCCTLrs> {
        STPPCLK_W::new(self, 0)
    }
    ///Bit 1 - Gate HCLK The application sets this bit to gate HCLK to modules other than the AHB Slave and Master and wakeup logic when the USB is suspended or the session is not valid. The application clears this bit when the USB is resumed or a new session starts.
    #[inline(always)]
    pub fn gatehclk(&mut self) -> GATEHCLK_W<'_, PCGCCTLrs> {
        GATEHCLK_W::new(self, 1)
    }
    ///Bit 5 - Enable sleep clock gating When this bit is set, core internal clock gating is enabled in Sleep state if the core cannot assert utmi_l1_suspend_n. When this bit is not set, the PHY clock is not gated in Sleep state.
    #[inline(always)]
    pub fn enl1gtg(&mut self) -> ENL1GTG_W<'_, PCGCCTLrs> {
        ENL1GTG_W::new(self, 5)
    }
}
/**OTG power and clock gating control register

You can [`read`](crate::Reg::read) this register and get [`pcgcctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcgcctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#OTG_HS:PCGCCTL)*/
pub struct PCGCCTLrs;
impl crate::RegisterSpec for PCGCCTLrs {
    type Ux = u32;
}
///`read()` method returns [`pcgcctl::R`](R) reader structure
impl crate::Readable for PCGCCTLrs {}
///`write(|w| ..)` method takes [`pcgcctl::W`](W) writer structure
impl crate::Writable for PCGCCTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCGCCTL to value 0x200b_8000
impl crate::Resettable for PCGCCTLrs {
    const RESET_VALUE: u32 = 0x200b_8000;
}
