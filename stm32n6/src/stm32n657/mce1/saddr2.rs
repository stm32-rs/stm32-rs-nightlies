///Register `SADDR2` reader
pub type R = crate::R<SADDR2rs>;
///Register `SADDR2` writer
pub type W = crate::W<SADDR2rs>;
///Field `BADDSTART` reader - Region address start
pub type BADDSTART_R = crate::FieldReader<u32>;
///Field `BADDSTART` writer - Region address start
pub type BADDSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 12:31 - Region address start
    #[inline(always)]
    pub fn baddstart(&self) -> BADDSTART_R {
        BADDSTART_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SADDR2")
            .field("baddstart", &self.baddstart())
            .finish()
    }
}
impl W {
    ///Bits 12:31 - Region address start
    #[inline(always)]
    pub fn baddstart(&mut self) -> BADDSTART_W<'_, SADDR2rs> {
        BADDSTART_W::new(self, 12)
    }
}
/**MCE start address for region 2 register

You can [`read`](crate::Reg::read) this register and get [`saddr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MCE1:SADDR2)*/
pub struct SADDR2rs;
impl crate::RegisterSpec for SADDR2rs {
    type Ux = u32;
}
///`read()` method returns [`saddr2::R`](R) reader structure
impl crate::Readable for SADDR2rs {}
///`write(|w| ..)` method takes [`saddr2::W`](W) writer structure
impl crate::Writable for SADDR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SADDR2 to value 0
impl crate::Resettable for SADDR2rs {}
