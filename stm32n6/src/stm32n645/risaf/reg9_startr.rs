///Register `REG9_STARTR` reader
pub type R = crate::R<REG9_STARTRrs>;
///Register `REG9_STARTR` writer
pub type W = crate::W<REG9_STARTRrs>;
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
        f.debug_struct("REG9_STARTR")
            .field("baddstart", &self.baddstart())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Base region address start
    #[inline(always)]
    pub fn baddstart(&mut self) -> BADDSTART_W<'_, REG9_STARTRrs> {
        BADDSTART_W::new(self, 0)
    }
}
/**RISAF region 9 start-address register

You can [`read`](crate::Reg::read) this register and get [`reg9_startr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg9_startr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RISAF:REG9_STARTR)*/
pub struct REG9_STARTRrs;
impl crate::RegisterSpec for REG9_STARTRrs {
    type Ux = u32;
}
///`read()` method returns [`reg9_startr::R`](R) reader structure
impl crate::Readable for REG9_STARTRrs {}
///`write(|w| ..)` method takes [`reg9_startr::W`](W) writer structure
impl crate::Writable for REG9_STARTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REG9_STARTR to value 0
impl crate::Resettable for REG9_STARTRrs {}
