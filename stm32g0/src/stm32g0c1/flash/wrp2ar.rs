///Register `WRP2AR` reader
pub type R = crate::R<WRP2ARrs>;
///Register `WRP2AR` writer
pub type W = crate::W<WRP2ARrs>;
///Field `WRP2A_STRT` reader - WRP area A start offset, Bank 2
pub type WRP2A_STRT_R = crate::FieldReader;
///Field `WRP2A_STRT` writer - WRP area A start offset, Bank 2
pub type WRP2A_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `WRP2A_END` reader - WRP area A end offset, Bank 2
pub type WRP2A_END_R = crate::FieldReader;
///Field `WRP2A_END` writer - WRP area A end offset, Bank 2
pub type WRP2A_END_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - WRP area A start offset, Bank 2
    #[inline(always)]
    pub fn wrp2a_strt(&self) -> WRP2A_STRT_R {
        WRP2A_STRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - WRP area A end offset, Bank 2
    #[inline(always)]
    pub fn wrp2a_end(&self) -> WRP2A_END_R {
        WRP2A_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRP2AR")
            .field("wrp2a_strt", &self.wrp2a_strt())
            .field("wrp2a_end", &self.wrp2a_end())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - WRP area A start offset, Bank 2
    #[inline(always)]
    pub fn wrp2a_strt(&mut self) -> WRP2A_STRT_W<'_, WRP2ARrs> {
        WRP2A_STRT_W::new(self, 0)
    }
    ///Bits 16:22 - WRP area A end offset, Bank 2
    #[inline(always)]
    pub fn wrp2a_end(&mut self) -> WRP2A_END_W<'_, WRP2ARrs> {
        WRP2A_END_W::new(self, 16)
    }
}
/**Flash WRP2 area A address register

You can [`read`](crate::Reg::read) this register and get [`wrp2ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#FLASH:WRP2AR)*/
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
///`reset()` method sets WRP2AR to value 0xff
impl crate::Resettable for WRP2ARrs {
    const RESET_VALUE: u32 = 0xff;
}
