///Register `WRP2AR` reader
pub type R = crate::R<WRP2ARrs>;
///Register `WRP2AR` writer
pub type W = crate::W<WRP2ARrs>;
///Field `WRP2A_STRT` reader - Bank 2 WRP first area A start offset
pub type WRP2A_STRT_R = crate::FieldReader;
///Field `WRP2A_STRT` writer - Bank 2 WRP first area A start offset
pub type WRP2A_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `WRP2A_END` reader - Bank 2 WRP first area A end offset
pub type WRP2A_END_R = crate::FieldReader;
///Field `WRP2A_END` writer - Bank 2 WRP first area A end offset
pub type WRP2A_END_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Bank 2 WRP first area A start offset
    #[inline(always)]
    pub fn wrp2a_strt(&self) -> WRP2A_STRT_R {
        WRP2A_STRT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - Bank 2 WRP first area A end offset
    #[inline(always)]
    pub fn wrp2a_end(&self) -> WRP2A_END_R {
        WRP2A_END_R::new(((self.bits >> 16) & 0xff) as u8)
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
    ///Bits 0:7 - Bank 2 WRP first area A start offset
    #[inline(always)]
    pub fn wrp2a_strt(&mut self) -> WRP2A_STRT_W<'_, WRP2ARrs> {
        WRP2A_STRT_W::new(self, 0)
    }
    ///Bits 16:23 - Bank 2 WRP first area A end offset
    #[inline(always)]
    pub fn wrp2a_end(&mut self) -> WRP2A_END_W<'_, WRP2ARrs> {
        WRP2A_END_W::new(self, 16)
    }
}
/**Flash Bank 2 WRP area A address register

You can [`read`](crate::Reg::read) this register and get [`wrp2ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#FLASH:WRP2AR)*/
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
