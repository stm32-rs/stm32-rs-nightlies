///Register `L2CKCR` reader
pub type R = crate::R<L2CKCRrs>;
///Register `L2CKCR` writer
pub type W = crate::W<L2CKCRrs>;
///Field `CKBLUE` reader - Color Key Blue value
pub type CKBLUE_R = crate::FieldReader;
///Field `CKBLUE` writer - Color Key Blue value
pub type CKBLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CKGREEN` reader - Color Key Green value
pub type CKGREEN_R = crate::FieldReader;
///Field `CKGREEN` writer - Color Key Green value
pub type CKGREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `CKRED` reader - Color Key Red value
pub type CKRED_R = crate::FieldReader<u16>;
///Field `CKRED` writer - Color Key Red value
pub type CKRED_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:7 - Color Key Blue value
    #[inline(always)]
    pub fn ckblue(&self) -> CKBLUE_R {
        CKBLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:14 - Color Key Green value
    #[inline(always)]
    pub fn ckgreen(&self) -> CKGREEN_R {
        CKGREEN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bits 15:23 - Color Key Red value
    #[inline(always)]
    pub fn ckred(&self) -> CKRED_R {
        CKRED_R::new(((self.bits >> 15) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2CKCR")
            .field("ckred", &self.ckred())
            .field("ckgreen", &self.ckgreen())
            .field("ckblue", &self.ckblue())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Color Key Blue value
    #[inline(always)]
    pub fn ckblue(&mut self) -> CKBLUE_W<'_, L2CKCRrs> {
        CKBLUE_W::new(self, 0)
    }
    ///Bits 8:14 - Color Key Green value
    #[inline(always)]
    pub fn ckgreen(&mut self) -> CKGREEN_W<'_, L2CKCRrs> {
        CKGREEN_W::new(self, 8)
    }
    ///Bits 15:23 - Color Key Red value
    #[inline(always)]
    pub fn ckred(&mut self) -> CKRED_W<'_, L2CKCRrs> {
        CKRED_W::new(self, 15)
    }
}
/**Layerx Color Keying Configuration Register

You can [`read`](crate::Reg::read) this register and get [`l2ckcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2ckcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#LTDC:L2CKCR)*/
pub struct L2CKCRrs;
impl crate::RegisterSpec for L2CKCRrs {
    type Ux = u32;
}
///`read()` method returns [`l2ckcr::R`](R) reader structure
impl crate::Readable for L2CKCRrs {}
///`write(|w| ..)` method takes [`l2ckcr::W`](W) writer structure
impl crate::Writable for L2CKCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L2CKCR to value 0
impl crate::Resettable for L2CKCRrs {}
