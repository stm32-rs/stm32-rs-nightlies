///Register `L1CKCR` reader
pub type R = crate::R<L1CKCRrs>;
///Register `L1CKCR` writer
pub type W = crate::W<L1CKCRrs>;
///Field `CKBLUE` reader - Color Key Blue value
pub type CKBLUE_R = crate::FieldReader;
///Field `CKBLUE` writer - Color Key Blue value
pub type CKBLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `CKGREEN` reader - Color Key Green value
pub type CKGREEN_R = crate::FieldReader;
///Field `CKGREEN` writer - Color Key Green value
pub type CKGREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `CKRED` reader - Color Key Red value
pub type CKRED_R = crate::FieldReader;
///Field `CKRED` writer - Color Key Red value
pub type CKRED_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - Color Key Blue value
    #[inline(always)]
    pub fn ckblue(&self) -> CKBLUE_R {
        CKBLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Color Key Green value
    #[inline(always)]
    pub fn ckgreen(&self) -> CKGREEN_R {
        CKGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Color Key Red value
    #[inline(always)]
    pub fn ckred(&self) -> CKRED_R {
        CKRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1CKCR")
            .field("ckblue", &self.ckblue())
            .field("ckgreen", &self.ckgreen())
            .field("ckred", &self.ckred())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Color Key Blue value
    #[inline(always)]
    #[must_use]
    pub fn ckblue(&mut self) -> CKBLUE_W<L1CKCRrs> {
        CKBLUE_W::new(self, 0)
    }
    ///Bits 8:15 - Color Key Green value
    #[inline(always)]
    #[must_use]
    pub fn ckgreen(&mut self) -> CKGREEN_W<L1CKCRrs> {
        CKGREEN_W::new(self, 8)
    }
    ///Bits 16:23 - Color Key Red value
    #[inline(always)]
    #[must_use]
    pub fn ckred(&mut self) -> CKRED_W<L1CKCRrs> {
        CKRED_W::new(self, 16)
    }
}
/**LTDC Layer Color Keying Configuration Register

You can [`read`](crate::Reg::read) this register and get [`l1ckcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1ckcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#LTDC:L1CKCR)*/
pub struct L1CKCRrs;
impl crate::RegisterSpec for L1CKCRrs {
    type Ux = u32;
}
///`read()` method returns [`l1ckcr::R`](R) reader structure
impl crate::Readable for L1CKCRrs {}
///`write(|w| ..)` method takes [`l1ckcr::W`](W) writer structure
impl crate::Writable for L1CKCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L1CKCR to value 0
impl crate::Resettable for L1CKCRrs {
    const RESET_VALUE: u32 = 0;
}
