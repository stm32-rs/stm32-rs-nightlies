///Register `WRP2BR` reader
pub type R = crate::R<WRP2BRrs>;
///Register `WRP2BR` writer
pub type W = crate::W<WRP2BRrs>;
///Field `WRP2B_STRT` reader - WRP second area B start offset
pub type WRP2B_STRT_R = crate::FieldReader;
///Field `WRP2B_STRT` writer - WRP second area B start offset
pub type WRP2B_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `WRP2B_END` reader - WRP second area B end offset
pub type WRP2B_END_R = crate::FieldReader;
///Field `WRP2B_END` writer - WRP second area B end offset
pub type WRP2B_END_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - WRP second area B start offset
    #[inline(always)]
    pub fn wrp2b_strt(&self) -> WRP2B_STRT_R {
        WRP2B_STRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - WRP second area B end offset
    #[inline(always)]
    pub fn wrp2b_end(&self) -> WRP2B_END_R {
        WRP2B_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRP2BR")
            .field("wrp2b_strt", &self.wrp2b_strt())
            .field("wrp2b_end", &self.wrp2b_end())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - WRP second area B start offset
    #[inline(always)]
    pub fn wrp2b_strt(&mut self) -> WRP2B_STRT_W<'_, WRP2BRrs> {
        WRP2B_STRT_W::new(self, 0)
    }
    ///Bits 16:22 - WRP second area B end offset
    #[inline(always)]
    pub fn wrp2b_end(&mut self) -> WRP2B_END_W<'_, WRP2BRrs> {
        WRP2B_END_W::new(self, 16)
    }
}
/**Flash Bank 2 WRP area B address register

You can [`read`](crate::Reg::read) this register and get [`wrp2br::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2br::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#FLASH:WRP2BR)*/
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
///`reset()` method sets WRP2BR to value 0
impl crate::Resettable for WRP2BRrs {}
