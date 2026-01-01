///Register `COMP_ICFR` reader
pub type R = crate::R<COMP_ICFRrs>;
///Register `COMP_ICFR` writer
pub type W = crate::W<COMP_ICFRrs>;
///Field `CC1IF` reader - Clear COMP Channel1 interrupt flag Writing 1 clears the C1IF flag in the COMP_SR register.
pub type CC1IF_R = crate::BitReader;
///Field `CC1IF` writer - Clear COMP Channel1 interrupt flag Writing 1 clears the C1IF flag in the COMP_SR register.
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 16 - Clear COMP Channel1 interrupt flag Writing 1 clears the C1IF flag in the COMP_SR register.
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP_ICFR")
            .field("cc1if", &self.cc1if())
            .finish()
    }
}
impl W {
    ///Bit 16 - Clear COMP Channel1 interrupt flag Writing 1 clears the C1IF flag in the COMP_SR register.
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W<'_, COMP_ICFRrs> {
        CC1IF_W::new(self, 16)
    }
}
/**Comparator interrupt clear flag register

You can [`read`](crate::Reg::read) this register and get [`comp_icfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_icfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#COMP:COMP_ICFR)*/
pub struct COMP_ICFRrs;
impl crate::RegisterSpec for COMP_ICFRrs {
    type Ux = u32;
}
///`read()` method returns [`comp_icfr::R`](R) reader structure
impl crate::Readable for COMP_ICFRrs {}
///`write(|w| ..)` method takes [`comp_icfr::W`](W) writer structure
impl crate::Writable for COMP_ICFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMP_ICFR to value 0
impl crate::Resettable for COMP_ICFRrs {}
