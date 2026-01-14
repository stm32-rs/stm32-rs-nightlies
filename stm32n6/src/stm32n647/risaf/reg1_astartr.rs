///Register `REG1_ASTARTR` reader
pub type R = crate::R<REG1_ASTARTRrs>;
///Register `REG1_ASTARTR` writer
pub type W = crate::W<REG1_ASTARTRrs>;
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
        f.debug_struct("REG1_ASTARTR")
            .field("saddstart", &self.saddstart())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - subregion address start
    #[inline(always)]
    pub fn saddstart(&mut self) -> SADDSTART_W<'_, REG1_ASTARTRrs> {
        SADDSTART_W::new(self, 0)
    }
}
/**RISAF region 1 subregion A start-address register

You can [`read`](crate::Reg::read) this register and get [`reg1_astartr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1_astartr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG1_ASTARTR)*/
pub struct REG1_ASTARTRrs;
impl crate::RegisterSpec for REG1_ASTARTRrs {
    type Ux = u32;
}
///`read()` method returns [`reg1_astartr::R`](R) reader structure
impl crate::Readable for REG1_ASTARTRrs {}
///`write(|w| ..)` method takes [`reg1_astartr::W`](W) writer structure
impl crate::Writable for REG1_ASTARTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REG1_ASTARTR to value 0
impl crate::Resettable for REG1_ASTARTRrs {}
