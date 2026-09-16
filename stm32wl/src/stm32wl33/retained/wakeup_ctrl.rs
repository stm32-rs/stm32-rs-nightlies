///Register `WAKEUP_CTRL` reader
pub type R = crate::R<WAKEUP_CTRLrs>;
///Register `WAKEUP_CTRL` writer
pub type W = crate::W<WAKEUP_CTRLrs>;
///Field `SOC_WAKEUP_OFFSET` reader - Delay to be considered by the Wakeup block to anticipate the wakeup request to the PWRC of the SoC versus the target to wakeup the RFIP (or the CPU).
pub type SOC_WAKEUP_OFFSET_R = crate::FieldReader;
///Field `SOC_WAKEUP_OFFSET` writer - Delay to be considered by the Wakeup block to anticipate the wakeup request to the PWRC of the SoC versus the target to wakeup the RFIP (or the CPU).
pub type SOC_WAKEUP_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CPU_WAKEUP_EN` reader - Indicates if the wakeup timer has to wakeup the SoC (match on CPU_WAKEUPTIME\[31:4\] bit field only) + set the CPU_WAKEUP_F in the WAKEUP_IRQ_STATUS Misc register when match on CPU_WAKEUPTIME\[31:0\] occurs.
pub type CPU_WAKEUP_EN_R = crate::BitReader;
///Field `CPU_WAKEUP_EN` writer - Indicates if the wakeup timer has to wakeup the SoC (match on CPU_WAKEUPTIME\[31:4\] bit field only) + set the CPU_WAKEUP_F in the WAKEUP_IRQ_STATUS Misc register when match on CPU_WAKEUPTIME\[31:0\] occurs.
pub type CPU_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFIP_WAKEUP_EN` reader - Indicates if the wakeup timer has to wakeup the SoC (match on RFIP_WAKEUPTIME\[31:4\] bit field only) + trigger an event on the Sequencer and set the RFIP_WAKEUP_F in the WAKEUP_IRQ_STATUS Misc register when match on RFIP_WAKEUPTIME\[31:0\] occurs.
pub type RFIP_WAKEUP_EN_R = crate::BitReader;
impl R {
    ///Bits 0:7 - Delay to be considered by the Wakeup block to anticipate the wakeup request to the PWRC of the SoC versus the target to wakeup the RFIP (or the CPU).
    #[inline(always)]
    pub fn soc_wakeup_offset(&self) -> SOC_WAKEUP_OFFSET_R {
        SOC_WAKEUP_OFFSET_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 30 - Indicates if the wakeup timer has to wakeup the SoC (match on CPU_WAKEUPTIME\[31:4\] bit field only) + set the CPU_WAKEUP_F in the WAKEUP_IRQ_STATUS Misc register when match on CPU_WAKEUPTIME\[31:0\] occurs.
    #[inline(always)]
    pub fn cpu_wakeup_en(&self) -> CPU_WAKEUP_EN_R {
        CPU_WAKEUP_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Indicates if the wakeup timer has to wakeup the SoC (match on RFIP_WAKEUPTIME\[31:4\] bit field only) + trigger an event on the Sequencer and set the RFIP_WAKEUP_F in the WAKEUP_IRQ_STATUS Misc register when match on RFIP_WAKEUPTIME\[31:0\] occurs.
    #[inline(always)]
    pub fn rfip_wakeup_en(&self) -> RFIP_WAKEUP_EN_R {
        RFIP_WAKEUP_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAKEUP_CTRL")
            .field("soc_wakeup_offset", &self.soc_wakeup_offset())
            .field("cpu_wakeup_en", &self.cpu_wakeup_en())
            .field("rfip_wakeup_en", &self.rfip_wakeup_en())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Delay to be considered by the Wakeup block to anticipate the wakeup request to the PWRC of the SoC versus the target to wakeup the RFIP (or the CPU).
    #[inline(always)]
    pub fn soc_wakeup_offset(&mut self) -> SOC_WAKEUP_OFFSET_W<'_, WAKEUP_CTRLrs> {
        SOC_WAKEUP_OFFSET_W::new(self, 0)
    }
    ///Bit 30 - Indicates if the wakeup timer has to wakeup the SoC (match on CPU_WAKEUPTIME\[31:4\] bit field only) + set the CPU_WAKEUP_F in the WAKEUP_IRQ_STATUS Misc register when match on CPU_WAKEUPTIME\[31:0\] occurs.
    #[inline(always)]
    pub fn cpu_wakeup_en(&mut self) -> CPU_WAKEUP_EN_W<'_, WAKEUP_CTRLrs> {
        CPU_WAKEUP_EN_W::new(self, 30)
    }
}
/**WAKEUP_CTRL register

You can [`read`](crate::Reg::read) this register and get [`wakeup_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RETAINED:WAKEUP_CTRL)*/
pub struct WAKEUP_CTRLrs;
impl crate::RegisterSpec for WAKEUP_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`wakeup_ctrl::R`](R) reader structure
impl crate::Readable for WAKEUP_CTRLrs {}
///`write(|w| ..)` method takes [`wakeup_ctrl::W`](W) writer structure
impl crate::Writable for WAKEUP_CTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WAKEUP_CTRL to value 0
impl crate::Resettable for WAKEUP_CTRLrs {}
