///Register `DAC_SHHR` reader
pub type R = crate::R<DAC_SHHRrs>;
///Register `DAC_SHHR` writer
pub type W = crate::W<DAC_SHHRrs>;
///Field `THOLD1` reader - DAC Channel 1 hold Time (only valid in sample and hold mode)
pub type THOLD1_R = crate::FieldReader<u16>;
///Field `THOLD1` writer - DAC Channel 1 hold Time (only valid in sample and hold mode)
pub type THOLD1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `THOLD2` reader - DAC Channel 2 hold time (only valid in sample and hold mode)
pub type THOLD2_R = crate::FieldReader<u16>;
///Field `THOLD2` writer - DAC Channel 2 hold time (only valid in sample and hold mode)
pub type THOLD2_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - DAC Channel 1 hold Time (only valid in sample and hold mode)
    #[inline(always)]
    pub fn thold1(&self) -> THOLD1_R {
        THOLD1_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:25 - DAC Channel 2 hold time (only valid in sample and hold mode)
    #[inline(always)]
    pub fn thold2(&self) -> THOLD2_R {
        THOLD2_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_SHHR")
            .field("thold1", &self.thold1())
            .field("thold2", &self.thold2())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - DAC Channel 1 hold Time (only valid in sample and hold mode)
    #[inline(always)]
    #[must_use]
    pub fn thold1(&mut self) -> THOLD1_W<DAC_SHHRrs> {
        THOLD1_W::new(self, 0)
    }
    ///Bits 16:25 - DAC Channel 2 hold time (only valid in sample and hold mode)
    #[inline(always)]
    #[must_use]
    pub fn thold2(&mut self) -> THOLD2_W<DAC_SHHRrs> {
        THOLD2_W::new(self, 16)
    }
}
/**DAC Sample and Hold hold time register

You can [`read`](crate::Reg::read) this register and get [`dac_shhr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_shhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DAC1:DAC_SHHR)*/
pub struct DAC_SHHRrs;
impl crate::RegisterSpec for DAC_SHHRrs {
    type Ux = u32;
}
///`read()` method returns [`dac_shhr::R`](R) reader structure
impl crate::Readable for DAC_SHHRrs {}
///`write(|w| ..)` method takes [`dac_shhr::W`](W) writer structure
impl crate::Writable for DAC_SHHRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_SHHR to value 0x0001_0001
impl crate::Resettable for DAC_SHHRrs {
    const RESET_VALUE: u32 = 0x0001_0001;
}
