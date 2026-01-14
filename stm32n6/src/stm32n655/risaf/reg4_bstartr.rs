///Register `REG4_BSTARTR` reader
pub type R = crate::R<REG4_BSTARTRrs>;
///Register `REG4_BSTARTR` writer
pub type W = crate::W<REG4_BSTARTRrs>;
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
        f.debug_struct("REG4_BSTARTR")
            .field("saddstart", &self.saddstart())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - subregion address start
    #[inline(always)]
    pub fn saddstart(&mut self) -> SADDSTART_W<'_, REG4_BSTARTRrs> {
        SADDSTART_W::new(self, 0)
    }
}
/**RISAF region 4 subregion B start-address register

You can [`read`](crate::Reg::read) this register and get [`reg4_bstartr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg4_bstartr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RISAF:REG4_BSTARTR)*/
pub struct REG4_BSTARTRrs;
impl crate::RegisterSpec for REG4_BSTARTRrs {
    type Ux = u32;
}
///`read()` method returns [`reg4_bstartr::R`](R) reader structure
impl crate::Readable for REG4_BSTARTRrs {}
///`write(|w| ..)` method takes [`reg4_bstartr::W`](W) writer structure
impl crate::Writable for REG4_BSTARTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REG4_BSTARTR to value 0
impl crate::Resettable for REG4_BSTARTRrs {}
