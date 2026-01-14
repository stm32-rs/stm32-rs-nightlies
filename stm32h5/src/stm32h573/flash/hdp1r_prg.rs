///Register `HDP1R_PRG` reader
pub type R = crate::R<HDP1R_PRGrs>;
///Register `HDP1R_PRG` writer
pub type W = crate::W<HDP1R_PRGrs>;
///Field `HDP1_STRT` reader - HDPL barrier start set in number of 8-Kbyte sectors
pub type HDP1_STRT_R = crate::FieldReader;
///Field `HDP1_STRT` writer - HDPL barrier start set in number of 8-Kbyte sectors
pub type HDP1_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `HDP1_END` reader - HDPL barrier end set in number of 8-Kbyte sectors
pub type HDP1_END_R = crate::FieldReader;
///Field `HDP1_END` writer - HDPL barrier end set in number of 8-Kbyte sectors
pub type HDP1_END_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - HDPL barrier start set in number of 8-Kbyte sectors
    #[inline(always)]
    pub fn hdp1_strt(&self) -> HDP1_STRT_R {
        HDP1_STRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - HDPL barrier end set in number of 8-Kbyte sectors
    #[inline(always)]
    pub fn hdp1_end(&self) -> HDP1_END_R {
        HDP1_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HDP1R_PRG")
            .field("hdp1_strt", &self.hdp1_strt())
            .field("hdp1_end", &self.hdp1_end())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - HDPL barrier start set in number of 8-Kbyte sectors
    #[inline(always)]
    pub fn hdp1_strt(&mut self) -> HDP1_STRT_W<'_, HDP1R_PRGrs> {
        HDP1_STRT_W::new(self, 0)
    }
    ///Bits 16:22 - HDPL barrier end set in number of 8-Kbyte sectors
    #[inline(always)]
    pub fn hdp1_end(&mut self) -> HDP1_END_W<'_, HDP1R_PRGrs> {
        HDP1_END_W::new(self, 16)
    }
}
/**FLASH HDP Bank 1 configuration

You can [`read`](crate::Reg::read) this register and get [`hdp1r_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdp1r_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#FLASH:HDP1R_PRG)*/
pub struct HDP1R_PRGrs;
impl crate::RegisterSpec for HDP1R_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`hdp1r_prg::R`](R) reader structure
impl crate::Readable for HDP1R_PRGrs {}
///`write(|w| ..)` method takes [`hdp1r_prg::W`](W) writer structure
impl crate::Writable for HDP1R_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HDP1R_PRG to value 0
impl crate::Resettable for HDP1R_PRGrs {}
