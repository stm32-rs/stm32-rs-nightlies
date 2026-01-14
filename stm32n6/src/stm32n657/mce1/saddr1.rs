///Register `SADDR1` reader
pub type R = crate::R<SADDR1rs>;
///Register `SADDR1` writer
pub type W = crate::W<SADDR1rs>;
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
        f.debug_struct("SADDR1")
            .field("baddstart", &self.baddstart())
            .finish()
    }
}
impl W {
    ///Bits 12:31 - Region address start
    #[inline(always)]
    pub fn baddstart(&mut self) -> BADDSTART_W<'_, SADDR1rs> {
        BADDSTART_W::new(self, 12)
    }
}
/**MCE start address for region 1 register

You can [`read`](crate::Reg::read) this register and get [`saddr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MCE1:SADDR1)*/
pub struct SADDR1rs;
impl crate::RegisterSpec for SADDR1rs {
    type Ux = u32;
}
///`read()` method returns [`saddr1::R`](R) reader structure
impl crate::Readable for SADDR1rs {}
///`write(|w| ..)` method takes [`saddr1::W`](W) writer structure
impl crate::Writable for SADDR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SADDR1 to value 0
impl crate::Resettable for SADDR1rs {}
