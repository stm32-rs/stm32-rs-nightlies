///Register `DAC_SHRR` reader
pub type R = crate::R<DAC_SHRRrs>;
///Register `DAC_SHRR` writer
pub type W = crate::W<DAC_SHRRrs>;
///Field `TREFRESH1` reader - DAC Channel 1 refresh Time (only valid in sample and hold mode)
pub type TREFRESH1_R = crate::FieldReader;
///Field `TREFRESH1` writer - DAC Channel 1 refresh Time (only valid in sample and hold mode)
pub type TREFRESH1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TREFRESH2` reader - DAC Channel 2 refresh Time (only valid in sample and hold mode)
pub type TREFRESH2_R = crate::FieldReader;
///Field `TREFRESH2` writer - DAC Channel 2 refresh Time (only valid in sample and hold mode)
pub type TREFRESH2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - DAC Channel 1 refresh Time (only valid in sample and hold mode)
    #[inline(always)]
    pub fn trefresh1(&self) -> TREFRESH1_R {
        TREFRESH1_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - DAC Channel 2 refresh Time (only valid in sample and hold mode)
    #[inline(always)]
    pub fn trefresh2(&self) -> TREFRESH2_R {
        TREFRESH2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_SHRR")
            .field("trefresh1", &self.trefresh1())
            .field("trefresh2", &self.trefresh2())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - DAC Channel 1 refresh Time (only valid in sample and hold mode)
    #[inline(always)]
    #[must_use]
    pub fn trefresh1(&mut self) -> TREFRESH1_W<DAC_SHRRrs> {
        TREFRESH1_W::new(self, 0)
    }
    ///Bits 16:23 - DAC Channel 2 refresh Time (only valid in sample and hold mode)
    #[inline(always)]
    #[must_use]
    pub fn trefresh2(&mut self) -> TREFRESH2_W<DAC_SHRRrs> {
        TREFRESH2_W::new(self, 16)
    }
}
/**DAC Sample and Hold refresh time register

You can [`read`](crate::Reg::read) this register and get [`dac_shrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_shrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DAC1:DAC_SHRR)*/
pub struct DAC_SHRRrs;
impl crate::RegisterSpec for DAC_SHRRrs {
    type Ux = u32;
}
///`read()` method returns [`dac_shrr::R`](R) reader structure
impl crate::Readable for DAC_SHRRrs {}
///`write(|w| ..)` method takes [`dac_shrr::W`](W) writer structure
impl crate::Writable for DAC_SHRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_SHRR to value 0x0001_0001
impl crate::Resettable for DAC_SHRRrs {
    const RESET_VALUE: u32 = 0x0001_0001;
}
