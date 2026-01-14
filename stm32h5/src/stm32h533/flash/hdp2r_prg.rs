///Register `HDP2R_PRG` reader
pub type R = crate::R<HDP2R_PRGrs>;
///Register `HDP2R_PRG` writer
pub type W = crate::W<HDP2R_PRGrs>;
///Field `HDP2_STRT` reader - HDPL barrier start set in number of 8-Kbyte sectors
pub type HDP2_STRT_R = crate::FieldReader;
///Field `HDP2_STRT` writer - HDPL barrier start set in number of 8-Kbyte sectors
pub type HDP2_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `HDP2_END` reader - HDPL barrier end set in number of 8-Kbyte sectors
pub type HDP2_END_R = crate::FieldReader;
///Field `HDP2_END` writer - HDPL barrier end set in number of 8-Kbyte sectors
pub type HDP2_END_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - HDPL barrier start set in number of 8-Kbyte sectors
    #[inline(always)]
    pub fn hdp2_strt(&self) -> HDP2_STRT_R {
        HDP2_STRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - HDPL barrier end set in number of 8-Kbyte sectors
    #[inline(always)]
    pub fn hdp2_end(&self) -> HDP2_END_R {
        HDP2_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HDP2R_PRG")
            .field("hdp2_strt", &self.hdp2_strt())
            .field("hdp2_end", &self.hdp2_end())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - HDPL barrier start set in number of 8-Kbyte sectors
    #[inline(always)]
    pub fn hdp2_strt(&mut self) -> HDP2_STRT_W<'_, HDP2R_PRGrs> {
        HDP2_STRT_W::new(self, 0)
    }
    ///Bits 16:22 - HDPL barrier end set in number of 8-Kbyte sectors
    #[inline(always)]
    pub fn hdp2_end(&mut self) -> HDP2_END_W<'_, HDP2R_PRGrs> {
        HDP2_END_W::new(self, 16)
    }
}
/**FLASH HDP Bank2 configuration

You can [`read`](crate::Reg::read) this register and get [`hdp2r_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdp2r_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#FLASH:HDP2R_PRG)*/
pub struct HDP2R_PRGrs;
impl crate::RegisterSpec for HDP2R_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`hdp2r_prg::R`](R) reader structure
impl crate::Readable for HDP2R_PRGrs {}
///`write(|w| ..)` method takes [`hdp2r_prg::W`](W) writer structure
impl crate::Writable for HDP2R_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HDP2R_PRG to value 0
impl crate::Resettable for HDP2R_PRGrs {}
