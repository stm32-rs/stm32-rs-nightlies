///Register `DSI_TCCR2` reader
pub type R = crate::R<DSI_TCCR2rs>;
///Register `DSI_TCCR2` writer
pub type W = crate::W<DSI_TCCR2rs>;
///Field `LPRD_TOCNT` reader - Low-power read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
pub type LPRD_TOCNT_R = crate::FieldReader<u16>;
///Field `LPRD_TOCNT` writer - Low-power read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
pub type LPRD_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Low-power read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
    #[inline(always)]
    pub fn lprd_tocnt(&self) -> LPRD_TOCNT_R {
        LPRD_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_TCCR2")
            .field("lprd_tocnt", &self.lprd_tocnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Low-power read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
    #[inline(always)]
    #[must_use]
    pub fn lprd_tocnt(&mut self) -> LPRD_TOCNT_W<DSI_TCCR2rs> {
        LPRD_TOCNT_W::new(self, 0)
    }
}
/**DSI Host timeout counter configuration register 2

You can [`read`](crate::Reg::read) this register and get [`dsi_tccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_tccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:DSI_TCCR2)*/
pub struct DSI_TCCR2rs;
impl crate::RegisterSpec for DSI_TCCR2rs {
    type Ux = u32;
}
///`read()` method returns [`dsi_tccr2::R`](R) reader structure
impl crate::Readable for DSI_TCCR2rs {}
///`write(|w| ..)` method takes [`dsi_tccr2::W`](W) writer structure
impl crate::Writable for DSI_TCCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_TCCR2 to value 0
impl crate::Resettable for DSI_TCCR2rs {
    const RESET_VALUE: u32 = 0;
}
