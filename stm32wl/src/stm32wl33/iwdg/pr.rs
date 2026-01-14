///Register `PR` reader
pub type R = crate::R<PRrs>;
///Register `PR` writer
pub type W = crate::W<PRrs>;
///Field `PR` reader - Prescaler divider. Set and reset by software. These bits are write access protected. They are written by software to select the prescaler divider feeding the counter clock. PVU bit of IWDG_SR must be reset in order to be able to change the prescaler divider. 000: divider/4 001: divider/8 010: divider/16 011: divider/32 100: divider/64 101: divider/128 110: divider/256 111: divider/256
pub type PR_R = crate::FieldReader;
///Field `PR` writer - Prescaler divider. Set and reset by software. These bits are write access protected. They are written by software to select the prescaler divider feeding the counter clock. PVU bit of IWDG_SR must be reset in order to be able to change the prescaler divider. 000: divider/4 001: divider/8 010: divider/16 011: divider/32 100: divider/64 101: divider/128 110: divider/256 111: divider/256
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Prescaler divider. Set and reset by software. These bits are write access protected. They are written by software to select the prescaler divider feeding the counter clock. PVU bit of IWDG_SR must be reset in order to be able to change the prescaler divider. 000: divider/4 001: divider/8 010: divider/16 011: divider/32 100: divider/64 101: divider/128 110: divider/256 111: divider/256
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
    ///Bits 0:2 - Prescaler divider. Set and reset by software. These bits are write access protected. They are written by software to select the prescaler divider feeding the counter clock. PVU bit of IWDG_SR must be reset in order to be able to change the prescaler divider. 000: divider/4 001: divider/8 010: divider/16 011: divider/32 100: divider/64 101: divider/128 110: divider/256 111: divider/256
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 0)
    }
}
/**IWDG_PR register

You can [`read`](crate::Reg::read) this register and get [`pr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#IWDG:PR)*/
pub struct PRrs;
impl crate::RegisterSpec for PRrs {
    type Ux = u32;
}
///`read()` method returns [`pr::R`](R) reader structure
impl crate::Readable for PRrs {}
///`write(|w| ..)` method takes [`pr::W`](W) writer structure
impl crate::Writable for PRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PR to value 0
impl crate::Resettable for PRrs {}
