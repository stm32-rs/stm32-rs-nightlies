///Register `PR` reader
pub type R = crate::R<PRrs>;
///Register `PR` writer
pub type W = crate::W<PRrs>;
///Field `PR` reader - PR
pub type PR_R = crate::FieldReader;
///Field `PR` writer - PR
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - PR
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR").field("pr", &self.pr()).finish()
    }
}
impl W {
    ///Bits 0:2 - PR
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W<PRrs> {
        PR_W::new(self, 0)
    }
}
/**Prescaler register

You can [`read`](crate::Reg::read) this register and get [`pr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#IWDG1:PR)*/
pub struct PRrs;
impl crate::RegisterSpec for PRrs {
    type Ux = u16;
}
///`read()` method returns [`pr::R`](R) reader structure
impl crate::Readable for PRrs {}
///`write(|w| ..)` method takes [`pr::W`](W) writer structure
impl crate::Writable for PRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets PR to value 0x07
impl crate::Resettable for PRrs {
    const RESET_VALUE: u16 = 0x07;
}