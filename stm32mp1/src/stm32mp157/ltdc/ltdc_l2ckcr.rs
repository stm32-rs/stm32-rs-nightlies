///Register `LTDC_L2CKCR` reader
pub type R = crate::R<LTDC_L2CKCRrs>;
///Register `LTDC_L2CKCR` writer
pub type W = crate::W<LTDC_L2CKCRrs>;
///Field `CKBLUE` reader - CKBLUE
pub type CKBLUE_R = crate::FieldReader;
///Field `CKBLUE` writer - CKBLUE
pub type CKBLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CKGREEN` reader - CKGREEN
pub type CKGREEN_R = crate::FieldReader;
///Field `CKGREEN` writer - CKGREEN
pub type CKGREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CKRED` reader - CKRED
pub type CKRED_R = crate::FieldReader;
///Field `CKRED` writer - CKRED
pub type CKRED_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - CKBLUE
    #[inline(always)]
    pub fn ckblue(&self) -> CKBLUE_R {
        CKBLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - CKGREEN
    #[inline(always)]
    pub fn ckgreen(&self) -> CKGREEN_R {
        CKGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - CKRED
    #[inline(always)]
    pub fn ckred(&self) -> CKRED_R {
        CKRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTDC_L2CKCR")
            .field("ckblue", &self.ckblue())
            .field("ckgreen", &self.ckgreen())
            .field("ckred", &self.ckred())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - CKBLUE
    #[inline(always)]
    #[must_use]
    pub fn ckblue(&mut self) -> CKBLUE_W<LTDC_L2CKCRrs> {
        CKBLUE_W::new(self, 0)
    }
    ///Bits 8:15 - CKGREEN
    #[inline(always)]
    #[must_use]
    pub fn ckgreen(&mut self) -> CKGREEN_W<LTDC_L2CKCRrs> {
        CKGREEN_W::new(self, 8)
    }
    ///Bits 16:23 - CKRED
    #[inline(always)]
    #[must_use]
    pub fn ckred(&mut self) -> CKRED_W<LTDC_L2CKCRrs> {
        CKRED_W::new(self, 16)
    }
}
/**This register defines the color key value (RGB), that is used by the color keying.

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2ckcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2ckcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LTDC:LTDC_L2CKCR)*/
pub struct LTDC_L2CKCRrs;
impl crate::RegisterSpec for LTDC_L2CKCRrs {
    type Ux = u32;
}
///`read()` method returns [`ltdc_l2ckcr::R`](R) reader structure
impl crate::Readable for LTDC_L2CKCRrs {}
///`write(|w| ..)` method takes [`ltdc_l2ckcr::W`](W) writer structure
impl crate::Writable for LTDC_L2CKCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LTDC_L2CKCR to value 0
impl crate::Resettable for LTDC_L2CKCRrs {
    const RESET_VALUE: u32 = 0;
}
