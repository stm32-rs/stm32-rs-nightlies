///Register `WRP2AR` reader
pub type R = crate::R<WRP2ARrs>;
///Register `WRP2AR` writer
pub type W = crate::W<WRP2ARrs>;
///Field `WRP2A_PSTRT` reader - WRP2A_PSTRT
pub type WRP2A_PSTRT_R = crate::FieldReader;
///Field `WRP2A_PSTRT` writer - WRP2A_PSTRT
pub type WRP2A_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `WRP2A_PEND` reader - WRP2A_PEND
pub type WRP2A_PEND_R = crate::FieldReader;
///Field `WRP2A_PEND` writer - WRP2A_PEND
pub type WRP2A_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - WRP2A_PSTRT
    #[inline(always)]
    pub fn wrp2a_pstrt(&self) -> WRP2A_PSTRT_R {
        WRP2A_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - WRP2A_PEND
    #[inline(always)]
    pub fn wrp2a_pend(&self) -> WRP2A_PEND_R {
        WRP2A_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRP2AR")
            .field("wrp2a_pstrt", &self.wrp2a_pstrt())
            .field("wrp2a_pend", &self.wrp2a_pend())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - WRP2A_PSTRT
    #[inline(always)]
    pub fn wrp2a_pstrt(&mut self) -> WRP2A_PSTRT_W<'_, WRP2ARrs> {
        WRP2A_PSTRT_W::new(self, 0)
    }
    ///Bits 16:22 - WRP2A_PEND
    #[inline(always)]
    pub fn wrp2a_pend(&mut self) -> WRP2A_PEND_W<'_, WRP2ARrs> {
        WRP2A_PEND_W::new(self, 16)
    }
}
/**Flash WPR2 area A address register

You can [`read`](crate::Reg::read) this register and get [`wrp2ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#FLASH:WRP2AR)*/
pub struct WRP2ARrs;
impl crate::RegisterSpec for WRP2ARrs {
    type Ux = u32;
}
///`read()` method returns [`wrp2ar::R`](R) reader structure
impl crate::Readable for WRP2ARrs {}
///`write(|w| ..)` method takes [`wrp2ar::W`](W) writer structure
impl crate::Writable for WRP2ARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRP2AR to value 0xff00_ff00
impl crate::Resettable for WRP2ARrs {
    const RESET_VALUE: u32 = 0xff00_ff00;
}
