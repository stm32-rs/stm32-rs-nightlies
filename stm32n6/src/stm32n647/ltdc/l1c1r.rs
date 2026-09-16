///Register `L1C1R` reader
pub type R = crate::R<L1C1Rrs>;
///Register `L1C1R` writer
pub type W = crate::W<L1C1Rrs>;
///Field `YIA` writer - YCbCr 422 interleaved ability for that layer
pub type YIA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `YSPA` reader - YCbCr 420 semi-planar ability for that layer
pub type YSPA_R = crate::BitReader;
///Field `YSPA` writer - YCbCr 420 semi-planar ability for that layer
pub type YSPA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `YFPA` reader - YCbCr 420 full-planar ability for that layer
pub type YFPA_R = crate::BitReader;
///Field `YFPA` writer - YCbCr 420 full-planar ability for that layer
pub type YFPA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCA` reader - scaling ability for that layer
pub type SCA_R = crate::BitReader;
///Field `SCA` writer - scaling ability for that layer
pub type SCA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - YCbCr 420 semi-planar ability for that layer
    #[inline(always)]
    pub fn yspa(&self) -> YSPA_R {
        YSPA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - YCbCr 420 full-planar ability for that layer
    #[inline(always)]
    pub fn yfpa(&self) -> YFPA_R {
        YFPA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 31 - scaling ability for that layer
    #[inline(always)]
    pub fn sca(&self) -> SCA_R {
        SCA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1C1R")
            .field("yspa", &self.yspa())
            .field("yfpa", &self.yfpa())
            .field("sca", &self.sca())
            .finish()
    }
}
impl W {
    ///Bit 0 - YCbCr 422 interleaved ability for that layer
    #[inline(always)]
    pub fn yia(&mut self) -> YIA_W<'_, L1C1Rrs> {
        YIA_W::new(self, 0)
    }
    ///Bit 1 - YCbCr 420 semi-planar ability for that layer
    #[inline(always)]
    pub fn yspa(&mut self) -> YSPA_W<'_, L1C1Rrs> {
        YSPA_W::new(self, 1)
    }
    ///Bit 2 - YCbCr 420 full-planar ability for that layer
    #[inline(always)]
    pub fn yfpa(&mut self) -> YFPA_W<'_, L1C1Rrs> {
        YFPA_W::new(self, 2)
    }
    ///Bit 31 - scaling ability for that layer
    #[inline(always)]
    pub fn sca(&mut self) -> SCA_W<'_, L1C1Rrs> {
        SCA_W::new(self, 31)
    }
}
/**LTDC layerx configuration 1 register

You can [`read`](crate::Reg::read) this register and get [`l1c1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1c1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#LTDC:L1C1R)*/
pub struct L1C1Rrs;
impl crate::RegisterSpec for L1C1Rrs {
    type Ux = u32;
}
///`read()` method returns [`l1c1r::R`](R) reader structure
impl crate::Readable for L1C1Rrs {}
///`write(|w| ..)` method takes [`l1c1r::W`](W) writer structure
impl crate::Writable for L1C1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L1C1R to value 0x8000_0007
impl crate::Resettable for L1C1Rrs {
    const RESET_VALUE: u32 = 0x8000_0007;
}
