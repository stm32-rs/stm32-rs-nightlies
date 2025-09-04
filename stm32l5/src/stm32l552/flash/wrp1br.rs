///Register `WRP1BR` reader
pub type R = crate::R<WRP1BRrs>;
///Register `WRP1BR` writer
pub type W = crate::W<WRP1BRrs>;
///Field `WRP1B_PSTRT` reader - WRP1B_PSTRT
pub type WRP1B_PSTRT_R = crate::FieldReader;
///Field `WRP1B_PSTRT` writer - WRP1B_PSTRT
pub type WRP1B_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `WRP1B_PEND` reader - WRP1B_PEND
pub type WRP1B_PEND_R = crate::FieldReader;
///Field `WRP1B_PEND` writer - WRP1B_PEND
pub type WRP1B_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - WRP1B_PSTRT
    #[inline(always)]
    pub fn wrp1b_pstrt(&self) -> WRP1B_PSTRT_R {
        WRP1B_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - WRP1B_PEND
    #[inline(always)]
    pub fn wrp1b_pend(&self) -> WRP1B_PEND_R {
        WRP1B_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRP1BR")
            .field("wrp1b_pstrt", &self.wrp1b_pstrt())
            .field("wrp1b_pend", &self.wrp1b_pend())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - WRP1B_PSTRT
    #[inline(always)]
    pub fn wrp1b_pstrt(&mut self) -> WRP1B_PSTRT_W<WRP1BRrs> {
        WRP1B_PSTRT_W::new(self, 0)
    }
    ///Bits 16:22 - WRP1B_PEND
    #[inline(always)]
    pub fn wrp1b_pend(&mut self) -> WRP1B_PEND_W<WRP1BRrs> {
        WRP1B_PEND_W::new(self, 16)
    }
}
/**Flash Bank 1 WRP area B address register

You can [`read`](crate::Reg::read) this register and get [`wrp1br::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp1br::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:WRP1BR)*/
pub struct WRP1BRrs;
impl crate::RegisterSpec for WRP1BRrs {
    type Ux = u32;
}
///`read()` method returns [`wrp1br::R`](R) reader structure
impl crate::Readable for WRP1BRrs {}
///`write(|w| ..)` method takes [`wrp1br::W`](W) writer structure
impl crate::Writable for WRP1BRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRP1BR to value 0xff00_ff00
impl crate::Resettable for WRP1BRrs {
    const RESET_VALUE: u32 = 0xff00_ff00;
}
