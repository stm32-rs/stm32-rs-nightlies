///Register `WPSN_PRG` reader
pub type R = crate::R<WPSN_PRGrs>;
///Register `WPSN_PRG` writer
pub type W = crate::W<WPSN_PRGrs>;
///Field `WRPSn` reader - sector write protection configuration byte
pub type WRPSN_R = crate::FieldReader;
///Field `WRPSn` writer - sector write protection configuration byte
pub type WRPSN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - sector write protection configuration byte
    #[inline(always)]
    pub fn wrpsn(&self) -> WRPSN_R {
        WRPSN_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPSN_PRG")
            .field("wrpsn", &self.wrpsn())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - sector write protection configuration byte
    #[inline(always)]
    pub fn wrpsn(&mut self) -> WRPSN_W<'_, WPSN_PRGrs> {
        WRPSN_W::new(self, 0)
    }
}
/**FLASH write sector protection for bank 1

You can [`read`](crate::Reg::read) this register and get [`wpsn_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpsn_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:WPSN_PRG)*/
pub struct WPSN_PRGrs;
impl crate::RegisterSpec for WPSN_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`wpsn_prg::R`](R) reader structure
impl crate::Readable for WPSN_PRGrs {}
///`write(|w| ..)` method takes [`wpsn_prg::W`](W) writer structure
impl crate::Writable for WPSN_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WPSN_PRG to value 0
impl crate::Resettable for WPSN_PRGrs {}
