///Register `SADDR3` reader
pub type R = crate::R<SADDR3rs>;
///Register `SADDR3` writer
pub type W = crate::W<SADDR3rs>;
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
        f.debug_struct("SADDR3")
            .field("baddstart", &self.baddstart())
            .finish()
    }
}
impl W {
    ///Bits 12:31 - Region address start
    #[inline(always)]
    pub fn baddstart(&mut self) -> BADDSTART_W<'_, SADDR3rs> {
        BADDSTART_W::new(self, 12)
    }
}
/**MCE start address for region 3 register

You can [`read`](crate::Reg::read) this register and get [`saddr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:SADDR3)*/
pub struct SADDR3rs;
impl crate::RegisterSpec for SADDR3rs {
    type Ux = u32;
}
///`read()` method returns [`saddr3::R`](R) reader structure
impl crate::Readable for SADDR3rs {}
///`write(|w| ..)` method takes [`saddr3::W`](W) writer structure
impl crate::Writable for SADDR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SADDR3 to value 0
impl crate::Resettable for SADDR3rs {}
