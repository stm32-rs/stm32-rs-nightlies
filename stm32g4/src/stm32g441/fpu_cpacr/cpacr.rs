///Register `CPACR` reader
pub type R = crate::R<CPACRrs>;
///Register `CPACR` writer
pub type W = crate::W<CPACRrs>;
///Field `CP` reader - CP
pub type CP_R = crate::FieldReader;
///Field `CP` writer - CP
pub type CP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 20:23 - CP
    #[inline(always)]
    pub fn cp(&self) -> CP_R {
        CP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPACR").field("cp", &self.cp()).finish()
    }
}
impl W {
    ///Bits 20:23 - CP
    #[inline(always)]
    #[must_use]
    pub fn cp(&mut self) -> CP_W<CPACRrs> {
        CP_W::new(self, 20)
    }
}
/**Coprocessor access control register

You can [`read`](crate::Reg::read) this register and get [`cpacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G441xx.html#FPU_CPACR:CPACR)*/
pub struct CPACRrs;
impl crate::RegisterSpec for CPACRrs {
    type Ux = u32;
}
///`read()` method returns [`cpacr::R`](R) reader structure
impl crate::Readable for CPACRrs {}
///`write(|w| ..)` method takes [`cpacr::W`](W) writer structure
impl crate::Writable for CPACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CPACR to value 0
impl crate::Resettable for CPACRrs {
    const RESET_VALUE: u32 = 0;
}
