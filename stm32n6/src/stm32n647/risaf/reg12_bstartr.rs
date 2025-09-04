///Register `REG12_BSTARTR` reader
pub type R = crate::R<REG12_BSTARTRrs>;
///Register `REG12_BSTARTR` writer
pub type W = crate::W<REG12_BSTARTRrs>;
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
        f.debug_struct("REG12_BSTARTR")
            .field("saddstart", &self.saddstart())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - subregion address start
    #[inline(always)]
    pub fn saddstart(&mut self) -> SADDSTART_W<REG12_BSTARTRrs> {
        SADDSTART_W::new(self, 0)
    }
}
/**RISAF region 12 subregion B start-address register

You can [`read`](crate::Reg::read) this register and get [`reg12_bstartr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg12_bstartr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG12_BSTARTR)*/
pub struct REG12_BSTARTRrs;
impl crate::RegisterSpec for REG12_BSTARTRrs {
    type Ux = u32;
}
///`read()` method returns [`reg12_bstartr::R`](R) reader structure
impl crate::Readable for REG12_BSTARTRrs {}
///`write(|w| ..)` method takes [`reg12_bstartr::W`](W) writer structure
impl crate::Writable for REG12_BSTARTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REG12_BSTARTR to value 0
impl crate::Resettable for REG12_BSTARTRrs {}
