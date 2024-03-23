#[doc = "Register `DSI_TCCR3` reader"]
pub type R = crate::R<DSI_TCCR3rs>;
#[doc = "Register `DSI_TCCR3` writer"]
pub type W = crate::W<DSI_TCCR3rs>;
#[doc = "Field `HSWR_TOCNT` reader - High-speed write timeout counter This field sets a period for which the DSI Host keeps the link inactive after sending a high-speed write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
pub type HSWR_TOCNT_R = crate::FieldReader<u16>;
#[doc = "Field `HSWR_TOCNT` writer - High-speed write timeout counter This field sets a period for which the DSI Host keeps the link inactive after sending a high-speed write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
pub type HSWR_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PM` reader - Presp mode When set to 1, this bit ensures that the peripheral response timeout caused by HSWR_TOCNT is used only once per LTDC frame in command mode, when both the following conditions are met: dpivsync_edpiwms has risen and fallen. Packets originated from LTDC in command mode have been transmitted and its FIFO is empty again. In this scenario no non-LTDC command requests are sent to the D-PHY, even if there is traffic from generic interface ready to be sent, making it return to stop state. When it does so, PRESP_TO counter is activated and only when it finishes does the controller send any other traffic that is ready."]
pub type PM_R = crate::BitReader;
#[doc = "Field `PM` writer - Presp mode When set to 1, this bit ensures that the peripheral response timeout caused by HSWR_TOCNT is used only once per LTDC frame in command mode, when both the following conditions are met: dpivsync_edpiwms has risen and fallen. Packets originated from LTDC in command mode have been transmitted and its FIFO is empty again. In this scenario no non-LTDC command requests are sent to the D-PHY, even if there is traffic from generic interface ready to be sent, making it return to stop state. When it does so, PRESP_TO counter is activated and only when it finishes does the controller send any other traffic that is ready."]
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - High-speed write timeout counter This field sets a period for which the DSI Host keeps the link inactive after sending a high-speed write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
    #[inline(always)]
    pub fn hswr_tocnt(&self) -> HSWR_TOCNT_R {
        HSWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - Presp mode When set to 1, this bit ensures that the peripheral response timeout caused by HSWR_TOCNT is used only once per LTDC frame in command mode, when both the following conditions are met: dpivsync_edpiwms has risen and fallen. Packets originated from LTDC in command mode have been transmitted and its FIFO is empty again. In this scenario no non-LTDC command requests are sent to the D-PHY, even if there is traffic from generic interface ready to be sent, making it return to stop state. When it does so, PRESP_TO counter is activated and only when it finishes does the controller send any other traffic that is ready."]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - High-speed write timeout counter This field sets a period for which the DSI Host keeps the link inactive after sending a high-speed write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn hswr_tocnt(&mut self) -> HSWR_TOCNT_W<DSI_TCCR3rs> {
        HSWR_TOCNT_W::new(self, 0)
    }
    #[doc = "Bit 24 - Presp mode When set to 1, this bit ensures that the peripheral response timeout caused by HSWR_TOCNT is used only once per LTDC frame in command mode, when both the following conditions are met: dpivsync_edpiwms has risen and fallen. Packets originated from LTDC in command mode have been transmitted and its FIFO is empty again. In this scenario no non-LTDC command requests are sent to the D-PHY, even if there is traffic from generic interface ready to be sent, making it return to stop state. When it does so, PRESP_TO counter is activated and only when it finishes does the controller send any other traffic that is ready."]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<DSI_TCCR3rs> {
        PM_W::new(self, 24)
    }
}
#[doc = "DSI Host timeout counter configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_tccr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_tccr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_TCCR3rs;
impl crate::RegisterSpec for DSI_TCCR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_tccr3::R`](R) reader structure"]
impl crate::Readable for DSI_TCCR3rs {}
#[doc = "`write(|w| ..)` method takes [`dsi_tccr3::W`](W) writer structure"]
impl crate::Writable for DSI_TCCR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_TCCR3 to value 0"]
impl crate::Resettable for DSI_TCCR3rs {
    const RESET_VALUE: u32 = 0;
}
