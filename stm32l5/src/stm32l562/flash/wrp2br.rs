///Register `WRP2BR` reader
pub type R = crate::R<WRP2BRrs>;
///Register `WRP2BR` writer
pub type W = crate::W<WRP2BRrs>;
///Field `WRP2B_PSTRT` reader - WRP2B_PSTRT
pub type WRP2B_PSTRT_R = crate::FieldReader;
///Field `WRP2B_PSTRT` writer - WRP2B_PSTRT
pub type WRP2B_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `WRP2B_PEND` reader - WRP2B_PEND
pub type WRP2B_PEND_R = crate::FieldReader;
///Field `WRP2B_PEND` writer - WRP2B_PEND
pub type WRP2B_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - WRP2B_PSTRT
    #[inline(always)]
    pub fn wrp2b_pstrt(&self) -> WRP2B_PSTRT_R {
        WRP2B_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - WRP2B_PEND
    #[inline(always)]
    pub fn wrp2b_pend(&self) -> WRP2B_PEND_R {
        WRP2B_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRP2BR")
            .field("wrp2b_pstrt", &self.wrp2b_pstrt())
            .field("wrp2b_pend", &self.wrp2b_pend())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - WRP2B_PSTRT
    #[inline(always)]
    pub fn wrp2b_pstrt(&mut self) -> WRP2B_PSTRT_W<'_, WRP2BRrs> {
        WRP2B_PSTRT_W::new(self, 0)
    }
    ///Bits 16:22 - WRP2B_PEND
    #[inline(always)]
    pub fn wrp2b_pend(&mut self) -> WRP2B_PEND_W<'_, WRP2BRrs> {
        WRP2B_PEND_W::new(self, 16)
    }
}
/**Flash WPR2 area B address register

You can [`read`](crate::Reg::read) this register and get [`wrp2br::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2br::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#FLASH:WRP2BR)*/
pub struct WRP2BRrs;
impl crate::RegisterSpec for WRP2BRrs {
    type Ux = u32;
}
///`read()` method returns [`wrp2br::R`](R) reader structure
impl crate::Readable for WRP2BRrs {}
///`write(|w| ..)` method takes [`wrp2br::W`](W) writer structure
impl crate::Writable for WRP2BRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRP2BR to value 0xff00_ff00
impl crate::Resettable for WRP2BRrs {
    const RESET_VALUE: u32 = 0xff00_ff00;
}
