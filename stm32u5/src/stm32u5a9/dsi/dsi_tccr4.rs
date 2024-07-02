///Register `DSI_TCCR4` reader
pub type R = crate::R<DSI_TCCR4rs>;
///Register `DSI_TCCR4` writer
pub type W = crate::W<DSI_TCCR4rs>;
///Field `LPWR_TOCNT` reader - Low-power write timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
pub type LPWR_TOCNT_R = crate::FieldReader<u16>;
///Field `LPWR_TOCNT` writer - Low-power write timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
pub type LPWR_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Low-power write timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
    #[inline(always)]
    pub fn lpwr_tocnt(&self) -> LPWR_TOCNT_R {
        LPWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_TCCR4")
            .field("lpwr_tocnt", &self.lpwr_tocnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Low-power write timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts.
    #[inline(always)]
    #[must_use]
    pub fn lpwr_tocnt(&mut self) -> LPWR_TOCNT_W<DSI_TCCR4rs> {
        LPWR_TOCNT_W::new(self, 0)
    }
}
/**DSI Host timeout counter configuration register 4

You can [`read`](crate::Reg::read) this register and get [`dsi_tccr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_tccr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:DSI_TCCR4)*/
pub struct DSI_TCCR4rs;
impl crate::RegisterSpec for DSI_TCCR4rs {
    type Ux = u32;
}
///`read()` method returns [`dsi_tccr4::R`](R) reader structure
impl crate::Readable for DSI_TCCR4rs {}
///`write(|w| ..)` method takes [`dsi_tccr4::W`](W) writer structure
impl crate::Writable for DSI_TCCR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_TCCR4 to value 0
impl crate::Resettable for DSI_TCCR4rs {
    const RESET_VALUE: u32 = 0;
}
