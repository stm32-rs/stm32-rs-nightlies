///Register `OPTCCR_` reader
pub type R = crate::R<OPTCCR_rs>;
///Register `OPTCCR_` writer
pub type W = crate::W<OPTCCR_rs>;
///Field `CLR_OPTCHANGEERR` reader - OPTCHANGEERR reset bit
pub type CLR_OPTCHANGEERR_R = crate::BitReader;
///Field `CLR_OPTCHANGEERR` writer - OPTCHANGEERR reset bit
pub type CLR_OPTCHANGEERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 30 - OPTCHANGEERR reset bit
    #[inline(always)]
    pub fn clr_optchangeerr(&self) -> CLR_OPTCHANGEERR_R {
        CLR_OPTCHANGEERR_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTCCR_")
            .field("clr_optchangeerr", &self.clr_optchangeerr())
            .finish()
    }
}
impl W {
    ///Bit 30 - OPTCHANGEERR reset bit
    #[inline(always)]
    pub fn clr_optchangeerr(&mut self) -> CLR_OPTCHANGEERR_W<'_, OPTCCR_rs> {
        CLR_OPTCHANGEERR_W::new(self, 30)
    }
}
/**FLASH option clear control register

You can [`read`](crate::Reg::read) this register and get [`optccr_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optccr_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#FLASH:OPTCCR_)*/
pub struct OPTCCR_rs;
impl crate::RegisterSpec for OPTCCR_rs {
    type Ux = u32;
}
///`read()` method returns [`optccr_::R`](R) reader structure
impl crate::Readable for OPTCCR_rs {}
///`write(|w| ..)` method takes [`optccr_::W`](W) writer structure
impl crate::Writable for OPTCCR_rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTCCR_ to value 0
impl crate::Resettable for OPTCCR_rs {}
