///Register `UNLOCK012` reader
pub type R = crate::R<UNLOCK012rs>;
///Register `UNLOCK012` writer
pub type W = crate::W<UNLOCK012rs>;
///Field `UNLOCK012` reader - (NOT TO BE DOCUMENTED) Remove read-write protection from IFR0, IFR1, IFR2 sectors
pub type UNLOCK012_R = crate::FieldReader<u32>;
///Field `UNLOCK012` writer - (NOT TO BE DOCUMENTED) Remove read-write protection from IFR0, IFR1, IFR2 sectors
pub type UNLOCK012_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - (NOT TO BE DOCUMENTED) Remove read-write protection from IFR0, IFR1, IFR2 sectors
    #[inline(always)]
    pub fn unlock012(&self) -> UNLOCK012_R {
        UNLOCK012_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UNLOCK012")
            .field("unlock012", &self.unlock012())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - (NOT TO BE DOCUMENTED) Remove read-write protection from IFR0, IFR1, IFR2 sectors
    #[inline(always)]
    pub fn unlock012(&mut self) -> UNLOCK012_W<'_, UNLOCK012rs> {
        UNLOCK012_W::new(self, 0)
    }
}
/**UNLOCK012 register

You can [`read`](crate::Reg::read) this register and get [`unlock012::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unlock012::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#FLASH_CTRL:UNLOCK012)*/
pub struct UNLOCK012rs;
impl crate::RegisterSpec for UNLOCK012rs {
    type Ux = u32;
}
///`read()` method returns [`unlock012::R`](R) reader structure
impl crate::Readable for UNLOCK012rs {}
///`write(|w| ..)` method takes [`unlock012::W`](W) writer structure
impl crate::Writable for UNLOCK012rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UNLOCK012 to value 0xffff_ffff
impl crate::Resettable for UNLOCK012rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
