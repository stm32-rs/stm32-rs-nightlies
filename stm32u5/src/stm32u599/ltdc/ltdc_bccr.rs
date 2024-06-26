///Register `LTDC_BCCR` reader
pub type R = crate::R<LTDC_BCCRrs>;
///Register `LTDC_BCCR` writer
pub type W = crate::W<LTDC_BCCRrs>;
///Field `BCBLUE` reader - background color blue value These bits configure the background blue value.
pub type BCBLUE_R = crate::FieldReader;
///Field `BCBLUE` writer - background color blue value These bits configure the background blue value.
pub type BCBLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BCGREEN` reader - background color green value These bits configure the background green value.
pub type BCGREEN_R = crate::FieldReader;
///Field `BCGREEN` writer - background color green value These bits configure the background green value.
pub type BCGREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BCRED` reader - background color red value These bits configure the background red value.
pub type BCRED_R = crate::FieldReader;
///Field `BCRED` writer - background color red value These bits configure the background red value.
pub type BCRED_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - background color blue value These bits configure the background blue value.
    #[inline(always)]
    pub fn bcblue(&self) -> BCBLUE_R {
        BCBLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - background color green value These bits configure the background green value.
    #[inline(always)]
    pub fn bcgreen(&self) -> BCGREEN_R {
        BCGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - background color red value These bits configure the background red value.
    #[inline(always)]
    pub fn bcred(&self) -> BCRED_R {
        BCRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTDC_BCCR")
            .field("bcblue", &self.bcblue())
            .field("bcgreen", &self.bcgreen())
            .field("bcred", &self.bcred())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - background color blue value These bits configure the background blue value.
    #[inline(always)]
    #[must_use]
    pub fn bcblue(&mut self) -> BCBLUE_W<LTDC_BCCRrs> {
        BCBLUE_W::new(self, 0)
    }
    ///Bits 8:15 - background color green value These bits configure the background green value.
    #[inline(always)]
    #[must_use]
    pub fn bcgreen(&mut self) -> BCGREEN_W<LTDC_BCCRrs> {
        BCGREEN_W::new(self, 8)
    }
    ///Bits 16:23 - background color red value These bits configure the background red value.
    #[inline(always)]
    #[must_use]
    pub fn bcred(&mut self) -> BCRED_W<LTDC_BCCRrs> {
        BCRED_W::new(self, 16)
    }
}
/**LTDC background color configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_bccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_bccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#LTDC:LTDC_BCCR)*/
pub struct LTDC_BCCRrs;
impl crate::RegisterSpec for LTDC_BCCRrs {
    type Ux = u32;
}
///`read()` method returns [`ltdc_bccr::R`](R) reader structure
impl crate::Readable for LTDC_BCCRrs {}
///`write(|w| ..)` method takes [`ltdc_bccr::W`](W) writer structure
impl crate::Writable for LTDC_BCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LTDC_BCCR to value 0
impl crate::Resettable for LTDC_BCCRrs {
    const RESET_VALUE: u32 = 0;
}
