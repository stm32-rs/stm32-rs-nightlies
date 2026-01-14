///Register `CMP` reader
pub type R = crate::R<CMPrs>;
///Register `CMP` writer
pub type W = crate::W<CMPrs>;
///Field `CMP` reader - CMP
pub type CMP_R = crate::FieldReader<u16>;
///Field `CMP` writer - CMP
pub type CMP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - CMP
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP").field("cmp", &self.cmp()).finish()
    }
}
impl W {
    ///Bits 0:15 - CMP
    #[inline(always)]
    pub fn cmp(&mut self) -> CMP_W<'_, CMPrs> {
        CMP_W::new(self, 0)
    }
}
/**compare register

You can [`read`](crate::Reg::read) this register and get [`cmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#LPTIM2:CMP)*/
pub struct CMPrs;
impl crate::RegisterSpec for CMPrs {
    type Ux = u32;
}
///`read()` method returns [`cmp::R`](R) reader structure
impl crate::Readable for CMPrs {}
///`write(|w| ..)` method takes [`cmp::W`](W) writer structure
impl crate::Writable for CMPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMP to value 0
impl crate::Resettable for CMPrs {}
