///Register `SHRR` reader
pub type R = crate::R<SHRRrs>;
///Register `SHRR` writer
pub type W = crate::W<SHRRrs>;
///Field `TREFRESH1` reader - DAC channel1 refresh time (only valid in Sample and hold mode) Refresh time1=1(TREFRESH\[7:0\]) x LSI clock period Note: This register can be modified only when EN11=10.
pub type TREFRESH1_R = crate::FieldReader;
///Field `TREFRESH1` writer - DAC channel1 refresh time (only valid in Sample and hold mode) Refresh time1=1(TREFRESH\[7:0\]) x LSI clock period Note: This register can be modified only when EN11=10.
pub type TREFRESH1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - DAC channel1 refresh time (only valid in Sample and hold mode) Refresh time1=1(TREFRESH\[7:0\]) x LSI clock period Note: This register can be modified only when EN11=10.
    #[inline(always)]
    pub fn trefresh1(&self) -> TREFRESH1_R {
        TREFRESH1_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHRR")
            .field("trefresh1", &self.trefresh1())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - DAC channel1 refresh time (only valid in Sample and hold mode) Refresh time1=1(TREFRESH\[7:0\]) x LSI clock period Note: This register can be modified only when EN11=10.
    #[inline(always)]
    pub fn trefresh1(&mut self) -> TREFRESH1_W<SHRRrs> {
        TREFRESH1_W::new(self, 0)
    }
}
/**DAC sample and hold refresh time register

You can [`read`](crate::Reg::read) this register and get [`shrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DAC:SHRR)*/
pub struct SHRRrs;
impl crate::RegisterSpec for SHRRrs {
    type Ux = u32;
}
///`read()` method returns [`shrr::R`](R) reader structure
impl crate::Readable for SHRRrs {}
///`write(|w| ..)` method takes [`shrr::W`](W) writer structure
impl crate::Writable for SHRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SHRR to value 0x0001_0001
impl crate::Resettable for SHRRrs {
    const RESET_VALUE: u32 = 0x0001_0001;
}
