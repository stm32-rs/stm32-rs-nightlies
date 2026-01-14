///Register `APB5LPENR` reader
pub type R = crate::R<APB5LPENRrs>;
///Register `APB5LPENR` writer
pub type W = crate::W<APB5LPENRrs>;
///Field `LTDCLPEN` reader - LTDC sleep enable
pub type LTDCLPEN_R = crate::BitReader;
///Field `LTDCLPEN` writer - LTDC sleep enable
pub type LTDCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCMIPPLPEN` reader - DCMIPP sleep enable
pub type DCMIPPLPEN_R = crate::BitReader;
///Field `DCMIPPLPEN` writer - DCMIPP sleep enable
pub type DCMIPPLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXTIMLPEN` reader - GFXTIM sleep enable
pub type GFXTIMLPEN_R = crate::BitReader;
///Field `GFXTIMLPEN` writer - GFXTIM sleep enable
pub type GFXTIMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCLPEN` reader - VENC sleep enable
pub type VENCLPEN_R = crate::BitReader;
///Field `VENCLPEN` writer - VENC sleep enable
pub type VENCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSILPEN` reader - CSI sleep enable
pub type CSILPEN_R = crate::BitReader;
///Field `CSILPEN` writer - CSI sleep enable
pub type CSILPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - LTDC sleep enable
    #[inline(always)]
    pub fn ltdclpen(&self) -> LTDCLPEN_R {
        LTDCLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DCMIPP sleep enable
    #[inline(always)]
    pub fn dcmipplpen(&self) -> DCMIPPLPEN_R {
        DCMIPPLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - GFXTIM sleep enable
    #[inline(always)]
    pub fn gfxtimlpen(&self) -> GFXTIMLPEN_R {
        GFXTIMLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - VENC sleep enable
    #[inline(always)]
    pub fn venclpen(&self) -> VENCLPEN_R {
        VENCLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CSI sleep enable
    #[inline(always)]
    pub fn csilpen(&self) -> CSILPEN_R {
        CSILPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB5LPENR")
            .field("ltdclpen", &self.ltdclpen())
            .field("dcmipplpen", &self.dcmipplpen())
            .field("gfxtimlpen", &self.gfxtimlpen())
            .field("venclpen", &self.venclpen())
            .field("csilpen", &self.csilpen())
            .finish()
    }
}
impl W {
    ///Bit 1 - LTDC sleep enable
    #[inline(always)]
    pub fn ltdclpen(&mut self) -> LTDCLPEN_W<'_, APB5LPENRrs> {
        LTDCLPEN_W::new(self, 1)
    }
    ///Bit 2 - DCMIPP sleep enable
    #[inline(always)]
    pub fn dcmipplpen(&mut self) -> DCMIPPLPEN_W<'_, APB5LPENRrs> {
        DCMIPPLPEN_W::new(self, 2)
    }
    ///Bit 4 - GFXTIM sleep enable
    #[inline(always)]
    pub fn gfxtimlpen(&mut self) -> GFXTIMLPEN_W<'_, APB5LPENRrs> {
        GFXTIMLPEN_W::new(self, 4)
    }
    ///Bit 5 - VENC sleep enable
    #[inline(always)]
    pub fn venclpen(&mut self) -> VENCLPEN_W<'_, APB5LPENRrs> {
        VENCLPEN_W::new(self, 5)
    }
    ///Bit 6 - CSI sleep enable
    #[inline(always)]
    pub fn csilpen(&mut self) -> CSILPEN_W<'_, APB5LPENRrs> {
        CSILPEN_W::new(self, 6)
    }
}
/**RCC APB5 Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`apb5lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB5LPENR)*/
pub struct APB5LPENRrs;
impl crate::RegisterSpec for APB5LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb5lpenr::R`](R) reader structure
impl crate::Readable for APB5LPENRrs {}
///`write(|w| ..)` method takes [`apb5lpenr::W`](W) writer structure
impl crate::Writable for APB5LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB5LPENR to value 0
impl crate::Resettable for APB5LPENRrs {}
