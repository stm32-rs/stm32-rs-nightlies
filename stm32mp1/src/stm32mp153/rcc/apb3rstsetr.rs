///Register `APB3RSTSETR` reader
pub type R = crate::R<APB3RSTSETRrs>;
///Register `APB3RSTSETR` writer
pub type W = crate::W<APB3RSTSETRrs>;
///Field `LPTIM2RST` reader - LPTIM2RST
pub type LPTIM2RST_R = crate::BitReader;
///Field `LPTIM2RST` writer - LPTIM2RST
pub type LPTIM2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3RST` reader - LPTIM3RST
pub type LPTIM3RST_R = crate::BitReader;
///Field `LPTIM3RST` writer - LPTIM3RST
pub type LPTIM3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM4RST` reader - LPTIM4RST
pub type LPTIM4RST_R = crate::BitReader;
///Field `LPTIM4RST` writer - LPTIM4RST
pub type LPTIM4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM5RST` reader - LPTIM5RST
pub type LPTIM5RST_R = crate::BitReader;
///Field `LPTIM5RST` writer - LPTIM5RST
pub type LPTIM5RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI4RST` reader - SAI4RST
pub type SAI4RST_R = crate::BitReader;
///Field `SAI4RST` writer - SAI4RST
pub type SAI4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCFGRST` reader - SYSCFGRST
pub type SYSCFGRST_R = crate::BitReader;
///Field `SYSCFGRST` writer - SYSCFGRST
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFRST` reader - VREFRST
pub type VREFRST_R = crate::BitReader;
///Field `VREFRST` writer - VREFRST
pub type VREFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTSRST` reader - DTSRST
pub type DTSRST_R = crate::BitReader;
///Field `DTSRST` writer - DTSRST
pub type DTSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LPTIM2RST
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPTIM3RST
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LPTIM4RST
    #[inline(always)]
    pub fn lptim4rst(&self) -> LPTIM4RST_R {
        LPTIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LPTIM5RST
    #[inline(always)]
    pub fn lptim5rst(&self) -> LPTIM5RST_R {
        LPTIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - SAI4RST
    #[inline(always)]
    pub fn sai4rst(&self) -> SAI4RST_R {
        SAI4RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - SYSCFGRST
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - VREFRST
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - DTSRST
    #[inline(always)]
    pub fn dtsrst(&self) -> DTSRST_R {
        DTSRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB3RSTSETR")
            .field("lptim2rst", &self.lptim2rst())
            .field("lptim3rst", &self.lptim3rst())
            .field("lptim4rst", &self.lptim4rst())
            .field("lptim5rst", &self.lptim5rst())
            .field("sai4rst", &self.sai4rst())
            .field("syscfgrst", &self.syscfgrst())
            .field("vrefrst", &self.vrefrst())
            .field("dtsrst", &self.dtsrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPTIM2RST
    #[inline(always)]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<'_, APB3RSTSETRrs> {
        LPTIM2RST_W::new(self, 0)
    }
    ///Bit 1 - LPTIM3RST
    #[inline(always)]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W<'_, APB3RSTSETRrs> {
        LPTIM3RST_W::new(self, 1)
    }
    ///Bit 2 - LPTIM4RST
    #[inline(always)]
    pub fn lptim4rst(&mut self) -> LPTIM4RST_W<'_, APB3RSTSETRrs> {
        LPTIM4RST_W::new(self, 2)
    }
    ///Bit 3 - LPTIM5RST
    #[inline(always)]
    pub fn lptim5rst(&mut self) -> LPTIM5RST_W<'_, APB3RSTSETRrs> {
        LPTIM5RST_W::new(self, 3)
    }
    ///Bit 8 - SAI4RST
    #[inline(always)]
    pub fn sai4rst(&mut self) -> SAI4RST_W<'_, APB3RSTSETRrs> {
        SAI4RST_W::new(self, 8)
    }
    ///Bit 11 - SYSCFGRST
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<'_, APB3RSTSETRrs> {
        SYSCFGRST_W::new(self, 11)
    }
    ///Bit 13 - VREFRST
    #[inline(always)]
    pub fn vrefrst(&mut self) -> VREFRST_W<'_, APB3RSTSETRrs> {
        VREFRST_W::new(self, 13)
    }
    ///Bit 16 - DTSRST
    #[inline(always)]
    pub fn dtsrst(&mut self) -> DTSRST_W<'_, APB3RSTSETRrs> {
        DTSRST_W::new(self, 16)
    }
}
/**This register is used to activate the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`apb3rstsetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3rstsetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB3RSTSETR)*/
pub struct APB3RSTSETRrs;
impl crate::RegisterSpec for APB3RSTSETRrs {
    type Ux = u32;
}
///`read()` method returns [`apb3rstsetr::R`](R) reader structure
impl crate::Readable for APB3RSTSETRrs {}
///`write(|w| ..)` method takes [`apb3rstsetr::W`](W) writer structure
impl crate::Writable for APB3RSTSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3RSTSETR to value 0
impl crate::Resettable for APB3RSTSETRrs {}
