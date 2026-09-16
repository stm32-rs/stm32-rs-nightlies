///Register `PLL2DIVR` reader
pub type R = crate::R<PLL2DIVRrs>;
///Register `PLL2DIVR` writer
pub type W = crate::W<PLL2DIVRrs>;
///Field `DIVN2` reader - Multiplication factor for PLL1 VCO
pub type DIVN2_R = crate::FieldReader<u16>;
///Field `DIVN2` writer - Multiplication factor for PLL1 VCO
pub type DIVN2_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DIVP2` reader - PLL1 DIVP division factor
pub type DIVP2_R = crate::FieldReader;
///Field `DIVP2` writer - PLL1 DIVP division factor
pub type DIVP2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `DIVQ2` reader - PLL1 DIVQ division factor
pub type DIVQ2_R = crate::FieldReader;
///Field `DIVQ2` writer - PLL1 DIVQ division factor
pub type DIVQ2_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
///Field `DIVR2` reader - PLL1 DIVR division factor
pub type DIVR2_R = crate::FieldReader;
///Field `DIVR2` writer - PLL1 DIVR division factor
pub type DIVR2_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
impl R {
    ///Bits 0:8 - Multiplication factor for PLL1 VCO
    #[inline(always)]
    pub fn divn2(&self) -> DIVN2_R {
        DIVN2_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:15 - PLL1 DIVP division factor
    #[inline(always)]
    pub fn divp2(&self) -> DIVP2_R {
        DIVP2_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    ///Bits 16:22 - PLL1 DIVQ division factor
    #[inline(always)]
    pub fn divq2(&self) -> DIVQ2_R {
        DIVQ2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:30 - PLL1 DIVR division factor
    #[inline(always)]
    pub fn divr2(&self) -> DIVR2_R {
        DIVR2_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL2DIVR")
            .field("divn2", &self.divn2())
            .field("divp2", &self.divp2())
            .field("divq2", &self.divq2())
            .field("divr2", &self.divr2())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Multiplication factor for PLL1 VCO
    #[inline(always)]
    pub fn divn2(&mut self) -> DIVN2_W<'_, PLL2DIVRrs> {
        DIVN2_W::new(self, 0)
    }
    ///Bits 9:15 - PLL1 DIVP division factor
    #[inline(always)]
    pub fn divp2(&mut self) -> DIVP2_W<'_, PLL2DIVRrs> {
        DIVP2_W::new(self, 9)
    }
    ///Bits 16:22 - PLL1 DIVQ division factor
    #[inline(always)]
    pub fn divq2(&mut self) -> DIVQ2_W<'_, PLL2DIVRrs> {
        DIVQ2_W::new(self, 16)
    }
    ///Bits 24:30 - PLL1 DIVR division factor
    #[inline(always)]
    pub fn divr2(&mut self) -> DIVR2_W<'_, PLL2DIVRrs> {
        DIVR2_W::new(self, 24)
    }
}
/**RCC PLL2 Dividers Configuration Register

You can [`read`](crate::Reg::read) this register and get [`pll2divr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2divr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RCC:PLL2DIVR)*/
pub struct PLL2DIVRrs;
impl crate::RegisterSpec for PLL2DIVRrs {
    type Ux = u32;
}
///`read()` method returns [`pll2divr::R`](R) reader structure
impl crate::Readable for PLL2DIVRrs {}
///`write(|w| ..)` method takes [`pll2divr::W`](W) writer structure
impl crate::Writable for PLL2DIVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL2DIVR to value 0x0101_0280
impl crate::Resettable for PLL2DIVRrs {
    const RESET_VALUE: u32 = 0x0101_0280;
}
