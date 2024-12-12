///Register `EGR` writer
pub type W = crate::W<EGRrs>;
///Field `UG` writer - Update generation This bit can be set by software, it is automatically cleared by hardware.
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1G` writer - Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.
pub type CC1G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2G` writer - Capture/compare 2 generation Refer to CC1G description
pub type CC2G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3G` writer - Capture/compare 3 generation Refer to CC1G description
pub type CC3G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4G` writer - Capture/compare 4 generation Refer to CC1G description
pub type CC4G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMG` writer - Capture/compare control update generation This bit can be set by software, it is automatically cleared by hardware Note: This bit acts only on channels having a complementary output.
pub type COMG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TG` writer - Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
pub type TG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BG` writer - Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
pub type BG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B2G` writer - Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
pub type B2G_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<EGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Update generation This bit can be set by software, it is automatically cleared by hardware.
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<EGRrs> {
        UG_W::new(self, 0)
    }
    ///Bit 1 - Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W<EGRrs> {
        CC1G_W::new(self, 1)
    }
    ///Bit 2 - Capture/compare 2 generation Refer to CC1G description
    #[inline(always)]
    pub fn cc2g(&mut self) -> CC2G_W<EGRrs> {
        CC2G_W::new(self, 2)
    }
    ///Bit 3 - Capture/compare 3 generation Refer to CC1G description
    #[inline(always)]
    pub fn cc3g(&mut self) -> CC3G_W<EGRrs> {
        CC3G_W::new(self, 3)
    }
    ///Bit 4 - Capture/compare 4 generation Refer to CC1G description
    #[inline(always)]
    pub fn cc4g(&mut self) -> CC4G_W<EGRrs> {
        CC4G_W::new(self, 4)
    }
    ///Bit 5 - Capture/compare control update generation This bit can be set by software, it is automatically cleared by hardware Note: This bit acts only on channels having a complementary output.
    #[inline(always)]
    pub fn comg(&mut self) -> COMG_W<EGRrs> {
        COMG_W::new(self, 5)
    }
    ///Bit 6 - Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W<EGRrs> {
        TG_W::new(self, 6)
    }
    ///Bit 7 - Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
    #[inline(always)]
    pub fn bg(&mut self) -> BG_W<EGRrs> {
        BG_W::new(self, 7)
    }
    ///Bit 8 - Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
    #[inline(always)]
    pub fn b2g(&mut self) -> B2G_W<EGRrs> {
        B2G_W::new(self, 8)
    }
}
/**TIM1 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#TIM1:EGR)*/
pub struct EGRrs;
impl crate::RegisterSpec for EGRrs {
    type Ux = u16;
}
///`write(|w| ..)` method takes [`egr::W`](W) writer structure
impl crate::Writable for EGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets EGR to value 0
impl crate::Resettable for EGRrs {
    const RESET_VALUE: u16 = 0;
}
