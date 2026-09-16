///Register `APB5RSTR` reader
pub type R = crate::R<APB5RSTRrs>;
///Register `APB5RSTR` writer
pub type W = crate::W<APB5RSTRrs>;
///Field `LTDCRST` reader - LTDC reset
pub type LTDCRST_R = crate::BitReader;
///Field `LTDCRST` writer - LTDC reset
pub type LTDCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCMIPPRST` reader - DCMIPP reset
pub type DCMIPPRST_R = crate::BitReader;
///Field `DCMIPPRST` writer - DCMIPP reset
pub type DCMIPPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXTIMRST` reader - GFXTIM reset
pub type GFXTIMRST_R = crate::BitReader;
///Field `GFXTIMRST` writer - GFXTIM reset
pub type GFXTIMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCRST` reader - VENC reset
pub type VENCRST_R = crate::BitReader;
///Field `VENCRST` writer - VENC reset
pub type VENCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSIRST` reader - CSI reset
pub type CSIRST_R = crate::BitReader;
///Field `CSIRST` writer - CSI reset
pub type CSIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - LTDC reset
    #[inline(always)]
    pub fn ltdcrst(&self) -> LTDCRST_R {
        LTDCRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DCMIPP reset
    #[inline(always)]
    pub fn dcmipprst(&self) -> DCMIPPRST_R {
        DCMIPPRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - GFXTIM reset
    #[inline(always)]
    pub fn gfxtimrst(&self) -> GFXTIMRST_R {
        GFXTIMRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - VENC reset
    #[inline(always)]
    pub fn vencrst(&self) -> VENCRST_R {
        VENCRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CSI reset
    #[inline(always)]
    pub fn csirst(&self) -> CSIRST_R {
        CSIRST_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB5RSTR")
            .field("ltdcrst", &self.ltdcrst())
            .field("dcmipprst", &self.dcmipprst())
            .field("gfxtimrst", &self.gfxtimrst())
            .field("vencrst", &self.vencrst())
            .field("csirst", &self.csirst())
            .finish()
    }
}
impl W {
    ///Bit 1 - LTDC reset
    #[inline(always)]
    pub fn ltdcrst(&mut self) -> LTDCRST_W<'_, APB5RSTRrs> {
        LTDCRST_W::new(self, 1)
    }
    ///Bit 2 - DCMIPP reset
    #[inline(always)]
    pub fn dcmipprst(&mut self) -> DCMIPPRST_W<'_, APB5RSTRrs> {
        DCMIPPRST_W::new(self, 2)
    }
    ///Bit 4 - GFXTIM reset
    #[inline(always)]
    pub fn gfxtimrst(&mut self) -> GFXTIMRST_W<'_, APB5RSTRrs> {
        GFXTIMRST_W::new(self, 4)
    }
    ///Bit 5 - VENC reset
    #[inline(always)]
    pub fn vencrst(&mut self) -> VENCRST_W<'_, APB5RSTRrs> {
        VENCRST_W::new(self, 5)
    }
    ///Bit 6 - CSI reset
    #[inline(always)]
    pub fn csirst(&mut self) -> CSIRST_W<'_, APB5RSTRrs> {
        CSIRST_W::new(self, 6)
    }
}
/**RCC APB5 reset register

You can [`read`](crate::Reg::read) this register and get [`apb5rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:APB5RSTR)*/
pub struct APB5RSTRrs;
impl crate::RegisterSpec for APB5RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb5rstr::R`](R) reader structure
impl crate::Readable for APB5RSTRrs {}
///`write(|w| ..)` method takes [`apb5rstr::W`](W) writer structure
impl crate::Writable for APB5RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB5RSTR to value 0
impl crate::Resettable for APB5RSTRrs {}
