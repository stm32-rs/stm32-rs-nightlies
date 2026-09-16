///Register `REG8_STARTR` reader
pub type R = crate::R<REG8_STARTRrs>;
///Register `REG8_STARTR` writer
pub type W = crate::W<REG8_STARTRrs>;
///Field `BADDSTART` reader - Base region address start
pub type BADDSTART_R = crate::FieldReader<u32>;
///Field `BADDSTART` writer - Base region address start
pub type BADDSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Base region address start
    #[inline(always)]
    pub fn baddstart(&self) -> BADDSTART_R {
        BADDSTART_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG8_STARTR")
            .field("baddstart", &self.baddstart())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Base region address start
    #[inline(always)]
    pub fn baddstart(&mut self) -> BADDSTART_W<'_, REG8_STARTRrs> {
        BADDSTART_W::new(self, 0)
    }
}
/**RISAF region 8 start-address register

You can [`read`](crate::Reg::read) this register and get [`reg8_startr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg8_startr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RISAF:REG8_STARTR)*/
pub struct REG8_STARTRrs;
impl crate::RegisterSpec for REG8_STARTRrs {
    type Ux = u32;
}
///`read()` method returns [`reg8_startr::R`](R) reader structure
impl crate::Readable for REG8_STARTRrs {}
///`write(|w| ..)` method takes [`reg8_startr::W`](W) writer structure
impl crate::Writable for REG8_STARTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REG8_STARTR to value 0
impl crate::Resettable for REG8_STARTRrs {}
