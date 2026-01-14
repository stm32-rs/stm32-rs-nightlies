///Register `L1PCR` reader
pub type R = crate::R<L1PCRrs>;
///Register `L1PCR` writer
pub type W = crate::W<L1PCRrs>;
///Field `YCEN` reader - YCbCr-to-RGB conversion enable
pub type YCEN_R = crate::BitReader;
///Field `YCEN` writer - YCbCr-to-RGB conversion enable
pub type YCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `YCM` reader - YCbCr conversion mode
pub type YCM_R = crate::FieldReader;
///Field `YCM` writer - YCbCr conversion mode
pub type YCM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `YF` reader - Y component first
pub type YF_R = crate::BitReader;
///Field `YF` writer - Y component first
pub type YF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CBF` reader - Cb component first
pub type CBF_R = crate::BitReader;
///Field `CBF` writer - Cb component first
pub type CBF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OF` reader - Odd pixel first
pub type OF_R = crate::BitReader;
///Field `OF` writer - Odd pixel first
pub type OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `YREN` reader - Y rescale enable for the color dynamic range
pub type YREN_R = crate::BitReader;
///Field `YREN` writer - Y rescale enable for the color dynamic range
pub type YREN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - YCbCr-to-RGB conversion enable
    #[inline(always)]
    pub fn ycen(&self) -> YCEN_R {
        YCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - YCbCr conversion mode
    #[inline(always)]
    pub fn ycm(&self) -> YCM_R {
        YCM_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Y component first
    #[inline(always)]
    pub fn yf(&self) -> YF_R {
        YF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Cb component first
    #[inline(always)]
    pub fn cbf(&self) -> CBF_R {
        CBF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Odd pixel first
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Y rescale enable for the color dynamic range
    #[inline(always)]
    pub fn yren(&self) -> YREN_R {
        YREN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1PCR")
            .field("ycen", &self.ycen())
            .field("ycm", &self.ycm())
            .field("yf", &self.yf())
            .field("cbf", &self.cbf())
            .field("of", &self.of())
            .field("yren", &self.yren())
            .finish()
    }
}
impl W {
    ///Bit 3 - YCbCr-to-RGB conversion enable
    #[inline(always)]
    pub fn ycen(&mut self) -> YCEN_W<'_, L1PCRrs> {
        YCEN_W::new(self, 3)
    }
    ///Bits 4:5 - YCbCr conversion mode
    #[inline(always)]
    pub fn ycm(&mut self) -> YCM_W<'_, L1PCRrs> {
        YCM_W::new(self, 4)
    }
    ///Bit 6 - Y component first
    #[inline(always)]
    pub fn yf(&mut self) -> YF_W<'_, L1PCRrs> {
        YF_W::new(self, 6)
    }
    ///Bit 7 - Cb component first
    #[inline(always)]
    pub fn cbf(&mut self) -> CBF_W<'_, L1PCRrs> {
        CBF_W::new(self, 7)
    }
    ///Bit 8 - Odd pixel first
    #[inline(always)]
    pub fn of(&mut self) -> OF_W<'_, L1PCRrs> {
        OF_W::new(self, 8)
    }
    ///Bit 9 - Y rescale enable for the color dynamic range
    #[inline(always)]
    pub fn yren(&mut self) -> YREN_W<'_, L1PCRrs> {
        YREN_W::new(self, 9)
    }
}
/**LTDC layerx planar configuration register

You can [`read`](crate::Reg::read) this register and get [`l1pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#LTDC:L1PCR)*/
pub struct L1PCRrs;
impl crate::RegisterSpec for L1PCRrs {
    type Ux = u32;
}
///`read()` method returns [`l1pcr::R`](R) reader structure
impl crate::Readable for L1PCRrs {}
///`write(|w| ..)` method takes [`l1pcr::W`](W) writer structure
impl crate::Writable for L1PCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L1PCR to value 0
impl crate::Resettable for L1PCRrs {}
