///Register `CKPROTR` reader
pub type R = crate::R<CKPROTRrs>;
///Register `CKPROTR` writer
pub type W = crate::W<CKPROTRrs>;
///Field `XSPICKP` reader - XSPI clock protection
pub type XSPICKP_R = crate::BitReader;
///Field `XSPICKP` writer - XSPI clock protection
pub type XSPICKP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMCCKP` reader - FMC clock protection
pub type FMCCKP_R = crate::BitReader;
///Field `FMCCKP` writer - FMC clock protection
pub type FMCCKP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI1SWP` reader - XSPI1 kernel clock switch position
pub type XSPI1SWP_R = crate::FieldReader;
///Field `XSPI2SWP` reader - XSPI2 kernel clock switch position
pub type XSPI2SWP_R = crate::FieldReader;
///Field `FMCSWP` reader - FMC kernel clock switch position
pub type FMCSWP_R = crate::FieldReader;
impl R {
    ///Bit 0 - XSPI clock protection
    #[inline(always)]
    pub fn xspickp(&self) -> XSPICKP_R {
        XSPICKP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FMC clock protection
    #[inline(always)]
    pub fn fmcckp(&self) -> FMCCKP_R {
        FMCCKP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 4:6 - XSPI1 kernel clock switch position
    #[inline(always)]
    pub fn xspi1swp(&self) -> XSPI1SWP_R {
        XSPI1SWP_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - XSPI2 kernel clock switch position
    #[inline(always)]
    pub fn xspi2swp(&self) -> XSPI2SWP_R {
        XSPI2SWP_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - FMC kernel clock switch position
    #[inline(always)]
    pub fn fmcswp(&self) -> FMCSWP_R {
        FMCSWP_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CKPROTR")
            .field("xspickp", &self.xspickp())
            .field("fmcckp", &self.fmcckp())
            .field("xspi1swp", &self.xspi1swp())
            .field("xspi2swp", &self.xspi2swp())
            .field("fmcswp", &self.fmcswp())
            .finish()
    }
}
impl W {
    ///Bit 0 - XSPI clock protection
    #[inline(always)]
    pub fn xspickp(&mut self) -> XSPICKP_W<'_, CKPROTRrs> {
        XSPICKP_W::new(self, 0)
    }
    ///Bit 1 - FMC clock protection
    #[inline(always)]
    pub fn fmcckp(&mut self) -> FMCCKP_W<'_, CKPROTRrs> {
        FMCCKP_W::new(self, 1)
    }
}
/**RCC clock protection register

You can [`read`](crate::Reg::read) this register and get [`ckprotr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckprotr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:CKPROTR)*/
pub struct CKPROTRrs;
impl crate::RegisterSpec for CKPROTRrs {
    type Ux = u32;
}
///`read()` method returns [`ckprotr::R`](R) reader structure
impl crate::Readable for CKPROTRrs {}
///`write(|w| ..)` method takes [`ckprotr::W`](W) writer structure
impl crate::Writable for CKPROTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKPROTR to value 0
impl crate::Resettable for CKPROTRrs {}
