///Register `BCCR` reader
pub type R = crate::R<BCCRrs>;
///Register `BCCR` writer
pub type W = crate::W<BCCRrs>;
///Field `BCBLUE` reader - Background Color Blue value
pub type BCBLUE_R = crate::FieldReader;
///Field `BCBLUE` writer - Background Color Blue value
pub type BCBLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `BCGREEN` reader - Background Color Green value
pub type BCGREEN_R = crate::FieldReader;
///Field `BCGREEN` writer - Background Color Green value
pub type BCGREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `BCRED` reader - Background Color Red value
pub type BCRED_R = crate::FieldReader;
///Field `BCRED` writer - Background Color Red value
pub type BCRED_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - Background Color Blue value
    #[inline(always)]
    pub fn bcblue(&self) -> BCBLUE_R {
        BCBLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Background Color Green value
    #[inline(always)]
    pub fn bcgreen(&self) -> BCGREEN_R {
        BCGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Background Color Red value
    #[inline(always)]
    pub fn bcred(&self) -> BCRED_R {
        BCRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCCR")
            .field("bcblue", &self.bcblue())
            .field("bcgreen", &self.bcgreen())
            .field("bcred", &self.bcred())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Background Color Blue value
    #[inline(always)]
    pub fn bcblue(&mut self) -> BCBLUE_W<'_, BCCRrs> {
        BCBLUE_W::new(self, 0)
    }
    ///Bits 8:15 - Background Color Green value
    #[inline(always)]
    pub fn bcgreen(&mut self) -> BCGREEN_W<'_, BCCRrs> {
        BCGREEN_W::new(self, 8)
    }
    ///Bits 16:23 - Background Color Red value
    #[inline(always)]
    pub fn bcred(&mut self) -> BCRED_W<'_, BCCRrs> {
        BCRED_W::new(self, 16)
    }
}
/**Background Color Configuration Register

You can [`read`](crate::Reg::read) this register and get [`bccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#LTDC:BCCR)*/
pub struct BCCRrs;
impl crate::RegisterSpec for BCCRrs {
    type Ux = u32;
}
///`read()` method returns [`bccr::R`](R) reader structure
impl crate::Readable for BCCRrs {}
///`write(|w| ..)` method takes [`bccr::W`](W) writer structure
impl crate::Writable for BCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCCR to value 0
impl crate::Resettable for BCCRrs {}
