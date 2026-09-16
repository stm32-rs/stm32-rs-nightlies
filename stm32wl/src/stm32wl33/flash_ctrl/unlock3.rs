///Register `UNLOCK3` reader
pub type R = crate::R<UNLOCK3rs>;
///Register `UNLOCK3` writer
pub type W = crate::W<UNLOCK3rs>;
///Field `UNLOCK3` reader - (NOT TO BE DOCUMENTED) Remove read-write protection from IFR3 sector
pub type UNLOCK3_R = crate::FieldReader<u32>;
///Field `UNLOCK3` writer - (NOT TO BE DOCUMENTED) Remove read-write protection from IFR3 sector
pub type UNLOCK3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - (NOT TO BE DOCUMENTED) Remove read-write protection from IFR3 sector
    #[inline(always)]
    pub fn unlock3(&self) -> UNLOCK3_R {
        UNLOCK3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UNLOCK3")
            .field("unlock3", &self.unlock3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - (NOT TO BE DOCUMENTED) Remove read-write protection from IFR3 sector
    #[inline(always)]
    pub fn unlock3(&mut self) -> UNLOCK3_W<'_, UNLOCK3rs> {
        UNLOCK3_W::new(self, 0)
    }
}
/**UNLOCK3 register

You can [`read`](crate::Reg::read) this register and get [`unlock3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unlock3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#FLASH_CTRL:UNLOCK3)*/
pub struct UNLOCK3rs;
impl crate::RegisterSpec for UNLOCK3rs {
    type Ux = u32;
}
///`read()` method returns [`unlock3::R`](R) reader structure
impl crate::Readable for UNLOCK3rs {}
///`write(|w| ..)` method takes [`unlock3::W`](W) writer structure
impl crate::Writable for UNLOCK3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UNLOCK3 to value 0xffff_ffff
impl crate::Resettable for UNLOCK3rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
