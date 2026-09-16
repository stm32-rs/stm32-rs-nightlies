///Register `SHRR` reader
pub type R = crate::R<SHRRrs>;
///Register `SHRR` writer
pub type W = crate::W<SHRRrs>;
///Field `TREFRESH1` reader - TREFRESH1
pub type TREFRESH1_R = crate::FieldReader;
///Field `TREFRESH1` writer - TREFRESH1
pub type TREFRESH1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TREFRESH2` reader - TREFRESH2
pub type TREFRESH2_R = crate::FieldReader;
///Field `TREFRESH2` writer - TREFRESH2
pub type TREFRESH2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - TREFRESH1
    #[inline(always)]
    pub fn trefresh1(&self) -> TREFRESH1_R {
        TREFRESH1_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - TREFRESH2
    #[inline(always)]
    pub fn trefresh2(&self) -> TREFRESH2_R {
        TREFRESH2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHRR")
            .field("trefresh1", &self.trefresh1())
            .field("trefresh2", &self.trefresh2())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - TREFRESH1
    #[inline(always)]
    pub fn trefresh1(&mut self) -> TREFRESH1_W<'_, SHRRrs> {
        TREFRESH1_W::new(self, 0)
    }
    ///Bits 16:23 - TREFRESH2
    #[inline(always)]
    pub fn trefresh2(&mut self) -> TREFRESH2_W<'_, SHRRrs> {
        TREFRESH2_W::new(self, 16)
    }
}
/**DAC sample and hold refresh time register

You can [`read`](crate::Reg::read) this register and get [`shrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DAC1:SHRR)*/
pub struct SHRRrs;
impl crate::RegisterSpec for SHRRrs {
    type Ux = u32;
}
///`read()` method returns [`shrr::R`](R) reader structure
impl crate::Readable for SHRRrs {}
///`write(|w| ..)` method takes [`shrr::W`](W) writer structure
impl crate::Writable for SHRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SHRR to value 0x0001_0001
impl crate::Resettable for SHRRrs {
    const RESET_VALUE: u32 = 0x0001_0001;
}
