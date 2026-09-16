///Register `CMP1R` reader
pub type R = crate::R<CMP1Rrs>;
///Register `CMP1R` writer
pub type W = crate::W<CMP1Rrs>;
///Field `CMP` reader - Master Timer Compare 1 value
pub type CMP_R = crate::FieldReader<u16>;
///Field `CMP` writer - Master Timer Compare 1 value
pub type CMP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Master Timer Compare 1 value
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP1R").field("cmp", &self.cmp()).finish()
    }
}
impl W {
    ///Bits 0:15 - Master Timer Compare 1 value
    #[inline(always)]
    pub fn cmp(&mut self) -> CMP_W<'_, CMP1Rrs> {
        CMP_W::new(self, 0)
    }
}
/**Master Timer Compare 1 Register

You can [`read`](crate::Reg::read) this register and get [`cmp1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#HRTIM_Master:CMP1R)*/
pub struct CMP1Rrs;
impl crate::RegisterSpec for CMP1Rrs {
    type Ux = u32;
}
///`read()` method returns [`cmp1r::R`](R) reader structure
impl crate::Readable for CMP1Rrs {}
///`write(|w| ..)` method takes [`cmp1r::W`](W) writer structure
impl crate::Writable for CMP1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMP1R to value 0
impl crate::Resettable for CMP1Rrs {}
