///Register `REG14_STARTR` reader
pub type R = crate::R<REG14_STARTRrs>;
///Register `REG14_STARTR` writer
pub type W = crate::W<REG14_STARTRrs>;
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
        f.debug_struct("REG14_STARTR")
            .field("baddstart", &self.baddstart())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Base region address start
    #[inline(always)]
    pub fn baddstart(&mut self) -> BADDSTART_W<'_, REG14_STARTRrs> {
        BADDSTART_W::new(self, 0)
    }
}
/**RISAF region 14 start-address register

You can [`read`](crate::Reg::read) this register and get [`reg14_startr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg14_startr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG14_STARTR)*/
pub struct REG14_STARTRrs;
impl crate::RegisterSpec for REG14_STARTRrs {
    type Ux = u32;
}
///`read()` method returns [`reg14_startr::R`](R) reader structure
impl crate::Readable for REG14_STARTRrs {}
///`write(|w| ..)` method takes [`reg14_startr::W`](W) writer structure
impl crate::Writable for REG14_STARTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REG14_STARTR to value 0
impl crate::Resettable for REG14_STARTRrs {}
