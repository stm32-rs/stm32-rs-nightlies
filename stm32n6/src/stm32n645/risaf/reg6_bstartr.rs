///Register `REG6_BSTARTR` reader
pub type R = crate::R<REG6_BSTARTRrs>;
///Register `REG6_BSTARTR` writer
pub type W = crate::W<REG6_BSTARTRrs>;
///Field `SADDSTART` reader - subregion address start
pub type SADDSTART_R = crate::FieldReader<u32>;
///Field `SADDSTART` writer - subregion address start
pub type SADDSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - subregion address start
    #[inline(always)]
    pub fn saddstart(&self) -> SADDSTART_R {
        SADDSTART_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG6_BSTARTR")
            .field("saddstart", &self.saddstart())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - subregion address start
    #[inline(always)]
    pub fn saddstart(&mut self) -> SADDSTART_W<REG6_BSTARTRrs> {
        SADDSTART_W::new(self, 0)
    }
}
/**RISAF region 6 subregion B start-address register

You can [`read`](crate::Reg::read) this register and get [`reg6_bstartr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg6_bstartr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RISAF:REG6_BSTARTR)*/
pub struct REG6_BSTARTRrs;
impl crate::RegisterSpec for REG6_BSTARTRrs {
    type Ux = u32;
}
///`read()` method returns [`reg6_bstartr::R`](R) reader structure
impl crate::Readable for REG6_BSTARTRrs {}
///`write(|w| ..)` method takes [`reg6_bstartr::W`](W) writer structure
impl crate::Writable for REG6_BSTARTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REG6_BSTARTR to value 0
impl crate::Resettable for REG6_BSTARTRrs {}
